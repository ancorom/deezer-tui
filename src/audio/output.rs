//! CPAL 0.18 real-time output stream.
//!
//! # Design
//!
//! - Opens the host's default output device.
//! - Explicitly selects a **44.1 kHz** configuration
//!   (`cpal::SAMPLE_RATE_CD`). The 0.18 default now prefers 48 kHz, so the
//!   CD sample rate must be requested by hand.
//! - The real-time callback reads from the interleaved-stereo rtrb
//!   `AudioRingBuffer` via `Consumer::pop()` (non-blocking, wait-free). Empty
//!   slots are filled with silence (`0.0`) so the callback never blocks.
//! - Every sample is multiplied by an **atomic volume knob** — an `AtomicU32`
//!   whose bits are a packed `f32` in [0.0, 1.0] (`1.0` = full gain). The
//!   knob is set once at startup and updated atomically on the UI thread; the
//!   callback reads it with a relaxed atomic load.
//!
//! The consumer is **moved into the real-time callback**, so the callback owns
//! it and never blocks on it.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use cpal::OutputCallbackInfo;
use cpal::traits::{DeviceTrait, HostTrait};
use rtrb::{Consumer, PopError};

/// The sample rate explicitly requested for the output stream.
///
/// `cpal::SAMPLE_RATE_CD` is `44_100`. It is requested by hand because the
/// 0.18 default sample rate is now 48 kHz.
#[allow(dead_code)]
pub const SAMPLE_RATE_HZ: u32 = 44_100;

/// Default (full-gain) volume in packed `f32` bits.
#[allow(dead_code)]
pub fn default_volume_bits() -> u32 {
    f32::to_bits(1.0)
}

/// Zero (mute) in packed `f32` bits.
#[allow(dead_code)]
pub fn muted_volume_bits() -> u32 {
    f32::to_bits(0.0)
}

/// Fill `buffer` from `consumer`, multiplying each sample by the atomic
/// `volume` (a packed `f32` in [0.0, 1.0], via `f32::from_bits`).
///
/// Empty consumer slots are filled with silence (`0.0`) so the function never
/// blocks — safe to call from a real-time audio callback.
///
/// # Returns
///
/// The number of slots that received a non-empty sample (i.e. the number of
/// `pop()` successes).
#[allow(dead_code)]
pub fn fill_buffer_from_consumer(
    buffer: &mut [f32],
    consumer: &mut Consumer<f32>,
    volume: &AtomicU32,
) -> usize {
    let vol = f32::from_bits(volume.load(Ordering::Relaxed));
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

/// Holds the CPAL output stream and the atomic volume knob.
///
/// The rtrb consumer is moved into the real-time callback (see `new`), so the
/// callback owns it and never blocks on it.
#[allow(dead_code)]
pub struct AudioOutput {
    /// The stream; it keeps the consumer alive via the callback.
    stream: cpal::Stream,
    /// The atomic volume knob (packed `f32` bits).
    volume: Arc<AtomicU32>,
}

#[allow(dead_code)]
impl AudioOutput {
    /// Open the output stream and move `consumer` into the real-time callback.
    ///
    /// # Errors
    ///
    /// Returns the CPAL error on device or stream failure.
    pub fn new(consumer: Consumer<f32>, volume_bits: u32) -> Result<Self, cpal::Error> {
        let volume = Arc::new(AtomicU32::new(volume_bits));
        let host = cpal::Host::default();
        let device = host
            .default_output_device()
            .ok_or(cpal::Error::new(cpal::ErrorKind::DeviceNotAvailable))?;

        // Explicit 44.1 kHz (SAMPLE_RATE_CD). The 0.18 default now prefers 48 kHz.
        let config: cpal::StreamConfig = device
            .supported_output_configs()
            .expect("error while querying configs")
            .find_map(|r| r.try_with_sample_rate(cpal::SAMPLE_RATE_CD))
            .expect("device does not support 44.1 kHz")
            .into();

        debug_assert!(config.sample_rate == cpal::SAMPLE_RATE_CD, "sample rate mismatch");

        let volume_for_cb = volume.clone();
        // The consumer is moved into the callback, so the callback owns it and
        // never blocks on it.
        let mut consumer_for_cb = consumer;
        let error_callback = move |e: cpal::Error| {
            tracing::warn!(target = "audio_output", "stream error: {e:?}");
        };

        let stream = device.build_output_stream(
            config,
            move |buffer: &mut [f32], _info: &OutputCallbackInfo| {
                // Non-blocking: read `buffer.len()` samples from the consumer
                // (silence on empty) and multiply by the atomic volume.
                fill_buffer_from_consumer(buffer, &mut consumer_for_cb, &volume_for_cb);
            },
            error_callback,
            None,
        )?;

        Ok(AudioOutput {
            stream,
            volume,
        })
    }

    /// The atomic volume knob (packed `f32` bits).
    pub fn volume(&self) -> &AtomicU32 {
        &self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::ring_buffers::{ AudioRing, VisRing };
    use crate::audio::decoder::write_interleaved_stereo_to_rings;
    use rtrb::PopError;
    use rtrb::RingBuffer;

    /// Drain a consumer into a `Vec<f32>`; used to verify ring contents.
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

    /// A fresh consumer seeded with the given samples.
    fn consumer_with(samples: &[f32]) -> Consumer<f32> {
        let (mut producer, consumer) = RingBuffer::<f32>::new(samples.len());
        for &s in samples {
            producer.push(s).unwrap();
        }
        consumer
    }

    /// The output callback must fill the buffer from the rtrb consumer
    /// (non-blocking) and apply the atomic volume.
    #[test]
    fn fill_buffer_applies_volume_and_silence_on_empty() {
        let mut consumer = consumer_with(&[0.5f32, -0.5f32, 0.25f32, 1.0, 0.0, -1.0]);
        let mut buffer = [0.0f32; 6];
        let volume = Arc::new(AtomicU32::new(f32::to_bits(1.0)));
        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 6);
        assert!((buffer[0] - 0.5f32).abs() < 1e-6);
        assert!((buffer[1] + 0.5f32).abs() < 1e-6);
        assert!((buffer[2] - 0.25f32).abs() < 1e-6);
        assert!((buffer[3] - 1.0f32).abs() < 1e-6);
        assert!((buffer[4] - 0.0f32).abs() < 1e-6);
        assert!((buffer[5] + 1.0f32).abs() < 1e-6);
    }

    /// Half gain (0.5) halves every sample.
    #[test]
    fn fill_buffer_halves_with_half_gain() {
        let mut consumer = consumer_with(&[1.0f32, -1.0f32, 0.0]);
        let mut buffer = [0.0f32; 3];
        let volume = Arc::new(AtomicU32::new(f32::to_bits(0.5f32)));
        fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert!((buffer[0] - 0.5f32).abs() < 1e-6);
        assert!((buffer[1] + 0.5f32).abs() < 1e-6);
        assert!((buffer[2] - 0.0f32).abs() < 1e-6);
    }

    /// An empty consumer fills every slot with silence and never blocks.
    #[test]
    fn fill_buffer_empty_consumer_fills_silence() {
        let mut consumer = consumer_with(&[0.0f32]);
        let mut buffer = [0.0f32; 16];
        let volume = Arc::new(AtomicU32::new(f32::to_bits(1.0f32)));
        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 1);
        assert_eq!(buffer[0], 0.0f32);
        for s in buffer.iter().skip(1) {
            assert_eq!(*s, 0.0f32);
        }
    }

    /// A partially-filled consumer fills the available samples and
    /// silences the rest.
    #[test]
    fn fill_buffer_partial_fills_available_then_silence() {
        let mut consumer = consumer_with(&[0.3f32, 0.7f32]);
        let mut buffer = [0.0f32; 4];
        let volume = Arc::new(AtomicU32::new(f32::to_bits(1.0f32)));
        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 2);
        assert!((buffer[0] - 0.3f32).abs() < 1e-6);
        assert!((buffer[1] - 0.7f32).abs() < 1e-6);
        assert_eq!(buffer[2], 0.0f32);
        assert_eq!(buffer[3], 0.0f32);
    }

    /// End-to-end: write a generated stereo buffer into the rtrb audio ring,
    /// drain it into a `Consumer`, then feed it to `fill_buffer_from_consumer`
    /// at full gain — the output buffer must match the input exactly.
    #[test]
    fn end_to_end_rtrb_to_output_buffer() {
        let frames = 1_000usize;
        let mut stereo: Vec<f32> = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let t = (i as f64) * 2.0 * std::f64::consts::PI * 440.0 / 44_100.0;
            let s = t.sin() as f32;
            stereo.push(s);
            stereo.push(s);
        }
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();
        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());
        let drained = drain(&mut audio.consumer());
        assert_eq!(drained, stereo);

        // The CPAL callback reads the ring one buffer at a time; feed the
        // drained stereo through the consumer at full gain and check the
        // output matches the input.
        let mut consumer = consumer_with(&drained);
        let mut buffer = vec![0.0f32; drained.len()];
        fill_buffer_from_consumer(
            &mut buffer,
            &mut consumer,
            &Arc::new(AtomicU32::new(f32::to_bits(1.0f32))),
        );
        assert_eq!(buffer, drained, "full-gain output must match input");
    }
}