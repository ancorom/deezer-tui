//! Реалтаймовый аудиовывод на базе `cpal` 0.18 с синхронизацией спектра и мгновенной паузой.
//!
//! Особенности:
//! - В `cpal` 0.18 `default_output_config()` по умолчанию выбирает 48 000 Гц, поэтому мы
//!   явно запрашиваем стерео-конфигурацию `f32` с частотой `cpal::SAMPLE_RATE_CD` (44 100 Гц).
//! - Внутри callback-функции нет ни одной блокировки или аллокации памяти:
//!   - если `paused == true`, выходной буфер заполняется `0.0f32` без чтения `AudioRingBuffer`;
//!   - если `paused == false`, стерео-пары `(L, R)` читаются из `rtrb::Consumer<f32>`,
//!     умножаются на громкость (`AtomicU32`), моно-сэмпл `(L + R) * 0.5` пушится в
//!     `VisRingBuffer`, а счётчик `played_frames` (`AtomicU64`) увеличивается.

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{OutputCallbackInfo, SampleFormat};
use rtrb::{Consumer, PopError, Producer};

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

/// Базовое заполнение буфера из `Consumer<f32>` (сохранено для совместимости с unit-тестами).
#[inline]
pub fn fill_buffer_from_consumer(
    buffer: &mut [f32],
    consumer: &mut Consumer<f32>,
    volume: &AtomicU32,
) -> usize {
    let paused = AtomicBool::new(false);
    let played_frames = AtomicU64::new(0);
    fill_buffer_synced(buffer, consumer, None, volume, &paused, &played_frames)
}

/// Синхронизированное заполнение буфера звуковой карты:
/// - При `paused == true` пишет `0.0` в `buffer`, не читая `consumer`.
 /// - При `paused == false` читает стерео-сэмплы, пишет моно `(L + R) * 0.5` в `vis_producer` 
///   и инкрементирует `played_frames`.
#[inline]
pub fn fill_buffer_synced(
    buffer: &mut [f32],
    consumer: &mut Consumer<f32>,
    mut vis_producer: Option<&mut Producer<f32>>,
    volume: &AtomicU32,
    paused: &AtomicBool,
    played_frames: &AtomicU64,
) -> usize {
    if paused.load(Ordering::Relaxed) {
        buffer.fill(0.0);
        return 0;
    }

    let vol = f32::from_bits(volume.load(Ordering::Relaxed)).clamp(0.0, 1.0);
    let mut filled = 0usize;
    let mut frames_played = 0u64;

    let (chunks, rem) = buffer.as_chunks_mut::<2>();
    for frame in chunks.iter_mut() {
        match (consumer.pop(), consumer.pop()) {
            (Ok(l), Ok(r)) => {
                frame[0] = l * vol;
                frame[1] = r * vol;
                filled += 2;
                frames_played += 1;
                if let Some(vp) = vis_producer.as_mut() {
                    let _ = vp.push((l + r) * 0.5);
                }
            }
            (Ok(l), Err(PopError::Empty)) => {
                frame[0] = l * vol;
                frame[1] = 0.0;
                filled += 1;
                frames_played += 1;
                if let Some(vp) = vis_producer.as_mut() {
                    let _ = vp.push(l * 0.5);
                }
            }
            _ => {
                frame[0] = 0.0;
                frame[1] = 0.0;
            }
        }
    }

    if let Some(last_slot) = rem.first_mut() {
        match consumer.pop() {
            Ok(s) => {
                *last_slot = s * vol;
                filled += 1;
            }
            Err(PopError::Empty) => {
                *last_slot = 0.0;
            }
        }
    }

    if frames_played > 0 {
        played_frames.fetch_add(frames_played, Ordering::Relaxed);
    }

    filled
}

/// Управляет потоком вывода `cpal::Stream`, мгновенной паузой и счётчиком сыгранных фреймов.
pub struct AudioOutput {
    stream: cpal::Stream,
    volume: Arc<AtomicU32>,
    paused: Arc<AtomicBool>,
    played_frames: Arc<AtomicU64>,
    sample_rate: u32,
}

impl AudioOutput {
    /// Открывает аудиопоток с базовыми параметрами (без внешнего `vis_producer`).
    pub fn new(consumer: Consumer<f32>, initial_volume: f32) -> Result<Self> {
        Self::new_with_sync(
            consumer,
            None,
            initial_volume,
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicU64::new(0)),
        )
    }

    /// Открывает аудиопоток (предпочтительно 44 100 Гц, 2 канала, `f32`) и связывает
    /// real-time коллбэк `CPAL` с `paused`, `played_frames` и `vis_producer`.
    pub fn new_with_sync(
        consumer: Consumer<f32>,
        vis_producer: Option<Producer<f32>>,
        initial_volume: f32,
        paused: Arc<AtomicBool>,
        played_frames: Arc<AtomicU64>,
    ) -> Result<Self> {
        let clamped_vol = initial_volume.clamp(0.0, 1.0);
        let volume = Arc::new(AtomicU32::new(f32::to_bits(clamped_vol)));

        let host = cpal::Host::default();
        let device = host
            .default_output_device()
            .context("аудиоустройство вывода по умолчанию не найдено")?;

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
        let paused_for_cb = Arc::clone(&paused);
        let frames_for_cb = Arc::clone(&played_frames);
        let mut consumer_for_cb = consumer;
        let mut vis_for_cb = vis_producer;

        let error_callback = move |err: cpal::Error| {
            tracing::warn!(target: "audio_output", "ошибка потока CPAL: {err:?}");
        };

        let stream = device
            .build_output_stream(
                config,
                move |buffer: &mut [f32], _info: &OutputCallbackInfo| {
                    fill_buffer_synced(
                        buffer,
                        &mut consumer_for_cb,
                        vis_for_cb.as_mut(),
                        &volume_for_cb,
                        &paused_for_cb,
                        &frames_for_cb,
                    );
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
            paused,
            played_frames,
            sample_rate: actual_sample_rate,
        })
    }

    pub fn volume_atomic(&self) -> &Arc<AtomicU32> {
        &self.volume
    }

    pub fn paused_atomic(&self) -> &Arc<AtomicBool> {
        &self.paused
    }

    pub fn played_frames_atomic(&self) -> &Arc<AtomicU64> {
        &self.played_frames
    }

    pub fn set_volume(&self, vol: f32) {
        let clamped = vol.clamp(0.0, 1.0);
        self.volume.store(f32::to_bits(clamped), Ordering::Relaxed);
    }

    pub fn get_volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Relaxed))
    }

    pub fn set_paused(&self, is_paused: bool) {
        self.paused.store(is_paused, Ordering::SeqCst);
    }

    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    pub fn played_frames(&self) -> u64 {
        self.played_frames.load(Ordering::Relaxed)
    }

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
    fn paused_callback_writes_silence_without_draining_consumer() {
        let mut consumer = consumer_with(&[0.8, 0.6, 0.4, 0.2]);
        let (mut vis_prod, mut vis_cons) = RingBuffer::<f32>::new(8);
        let mut buffer = [1.0f32; 4];
        let volume = AtomicU32::new(default_volume_bits());
        let paused = AtomicBool::new(true);
        let played_frames = AtomicU64::new(0);

        let filled = fill_buffer_synced(
            &mut buffer,
            &mut consumer,
            Some(&mut vis_prod),
            &volume,
            &paused,
            &played_frames,
        );
        assert_eq!(filled, 0);
        assert_eq!(buffer, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(played_frames.load(Ordering::Relaxed), 0);
        assert!(vis_cons.pop().is_err());

        // Снимаем паузу — сэмплы читаются, попадают в vis_producer и увеличивают played_frames
        paused.store(false, Ordering::Relaxed);
        let filled2 = fill_buffer_synced(
            &mut buffer,
            &mut consumer,
            Some(&mut vis_prod),
            &volume,
            &paused,
            &played_frames,
        );
        assert_eq!(filled2, 4);
        assert_eq!(played_frames.load(Ordering::Relaxed), 2);
        assert!((vis_cons.pop().unwrap() - 0.7).abs() < 1e-6);
        assert!((vis_cons.pop().unwrap() - 0.3).abs() < 1e-6);
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
