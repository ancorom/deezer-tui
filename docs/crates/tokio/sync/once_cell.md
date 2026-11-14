**tokio > sync > once_cell**

# Module: sync::once_cell

## Contents

**Structs**

- [`OnceCell`](#oncecell) - A thread-safe cell that can be written to only once.

**Enums**

- [`SetError`](#seterror) - Errors that can be returned from [`OnceCell::set`].

---

## tokio::sync::once_cell::OnceCell

*Struct*

A thread-safe cell that can be written to only once.

A `OnceCell` is typically used for global variables that need to be
initialized once on first use, but need no further changes. The `OnceCell`
in Tokio allows the initialization procedure to be asynchronous.

# Examples

```
use tokio::sync::OnceCell;

async fn some_computation() -> u32 {
    1 + 1
}

static ONCE: OnceCell<u32> = OnceCell::const_new();

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let result = ONCE.get_or_init(some_computation).await;
assert_eq!(*result, 2);
# }
```

It is often useful to write a wrapper method for accessing the value.

```
use tokio::sync::OnceCell;

static ONCE: OnceCell<u32> = OnceCell::const_new();

async fn get_global_integer() -> &'static u32 {
    ONCE.get_or_init(|| async {
        1 + 1
    }).await
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let result = get_global_integer().await;
assert_eq!(*result, 2);
# }
```

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Creates a new empty `OnceCell` instance.
- `fn const_new() -> Self` - Creates a new empty `OnceCell` instance.
- `fn new_with(value: Option<T>) -> Self` - Creates a new `OnceCell` that contains the provided value, if any.
- `fn const_new_with(value: T) -> Self` - Creates a new `OnceCell` that contains the provided value.
- `fn initialized(self: &Self) -> bool` - Returns `true` if the `OnceCell` currently contains a value, and `false`
- `fn get(self: &Self) -> Option<&T>` - Returns a reference to the value currently stored in the `OnceCell`, or
- `fn get_mut(self: & mut Self) -> Option<& mut T>` - Returns a mutable reference to the value currently stored in the
- `fn set(self: &Self, value: T) -> Result<(), SetError<T>>` - Sets the value of the `OnceCell` to the given value if the `OnceCell` is
- `fn get_or_init<F, Fut>(self: &Self, f: F) -> &T` - Gets the value currently in the `OnceCell`, or initialize it with the
- `fn get_or_try_init<E, F, Fut>(self: &Self, f: F) -> Result<&T, E>` - Gets the value currently in the `OnceCell`, or initialize it with the
- `fn into_inner(self: Self) -> Option<T>` - Takes the value from the cell, destroying the cell in the process.
- `fn take(self: & mut Self) -> Option<T>` - Takes ownership of the current value, leaving the cell empty.  Returns

**Traits:** Eq, Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: T) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> OnceCell<T>`
- **Default**
  - `fn default() -> OnceCell<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &OnceCell<T>) -> bool`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::once_cell::SetError

*Enum*

Errors that can be returned from [`OnceCell::set`].

[`OnceCell::set`]: crate::sync::OnceCell::set

**Generic Parameters:**
- T

**Variants:**
- `AlreadyInitializedError(T)` - The cell was already initialized when [`OnceCell::set`] was called.
- `InitializingError(T)` - The cell is currently being initialized.

**Methods:**

- `fn is_already_init_err(self: &Self) -> bool` - Whether `SetError` is `SetError::AlreadyInitializedError`.
- `fn is_initializing_err(self: &Self) -> bool` - Whether `SetError` is `SetError::InitializingError`

**Traits:** Eq, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



