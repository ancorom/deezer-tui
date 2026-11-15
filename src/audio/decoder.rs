//! Декодер аудиопотока на базе `symphonia` 0.6 с поддержкой аппаратного `Seek`,
//! мгновенной отмены без утечки CPU-потоков и извлечением `StreamTechInfo`.

#![allow(dead_code)]

use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use rtrb::Producer;
use symphonia::core::codecs::audio::well_known::{CODEC_ID_AAC, CODEC_ID_FLAC, CODEC_ID_MP3};
use symphonia::core::codecs::audio::{AudioCodecId, AudioDecoder, AudioDecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo, TrackType};
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::units::{Time, Timestamp};

/// Команды управления потоком декодера из UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderCommand {
    /// Перемотка потока на указанную секунду.
    Seek(u64),
}

/// Реальные технические параметры аудиопотока, извлечённые из заголовков `Symphonia`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamTechInfo {
    pub codec_name: String,
    pub sample_rate: u32,
    pub bits_per_sample: u32,
    pub total_frames: u64,
    pub duration_secs: u64,
    pub bitrate_kbps: u32,
    pub file_size_bytes: u64,
}

impl Default for StreamTechInfo {
    fn default() -> Self {
        Self {
            codec_name: "FLAC".into(),
            sample_rate: 44_100,
            bits_per_sample: 16,
            total_frames: 0,
            duration_secs: 30,
            bitrate_kbps: 1411,
            file_size_bytes: 0,
        }
    }
}

impl StreamTechInfo {
    /// Форматирует бейдж качества для нижней панели плеера и экрана Now Playing.
    pub fn badge(&self, is_preview: bool) -> String {
        let khz = self.sample_rate as f32 / 1000.0;
        if is_preview {
            format!(
                "{} {:.1}kHz | {} kbps (PREVIEW)",
                self.codec_name, khz, self.bitrate_kbps
            )
        } else if self.codec_name == "FLAC" {
            format!(
                "FLAC {}-bit/{:.1}kHz | {} kbps",
                self.bits_per_sample, khz, self.bitrate_kbps
            )
        } else {
            format!(
                "{} {:.1}kHz | {} kbps",
                self.codec_name, khz, self.bitrate_kbps
            )
        }
    }
}

fn codec_display_name(codec: AudioCodecId) -> String {
    if codec == CODEC_ID_FLAC {
        "FLAC".to_string()
    } else if codec == CODEC_ID_MP3 {
        "MP3".to_string()
    } else if codec == CODEC_ID_AAC {
        "AAC".to_string()
    } else {
        "PCM".to_string()
    }
}

/// Записывает сэмплы `src` в `dst` с проверкой `stop_flag` и `dst.is_abandoned()`.
/// Когда буфер полон, спит 5 мс вместо busy-spin `yield_now()`, полностью устраняя
/// нагрузку на CPU и зомби-потоки при переключении треков.
pub fn push_all_cancelable(
    dst: &mut Producer<f32>,
    src: &[f32],
    stop_flag: &AtomicBool,
) -> usize {
    let mut remaining = src;
    let mut written = 0;
    while !remaining.is_empty() {
        if stop_flag.load(Ordering::Relaxed) || dst.is_abandoned() {
            break;
        }
        if dst.is_full() {
            std::thread::sleep(Duration::from_millis(5));
            continue;
        }
        let (pushed, rest) = dst.push_partial_slice(remaining);
        written += pushed.len();
        remaining = rest;
    }
    written
}

/// Обёртка для обратной совместимости с тестами.
pub fn push_all_blocking(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    let never_stop = AtomicBool::new(false);
    push_all_cancelable(dst, src, &never_stop)
}

/// Неблокирующая запись в `VisRingBuffer`.
fn push_vis_nonblocking(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    if dst.is_full() || src.is_empty() {
        return 0;
    }
    let (pushed, _) = dst.push_partial_slice(src);
    pushed.len()
}

/// Записывает чанк интерливированного стерео `[L0, R0, L1, R1, ...]` в оба кольцевых буфера.
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

/// Декодер `symphonia` 0.6.
pub struct Decoder {
    format: Box<dyn FormatReader>,
    track_id: u32,
    sample_rate: u32,
    decoder: Box<dyn AudioDecoder>,
    pub audio_producer: Producer<f32>,
    vis_producer: Option<Producer<f32>>,
    stop_flag: Arc<AtomicBool>,
    interleaved_buf: Vec<f32>,
}

impl Decoder {
    /// Инициализирует декодер для произвольного `Read` и пары `Producer<f32>` (для совместимости с тестами).
    pub fn new(
        reader: Box<dyn Read + Send + Sync>,
        audio_producer: Producer<f32>,
        vis_producer: Producer<f32>,
    ) -> Result<Self> {
        let mss = MediaSourceStream::new(
            Box::new(ReadOnlySource::new(reader)),
            Default::default(),
        );
        let (mut dec, _) = Self::from_mss(
            mss,
            0,
            audio_producer,
            30,
            Arc::new(AtomicBool::new(false)),
        )?;
        dec.vis_producer = Some(vis_producer);
        Ok(dec)
    }

    /// Инициализирует декодер для `MediaSource` с поддержкой `Seek`.
    pub fn new_seekable(
        source: Box<dyn MediaSource>,
        audio_producer: Producer<f32>,
        fallback_duration_secs: u64,
    ) -> Result<(Self, StreamTechInfo)> {
        Self::new_seekable_with_stop(
            source,
            audio_producer,
            fallback_duration_secs,
            Arc::new(AtomicBool::new(false)),
        )
    }

    /// Инициализирует декодер с поддержкой `Seek` и общим атомарным флагом остановки `stop_flag`.
    pub fn new_seekable_with_stop(
        source: Box<dyn MediaSource>,
        audio_producer: Producer<f32>,
        fallback_duration_secs: u64,
        stop_flag: Arc<AtomicBool>,
    ) -> Result<(Self, StreamTechInfo)> {
        let file_size_bytes = source.byte_len().unwrap_or(0);
        let mss = MediaSourceStream::new(source, Default::default());
        Self::from_mss(
            mss,
            file_size_bytes,
            audio_producer,
            fallback_duration_secs,
            stop_flag,
        )
    }

    fn from_mss(
        mss: MediaSourceStream<'static>,
        file_size_bytes: u64,
        audio_producer: Producer<f32>,
        fallback_duration_secs: u64,
        stop_flag: Arc<AtomicBool>,
    ) -> Result<(Self, StreamTechInfo)> {
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
        let total_frames = track.num_frames.unwrap_or(0);

        let audio_params = track
            .codec_params
            .as_ref()
            .context("отсутствуют параметры кодека")?
            .audio()
            .context("дорожка не является аудиокодеком")?;

        let sample_rate = audio_params.sample_rate.unwrap_or(44_100).max(1);
        let bits_per_sample = audio_params.bits_per_sample.unwrap_or(16);
        let codec_name = codec_display_name(audio_params.codec);

        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(audio_params, &AudioDecoderOptions::default())
            .context("неподдерживаемый аудиокодек")?;

        let computed_duration = if total_frames > 0 {
            (total_frames / u64::from(sample_rate)).max(1)
        } else {
            fallback_duration_secs.max(1)
        };

        let bitrate_kbps = if file_size_bytes > 0 && computed_duration > 0 {
            ((file_size_bytes * 8) / (computed_duration * 1000)).max(32) as u32
        } else if codec_name == "FLAC" {
            1411
        } else {
            128
        };

        let tech_info = StreamTechInfo {
            codec_name,
            sample_rate,
            bits_per_sample,
            total_frames,
            duration_secs: computed_duration,
            bitrate_kbps,
            file_size_bytes,
        };

        Ok((
            Self {
                format,
                track_id,
                sample_rate,
                decoder,
                audio_producer,
                vis_producer: None,
                stop_flag,
                interleaved_buf: Vec::with_capacity(8192),
            },
            tech_info,
        ))
    }

    /// Выполняет аппаратный `Seek` к целевой секунде и сбрасывает внутреннее состояние декодера.
    pub fn seek(&mut self, target_secs: u64) -> Result<u64, SymphoniaError> {
        let seeked = self.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: Time::from(u32::try_from(target_secs).unwrap_or(u32::MAX)),
                track_id: Some(self.track_id),
            },
        )?;
        self.decoder.reset();
        let actual_frames = if seeked.actual_ts > Timestamp::ZERO {
            seeked.actual_ts.get().max(0) as u64
        } else {
            target_secs.saturating_mul(u64::from(self.sample_rate))
        };
        Ok(actual_frames)
    }

    /// Декодирует следующий пакет и записывает сэмплы в кольцевой буфер.
    /// Возвращает `Ok(Some(frames))` или `Ok(None)` при достижении конца потока или остановке.
    pub fn process_next(&mut self) -> Result<Option<usize>, SymphoniaError> {
        loop {
            if self.stop_flag.load(Ordering::Relaxed) || self.audio_producer.is_abandoned() {
                return Ok(None);
            }
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
                let frames = total_samples;
                let mut stereo = Vec::with_capacity(frames * 2);
                for &s in &self.interleaved_buf[..frames] {
                    stereo.push(s);
                    stereo.push(s);
                }
                if let Some(ref mut vp) = self.vis_producer {
                    write_interleaved_stereo_to_rings(&stereo, &mut self.audio_producer, vp);
                } else {
                    push_all_cancelable(&mut self.audio_producer, &stereo, &self.stop_flag);
                }
                return Ok(Some(frames));
            } else {
                let frames = total_samples / 2;
                if let Some(ref mut vp) = self.vis_producer {
                    write_interleaved_stereo_to_rings(
                        &self.interleaved_buf,
                        &mut self.audio_producer,
                        vp,
                    );
                } else {
                    push_all_cancelable(
                        &mut self.audio_producer,
                        &self.interleaved_buf,
                        &self.stop_flag,
                    );
                }
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

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::ring_buffers::{
        create_rings, AudioRing, VisRing, VIS_RING_BUFFER_CAPACITY,
    };
    use crate::dsp::cavacore::{Cavacore, N_BASS};
    use rtrb::{Consumer, PopError, RingBuffer};

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
    fn abandoned_consumer_or_stop_flag_never_spins_forever() {
        let (mut prod, cons) = RingBuffer::<f32>::new(4);
        // Заполняем буфер до предела и дропаем consumer (как при переключении трека!)
        let _ = prod.push_partial_slice(&[1.0, 2.0, 3.0, 4.0]);
        assert!(prod.is_full());
        drop(cons);

        let stop = AtomicBool::new(false);
        let written = push_all_cancelable(&mut prod, &[5.0, 6.0], &stop);
        assert_eq!(written, 0, "должен мгновенно выйти при дропнутом Consumer");
    }

    #[test]
    fn tech_info_badge_formats_correctly() {
        let flac_info = StreamTechInfo::default();
        assert_eq!(flac_info.badge(false), "FLAC 16-bit/44.1kHz | 1411 kbps");

        let mp3_preview = StreamTechInfo {
            codec_name: "MP3".into(),
            sample_rate: 44_100,
            bits_per_sample: 16,
            total_frames: 44_100 * 30,
            duration_secs: 30,
            bitrate_kbps: 128,
            file_size_bytes: 480_000,
        };
        assert_eq!(mp3_preview.badge(true), "MP3 44.1kHz | 128 kbps (PREVIEW)");
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

        let mono_vis = drain(vis.consumer());
        assert_eq!(mono_vis.len(), N_BASS * 2);

        let mut cava = Cavacore::new(44_100.0, 32);
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
        let mut bars = cava.process(&mono_vis);
        for _ in 0..8 {
            bars = cava.process(&mono_vis);
        }
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        assert!(high_max > 0.15, "верхние полосы должны активироваться на 5000 Гц: {high_max}");
        assert!(
            high_max > low_max * 2.0,
            "ВЧ ({high_max}) должны превышать НЧ ({low_max}) для сигнала 5000 Гц"
        );
    }

    #[test]
    fn vis_ring_overflow_never_deadlocks_audio_ring() {
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

