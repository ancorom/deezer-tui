**tokio > sync > broadcast > error**

# Module: sync::broadcast::error

## Contents

**Structs**

- [`SendError`](#senderror) - Error returned by the [`send`] function on a [`Sender`].

**Enums**

- [`RecvError`](#recverror) - An error returned from the [`recv`] function on a [`Receiver`].
- [`TryRecvError`](#tryrecverror) - An error returned from the [`try_recv`] function on a [`Receiver`].

---

## tokio::sync::broadcast::error::RecvError

*Enum*

An error returned from the [`recv`] function on a [`Receiver`].

[`recv`]: crate::sync::broadcast::Receiver::recv
[`Receiver`]: crate::sync::broadcast::Receiver

**Variants:**
- `Closed` - There are no more active senders implying no further messages will ever
- `Lagged(u64)` - The receiver lagged too far behind: one or more messages were

**Traits:** Eq, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tokio::sync::broadcast::error::SendError

*Struct*

Error returned by the [`send`] function on a [`Sender`].

A **send** operation can only fail if there are no active receivers,
implying that the message could never be received. The error contains the
message being sent as a payload so it can be recovered.

[`send`]: crate::sync::broadcast::Sender::send
[`Sender`]: crate::sync::broadcast::Sender

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::broadcast::error::TryRecvError

*Enum*

An error returned from the [`try_recv`] function on a [`Receiver`].

[`try_recv`]: crate::sync::broadcast::Receiver::try_recv
[`Receiver`]: crate::sync::broadcast::Receiver

**Variants:**
- `Empty` - The channel is currently empty. There are still active
- `Closed` - There are no more active senders implying no further messages will ever
- `Lagged(u64)` - The receiver lagged too far behind: one or more messages were

**Traits:** Eq, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



