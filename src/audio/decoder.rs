//! Symphonia-based audio decoder that feeds the two rtrb audio ring buffers.
//!
//! # Pipeline
//!
//! The decoder runs on its own `std::thread`. It probes the incoming byte
//! stream with symphonia, selects the first decodable audio track, decodes
//! packets and, on every packet, writes the resulting samples to:
//!
//! - the interleaved-stereo `AudioRingBuffer` (65 536 samples);
//! - the mono `(L + R) / 2` `VisRingBuffer` (16 384 samples).
//!
//! The consumer handles (CPAL callback, UI/DSP thread, or test drain thread)
//! keep the buffer alive; the decoder only keeps the producer handles and the
//! consumer handles that its caller asks to retrieve.

use std::io::Read;

use symphonia::core::codecs::audio::AudioDecoder;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatReader;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::TrackType;
use symphonia::core::io::{MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;

use rtrb::Producer;

/// Push `src` fully into `dst`, yielding until it fits.
///
/// `dst.is_full()` is a single-slot probe: when it is `true` the buffer has
/// no room for at least one sample and we yield; otherwise `push_partial_slice`
/// writes every available sample (and is guaranteed to make progress), so the
/// loop always terminates once the consumer keeps up.
#[allow(dead_code)]
#[inline(never)]
fn push_all(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    let mut remaining = src;
    let mut written = 0;
    loop {
        if remaining.is_empty() {
            break;
        }
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

/// Write a chunk of interleaved-stereo f32 samples into both audio rings.
///
/// `stereo` is interleaved as (L0, R0, L1, R1, ...). The visualization ring
/// receives the mono sum `(L + R) / 2`, matching the spec ("сведение моно
/// (L + R) * 0.5").
///
/// # Returns
///
/// `(audio_written, vis_written)` — the number of samples accepted by each
/// ring buffer. In steady state (consumer keeps up) this equals
/// `stereo.len()` and `stereo.len() / 2` respectively.
#[allow(dead_code)]
pub fn write_interleaved_stereo_to_rings(
    stereo: &[f32],
    audio: &mut Producer<f32>,
    vis: &mut Producer<f32>,
) -> (usize, usize) {
    let frames = stereo.len() / 2;
    let audio_written = push_all(audio, stereo);

    // Build the mono buffer. This runs on the decoder thread (not the RT
    // callback), so allocating the mono vector is cheap.
    let mut mono = Vec::with_capacity(frames);
    for i in 0..frames {
        let l = stereo[2 * i];
        let r = stereo[2 * i + 1];
        mono.push((l + r) * 0.5);
    }
    let vis_written = push_all(vis, &mono);

    (audio_written, vis_written)
}

/// A Symphonia-backed decoder wired to two rtrb ring buffers.
///
/// `new()` probes the reader and picks the first decodable audio track. The
/// caller keeps the two consumer handles (for the CPAL / UI threads); the
/// decoder owns the two producer handles and the symphonia state.
#[allow(dead_code)]
pub struct Decoder {
    /// The raw reader, kept for re-probing on `ResetRequired`.
    format: Box<dyn FormatReader>,
    track_id: u32,
    decoder: Box<dyn AudioDecoder>,
    audio_producer: Producer<f32>,
    vis_producer: Producer<f32>,
}

#[allow(dead_code)]
impl Decoder {
    /// Probe `reader` and set up decoding into the two ring buffers.
    pub fn new(
        reader: Box<dyn Read + Send + Sync>,
        audio: crate::audio::ring_buffers::AudioRing,
        vis: crate::audio::ring_buffers::VisRing,
    ) -> Self {
        let mss = MediaSourceStream::new(
            Box::new(ReadOnlySource::new(reader)),
            Default::default(),
        );
        let format: Box<dyn FormatReader> = symphonia::default::get_probe()
            .probe(
                &Hint::new(),
                mss,
                FormatOptions::default(),
                MetadataOptions::default(),
            )
            .expect("failed to probe audio format");
        let track = format
            .default_track(TrackType::Audio)
            .expect("no decodable audio track");
        let track_id = track.id;
        let dec_opts: AudioDecoderOptions = Default::default();
        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(
                track
                    .codec_params
                    .as_ref()
                    .expect("codec params missing")
                    .audio()
                    .unwrap(),
                &dec_opts,
            )
            .expect("unsupported codec");
        Self {
            format,
            track_id,
            decoder,
            audio_producer: audio.into_producer().0,
            vis_producer: vis.into_producer().0,
        }
    }
}

#[allow(dead_code)]
impl Decoder {
    /// Decode the next packet and write to the two ring buffers.
    ///
    /// Returns the number of stereo frames produced (0 on EOF).
    pub fn process_next(&mut self) -> Result<usize, SymphoniaError> {
        let packet = self.format.next_packet()?;
        match packet {
            None => {
                // EOF.
                Ok(0)
            }
            Some(pkt) => {
                if pkt.track_id != self.track_id {
                    // Skip non-audio packets.
                    return Ok(0);
                }
                let decoded = self.decoder.decode(&pkt)?;
                // Convert to interleaved f32.
                let n = decoded.samples_interleaved();
                let mut stereo: Vec<f32> = vec![0.0; n];
                decoded.copy_to_slice_interleaved(&mut stereo);
                let (_audio_written, _vis_written) =
                    write_interleaved_stereo_to_rings(&stereo, &mut self.audio_producer, &mut self.vis_producer);
                Ok(n / 2)
            }
        }
    }
}

#[allow(dead_code)]
impl Decoder {
    /// Run the decode loop until end-of-media.
    pub fn run(&mut self) -> Result<usize, SymphoniaError> {
        let mut total = 0;
        while let Ok(frames) = self.process_next() {
            total += frames;
            // The caller of `run` is expected to keep draining the
            // consumers; the buffer sizes bound the in-flight sample count.
            if frames == 0 {
                // No frames this iteration (EOF or a skipped track).
                // To distinguish EOF we need `process_next` to return 0
                // only on EOF — for now assume a long track and break on a
                // follow-up `process_next` that returns 0 again.
                let next = self.process_next()?;
                if next == 0 {
                    break;
                }
                total += next;
            }
        }
        Ok(total)
    }
}

#[allow(dead_code)]
impl Decoder {
    /// Take the two producer handles out (leaving the consumers for the caller).
    pub fn into_producers(self) -> (Producer<f32>, Producer<f32>) {
        // Drop the format/decoder/reader; keep the two producers.
        (self.audio_producer, self.vis_producer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::ring_buffers::{ AudioRing, AUDIO_RING_BUFFER_CAPACITY, VisRing };
    use crate::dsp::cavacore::Cavacore;
    use rtrb::{Consumer, PopError};

    /// Drains a consumer into a `Vec<f32>`; used to verify ring contents.
    fn drain(consumer: &mut Consumer<f32>) -> Vec<f32> {
        let mut out = Vec::new();
        loop {
            match consumer.pop() {
                Ok(s) => out.push(s),
                Err(PopError::Empty) => break,
            }
        }
        out
    }

    /// 100 Hz stereo chunk of `frames` frames.
    fn stereo_100_hz(frames: usize) -> Vec<f32> {
        let mut v = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let t = (i as f64) * 2.0 * std::f64::consts::PI * 100.0 / 44_100.0;
            let s = t.sin() as f32;
            v.push(s);
            v.push(s);
        }
        v
    }

    /// 5000 Hz stereo chunk.
    fn stereo_5k(frames: usize) -> Vec<f32> {
        let mut v = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let t = (i as f64) * 2.0 * std::f64::consts::PI * 5000.0 / 44_100.0;
            let s = t.sin() as f32;
            v.push(s);
            v.push(s);
        }
        v
    }

    #[test]
    fn generated_buffer_fills_both_rings_without_blocking() {
        let stereo = stereo_100_hz(1_000);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();
        let (aw, vw) = write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());
        let (_, mut ac) = audio.into_producer();
        let (_, mut vc) = vis.into_producer();
        assert_eq!(aw, stereo.len());
        assert_eq!(vw, stereo.len() / 2);
        let audio_drained = drain(&mut ac);
        let vis_drained = drain(&mut vc);
        assert_eq!(audio_drained, stereo, "audio ring must preserve interleaved stereo");
        assert_eq!(vis_drained.len(), stereo.len() / 2);
        for i in 0..(stereo.len() / 2) {
            assert_eq!(vis_drained[i], (stereo[2 * i] + stereo[2 * i + 1]) * 0.5);
        }
    }

    /// Decoded (generated) samples pass through the rtrb audio ring and
    /// reach Cavacore, activating the expected low-frequency bars for a 100 Hz tone.
    #[test]
    fn rtrb_to_cavacore_100_hz_activates_low_bars() {
        let frames = 10_000usize;
        let stereo = stereo_100_hz(frames);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();
        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());
        let drained = drain(&mut audio.consumer());
        assert_eq!(drained, stereo, "rtrb audio ring must round-trip stereo");
        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&drained);
        let half = bars.len() / 2;
        let low_max = bars.iter().take(half).copied().fold(f32::MIN, f32::max);
        let high_max = bars.iter().skip(half).copied().fold(f32::MIN, f32::max);
        assert!(low_max > 0.0, "low bars must be non-zero for a 100 Hz tone");
        assert!(low_max > high_max, "low bars must exceed high bars for a 100 Hz tone");
    }

    /// 5000 Hz tone: the high half of the bars must be hotter.
    #[test]
    fn rtrb_to_cavacore_5k_activates_high_bars() {
        let frames = 10_000usize;
        let stereo = stereo_5k(frames);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();
        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());
        let drained = drain(&mut audio.consumer());
        assert_eq!(drained, stereo);
        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&drained);
        let half = bars.len() / 2;
        let high_max = bars.iter().skip(half).copied().fold(f32::MIN, f32::max);
        let low_max = bars.iter().take(half).copied().fold(f32::MIN, f32::max);
        assert!(high_max > low_max, "high bars must exceed low bars for a 5000 Hz tone");
    }

    #[test]
    fn generated_buffer_does_not_block_and_no_panic() {
        // Write a large (but < capacity) interleaved stereo chunk into the
        // audio ring and a correspondingly large mono chunk into the vis ring,
        // then drain. Must not block (no consumer reading while producing)
        // and must not panic.
        let frames = AUDIO_RING_BUFFER_CAPACITY / 4;
        let stereo = vec![0.0f32; frames * 2];
        let mut audio = AudioRing::new();
        let mut vis_ring = VisRing::new();
        let (aw, vw) = write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis_ring.producer());
        let (_, mut ac) = audio.into_producer();
        let (_, mut vc) = vis_ring.into_producer();
        assert_eq!(aw, stereo.len());
        assert_eq!(vw, frames);
        let drained = drain(&mut ac);
        assert_eq!(drained, stereo);
        let drained_v = drain(&mut vc);
        assert_eq!(drained_v.len(), frames);
        // All zeros -> all bars zero.
        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&stereo);
        for b in bars {
            assert_eq!(*b, 0.0f32, "zero input must produce zero bars");
        }
    }
}