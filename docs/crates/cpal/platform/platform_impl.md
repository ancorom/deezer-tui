**cpal > platform > platform_impl**

# Module: platform::platform_impl

## Contents

**Structs**

- [`Device`](#device) - The `Device` implementation associated with the platform's dynamically dispatched
- [`Devices`](#devices) - The `Devices` iterator associated with the platform's dynamically dispatched [`Host`]
- [`Host`](#host) - The platform's dynamically dispatched `Host` type.
- [`Stream`](#stream) - The `Stream` implementation associated with the platform's dynamically dispatched
- [`SupportedInputConfigs`](#supportedinputconfigs) - The `SupportedInputConfigs` iterator associated with the platform's dynamically
- [`SupportedOutputConfigs`](#supportedoutputconfigs) - The `SupportedOutputConfigs` iterator associated with the platform's dynamically

**Enums**

- [`HostId`](#hostid) - Unique identifier for available hosts on the platform.

**Functions**

- [`available_hosts`](#available_hosts) - Produces a list of hosts that are currently available on the system.
- [`default_host`](#default_host) - The default host for the current compilation target platform.
- [`host_from_id`](#host_from_id) - Given a unique host identifier, initialise and produce the host if it is available.

**Constants**

- [`ALL_HOSTS`](#all_hosts) - All hosts supported by CPAL on this platform.

---

## cpal::platform::platform_impl::ALL_HOSTS

*Constant*: `&'static [HostId]`

All hosts supported by CPAL on this platform.



## cpal::platform::platform_impl::Device

*Struct*

The `Device` implementation associated with the platform's dynamically dispatched
[`Host`] type.

**Tuple Struct**: `()`

**Methods:**

- `fn as_inner(self: &Self) -> &DeviceInner` - Returns a reference to the underlying platform-specific [`DeviceInner`].
- `fn as_inner_mut(self: & mut Self) -> & mut DeviceInner` - Returns a mutable reference to the underlying platform-specific [`DeviceInner`].
- `fn into_inner(self: Self) -> DeviceInner` - Consumes this `Device`, returning the underlying platform-specific [`DeviceInner`].

**Traits:** Eq

**Trait Implementations:**

- **DeviceTrait**
  - `fn description(self: &Self) -> Result<crate::DeviceDescription, crate::Error>`
  - `fn id(self: &Self) -> Result<crate::DeviceId, crate::Error>`
  - `fn supports_input(self: &Self) -> bool`
  - `fn supports_output(self: &Self) -> bool`
  - `fn supported_input_configs(self: &Self) -> Result<<Self as >::SupportedInputConfigs, crate::Error>`
  - `fn supported_output_configs(self: &Self) -> Result<<Self as >::SupportedOutputConfigs, crate::Error>`
  - `fn default_input_config(self: &Self) -> Result<crate::SupportedStreamConfig, crate::Error>`
  - `fn default_output_config(self: &Self) -> Result<crate::SupportedStreamConfig, crate::Error>`
  - `fn build_input_stream_raw<D, E>(self: &Self, config: crate::StreamConfig, sample_format: crate::SampleFormat, data_callback: D, error_callback: E, timeout: Option<std::time::Duration>) -> Result<<Self as >::Stream, crate::Error>`
  - `fn build_output_stream_raw<D, E>(self: &Self, config: crate::StreamConfig, sample_format: crate::SampleFormat, data_callback: D, error_callback: E, timeout: Option<std::time::Duration>) -> Result<<Self as >::Stream, crate::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Device) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::platform::platform_impl::Devices

*Struct*

The `Devices` iterator associated with the platform's dynamically dispatched [`Host`]
type.

**Tuple Struct**: `()`

**Methods:**

- `fn as_inner(self: &Self) -> &DevicesInner` - Returns a reference to the underlying platform-specific [`DevicesInner`].
- `fn as_inner_mut(self: & mut Self) -> & mut DevicesInner` - Returns a mutable reference to the underlying platform-specific [`DevicesInner`].
- `fn into_inner(self: Self) -> DevicesInner` - Consumes this `Devices`, returning the underlying platform-specific [`DevicesInner`].

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## cpal::platform::platform_impl::Host

*Struct*

The platform's dynamically dispatched `Host` type.

An instance of this `Host` type may represent one of the `Host`s available
on the platform.

Use this type if you require switching between available hosts at runtime.

This type may be constructed via the [`host_from_id`] function. [`HostId`]s may
be acquired via the [`ALL_HOSTS`] const, and the [`available_hosts`] function.

**Tuple Struct**: `()`

**Methods:**

- `fn id(self: &Self) -> HostId` - The unique identifier associated with this `Host`.
- `fn as_inner(self: &Self) -> &HostInner` - Returns a reference to the underlying platform-specific [`HostInner`].
- `fn as_inner_mut(self: & mut Self) -> & mut HostInner` - Returns a mutable reference to the underlying platform-specific [`HostInner`].
- `fn into_inner(self: Self) -> HostInner` - Consumes this `Host`, returning the underlying platform-specific [`HostInner`].

**Trait Implementations:**

- **HostTrait**
  - `fn is_available() -> bool`
  - `fn devices(self: &Self) -> Result<<Self as >::Devices, crate::Error>`
  - `fn device_by_id(self: &Self, id: &crate::DeviceId) -> Option<<Self as >::Device>`
  - `fn default_input_device(self: &Self) -> Option<<Self as >::Device>`
  - `fn default_output_device(self: &Self) -> Option<<Self as >::Device>`
- **Default**
  - `fn default() -> Host`



## cpal::platform::platform_impl::HostId

*Enum*

Unique identifier for available hosts on the platform.

Only the hosts supported by the current platform are available as enum variants.
For cross-platform code that needs to handle hosts from other platforms,
use the string representation via [`std::fmt::Display`]/[`std::str::FromStr`].

# Available Host Strings

For cross-platform matching, these host strings are available:

- `"aaudio"` - Android Audio
- `"alsa"` - Advanced Linux Sound Architecture
- `"asio"` - ASIO
- `"audioworklet"` - Audio Worklet
- `"coreaudio"` - CoreAudio
- `"custom"` - Custom host (requires `custom` feature)
- `"jack"` - JACK Audio Connection Kit
- `"null"` - Null host
- `"wasapi"` - Windows Audio Session API
- `"webaudio"` - Web Audio API

# Cross-Platform Example

```
use cpal::HostId;
use std::str::FromStr;

fn handle_host_string(host_string: &str) {
    // String matching works on all platforms
    match host_string {
        "alsa" => println!("ALSA host"),
        "coreaudio" => println!("CoreAudio host"),
        "jack" => println!("JACK host"),
        "wasapi" => println!("WASAPI host"),
        "asio" => println!("ASIO host"),
        "aaudio" => println!("AAudio host"),
        _ => println!("Other host"),
    }

    // Parse host string (may fail if host is not available on this platform)
    if let Ok(host_id) = HostId::from_str(host_string) {
        println!("Successfully parsed: {}", host_id);
    }
}
```

**Variants:**
- `Alsa`

**Methods:**

- `fn name(self: &Self) -> &'static str` - Returns the human-readable host name.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>` - Parse a host identifier from its string representation (e.g. `"alsa"`,
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::platform::platform_impl::Stream

*Struct*

The `Stream` implementation associated with the platform's dynamically dispatched
[`Host`] type.

**Tuple Struct**: `()`

**Methods:**

- `fn as_inner(self: &Self) -> &StreamInner` - Returns a reference to the underlying platform-specific [`StreamInner`].
- `fn as_inner_mut(self: & mut Self) -> & mut StreamInner` - Returns a mutable reference to the underlying platform-specific [`StreamInner`].
- `fn into_inner(self: Self) -> StreamInner` - Consumes this `Stream`, returning the underlying platform-specific [`StreamInner`].

**Trait Implementations:**

- **StreamTrait**
  - `fn play(self: &Self) -> Result<(), crate::Error>`
  - `fn pause(self: &Self) -> Result<(), crate::Error>`
  - `fn buffer_size(self: &Self) -> Result<crate::FrameCount, crate::Error>`
  - `fn now(self: &Self) -> crate::StreamInstant`



## cpal::platform::platform_impl::SupportedInputConfigs

*Struct*

The `SupportedInputConfigs` iterator associated with the platform's dynamically
dispatched [`Host`] type.

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## cpal::platform::platform_impl::SupportedOutputConfigs

*Struct*

The `SupportedOutputConfigs` iterator associated with the platform's dynamically
dispatched [`Host`] type.

**Tuple Struct**: `()`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::platform::platform_impl::available_hosts

*Function*

Produces a list of hosts that are currently available on the system.

```rust
fn available_hosts() -> Vec<HostId>
```



## cpal::platform::platform_impl::default_host

*Function*

The default host for the current compilation target platform.

```rust
fn default_host() -> Host
```



## cpal::platform::platform_impl::host_from_id

*Function*

Given a unique host identifier, initialise and produce the host if it is available.

# Errors

- [`ErrorKind::HostUnavailable`] if the host identified by `id` is not currently
  reachable (e.g. the audio daemon is not running).
- [`ErrorKind::BackendError`] for unclassifiable initialization failures.

[`ErrorKind::HostUnavailable`]: crate::ErrorKind::HostUnavailable
[`ErrorKind::BackendError`]: crate::ErrorKind::BackendError

```rust
fn host_from_id(id: HostId) -> Result<Host, crate::Error>
```



