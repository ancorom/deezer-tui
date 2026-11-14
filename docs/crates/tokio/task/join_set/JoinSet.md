**tokio > task > join_set > JoinSet**

# Module: task::join_set::JoinSet

## Contents

**Functions**

- [`abort_all`](#abort_all) - Aborts all tasks on this `JoinSet`.
- [`join_next`](#join_next) - Waits until one of the tasks in the set completes and returns its output.
- [`spawn`](#spawn) - Spawn the provided task on the `JoinSet`, returning an [`AbortHandle`]
- [`spawn_local`](#spawn_local) - Spawn the provided task on the current [`LocalSet`] or [`LocalRuntime`]

---

## tokio::task::join_set::JoinSet::abort_all

*Function*

Aborts all tasks on this `JoinSet`.

This does not remove the tasks from the `JoinSet`. To wait for the tasks to complete
cancellation, you should call `join_next` in a loop until the `JoinSet` is empty.

```rust
fn abort_all(self: & mut Self)
```



## tokio::task::join_set::JoinSet::join_next

*Function*

Waits until one of the tasks in the set completes and returns its output.

Returns `None` if the set is empty.

# Cancel safety

This method is cancel safe. If `join_next` is used as a branch in
`tokio::select!` and another branch completes first, it is guaranteed
that no tasks were removed from this `JoinSet`.

```rust
fn join_next(self: & mut Self) -> Option<Result<T, JoinError>>
```



## tokio::task::join_set::JoinSet::spawn

*Function*

Spawn the provided task on the `JoinSet`, returning an [`AbortHandle`]
that can be used to remotely cancel the task.

The provided future will start running in the background immediately
when this method is called, even if you don't await anything on this
`JoinSet`.

# Panics

This method panics if called outside of a Tokio runtime.

[`AbortHandle`]: crate::task::AbortHandle

```rust
fn spawn<F>(self: & mut Self, task: F) -> AbortHandle
```



## tokio::task::join_set::JoinSet::spawn_local

*Function*

Spawn the provided task on the current [`LocalSet`] or [`LocalRuntime`]
and store it in this `JoinSet`, returning an [`AbortHandle`] that can
be used to remotely cancel the task.

The provided future will start running in the background immediately
when this method is called, even if you don't await anything on this
`JoinSet`.

# Panics

This method panics if it is called outside of a `LocalSet` or `LocalRuntime`.

[`LocalSet`]: crate::task::LocalSet
[`LocalRuntime`]: crate::runtime::LocalRuntime
[`AbortHandle`]: crate::task::AbortHandle

```rust
fn spawn_local<F>(self: & mut Self, task: F) -> AbortHandle
```



