**tokio > runtime > task > error**

# Module: runtime::task::error

## Contents

**Structs**

- [`JoinError`](#joinerror) - Task failed to execute to completion.

---

## tokio::runtime::task::error::JoinError

*Struct*

Task failed to execute to completion.

**Methods:**

- `fn is_cancelled(self: &Self) -> bool` - Returns true if the error was caused by the task being cancelled.
- `fn is_panic(self: &Self) -> bool` - Returns true if the error was caused by the task panicking.
- `fn into_panic(self: Self) -> Box<dyn Any>` - Consumes the join error, returning the object with which the task panicked.
- `fn try_into_panic(self: Self) -> Result<Box<dyn Any>, JoinError>` - Consumes the join error, returning the object with which the task
- `fn id(self: &Self) -> Id` - Returns a [task ID] that identifies the task which errored relative to

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



