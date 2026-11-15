//! MPRIS integration (souvlaki, `use_zbus`): hardware media keys (play/pause,
//! next, previous) and desktop "now playing" widgets (GNOME/KDE, `playerctl`)
//! can control and observe playback.
//!
//! The D-Bus service name is `org.mpris.MediaPlayer2.deezer_tui`. The service
//! thread is started by [`Mpris::attach`]; until then, state-setting methods
//! fail with [`souvlaki::Error::ThreadNotRunning`].

use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig,
};

/// D-Bus name suffix: the full name is `org.mpris.MediaPlayer2.deezer_tui`.
#[allow(dead_code)]
pub const DBUS_NAME: &str = "deezer_tui";

/// Display name shown in the desktop media-player list.
#[allow(dead_code)]
pub const DISPLAY_NAME: &str = "Deezer TUI";

/// MPRIS adapter wrapping souvlaki's `MediaControls`.
#[allow(dead_code)]
pub struct Mpris {
    controls: Mutex<MediaControls>,
}

impl Mpris {
    /// Create the media controls. The D-Bus service thread is only started by
    /// [`Mpris::attach`].
    #[allow(dead_code)]
    pub fn new() -> Result<Self, souvlaki::Error> {
        let config = PlatformConfig {
            display_name: DISPLAY_NAME,
            dbus_name: DBUS_NAME,
            hwnd: None,
        };
        Ok(Self {
            controls: Mutex::new(MediaControls::new(config)?),
        })
    }

    /// Start the D-Bus service thread; every media-control event (media keys,
    /// desktop widget buttons) is forwarded to `handler`.
    #[allow(dead_code)]
    pub fn attach(
        &self,
        handler: impl Fn(MediaControlEvent) + Send + 'static,
    ) -> Result<(), souvlaki::Error> {
        self.controls().attach(handler)
    }

    /// Push metadata for the new track (title/artist/album/cover/duration).
    #[allow(dead_code)]
    pub fn set_track(
        &self,
        title: &str,
        artist: &str,
        album: &str,
        duration: Duration,
        cover_url: Option<&str>,
    ) -> Result<(), souvlaki::Error> {
        self.controls().set_metadata(MediaMetadata {
            title: Some(title),
            artist: Some(artist),
            album: Some(album),
            cover_url,
            duration: Some(duration),
        })
    }

    /// Set the playback status and position (drives `PlaybackStatus` and
    /// `Position` properties).
    #[allow(dead_code)]
    pub fn set_playback(
        &self,
        playing: bool,
        position: Duration,
    ) -> Result<(), souvlaki::Error> {
        let playback = if playing {
            MediaPlayback::Playing {
                progress: Some(souvlaki::MediaPosition(position)),
            }
        } else {
            MediaPlayback::Paused {
                progress: Some(souvlaki::MediaPosition(position)),
            }
        };
        self.controls().set_playback(playback)
    }

    /// Set the volume level (`0.0`–`1.0`).
    #[allow(dead_code)]
    pub fn set_volume(&self, volume: f64) -> Result<(), souvlaki::Error> {
        self.controls().set_volume(volume)
    }

    fn controls<'s>(&'s self) -> MutexGuard<'s, MediaControls> {
        self.controls.lock().expect("mpris mutex poisoned")
    }
}

/// Build the Deezer CDN cover URL from the `ALB_PICTURE` md5 hash (500x500 HD).
#[allow(dead_code)]
pub fn cover_url(album_picture: &str) -> Option<String> {
    if album_picture.is_empty() {
        return None;
    }
    Some(format!(
        "https://e-cdns-images.dzcdn.net/images/cover/{album_picture}/500x500-000000-80-0-0.jpg"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_controls() {
        let mpris = Mpris::new().expect("MediaControls::new failed");
        drop(mpris);
    }

    #[test]
    fn state_setters_before_attach_report_thread_not_running() {
        let mpris = Mpris::new().expect("MediaControls::new failed");
        let err = mpris
            .set_track("t", "a", "b", Duration::from_secs(10), None)
            .expect_err("no attach yet");
        assert!(
            matches!(err, souvlaki::Error::ThreadNotRunning),
            "unexpected error: {err:?}"
        );
        let err = mpris
            .set_playback(true, Duration::from_secs(1))
            .expect_err("no attach yet");
        assert!(matches!(err, souvlaki::Error::ThreadNotRunning));
        let err = mpris.set_volume(0.5).expect_err("no attach yet");
        assert!(matches!(err, souvlaki::Error::ThreadNotRunning));
    }

    #[test]
    fn cover_url_from_album_picture() {
        assert!(cover_url("").is_none());
        assert_eq!(
            cover_url("abc123"),
            Some(
                "https://e-cdns-images.dzcdn.net/images/cover/abc123/500x500-000000-80-0-0.jpg"
                    .to_string()
            )
        );
    }

    #[test]
    fn attach_and_drop_does_not_panic() {
        let mpris = Mpris::new().expect("MediaControls::new failed");
        // With a session bus: Ok. Headless (no bus): Err(DbusError).
        let attached = mpris.attach(|_event| {}).is_ok();
        if attached {
            mpris
                .set_playback(true, Duration::from_secs(5))
                .expect("service thread running");
        }
        // Drop detaches (joins the service thread).
    }
}
