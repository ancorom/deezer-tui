**tokio > time > interval > Interval**

# Module: time::interval::Interval

## Contents

**Functions**

- [`set_missed_tick_behavior`](#set_missed_tick_behavior) - Sets the [`MissedTickBehavior`] strategy that should be used.
- [`tick`](#tick) - Completes when the next instant in the interval has been reached.

---

## tokio::time::interval::Interval::set_missed_tick_behavior

*Function*

Sets the [`MissedTickBehavior`] strategy that should be used.

```rust
fn set_missed_tick_behavior(self: & mut Self, behavior: MissedTickBehavior)
```



## tokio::time::interval::Interval::tick

*Function*

Completes when the next instant in the interval has been reached.

# Cancel safety

This method is cancel safe. If `tick` is used as a branch in
[`tokio::select!`](crate::select) and another branch completes first,
then no tick has been consumed.

# Examples

```
use tokio::time;

use std::time::Duration;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut interval = time::interval(Duration::from_millis(10));

interval.tick().await;
// approximately 0ms have elapsed. The first tick completes immediately.
interval.tick().await;
interval.tick().await;

// approximately 20ms have elapsed.
# }
```

```rust
fn tick(self: & mut Self) -> Instant
```



