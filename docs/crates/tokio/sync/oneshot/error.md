**tokio > sync > oneshot > error**

# Module: sync::oneshot::error

## Contents

**Structs**

- [`RecvError`](#recverror) - Error returned by the `Future` implementation for `Receiver`.

**Enums**

- [`TryRecvError`](#tryrecverror) - Error returned by the `try_recv` function on `Receiver`.

---

## tokio::sync::oneshot::error::RecvError

*Struct*

Error returned by the `Future` implementation for `Receiver`.

This error is returned by the receiver when the sender is dropped without sending.

**Tuple Struct**: `()`

**Traits:** Eq, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tokio::sync::oneshot::error::TryRecvError

*Enum*

Error returned by the `try_recv` function on `Receiver`.

**Variants:**
- `Empty` - The send half of the channel has not yet sent a value.
- `Closed` - The send half of the channel was dropped without sending a value.

**Traits:** Error, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



