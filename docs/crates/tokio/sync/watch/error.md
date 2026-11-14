**tokio > sync > watch > error**

# Module: sync::watch::error

## Contents

**Structs**

- [`RecvError`](#recverror) - Error produced when receiving a change notification.
- [`SendError`](#senderror) - Error produced when sending a value fails.

---

## tokio::sync::watch::error::RecvError

*Struct*

Error produced when receiving a change notification.

**Tuple Struct**: `()`

**Traits:** Error

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::watch::error::SendError

*Struct*

Error produced when sending a value fails.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



