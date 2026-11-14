**tokio > sync > once_cell > OnceCell**

# Module: sync::once_cell::OnceCell

## Contents

**Functions**

- [`new`](#new) - Creates a new empty `OnceCell` instance.
- [`new_with`](#new_with) - Creates a new `OnceCell` that contains the provided value, if any.
- [`set`](#set) - Sets the value of the `OnceCell` to the given value if the `OnceCell` is

---

## tokio::sync::once_cell::OnceCell::new

*Function*

Creates a new empty `OnceCell` instance.

```rust
fn new() -> Self
```



## tokio::sync::once_cell::OnceCell::new_with

*Function*

Creates a new `OnceCell` that contains the provided value, if any.

If the `Option` is `None`, this is equivalent to `OnceCell::new`.

[`OnceCell::new`]: crate::sync::OnceCell::new

```rust
fn new_with(value: Option<T>) -> Self
```



## tokio::sync::once_cell::OnceCell::set

*Function*

Sets the value of the `OnceCell` to the given value if the `OnceCell` is
empty.

If the `OnceCell` already has a value, this call will fail with an
[`SetError::AlreadyInitializedError`].

If the `OnceCell` is empty, but some other task is currently trying to
set the value, this call will fail with [`SetError::InitializingError`].

[`SetError::AlreadyInitializedError`]: crate::sync::SetError::AlreadyInitializedError
[`SetError::InitializingError`]: crate::sync::SetError::InitializingError

```rust
fn set(self: &Self, value: T) -> Result<(), SetError<T>>
```



