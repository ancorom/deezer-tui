//! Lock-free кольцевые SPSC-буферы на базе `rtrb` 0.4.
//!
//! - `AudioRingBuffer`: 65 536 сэмплов (32 768 стерео-фреймов `f32`).
//! - `VisRingBuffer`: 16 384 моно-сэмпла `(L + R) * 0.5`.

#![allow(dead_code)]

use rtrb::{Consumer, Producer, RingBuffer};

/// Ёмкость стерео-буфера воспроизведения (в сэмплах `f32`).
pub const AUDIO_RING_BUFFER_CAPACITY: usize = 65_536;

/// Ёмкость моно-буфера визуализации (в сэмплах `f32`).
pub const VIS_RING_BUFFER_CAPACITY: usize = 16_384;

/// Стерео-буфер для передачи аудио из потока декодера в callback `cpal`.
pub struct AudioRing {
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

impl Default for AudioRing {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioRing {
    /// Создаёт новый кольцевой буфер на 65 536 сэмплов.
    pub fn new() -> Self {
        let (producer, consumer) = RingBuffer::<f32>::new(AUDIO_RING_BUFFER_CAPACITY);
        Self { producer, consumer }
    }

    /// Мутабельная ссылка на `Producer<f32>`.
    pub fn producer(&mut self) -> &mut Producer<f32> {
        &mut self.producer
    }

    /// Мутабельная ссылка на `Consumer<f32>`.
    pub fn consumer(&mut self) -> &mut Consumer<f32> {
        &mut self.consumer
    }

    /// Разделяет буфер на независимые `(Producer<f32>, Consumer<f32>)` для передачи в разные потоки.
    pub fn split(self) -> (Producer<f32>, Consumer<f32>) {
        (self.producer, self.consumer)
    }

    /// Синоним `split()` для обратной совместимости с тестами.
    pub fn into_producer(self) -> (Producer<f32>, Consumer<f32>) {
        self.split()
    }
}

/// Моно-буфер для передачи сэмплов `(L + R) * 0.5` из декодера в поток UI/Cavacore.
pub struct VisRing {
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

impl Default for VisRing {
    fn default() -> Self {
        Self::new()
    }
}

impl VisRing {
    /// Создаёт новый кольцевой буфер на 16 384 моно-сэмпла.
    pub fn new() -> Self {
        let (producer, consumer) = RingBuffer::<f32>::new(VIS_RING_BUFFER_CAPACITY);
        Self { producer, consumer }
    }

    /// Мутабельная ссылка на `Producer<f32>`.
    pub fn producer(&mut self) -> &mut Producer<f32> {
        &mut self.producer
    }

    /// Мутабельная ссылка на `Consumer<f32>`.
    pub fn consumer(&mut self) -> &mut Consumer<f32> {
        &mut self.consumer
    }

    /// Разделяет буфер на независимые `(Producer<f32>, Consumer<f32>)` для передачи в разные потоки.
    pub fn split(self) -> (Producer<f32>, Consumer<f32>) {
        (self.producer, self.consumer)
    }

    /// Синоним `split()` для обратной совместимости с тестами.
    pub fn into_producer(self) -> (Producer<f32>, Consumer<f32>) {
        self.split()
    }
}

/// Создаёт пару буферов `(AudioRing, VisRing)`.
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
        while let Ok(s) = audio_consumer.pop() {
            drained.push(s);
        }
        assert!(matches!(audio_consumer.pop(), Err(PopError::Empty)));
        // Drain the vis ring and verify the mono content.
        let mut drained_v: Vec<f32> = Vec::new();
        while let Ok(s) = vis_consumer.pop() {
            drained_v.push(s);
        }
        assert!(matches!(vis_consumer.pop(), Err(PopError::Empty)));

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
