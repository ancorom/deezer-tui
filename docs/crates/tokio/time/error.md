**tokio > time > error**

# Module: time::error

## Contents

**Structs**

- [`Elapsed`](#elapsed) - Errors returned by `Timeout`.
- [`Error`](#error) - Errors encountered by the timer implementation.

---

## tokio::time::error::Elapsed

*Struct*

Errors returned by `Timeout`.

This error is returned when a timeout expires before the function was able
to finish.

**Tuple Struct**: `()`

**Traits:** Error, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::time::error::Error

*Struct*

Errors encountered by the timer implementation.

Currently, there are two different errors that can occur:

* `shutdown` occurs when a timer operation is attempted, but the timer
  instance has been dropped. In this case, the operation will never be able
  to complete and the `shutdown` error is returned. This is a permanent
  error, i.e., once this error is observed, timer operations will never
  succeed in the future.

* `at_capacity` occurs when a timer operation is attempted, but the timer
  instance is currently handling its maximum number of outstanding sleep instances.
  In this case, the operation is not able to be performed at the current
  moment, and `at_capacity` is returned. This is a transient error, i.e., at
  some point in the future, if the operation is attempted again, it might
  succeed. Callers that observe this error should attempt to [shed load]. One
  way to do this would be dropping the future that issued the timer operation.

[shed load]: https://en.wikipedia.org/wiki/Load_Shedding

**Tuple Struct**: `()`

**Methods:**

- `fn shutdown() -> Error` - Creates an error representing a shutdown timer.
- `fn is_shutdown(self: &Self) -> bool` - Returns `true` if the error was caused by the timer being shutdown.
- `fn at_capacity() -> Error` - Creates an error representing a timer at capacity.
- `fn is_at_capacity(self: &Self) -> bool` - Returns `true` if the error was caused by the timer being at capacity.
- `fn invalid() -> Error` - Creates an error representing a misconfigured timer.
- `fn is_invalid(self: &Self) -> bool` - Returns `true` if the error was caused by the timer being misconfigured.

**Traits:** Copy, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



