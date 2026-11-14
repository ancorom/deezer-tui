**tracing_subscriber > registry > extensions**

# Module: registry::extensions

## Contents

**Structs**

- [`Extensions`](#extensions) - An immutable, read-only reference to a Span's extensions.
- [`ExtensionsMut`](#extensionsmut) - An mutable reference to a Span's extensions.

---

## tracing_subscriber::registry::extensions::Extensions

*Struct*

An immutable, read-only reference to a Span's extensions.

**Generic Parameters:**
- 'a

**Methods:**

- `fn get<T>(self: &Self) -> Option<&T>` - Immutably borrows a type previously inserted into this `Extensions`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::registry::extensions::ExtensionsMut

*Struct*

An mutable reference to a Span's extensions.

**Generic Parameters:**
- 'a

**Methods:**

- `fn insert<T>(self: & mut Self, val: T)` - Insert a type into this `Extensions`.
- `fn replace<T>(self: & mut Self, val: T) -> Option<T>` - Replaces an existing `T` into this extensions.
- `fn get_mut<T>(self: & mut Self) -> Option<& mut T>` - Get a mutable reference to a type previously inserted on this `ExtensionsMut`.
- `fn remove<T>(self: & mut Self) -> Option<T>` - Remove a type from this `Extensions`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



