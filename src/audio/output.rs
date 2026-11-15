//! Реалтаймовый аудиовывод на базе `cpal` 0.18.
//!
//! Особенности (`AGENT.md`, п. 2.1–2.2):
//! - В `cpal` 0.18 `default_output_config()` по умолчанию выбирает 48 000 Гц, поэтому мы
//!   явно запрашиваем стерео-конфигурацию `f32` с частотой `cpal::SAMPLE_RATE_CD` (44 100 Гц).
//! - Внутри callback-функции нет ни одной блокировки или аллокации памяти: сэмплы читаются
//!   из `rtrb::Consumer<f32>`, умножаются на атомарный коэффициент громкости (`AtomicU32`)
//!   и при нехватке данных (underrun) заполняются нулями (`0.0f32`).

#![allow(dead_code)]

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{OutputCallbackInfo, SampleFormat};
use rtrb::{Consumer, PopError};

/// Стандартная частота дискретизации Deezer FLAC (44 100 Гц).
pub const SAMPLE_RATE_HZ: u32 = 44_100;

/// Битовое представление громкости `1.0f32` для `AtomicU32`.
pub fn default_volume_bits() -> u32 {
    f32::to_bits(1.0)
}

/// Битовое представление тишины `0.0f32` для `AtomicU32`.
pub fn muted_volume_bits() -> u32 {
    f32::to_bits(0.0)
}

/// Заполняет выходной буфер звуковой карты из `rtrb::Consumer<f32>` без блокировок и аллокаций.
/// При опустошении кольцевого буфера оставшиеся сэмплы заполняются тишиной (`0.0`).
#[inline]
pub fn fill_buffer_from_consumer(
    buffer: &mut [f32],
    consumer: &mut Consumer<f32>,
    volume: &AtomicU32,
) -> usize {
    let vol = f32::from_bits(volume.load(Ordering::Relaxed)).clamp(0.0, 1.0);
    let mut filled = 0;
    for slot in buffer.iter_mut() {
        match consumer.pop() {
            Ok(sample) => {
                *slot = sample * vol;
                filled += 1;
            }
            Err(PopError::Empty) => {
                *slot = 0.0;
            }
        }
    }
    filled
}

/// Управляет потоком вывода `cpal::Stream` и атомарным регулятором громкости.
pub struct AudioOutput {
    stream: cpal::Stream,
    volume: Arc<AtomicU32>,
    sample_rate: u32,
}

impl AudioOutput {
    /// Открывает аудиопоток (предпочтительно 44 100 Гц, 2 канала, `f32`), перемещает
    /// `consumer` внутрь real-time callback и запускает воспроизведение.
    pub fn new(consumer: Consumer<f32>, initial_volume: f32) -> Result<Self> {
        let clamped_vol = initial_volume.clamp(0.0, 1.0);
        let volume = Arc::new(AtomicU32::new(f32::to_bits(clamped_vol)));

        let host = cpal::Host::default();
        let device = host
            .default_output_device()
            .context("аудиоустройство вывода по умолчанию не найдено")?;

        // Ищем стерео F32 конфигурацию с частотой 44 100 Гц (SAMPLE_RATE_CD)
        let config: cpal::StreamConfig = device
            .supported_output_configs()
            .ok()
            .and_then(|configs| {
                configs
                    .filter(|r| r.channels() == 2 && r.sample_format() == SampleFormat::F32)
                    .find_map(|r| r.try_with_sample_rate(cpal::SAMPLE_RATE_CD))
            })
            .map(Into::into)
            .or_else(|| device.default_output_config().ok().map(Into::into))
            .context("не удалось подобрать конфигурацию аудиопотока")?;

        let actual_sample_rate = config.sample_rate;
        tracing::info!(
            sample_rate = actual_sample_rate,
            channels = config.channels,
            "аудиопоток CPAL сконфигурирован"
        );

        let volume_for_cb = Arc::clone(&volume);
        let mut consumer_for_cb = consumer;

        let error_callback = move |err: cpal::Error| {
            tracing::warn!(target: "audio_output", "ошибка потока CPAL: {err:?}");
        };

        let stream = device
            .build_output_stream(
                config,
                move |buffer: &mut [f32], _info: &OutputCallbackInfo| {
                    fill_buffer_from_consumer(buffer, &mut consumer_for_cb, &volume_for_cb);
                },
                error_callback,
                None,
            )
            .context("не удалось создать выходной поток CPAL")?;

        stream
            .play()
            .context("не удалось запустить воспроизведение потока CPAL")?;

        Ok(Self {
            stream,
            volume,
            sample_rate: actual_sample_rate,
        })
    }

    /// Возвращает ссылку на атомарный регулятор громкости.
    pub fn volume_atomic(&self) -> &Arc<AtomicU32> {
        &self.volume
    }

    /// Устанавливает громкость в диапазоне `0.0..=1.0` атомарно без блокировок.
    pub fn set_volume(&self, vol: f32) {
        let clamped = vol.clamp(0.0, 1.0);
        self.volume.store(f32::to_bits(clamped), Ordering::Relaxed);
    }

    /// Возвращает текущее значение громкости `0.0..=1.0`.
    pub fn get_volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Relaxed))
    }

    /// Возвращает фактическую частоту дискретизации открытого потока (Гц).
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::decoder::write_interleaved_stereo_to_rings;
    use crate::audio::ring_buffers::{AudioRing, VisRing};
    use rtrb::RingBuffer;

    fn consumer_with(samples: &[f32]) -> Consumer<f32> {
        let (mut producer, consumer) = RingBuffer::<f32>::new(samples.len().max(1));
        for &s in samples {
            producer.push(s).unwrap();
        }
        consumer
    }

    #[test]
    fn fill_buffer_applies_volume_and_silence_on_empty() {
        let mut consumer = consumer_with(&[0.5, -0.5, 0.25, 1.0, 0.0, -1.0]);
        let mut buffer = [0.0f32; 6];
        let volume = AtomicU32::new(default_volume_bits());

        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 6);
        assert!((buffer[0] - 0.5).abs() < 1e-6);
        assert!((buffer[1] + 0.5).abs() < 1e-6);
        assert!((buffer[2] - 0.25).abs() < 1e-6);
        assert!((buffer[3] - 1.0).abs() < 1e-6);
        assert!((buffer[4] - 0.0).abs() < 1e-6);
        assert!((buffer[5] + 1.0).abs() < 1e-6);
    }

    #[test]
    fn fill_buffer_halves_with_half_gain() {
        let mut consumer = consumer_with(&[1.0, -1.0, 0.4]);
        let mut buffer = [0.0f32; 3];
        let volume = AtomicU32::new(f32::to_bits(0.5));

        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 3);
        assert!((buffer[0] - 0.5).abs() < 1e-6);
        assert!((buffer[1] + 0.5).abs() < 1e-6);
        assert!((buffer[2] - 0.2).abs() < 1e-6);
    }

    #[test]
    fn fill_buffer_underrun_fills_silence_without_blocking() {
        let mut consumer = consumer_with(&[0.3, 0.7]);
        let mut buffer = [1.0f32; 6];
        let volume = AtomicU32::new(default_volume_bits());

        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 2);
        assert!((buffer[0] - 0.3).abs() < 1e-6);
        assert!((buffer[1] - 0.7).abs() < 1e-6);
        assert_eq!(&buffer[2..], &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn end_to_end_rtrb_to_output_buffer() {
        let frames = 1_000usize;
        let mut stereo = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let s = (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 44_100.0).sin();
            stereo.push(s);
            stereo.push(s);
        }

        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();
        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        let mut output_buf = vec![0.0f32; stereo.len()];
        let volume = AtomicU32::new(default_volume_bits());
        let filled = fill_buffer_from_consumer(&mut output_buf, audio.consumer(), &volume);

        assert_eq!(filled, stereo.len());
        assert_eq!(output_buf, stereo);
    }
}
