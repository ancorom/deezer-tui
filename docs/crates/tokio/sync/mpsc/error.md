**tokio > sync > mpsc > error**

# Module: sync::mpsc::error

## Contents

**Structs**

- [`SendError`](#senderror) - Error returned by [`Sender::send`](super::Sender::send).

**Enums**

- [`SendTimeoutError`](#sendtimeouterror) - Error returned by [`Sender::send_timeout`](super::Sender::send_timeout).
- [`TryRecvError`](#tryrecverror) - Error returned by [`Receiver::try_recv`](super::Receiver::try_recv).
- [`TrySendError`](#trysenderror) - Error returned by [`Sender::try_send`](super::Sender::try_send).

---

## tokio::sync::mpsc::error::SendError

*Struct*

Error returned by [`Sender::send`](super::Sender::send).

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::error::SendTimeoutError

*Enum*

Error returned by [`Sender::send_timeout`](super::Sender::send_timeout).

**Generic Parameters:**
- T

**Variants:**
- `Timeout(T)` - The data could not be sent on the channel because the channel is
- `Closed(T)` - The receive half of the channel was explicitly closed or has been

**Methods:**

- `fn into_inner(self: Self) -> T` - Consume the `SendTimeoutError`, returning the unsent value.

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::error::TryRecvError

*Enum*

Error returned by [`Receiver::try_recv`](super::Receiver::try_recv).

**Variants:**
- `Empty` - This **channel** is currently empty, but the **Sender**(s) have not yet
- `Disconnected` - The **channel**'s sending half has become disconnected, and there will

**Traits:** Eq, Copy, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::mpsc::error::TrySendError

*Enum*

Error returned by [`Sender::try_send`](super::Sender::try_send).

**Generic Parameters:**
- T

**Variants:**
- `Full(T)` - The data could not be sent on the channel because the channel is
- `Closed(T)` - The receive half of the channel was explicitly closed or has been

**Methods:**

- `fn into_inner(self: Self) -> T` - Consume the `TrySendError`, returning the unsent value.

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
- **From**
  - `fn from(src: SendError<T>) -> TrySendError<T>`



