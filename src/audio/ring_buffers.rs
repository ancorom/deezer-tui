//! Lock-free (SPSC) audio ring buffers built on top of [rtrb].
//!
//! The decoder thread is the sole producer, the CPAL real-time callback is
//! the sole consumer of the audio ring, and the UI/DSP thread is the sole
//! consumer of the visualization ring. `Producer` and `Consumer` are
//! `Send` but not `Sync`, so each side keeps its own handle on its own thread.
//!
//! # Capacities
//!
//! - `AudioRingBuffer` (interleaved stereo f32): 65 536 samples (32 768 frames).
//! - `VisRingBuffer` (mono f32 = (L + R) / 2): 16 384 samples.
//!
//! The capacities are sized so that, at 44.1 kHz, the audio ring holds about
//! 1.5 s of stereo audio and the vis ring about 0.37 s of mono audio.

use rtrb::{Consumer, Producer, RingBuffer};

/// Capacity of the interleaved-stereo audio ring buffer, in samples.
#[allow(dead_code)]
pub const AUDIO_RING_BUFFER_CAPACITY: usize = 65_536;

/// Capacity of the mono visualization ring buffer, in samples.
#[allow(dead_code)]
pub const VIS_RING_BUFFER_CAPACITY: usize = 16_384;

/// Lock-free stereo audio ring buffer (interleaved f32).
///
/// The `producer` is owned by the decoder thread, the `consumer` by the
/// real-time CPAL callback (and, for testing, by the drain thread).
#[allow(dead_code)]
pub struct AudioRing {
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

#[allow(dead_code)]
impl AudioRing {
    /// Allocate a new 65 536-sample interleaved-stereo ring buffer.
    pub fn new() -> Self {
        let (producer, consumer) = RingBuffer::<f32>::new(AUDIO_RING_BUFFER_CAPACITY);
        Self {
            producer,
            consumer,
        }
    }

    /// The decoder writes interleaved stereo through this handle.
    #[allow(dead_code)]
    pub fn producer(&mut self) -> &mut Producer<f32> {
        &mut self.producer
    }

    /// The CPAL / UI thread reads through this handle.
    #[allow(dead_code)]
    pub fn consumer(&mut self) -> &mut Consumer<f32> {
        &mut self.consumer
    }

    /// Transfer ownership of the buffer to the producing thread, keeping
    /// the consumer handle (used by the drain thread / tests).
    pub fn into_producer(self) -> (Producer<f32>, Consumer<f32>) {
        (self.producer, self.consumer)
    }
}

/// Lock-free mono visualization ring buffer (16 384 samples, mono = (L+R)/2).
#[allow(dead_code)]
pub struct VisRing {
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

#[allow(dead_code)]
impl VisRing {
    /// Allocate a new 16 384-sample mono ring buffer.
    pub fn new() -> Self {
        let (producer, consumer) = RingBuffer::<f32>::new(VIS_RING_BUFFER_CAPACITY);
        Self {
            producer,
            consumer,
        }
    }

    /// The decoder writes mono samples through this handle.
    #[allow(dead_code)]
    pub fn producer(&mut self) -> &mut Producer<f32> {
        &mut self.producer
    }

    /// The UI thread reads through this handle.
    #[allow(dead_code)]
    pub fn consumer(&mut self) -> &mut Consumer<f32> {
        &mut self.consumer
    }

    /// Transfer ownership of the buffer to the producing thread.
    pub fn into_producer(self) -> (Producer<f32>, Consumer<f32>) {
        (self.producer, self.consumer)
    }
}

/// Allocate both ring buffers used by the audio pipeline.
///
/// # Returns
///
/// A pair where the caller (typically the decoder thread) keeps the producers
/// and passes the consumers to the output / UI threads.
#[allow(dead_code)]
pub fn create_rings() -> (AudioRing, VisRing) {
    (AudioRing::new(), VisRing::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rtrb::PopError;

    /// Writing a small interleaved-stereo chunk must fill the audio ring and
    /// its mono counterpart without panics or blocking.
    #[test]
    fn generated_buffer_fills_both_rings_without_blocking() {
        let frames = 1_000usize;
        let mut stereo: Vec<f32> = Vec::with_capacity(frames * 2);
        // A 440 Hz-ish tone, one frame at a time.
        for i in 0..frames {
            let t = (i as f64) * 2.0 * std::f64::consts::PI * 440.0 / 44_100.0;
            let v = t.sin() as f32;
            stereo.push(v);
            stereo.push(v);
        }

        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        // Write both (interleaved stereo and mono) and record how much landed.
        let (audio_written, vis_written) =
            crate::audio::decoder::write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        let (producer, mut audio_consumer) = audio.into_producer();
        let (producer_v, mut vis_consumer) = vis.into_producer();

        // The producer must still own the buffer (used as a liveness check).
        let _ = producer;
        let _ = producer_v;

        // Drain the audio ring and verify the exact interleaved stereo content.
        let mut drained: Vec<f32> = Vec::new();
        loop {
            match audio_consumer.pop() {
                Ok(s) => drained.push(s),
                Err(PopError::Empty) => break,
            }
        }
        // Drain the vis ring and verify the mono content.
        let mut drained_v: Vec<f32> = Vec::new();
        loop {
            match vis_consumer.pop() {
                Ok(s) => drained_v.push(s),
                Err(PopError::Empty) => break,
            }
        }

        assert_eq!(audio_written, stereo.len(), "audio ring should accept all stereo samples");
        assert_eq!(vis_written, frames, "vis ring should accept all mono samples");
        assert_eq!(drained, stereo, "audio ring round-trip must preserve interleaved stereo");
        assert_eq!(drained_v.len(), frames, "vis ring must hold frames mono samples");
        // Each mono sample must equal (L + R) / 2.
        for i in 0..frames {
            let expected = (stereo[2 * i] + stereo[2 * i + 1]) * 0.5;
            // f32 arithmetic is exact for (x + x) / 2, so compare exactly.
            assert_eq!(drained_v[i], expected, "vis[{i}] != (L+R)/2");
        }
    }
}
