**souvlaki**

# Module: souvlaki

## Contents

**Structs**

- [`MediaMetadata`](#mediametadata) - The metadata of a media item.
- [`MediaPosition`](#mediaposition) - An instant in a media item.

**Enums**

- [`MediaControlEvent`](#mediacontrolevent) - Events sent by the OS media controls.
- [`MediaPlayback`](#mediaplayback) - The status of media playback.
- [`SeekDirection`](#seekdirection) - The direction to seek in.

---

## souvlaki::MediaControlEvent

*Enum*

Events sent by the OS media controls.

**Variants:**
- `Play`
- `Pause`
- `Toggle`
- `Next`
- `Previous`
- `Stop`
- `Seek(SeekDirection)` - Seek forward or backward by an undetermined amount.
- `SeekBy(SeekDirection, std::time::Duration)` - Seek forward or backward by a certain amount.
- `SetPosition(MediaPosition)` - Set the position/progress of the currently playing media item.
- `SetVolume(f64)` - Sets the volume. The value is intended to be from 0.0 to 1.0.
- `SetShuffle(bool)` - Turn shuffle on or off.
- `OpenUri(String)` - Open the URI in the media player.
- `Raise` - Bring the media player's user interface to the front using any appropriate mechanism available.
- `Quit` - Shut down the media player.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## souvlaki::MediaMetadata

*Struct*

The metadata of a media item.

**Generic Parameters:**
- 'a

**Fields:**
- `title: Option<&'a str>`
- `album: Option<&'a str>`
- `artist: Option<&'a str>`
- `cover_url: Option<&'a str>` - Very platform specific. As of now, Souvlaki leaves it up to the user to change the URL depending on the platform.
- `duration: Option<std::time::Duration>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## souvlaki::MediaPlayback

*Enum*

The status of media playback.

**Variants:**
- `Stopped`
- `Paused{ progress: Option<MediaPosition> }`
- `Playing{ progress: Option<MediaPosition> }`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## souvlaki::MediaPosition

*Struct*

An instant in a media item.

**Tuple Struct**: `(std::time::Duration)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## souvlaki::SeekDirection

*Enum*

The direction to seek in.

**Variants:**
- `Forward`
- `Backward`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



