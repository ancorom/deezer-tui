**tokio > io > interest**

# Module: io::interest

## Contents

**Structs**

- [`Interest`](#interest) - Readiness event interest.

---

## tokio::io::interest::Interest

*Struct*

Readiness event interest.

Specifies the readiness events the caller is interested in when awaiting on
I/O resource readiness states.

**Tuple Struct**: `()`

**Methods:**

- `fn is_readable(self: Self) -> bool` - Returns true if the value includes readable interest.
- `fn is_writable(self: Self) -> bool` - Returns true if the value includes writable interest.
- `fn is_error(self: Self) -> bool` - Returns true if the value includes error interest.
- `fn is_priority(self: Self) -> bool` - Returns true if the value includes priority interest.
- `fn add(self: Self, other: Interest) -> Interest` - Add together two `Interest` values.
- `fn remove(self: Self, other: Interest) -> Option<Interest>` - Remove `Interest` from `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, other: Self) -> Self`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



