**tokio > task > task_local > LocalKey**

# Module: task::task_local::LocalKey

## Contents

**Functions**

- [`scope`](#scope) - Sets a value `T` as the task-local value for the future `F`.
- [`try_with`](#try_with) - Accesses the current task-local and runs the provided closure.
- [`with`](#with) - Accesses the current task-local and runs the provided closure.

---

## tokio::task::task_local::LocalKey::scope

*Function*

Sets a value `T` as the task-local value for the future `F`.

On completion of `scope`, the task-local will be dropped.

### Panics

If you poll the returned future inside a call to [`with`] or
[`try_with`] on the same `LocalKey`, then the call to `poll` will panic.

### Examples

```
# async fn dox() {
tokio::task_local! {
    static NUMBER: u32;
}

NUMBER.scope(1, async move {
    println!("task local value: {}", NUMBER.get());
}).await;
# }
```

[`with`]: fn@Self::with
[`try_with`]: fn@Self::try_with

```rust
fn scope<F>(self: &'static Self, value: T, f: F) -> TaskLocalFuture<T, F>
```



## tokio::task::task_local::LocalKey::try_with

*Function*

Accesses the current task-local and runs the provided closure.

If the task-local with the associated key is not present, this
method will return an `AccessError`. For a panicking variant,
see `with`.

```rust
fn try_with<F, R>(self: &'static Self, f: F) -> Result<R, AccessError>
```



## tokio::task::task_local::LocalKey::with

*Function*

Accesses the current task-local and runs the provided closure.

# Panics

This function will panic if the task local doesn't have a value set.

```rust
fn with<F, R>(self: &'static Self, f: F) -> R
```



