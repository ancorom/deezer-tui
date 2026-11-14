**souvlaki > platform > platform**

# Module: platform::platform

## Contents

**Enums**

- [`Error`](#error) - A platform-specific error.

---

## souvlaki::platform::platform::Error

*Enum*

A platform-specific error.

**Variants:**
- `DbusError(zbus_crate::Error)`
- `ThreadNotRunning`
- `ThreadPanicked`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(source: zbus_crate::Error) -> Self`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private21::Error>`



