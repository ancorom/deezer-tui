**tokio > net > unix > socketaddr**

# Module: net::unix::socketaddr

## Contents

**Structs**

- [`SocketAddr`](#socketaddr) - An address associated with a Tokio Unix socket.

---

## tokio::net::unix::socketaddr::SocketAddr

*Struct*

An address associated with a Tokio Unix socket.

This type is a thin wrapper around [`std::os::unix::net::SocketAddr`]. You
can convert to and from the standard library `SocketAddr` type using the
[`From`] trait.

**Tuple Struct**: `()`

**Methods:**

- `fn is_unnamed(self: &Self) -> bool` - Returns `true` if the address is unnamed.
- `fn as_pathname(self: &Self) -> Option<&Path>` - Returns the contents of this address if it is a `pathname` address.
- `fn as_abstract_name(self: &Self) -> Option<&[u8]>` - Returns the contents of this address if it is in the abstract namespace.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: std::os::unix::net::SocketAddr) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



