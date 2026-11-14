**tokio > runtime > task > error > JoinError**

# Module: runtime::task::error::JoinError

## Contents

**Functions**

- [`id`](#id) - Returns a [task ID] that identifies the task which errored relative to
- [`is_cancelled`](#is_cancelled) - Returns true if the error was caused by the task being cancelled.

---

## tokio::runtime::task::error::JoinError::id

*Function*

Returns a [task ID] that identifies the task which errored relative to
other currently spawned tasks.

[task ID]: crate::task::Id

```rust
fn id(self: &Self) -> Id
```



## tokio::runtime::task::error::JoinError::is_cancelled

*Function*

Returns true if the error was caused by the task being cancelled.

See [the module level docs] for more information on cancellation.

[the module level docs]: crate::task#cancellation

```rust
fn is_cancelled(self: &Self) -> bool
```



