**cpal**

# Module: cpal

## Contents

**Modules**

- [`device_description`](#device_description) - Device metadata and description types.
- [`platform`](#platform) - Platform-specific items.
- [`traits`](#traits) - The suite of traits allowing CPAL to abstract over hosts, devices, event loops and stream IDs.

**Macros**

- [`assert_stream_send`](#assert_stream_send) - Compile-time assertion that a stream type implements [`Send`].
- [`assert_stream_sync`](#assert_stream_sync) - Compile-time assertion that a stream type implements [`Sync`].

**Structs**

- [`Data`](#data) - A buffer of dynamically typed audio data, passed to raw stream callbacks.
- [`DeviceId`](#deviceid) - A stable identifier for an audio device across all supported platforms.
- [`StreamConfig`](#streamconfig) - The set of parameters used to describe how to open a stream.
- [`SupportedStreamConfig`](#supportedstreamconfig) - Describes a single supported stream configuration, retrieved via either a
- [`SupportedStreamConfigRange`](#supportedstreamconfigrange) - Describes a range of supported stream configurations, retrieved via the

**Enums**

- [`BufferSize`](#buffersize) - The buffer size requests the callback size for audio streams.
- [`SupportedBufferSize`](#supportedbuffersize) - Describes the minimum and maximum supported buffer size for the device

**Constants**

- [`SAMPLE_RATE_48K`](#sample_rate_48k) - 48 kHz: the EBU/SMPTE broadcast standard, default on most modern hardware.
- [`SAMPLE_RATE_CD`](#sample_rate_cd) - 44.1 kHz: the CD standard, widely used in music production and consumer audio.

**Type Aliases**

- [`ChannelCount`](#channelcount) - Number of channels.
- [`DevicesFiltered`](#devicesfiltered) - Iterator of devices wrapped in a filter to only include certain device types
- [`FrameCount`](#framecount) - A frame represents one sample for each channel. For example, with stereo audio,
- [`InputDevices`](#inputdevices) - A host's device iterator yielding only *input* devices.
- [`OutputDevices`](#outputdevices) - A host's device iterator yielding only *output* devices.
- [`SampleRate`](#samplerate) - The number of samples processed per second for a single channel of audio.

---

## cpal::BufferSize

*Enum*

The buffer size requests the callback size for audio streams.

This controls the approximate size of the audio buffer passed to your callback.
The actual callback size depends on the host/platform implementation and hardware
constraints, and may differ from or vary around the requested size.

## Callback Size Expectations

When you specify [`BufferSize::Fixed(x)`], you are **requesting** that callbacks
receive approximately `x` frames of audio data. However, **no guarantees can be
made** about the actual callback size:

- The host may round to hardware-supported values
- Different devices have different constraints
- The callback size may vary between calls (especially on mobile platforms)
- The actual size might be larger or smaller than requested

## Latency Considerations

[`BufferSize::Default`] uses the host's default buffer size, which may be
surprisingly large, leading to higher latency. If low latency is desired,
[`BufferSize::Fixed`] should be used with a small value in accordance with
the [`SupportedBufferSize`] range from [`SupportedStreamConfig`].

Smaller buffer sizes reduce latency but may increase CPU usage and risk audio
dropouts if the callback cannot process audio quickly enough.

# Example

```no_run
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, SupportedBufferSize};

let host = cpal::default_host();
let device = host.default_output_device().unwrap();
let config = device.default_output_config().unwrap();

// Check supported buffer size range
match config.buffer_size() {
    SupportedBufferSize::Range { min, max } => {
        println!("Buffer size range: {} - {}", min, max);
        // Request a small buffer for low latency
        let mut stream_config = config.config();
        stream_config.buffer_size = BufferSize::Fixed(256);
    }
    SupportedBufferSize::Unknown => {
        // Platform doesn't expose buffer size control
        println!("Buffer size cannot be queried on this platform");
    }
}
```

[`BufferSize::Default`]: BufferSize::Default
[`BufferSize::Fixed`]: BufferSize::Fixed
[`BufferSize::Fixed(x)`]: BufferSize::Fixed
[`SupportedBufferSize`]: SupportedStreamConfig::buffer_size
[`SupportedStreamConfig`]: SupportedStreamConfig

**Variants:**
- `Default`
- `Fixed(FrameCount)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::ChannelCount

*Type Alias*: `u16`

Number of channels.



## cpal::Data

*Struct*

A buffer of dynamically typed audio data, passed to raw stream callbacks.

Raw input stream callbacks receive `&Data`, while raw output stream callbacks expect `&mut Data`.

**Methods:**

- `fn from_parts(data: *mut (), len: usize, sample_format: SampleFormat) -> Self` - Constructor for host implementations to use.
- `fn sample_format(self: &Self) -> SampleFormat` - The sample format of the internal audio data.
- `fn len(self: &Self) -> usize` - The full length of the buffer in samples.
- `fn bytes(self: &Self) -> &[u8]` - The raw slice of memory representing the underlying audio data as a slice of bytes.
- `fn bytes_mut(self: & mut Self) -> & mut [u8]` - The raw slice of memory representing the underlying audio data as a slice of bytes.
- `fn as_slice<T>(self: &Self) -> Option<&[T]>` - Access the data as a slice of sample type `T`.
- `fn as_slice_mut<T>(self: & mut Self) -> Option<& mut [T]>` - Access the data as a slice of sample type `T`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpal::DeviceId

*Struct*

A stable identifier for an audio device across all supported platforms.

Device IDs should remain stable across application restarts and can be serialized using `Display`/`FromStr`.

A device ID consists of a [`HostId`] identifying the audio backend and a device-specific identifier string.

# Example

```no_run
use cpal::traits::{HostTrait, DeviceTrait};
use cpal::DeviceId;
use std::str::FromStr;

let host = cpal::default_host();
let device = host.default_output_device().unwrap();
let device_id = device.id().unwrap();

// Serialize to string (e.g., for storage in config file)
let id_string = device_id.to_string();
println!("Device ID: {}", id_string); // e.g., "wasapi:device_identifier"

// Deserialize from string
match DeviceId::from_str(&id_string) {
    Ok(parsed_id) => {
        // Retrieve the device by its ID
        if let Some(device) = host.device_by_id(&parsed_id) {
            println!("Found device: {:?}", device.id());
        }
    }
    Err(e) => eprintln!("Failed to parse device ID: {}", e),
}
```

**Tuple Struct**: `()`

**Methods:**

- `fn new<impl AsRef<str>>(host: crate::platform::HostId, id: impl Trait) -> Self` - Creates a `DeviceId` from a host identifier and a device-specific identifier string.
- `fn host(self: &Self) -> crate::platform::HostId` - Returns the host this device belongs to.
- `fn id(self: &Self) -> &str` - Returns the backend-specific device identifier string.

**Traits:** Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>` - Parse a device identifier from its string representation (e.g. `"alsa:hw:0,0"`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpal::DevicesFiltered

*Type Alias*: `std::iter::Filter<I, fn(...)>`

Iterator of devices wrapped in a filter to only include certain device types



## cpal::FrameCount

*Type Alias*: `u32`

A frame represents one sample for each channel. For example, with stereo audio,
one frame contains two samples (left and right channels).



## cpal::InputDevices

*Type Alias*: `DevicesFiltered<I>`

A host's device iterator yielding only *input* devices.



## cpal::OutputDevices

*Type Alias*: `DevicesFiltered<I>`

A host's device iterator yielding only *output* devices.



## cpal::SAMPLE_RATE_48K

*Constant*: `SampleRate`

48 kHz: the EBU/SMPTE broadcast standard, default on most modern hardware.



## cpal::SAMPLE_RATE_CD

*Constant*: `SampleRate`

44.1 kHz: the CD standard, widely used in music production and consumer audio.



## cpal::SampleRate

*Type Alias*: `u32`

The number of samples processed per second for a single channel of audio.



## cpal::StreamConfig

*Struct*

The set of parameters used to describe how to open a stream.

The sample format is omitted in favour of using a sample type.

See also [`BufferSize`] for details on buffer size behavior and latency considerations.

**Fields:**
- `channels: ChannelCount`
- `sample_rate: SampleRate`
- `buffer_size: BufferSize`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(conf: SupportedStreamConfig) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::SupportedBufferSize

*Enum*

Describes the minimum and maximum supported buffer size for the device

**Variants:**
- `Range{ min: FrameCount, max: FrameCount }`
- `Unknown` - In the case that the platform provides no way of getting the default

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`



## cpal::SupportedStreamConfig

*Struct*

Describes a single supported stream configuration, retrieved via either a
[`SupportedStreamConfigRange`] instance or one of the
[`Device::default_input/output_config`](traits::DeviceTrait#required-methods) methods.

**Methods:**

- `fn new(channels: ChannelCount, sample_rate: SampleRate, buffer_size: SupportedBufferSize, sample_format: SampleFormat) -> Self`
- `fn channels(self: &Self) -> ChannelCount`
- `fn sample_rate(self: &Self) -> SampleRate`
- `fn buffer_size(self: &Self) -> &SupportedBufferSize`
- `fn sample_format(self: &Self) -> SampleFormat`
- `fn config(self: &Self) -> StreamConfig`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::SupportedStreamConfigRange

*Struct*

Describes a range of supported stream configurations, retrieved via the
[`Device::supported_input/output_configs`](traits::DeviceTrait#required-methods) method.

**Methods:**

- `fn new(channels: ChannelCount, min_sample_rate: SampleRate, max_sample_rate: SampleRate, buffer_size: SupportedBufferSize, sample_format: SampleFormat) -> Self`
- `fn channels(self: &Self) -> ChannelCount`
- `fn min_sample_rate(self: &Self) -> SampleRate`
- `fn max_sample_rate(self: &Self) -> SampleRate`
- `fn buffer_size(self: &Self) -> &SupportedBufferSize`
- `fn sample_format(self: &Self) -> SampleFormat`
- `fn with_sample_rate(self: Self, sample_rate: SampleRate) -> SupportedStreamConfig` - Retrieve a [`SupportedStreamConfig`] with the given sample rate.
- `fn try_with_sample_rate(self: Self, sample_rate: SampleRate) -> Option<SupportedStreamConfig>` - Retrieve a [`SupportedStreamConfig`] with the given sample rate.
- `fn with_max_sample_rate(self: Self) -> SupportedStreamConfig` - Turns this [`SupportedStreamConfigRange`] into a [`SupportedStreamConfig`] corresponding to the maximum sample rate.
- `fn contains_rate(self: &Self, rate: SampleRate) -> bool` - Returns `true` if `rate` falls within `[min_sample_rate, max_sample_rate]`.
- `fn try_with_standard_sample_rate(self: Self) -> Option<SupportedStreamConfig>` - Retrieve a [`SupportedStreamConfig`] with a standard sample rate.
- `fn with_standard_sample_rate(self: Self) -> SupportedStreamConfig` - Retrieve a [`SupportedStreamConfig`] with a standard sample rate.
- `fn cmp_default_heuristics(self: &Self, other: &Self) -> std::cmp::Ordering` - A comparison function which compares two [`SupportedStreamConfigRange`]s in terms of their

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpal::assert_stream_send

*Declarative Macro*

Compile-time assertion that a stream type implements [`Send`].

Custom host implementations should use this macro to verify their `Stream` type
can be safely transferred between threads, as required by CPAL's API.

# Example

```
use cpal::assert_stream_send;
struct MyStream { /* ... */ }
assert_stream_send!(MyStream);
```

```rust
macro_rules! assert_stream_send {
    ($t:ty) => { ... };
}
```



## cpal::assert_stream_sync

*Declarative Macro*

Compile-time assertion that a stream type implements [`Sync`].

Custom host implementations should use this macro to verify their `Stream` type
can be safely shared between threads, as required by CPAL's API.

# Example

```
use cpal::assert_stream_sync;
struct MyStream { /* ... */ }
assert_stream_sync!(MyStream);
```

```rust
macro_rules! assert_stream_sync {
    ($t:ty) => { ... };
}
```



## Module: device_description

Device metadata and description types.

This module provides structured information about audio devices including manufacturer,
device type, interface type, and connection details. Not all backends provide complete
information - availability depends on platform capabilities.



## Module: platform

Platform-specific items.

This module also contains the implementation of the platform's dynamically dispatched [`Host`]
type and its associated [`Device`], [`Stream`] and other associated types. These
types are useful in the case that users require switching between audio host APIs at runtime.



## Module: traits

The suite of traits allowing CPAL to abstract over hosts, devices, event loops and stream IDs.

# Custom Host Implementations

When implementing custom hosts with the `custom` feature, use the [`assert_stream_send!`](crate::assert_stream_send)
and [`assert_stream_sync!`](crate::assert_stream_sync) macros to verify your `Stream` type meets CPAL's requirements.



