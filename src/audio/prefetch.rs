//! Gapless prefetch: start preparing the next track (download + decrypt,
//! and optionally pre-probe) a fixed threshold (default 20 s) before the
//! current track ends, so that transitions between tracks have no audible
//! gap.
//!
//! The prefetch task writes the prepared track into a [`PrefetchSlot`]; the
//! playback loop reads it out. The slot is the only shared mutable state,
//! so the two sides never block each other.

use std::io::Read;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use super::stream::DeezerStream;

/// How far before the end of the current track to start preparing the next
/// one.
#[allow(dead_code)]
pub const DEFAULT_PREFETCH_THRESHOLD: Duration = Duration::from_secs(20);

/// Gapless policy: when to start preparing the next track.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GaplessPolicy {
    /// Start prefetching when the remaining playback time is `<= threshold`.
    pub threshold: Duration,
}

impl Default for GaplessPolicy {
    fn default() -> Self {
        Self {
            threshold: DEFAULT_PREFETCH_THRESHOLD,
        }
    }
}

impl GaplessPolicy {
    /// A policy with an explicit prefetch threshold.
    #[allow(dead_code)]
    pub fn new(threshold: Duration) -> Self {
        Self { threshold }
    }

    /// Whether the next track should be prepared now.
    #[allow(dead_code)]
    pub fn should_prefetch(&self, remaining: Duration) -> bool {
        remaining <= self.threshold
    }
}

/// A prepared next track, ready to be handed to the `Decoder`.
#[allow(dead_code)]
pub struct PreparedTrack {
    /// The decrypted stream of the next track.
    pub stream: DeezerStream,
    /// The track's duration (if known), for scheduling the *next* prefetch.
    pub duration: Option<Duration>,
}

impl PreparedTrack {
    /// The reader for the `Decoder`.
    #[allow(dead_code)]
    pub fn into_reader(self) -> Box<dyn Read + Send + Sync> {
        Box::new(self.stream)
    }
}

/// Shared slot between the prefetch task (writer) and the playback loop
/// (reader).
#[allow(dead_code)]
pub struct PrefetchSlot {
    inner: Mutex<Option<PreparedTrack>>,
}

impl Default for PrefetchSlot {
    fn default() -> Self {
        Self::new()
    }
}

impl PrefetchSlot {
    /// An empty slot.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    /// Store a prepared track (replaces any previous one).
    #[allow(dead_code)]
    pub fn store(&self, track: PreparedTrack) {
        *self.inner() = Some(track);
    }

    /// Take the prepared track (`None` if not yet prepared).
    #[allow(dead_code)]
    pub fn take(&self) -> Option<PreparedTrack> {
        self.inner().take()
    }

    /// Whether a prepared track is waiting.
    #[allow(dead_code)]
    pub fn is_ready(&self) -> bool {
        self.inner().is_some()
    }

    #[allow(dead_code)]
    fn inner(&self) -> MutexGuard<'_, Option<PreparedTrack>> {
        self.inner.lock().expect("prefetch slot poisoned")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i * 0x9F + 0x37) as u8).collect()
    }

    #[test]
    fn default_threshold_is_20s() {
        assert_eq!(
            GaplessPolicy::default().threshold,
            DEFAULT_PREFETCH_THRESHOLD
        );
        assert_eq!(DEFAULT_PREFETCH_THRESHOLD, Duration::from_secs(20));
    }

    #[test]
    fn should_prefetch_boundaries() {
        let policy = GaplessPolicy::default();
        assert!(policy.should_prefetch(Duration::from_secs(0)));
        assert!(policy.should_prefetch(Duration::from_secs(19)));
        assert!(policy.should_prefetch(Duration::from_secs(20)));
        assert!(!policy.should_prefetch(Duration::from_secs(21)));
        assert!(!policy.should_prefetch(Duration::from_secs(3600)));
    }

    #[test]
    fn slot_take_empty_then_store_then_take() {
        let slot = PrefetchSlot::new();
        assert!(!slot.is_ready());
        assert!(slot.take().is_none());

        let data = sample(4096);
        let stream = DeezerStream::from_decrypted(data);
        slot.store(PreparedTrack {
            stream,
            duration: Some(Duration::from_secs(30)),
        });
        assert!(slot.is_ready());

        let taken = slot.take().expect("stored track");
        assert_eq!(taken.stream.len(), 4096);
        assert_eq!(taken.duration, Some(Duration::from_secs(30)));
        // Taken out: empty again.
        assert!(!slot.is_ready());
        assert!(slot.take().is_none());
    }

    #[test]
    fn prepared_track_into_reader_reads_bytes() {
        let original = sample(8192);
        let data = original.clone();
        let stream = DeezerStream::from_decrypted(data);
        let mut reader = PreparedTrack {
            stream,
            duration: None,
        }
        .into_reader();
        let mut read_back = Vec::new();
        reader
            .read_to_end(&mut read_back)
            .expect("read prepared reader");
        assert_eq!(read_back, original);
    }
}
