//! Декодер аудиопотока на базе `symphonia` 0.6 с разветвлением в два lock-free буфера `rtrb`.
//!
//! Записывает интерливированные стерео-сэмплы в `AudioRingBuffer` (65 536 сэмплов)
//! и сведённые моно-сэмплы `(L + R) * 0.5` в `VisRingBuffer` (16 384 сэмпла).

#![allow(dead_code)]

use std::io::Read;

use anyhow::{Context, Result};
use rtrb::Producer;
use symphonia::core::codecs::audio::{AudioDecoder, AudioDecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, FormatReader, TrackType};
use symphonia::core::io::{MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;

/// Записывает все сэмплы `src` в `dst`, уступая квант времени потоку-потребителю при заполнении.
fn push_all_blocking(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    let mut remaining = src;
    let mut written = 0;
    while !remaining.is_empty() {
        if dst.is_full() {
            std::thread::yield_now();
            continue;
        }
        let (pushed, rest) = dst.push_partial_slice(remaining);
        written += pushed.len();
        remaining = rest;
    }
    written
}

/// Неблокирующая запись в `VisRingBuffer`: записывает столько моно-сэмплов, сколько помещается
/// прямо сейчас, чтобы визуализация никогда не блокировала декодирование звука.
fn push_vis_nonblocking(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    if dst.is_full() || src.is_empty() {
        return 0;
    }
    let (pushed, _) = dst.push_partial_slice(src);
    pushed.len()
}

/// Записывает чанк интерливированного стерео `[L0, R0, L1, R1, ...]` в оба кольцевых буфера:
/// - стерео целиком в `audio`;
/// - сведённое моно `(L + R) * 0.5` в `vis`.
pub fn write_interleaved_stereo_to_rings(
    stereo: &[f32],
    audio: &mut Producer<f32>,
    vis: &mut Producer<f32>,
) -> (usize, usize) {
    let frames = stereo.len() / 2;
    let audio_written = push_all_blocking(audio, stereo);

    let mut mono = Vec::with_capacity(frames);
    for i in 0..frames {
        mono.push((stereo[2 * i] + stereo[2 * i + 1]) * 0.5);
    }
    let vis_written = push_vis_nonblocking(vis, &mono);

    (audio_written, vis_written)
}

/// Декодер `symphonia` 0.6, подключённый к двум `rtrb::Producer<f32>`.
pub struct Decoder {
    format: Box<dyn FormatReader>,
    track_id: u32,
    decoder: Box<dyn AudioDecoder>,
    audio_producer: Producer<f32>,
    vis_producer: Producer<f32>,
    interleaved_buf: Vec<f32>,
}

impl Decoder {
    /// Инициализирует декодер для произвольного потока чтения `reader` и пары `Producer<f32>`.
    /// Вызывающая сторона сохраняет оба `Consumer<f32>` (для потока `cpal` и потока UI/DSP).
    pub fn new(
        reader: Box<dyn Read + Send + Sync>,
        audio_producer: Producer<f32>,
        vis_producer: Producer<f32>,
    ) -> Result<Self> {
        let mss = MediaSourceStream::new(
            Box::new(ReadOnlySource::new(reader)),
            Default::default(),
        );
        let format = symphonia::default::get_probe()
            .probe(
                &Hint::new(),
                mss,
                FormatOptions::default(),
                MetadataOptions::default(),
            )
            .context("не удалось распознать аудиоформат потока")?;

        let track = format
            .default_track(TrackType::Audio)
            .context("в медиапотоке не найдено аудиодорожек")?;
        let track_id = track.id;

        let audio_params = track
            .codec_params
            .as_ref()
            .context("отсутствуют параметры кодека")?
            .audio()
            .context("дорожка не является аудиокодеком")?;

        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(audio_params, &AudioDecoderOptions::default())
            .context("неподдерживаемый аудиокодек")?;

        Ok(Self {
            format,
            track_id,
            decoder,
            audio_producer,
            vis_producer,
            interleaved_buf: Vec::with_capacity(8192),
        })
    }

    /// Декодирует следующий пакет и записывает сэмплы в оба кольцевых буфера.
    /// Возвращает `Ok(Some(frames))` или `Ok(None)` при достижении конца потока (EOF).
    pub fn process_next(&mut self) -> Result<Option<usize>, SymphoniaError> {
        loop {
            let Some(pkt) = self.format.next_packet()? else {
                return Ok(None);
            };
            if pkt.track_id != self.track_id {
                continue;
            }

            let decoded = self.decoder.decode(&pkt)?;
            let channels = decoded.spec().channels().count();
            let total_samples = decoded.samples_interleaved();

            self.interleaved_buf.resize(total_samples, 0.0);
            decoded.copy_to_slice_interleaved(&mut self.interleaved_buf);

            if channels == 1 {
                // Если поток монофонический — дублируем каждый сэмпл в L и R
                let frames = total_samples;
                let mut stereo = Vec::with_capacity(frames * 2);
                for &s in &self.interleaved_buf[..frames] {
                    stereo.push(s);
                    stereo.push(s);
                }
                write_interleaved_stereo_to_rings(
                    &stereo,
                    &mut self.audio_producer,
                    &mut self.vis_producer,
                );
                return Ok(Some(frames));
            } else {
                let frames = total_samples / 2;
                write_interleaved_stereo_to_rings(
                    &self.interleaved_buf,
                    &mut self.audio_producer,
                    &mut self.vis_producer,
                );
                return Ok(Some(frames));
            }
        }
    }

    /// Декодирует поток до конца (EOF), возвращая общее число декодированных стерео-фреймов.
    pub fn run(&mut self) -> Result<usize, SymphoniaError> {
        let mut total_frames = 0;
        while let Some(frames) = self.process_next()? {
            total_frames += frames;
        }
        Ok(total_frames)
    }

    /// Извлекает оба `Producer<f32>` обратно из декодера.
    pub fn into_producers(self) -> (Producer<f32>, Producer<f32>) {
        (self.audio_producer, self.vis_producer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::ring_buffers::{
        create_rings, AudioRing, VisRing, VIS_RING_BUFFER_CAPACITY,
    };
    use crate::dsp::cavacore::{Cavacore, N_BASS};
    use rtrb::{Consumer, PopError};

    fn drain(consumer: &mut Consumer<f32>) -> Vec<f32> {
        let mut out = Vec::new();
        while let Ok(s) = consumer.pop() {
            out.push(s);
        }
        assert!(matches!(consumer.pop(), Err(PopError::Empty)));
        out
    }

    fn stereo_sine(freq_hz: f32, frames: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let s = (2.0 * std::f32::consts::PI * freq_hz * i as f32 / 44_100.0).sin();
            out.push(s);
            out.push(s);
        }
        out
    }

    #[test]
    fn generated_buffer_fills_both_rings_without_blocking() {
        let stereo = stereo_sine(440.0, 1_000);
        let (audio, vis) = create_rings();
        let (mut ap, mut ac) = audio.split();
        let (mut vp, mut vc) = vis.split();

        let (aw, vw) = write_interleaved_stereo_to_rings(&stereo, &mut ap, &mut vp);
        assert_eq!(aw, stereo.len());
        assert_eq!(vw, stereo.len() / 2);

        let audio_drained = drain(&mut ac);
        let vis_drained = drain(&mut vc);

        assert_eq!(audio_drained, stereo);
        assert_eq!(vis_drained.len(), 1_000);
        for i in 0..1_000 {
            let expected = (stereo[2 * i] + stereo[2 * i + 1]) * 0.5;
            assert_eq!(vis_drained[i], expected);
        }
    }

    #[test]
    fn vis_ring_to_cavacore_100hz_activates_low_bars() {
        let stereo = stereo_sine(100.0, N_BASS * 2);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        // Читаем именно из VisRingBuffer (моно), как это делает UI-поток в ARCHITECTURE.md!
        let mono_vis = drain(vis.consumer());
        assert_eq!(mono_vis.len(), N_BASS * 2);

        let mut cava = Cavacore::new(44_100.0, 32);
        // 9 кадров: интегральный фильтр (0.77) сходится к установившемуся уровню.
        let mut bars = cava.process(&mono_vis);
        for _ in 0..8 {
            bars = cava.process(&mono_vis);
        }
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        assert!(low_max > 0.2, "низкие полосы должны активироваться на 100 Гц: {low_max}");
        assert!(
            low_max > high_max * 2.0,
            "НЧ ({low_max}) должны превышать ВЧ ({high_max}) для сигнала 100 Гц"
        );
    }

    #[test]
    fn vis_ring_to_cavacore_5000hz_activates_high_bars() {
        let stereo = stereo_sine(5_000.0, N_BASS * 2);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        let mono_vis = drain(vis.consumer());
        let mut cava = Cavacore::new(44_100.0, 32);
        // 9 кадров: интегральный фильтр (0.77) сходится к установившемуся уровню.
        let mut bars = cava.process(&mono_vis);
        for _ in 0..8 {
            bars = cava.process(&mono_vis);
        }
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        // Для 32 полос (широкий верхний диапазон) установившийся уровень ВЧ ~0.17-0.19;
        // порог ниже, чем у 64-полосного теста в cavacore, но явно выше нуля.
        assert!(high_max > 0.15, "верхние полосы должны активироваться на 5000 Гц: {high_max}");
        assert!(
            high_max > low_max * 2.0,
            "ВЧ ({high_max}) должны превышать НЧ ({low_max}) для сигнала 5000 Гц"
        );
    }

    #[test]
    fn vis_ring_overflow_never_deadlocks_audio_ring() {
        // Подаём больше фреймов (20 000), чем ёмкость VisRingBuffer (16 384),
        // но меньше, чем ёмкость AudioRingBuffer (32 768 фреймов).
        // Запись не должна блокироваться!
        let frames = 20_000;
        let stereo = vec![0.0f32; frames * 2];
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        let (aw, vw) = write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());
        assert_eq!(aw, frames * 2);
        assert_eq!(vw, VIS_RING_BUFFER_CAPACITY);

        let drained_v = drain(vis.consumer());
        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&drained_v);
        assert!(bars.iter().all(|&b| b == 0.0));
    }
}
