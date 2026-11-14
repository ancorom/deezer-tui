**toml > ser > document > buffer**

# Module: ser::document::buffer

## Contents

**Structs**

- [`Buffer`](#buffer) - TOML Document serialization buffer

---

## toml::ser::document::buffer::Buffer

*Struct*

TOML Document serialization buffer

**Methods:**

- `fn new() -> Self` - Initialize a new serialization buffer
- `fn clear(self: & mut Self)` - Reset the buffer for serializing another document

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



