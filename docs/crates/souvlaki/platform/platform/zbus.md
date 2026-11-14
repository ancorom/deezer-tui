**souvlaki > platform > platform > zbus**

# Module: platform::platform::zbus

## Contents

**Structs**

- [`MediaControls`](#mediacontrols) - A handle to OS media controls.

---

## souvlaki::platform::platform::zbus::MediaControls

*Struct*

A handle to OS media controls.

**Methods:**

- `fn new(config: PlatformConfig) -> Result<Self, Error>` - Create media controls with the specified config.
- `fn attach<F>(self: & mut Self, event_handler: F) -> Result<(), Error>` - Attach the media control events to a handler.
- `fn detach(self: & mut Self) -> Result<(), Error>` - Detach the event handler.
- `fn set_playback(self: & mut Self, playback: MediaPlayback) -> Result<(), Error>` - Set the current playback status.
- `fn set_metadata(self: & mut Self, metadata: MediaMetadata) -> Result<(), Error>` - Set the metadata of the currently playing media item.
- `fn set_volume(self: & mut Self, volume: f64) -> Result<(), Error>` - Set the volume level (0.0 - 1.0) (Only available on MPRIS)

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



