**tokio > sync > barrier > Barrier**

# Module: sync::barrier::Barrier

## Contents

**Functions**

- [`wait`](#wait) - Does not resolve until all tasks have rendezvoused here.

---

## tokio::sync::barrier::Barrier::wait

*Function*

Does not resolve until all tasks have rendezvoused here.

Barriers are re-usable after all tasks have rendezvoused once, and can
be used continuously.

A single (arbitrary) future will receive a [`BarrierWaitResult`] that returns `true` from
[`BarrierWaitResult::is_leader`] when returning from this function, and all other tasks
will receive a result that will return `false` from `is_leader`.

# Cancel safety

This method is not cancel safe.

```rust
fn wait(self: &Self) -> BarrierWaitResult
```



