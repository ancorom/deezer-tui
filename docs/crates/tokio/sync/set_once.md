**tokio > sync > set_once**

# Module: sync::set_once

## Contents

**Structs**

- [`SetOnce`](#setonce) - A thread-safe cell that can be written to only once.
- [`SetOnceError`](#setonceerror) - Error that can be returned from [`SetOnce::set`].

---

## tokio::sync::set_once::SetOnce

*Struct*

A thread-safe cell that can be written to only once.

A `SetOnce` is inspired from python's [`asyncio.Event`] type. It can be
used to wait until the value of the `SetOnce` is set like a "Event" mechanism.

# Example

```
use tokio::sync::{SetOnce, SetOnceError};

static ONCE: SetOnce<u32> = SetOnce::const_new();

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), SetOnceError<u32>> {

// set the value inside a task somewhere...
tokio::spawn(async move { ONCE.set(20) });

// checking with .get doesn't block main thread
println!("{:?}", ONCE.get());

// wait until the value is set, blocks the thread
println!("{:?}", ONCE.wait().await);

Ok(())
# }
```

A `SetOnce` is typically used for global variables that need to be
initialized once on first use, but need no further changes. The `SetOnce`
in Tokio allows the initialization procedure to be asynchronous.

# Example

```
use tokio::sync::{SetOnce, SetOnceError};
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), SetOnceError<u32>> {
let once = SetOnce::new();

let arc = Arc::new(once);
let first_cl = Arc::clone(&arc);
let second_cl = Arc::clone(&arc);

// set the value inside a task
tokio::spawn(async move { first_cl.set(20) }).await.unwrap()?;

// wait inside task to not block the main thread
tokio::spawn(async move {
    // wait inside async context for the value to be set
    assert_eq!(*second_cl.wait().await, 20);
}).await.unwrap();

// subsequent set calls will fail
assert!(arc.set(30).is_err());

println!("{:?}", arc.get());

Ok(())
# }
```

[`asyncio.Event`]: https://docs.python.org/3/library/asyncio-sync.html#asyncio.Event

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Creates a new empty `SetOnce` instance.
- `fn const_new() -> Self` - Creates a new empty `SetOnce` instance.
- `fn new_with(value: Option<T>) -> Self` - Creates a new `SetOnce` that contains the provided value, if any.
- `fn const_new_with(value: T) -> Self` - Creates a new `SetOnce` that contains the provided value.
- `fn initialized(self: &Self) -> bool` - Returns `true` if the `SetOnce` currently contains a value, and `false`
- `fn get(self: &Self) -> Option<&T>` - Returns a reference to the value currently stored in the `SetOnce`, or
- `fn set(self: &Self, value: T) -> Result<(), SetOnceError<T>>` - Sets the value of the `SetOnce` to the given value if the `SetOnce` is
- `fn into_inner(self: Self) -> Option<T>` - Takes the value from the cell, destroying the cell in the process.
- `fn wait(self: &Self) -> &T` - Waits until the value is set.

**Traits:** Send, Sync, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> SetOnce<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &SetOnce<T>) -> bool`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: T) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> SetOnce<T>`



## tokio::sync::set_once::SetOnceError

*Struct*

Error that can be returned from [`SetOnce::set`].

This error means that the `SetOnce` was already initialized when
set was called

[`SetOnce::set`]: crate::sync::SetOnce::set

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



