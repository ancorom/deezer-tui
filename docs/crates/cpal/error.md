**cpal > error**

# Module: error

## Contents

**Structs**

- [`Error`](#error) - Error type for all CPAL operations.

**Enums**

- [`ErrorKind`](#errorkind) - A list specifying general categories of CPAL error.

---

## cpal::error::Error

*Struct*

Error type for all CPAL operations.

**Methods:**

- `fn new(kind: ErrorKind) -> Self` - Create a new error with the given kind and no message.
- `fn with_message<impl Into<Cow<'static, str>>>(kind: ErrorKind, message: impl Trait) -> Self` - Create a new error with the given kind and a human-readable message.
- `fn kind(self: &Self) -> ErrorKind` - Returns the error kind.
- `fn message(self: &Self) -> Option<&str>` - Returns the human-readable message, if any.

**Traits:** Eq, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(kind: ErrorKind) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(err: alsa::Error) -> Self`



## cpal::error::ErrorKind

*Enum*

A list specifying general categories of CPAL error.

**Variants:**
- `DeviceBusy` - The device is temporarily busy. This can happen when another application or stream
- `DeviceChanged` - The active audio route changed and the stream was automatically rerouted.
- `DeviceNotAvailable` - The requested audio device is not available.
- `HostUnavailable` - The audio host (server or subsystem) is not available on this system.
- `InvalidInput` - Invalid input or argument.
- `PermissionDenied` - Access to the device or resource was denied by the operating system or audio subsystem.
- `RealtimeDenied` - A real-time scheduling promotion or equivalent platform performance mode was refused.
- `ResourceExhausted` - An OS resource limit was reached, such as a system or process thread or memory limit.
- `StreamInvalidated` - The stream configuration is no longer valid and must be rebuilt.
- `UnsupportedConfig` - The requested stream configuration is not supported. This includes unsupported sample
- `UnsupportedOperation` - The requested operation is not supported. This includes unsupported stream directions
- `Xrun` - A buffer underrun or overrun occurred, causing a potential audio glitch.
- `BackendError` - The underlying platform audio API returned an error that CPAL cannot map to a more
- `Other` - A catch-all for errors that do not fall under any other CPAL error kind.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



