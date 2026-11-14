**cpal > timestamp**

# Module: timestamp

## Contents

**Structs**

- [`InputCallbackInfo`](#inputcallbackinfo) - Information relevant to a single call to the user's input stream data callback.
- [`InputStreamTimestamp`](#inputstreamtimestamp) - A timestamp associated with a call to an input stream's data callback.
- [`OutputCallbackInfo`](#outputcallbackinfo) - Information relevant to a single call to the user's output stream data callback.
- [`OutputStreamTimestamp`](#outputstreamtimestamp) - A timestamp associated with a call to an output stream's data callback.
- [`StreamInstant`](#streaminstant) - A monotonic time instance associated with a stream, retrieved from either:

---

## cpal::timestamp::InputCallbackInfo

*Struct*

Information relevant to a single call to the user's input stream data callback.

**Methods:**

- `fn new(timestamp: InputStreamTimestamp) -> Self`
- `fn timestamp(self: &Self) -> InputStreamTimestamp` - The timestamp associated with the call to an input stream's data callback.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::timestamp::InputStreamTimestamp

*Struct*

A timestamp associated with a call to an input stream's data callback.

**Fields:**
- `callback: StreamInstant` - The instant the stream's data callback was invoked.
- `capture: StreamInstant` - The instant that data was captured from the device.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpal::timestamp::OutputCallbackInfo

*Struct*

Information relevant to a single call to the user's output stream data callback.

**Methods:**

- `fn new(timestamp: OutputStreamTimestamp) -> Self`
- `fn timestamp(self: &Self) -> OutputStreamTimestamp` - The timestamp associated with the call to an output stream's data callback.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpal::timestamp::OutputStreamTimestamp

*Struct*

A timestamp associated with a call to an output stream's data callback.

**Fields:**
- `callback: StreamInstant` - The instant the stream's data callback was invoked.
- `playback: StreamInstant` - The predicted instant that data written will be delivered to the device for playback.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## cpal::timestamp::StreamInstant

*Struct*

A monotonic time instance associated with a stream, retrieved from either:

1. A timestamp provided to the stream's underlying audio data callback or
2. The same time source used to generate timestamps for a stream's underlying audio data
   callback.

`StreamInstant` represents a moment on a stream's monotonic clock. Because the underlying clock
is monotonic, `StreamInstant` values are always positive and increasing.

Within a single stream, all instants share the same clock, so arithmetic between them is
meaningful. Across different streams, origins are not guaranteed to be shared. On some hosts
each stream starts its own independent clock at zero, so subtracting a timestamp from one
stream and one from another may produce a meaningless result.

## Time sources by host

| Host | Time source |
| ---- | ----------- |
| AAudio | `AAudioStream_getTimestamp(CLOCK_MONOTONIC)` |
| ALSA | `snd_pcm_status_get_htstamp()` |
| ASIO | `timeGetTime()` |
| AudioWorklet | `AudioContext.currentTime` |
| CoreAudio | `mach_absolute_time()` |
| JACK | `jack_get_time()` |
| PipeWire | `pw_stream_get_time_n()` |
| PulseAudio | `std::time::Instant` |
| WASAPI | `QueryPerformanceCounter()` |
| WebAudio | `AudioContext.currentTime` |

> **Disclaimer:** These system calls might change over time.

> **Note:** The `+` and `-` operators on `StreamInstant` may panic if the result cannot be
> represented as a `StreamInstant`. Use [`checked_add`][StreamInstant::checked_add] or
> [`checked_sub`][StreamInstant::checked_sub] for non-panicking variants.

**Methods:**

- `fn checked_duration_since(self: &Self, earlier: StreamInstant) -> Option<Duration>` - Returns the amount of time elapsed from `earlier` to `self`, or `None` if `earlier` is
- `fn saturating_duration_since(self: &Self, earlier: StreamInstant) -> Duration` - Returns the amount of time elapsed from `earlier` to `self`, saturating to
- `fn duration_since(self: &Self, earlier: StreamInstant) -> Duration` - Returns the amount of time elapsed from `earlier` to `self`, saturating to
- `fn checked_add(self: &Self, duration: Duration) -> Option<Self>` - Returns `Some(t)` where `t` is `self + duration`, or `None` if the result cannot be
- `fn checked_sub(self: &Self, duration: Duration) -> Option<Self>` - Returns `Some(t)` where `t` is `self - duration`, or `None` if the result cannot be
- `fn as_nanos(self: &Self) -> u128` - Returns the total number of nanoseconds contained by this `StreamInstant`.
- `fn from_nanos(nanos: u64) -> Self` - Creates a new `StreamInstant` from the specified number of nanoseconds.
- `fn from_millis(millis: u64) -> Self` - Creates a new `StreamInstant` from the specified number of milliseconds.
- `fn from_micros(micros: u64) -> Self` - Creates a new `StreamInstant` from the specified number of microseconds.
- `fn from_secs_f64(secs: f64) -> Self` - Creates a new `StreamInstant` from the specified number of seconds represented as `f64`.
- `fn new(secs: u64, nanos: u32) -> Self` - Creates a new `StreamInstant` from the specified number of whole seconds and additional

**Traits:** Eq, Copy

**Trait Implementations:**

- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Sub**
  - `fn sub(self: Self, rhs: StreamInstant) -> <Self as >::Output` - Returns the duration from `rhs` to `self`, saturating to [`Duration::ZERO`] if `rhs` is
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Add**
  - `fn add(self: Self, rhs: Duration) -> <Self as >::Output` - # Panics
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> <Self as >::Output` - # Panics



