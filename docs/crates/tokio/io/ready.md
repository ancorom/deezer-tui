**tokio > io > ready**

# Module: io::ready

## Contents

**Structs**

- [`Ready`](#ready) - Describes the readiness state of an I/O resources.

---

## tokio::io::ready::Ready

*Struct*

Describes the readiness state of an I/O resources.

`Ready` tracks which operation an I/O resource is ready to perform.

**Tuple Struct**: `()`

**Methods:**

- `fn is_empty(self: Self) -> bool` - Returns true if `Ready` is the empty set.
- `fn is_readable(self: Self) -> bool` - Returns `true` if the value includes `readable`.
- `fn is_writable(self: Self) -> bool` - Returns `true` if the value includes writable `readiness`.
- `fn is_read_closed(self: Self) -> bool` - Returns `true` if the value includes read-closed `readiness`.
- `fn is_write_closed(self: Self) -> bool` - Returns `true` if the value includes write-closed `readiness`.
- `fn is_priority(self: Self) -> bool` - Returns `true` if the value includes priority `readiness`.
- `fn is_error(self: Self) -> bool` - Returns `true` if the value includes error `readiness`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **BitOr**
  - `fn bitor(self: Self, other: Ready) -> Ready`
- **Sub**
  - `fn sub(self: Self, other: Ready) -> Ready`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Ready)`
- **BitAnd**
  - `fn bitand(self: Self, other: Ready) -> Ready`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



