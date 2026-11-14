**tokio > sync > batch_semaphore**

# Module: sync::batch_semaphore

## Contents

**Structs**

- [`AcquireError`](#acquireerror) - Error returned from the [`Semaphore::acquire`] function.

**Enums**

- [`TryAcquireError`](#tryacquireerror) - Error returned from the [`Semaphore::try_acquire`] function.

---

## tokio::sync::batch_semaphore::AcquireError

*Struct*

Error returned from the [`Semaphore::acquire`] function.

An `acquire` operation can only fail if the semaphore has been
[closed].

[closed]: crate::sync::Semaphore::close
[`Semaphore::acquire`]: crate::sync::Semaphore::acquire

**Tuple Struct**: `()`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::batch_semaphore::TryAcquireError

*Enum*

Error returned from the [`Semaphore::try_acquire`] function.

[`Semaphore::try_acquire`]: crate::sync::Semaphore::try_acquire

**Variants:**
- `Closed` - The semaphore has been [closed] and cannot issue new permits.
- `NoPermits` - The semaphore has no available permits.

**Traits:** Eq, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



