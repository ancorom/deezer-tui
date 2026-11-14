**tokio > sync > set_once > SetOnce**

# Module: sync::set_once::SetOnce

## Contents

**Functions**

- [`new`](#new) - Creates a new empty `SetOnce` instance.
- [`new_with`](#new_with) - Creates a new `SetOnce` that contains the provided value, if any.
- [`set`](#set) - Sets the value of the `SetOnce` to the given value if the `SetOnce` is

---

## tokio::sync::set_once::SetOnce::new

*Function*

Creates a new empty `SetOnce` instance.

```rust
fn new() -> Self
```



## tokio::sync::set_once::SetOnce::new_with

*Function*

Creates a new `SetOnce` that contains the provided value, if any.

If the `Option` is `None`, this is equivalent to `SetOnce::new`.

[`SetOnce::new`]: crate::sync::SetOnce::new

```rust
fn new_with(value: Option<T>) -> Self
```



## tokio::sync::set_once::SetOnce::set

*Function*

Sets the value of the `SetOnce` to the given value if the `SetOnce` is
empty.

If the `SetOnce` already has a value, this call will fail with an
[`SetOnceError`].

[`SetOnceError`]: crate::sync::SetOnceError

```rust
fn set(self: &Self, value: T) -> Result<(), SetOnceError<T>>
```



