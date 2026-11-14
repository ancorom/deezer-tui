**reqwest > dns > resolve**

# Module: dns::resolve

## Contents

**Structs**

- [`Name`](#name) - A name that must be resolved to addresses.

**Traits**

- [`IntoResolve`](#intoresolve) - A more general trait implemented for types implementing `Resolve`.
- [`Resolve`](#resolve) - Trait for customizing DNS resolution in reqwest.

**Type Aliases**

- [`Addrs`](#addrs) - Alias for an `Iterator` trait object over `SocketAddr`.
- [`Resolving`](#resolving) - Alias for the `Future` type returned by a DNS resolver.

---

## reqwest::dns::resolve::Addrs

*Type Alias*: `Box<dyn Iterator>`

Alias for an `Iterator` trait object over `SocketAddr`.



## reqwest::dns::resolve::IntoResolve

*Trait*

A more general trait implemented for types implementing `Resolve`.

Unnameable, only exported to aid seeing what implements this.



## reqwest::dns::resolve::Name

*Struct*

A name that must be resolved to addresses.

**Tuple Struct**: `()`

**Methods:**

- `fn as_str(self: &Self) -> &str` - View the name as a string.

**Trait Implementations:**

- **FromStr**
  - `fn from_str(host: &str) -> Result<Self, <Self as >::Err>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## reqwest::dns::resolve::Resolve

*Trait*

Trait for customizing DNS resolution in reqwest.

**Methods:**

- `resolve`: Performs DNS resolution on a `Name`.



## reqwest::dns::resolve::Resolving

*Type Alias*: `std::pin::Pin<Box<dyn Future>>`

Alias for the `Future` type returned by a DNS resolver.



