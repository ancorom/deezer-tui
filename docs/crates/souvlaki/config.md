**souvlaki > config**

# Module: config

## Contents

**Structs**

- [`PlatformConfig`](#platformconfig) - OS-specific configuration needed to create media controls.

---

## souvlaki::config::PlatformConfig

*Struct*

OS-specific configuration needed to create media controls.

**Generic Parameters:**
- 'a

**Fields:**
- `display_name: &'a str` - The name to be displayed to the user. (*Required on Linux*)
- `dbus_name: &'a str` - Should follow [the D-Bus spec](https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus). (*Required on Linux*)
- `hwnd: Option<*mut std::ffi::c_void>` - An HWND. (*Required on Windows*)

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



