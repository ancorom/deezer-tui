**tokio > task > coop > RestoreOnPending**

# Module: task::coop::RestoreOnPending

## Contents

**Functions**

- [`made_progress`](#made_progress) - Signals that the task that obtained this `RestoreOnPending` was able to make

---

## tokio::task::coop::RestoreOnPending::made_progress

*Function*

Signals that the task that obtained this `RestoreOnPending` was able to make
progress. This prevents the task budget from being restored to the value
it had prior to obtaining this instance when it is dropped.

```rust
fn made_progress(self: &Self)
```



