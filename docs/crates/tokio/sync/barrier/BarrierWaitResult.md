**tokio > sync > barrier > BarrierWaitResult**

# Module: sync::barrier::BarrierWaitResult

## Contents

**Functions**

- [`is_leader`](#is_leader) - Returns `true` if this task from wait is the "leader task".

---

## tokio::sync::barrier::BarrierWaitResult::is_leader

*Function*

Returns `true` if this task from wait is the "leader task".

Only one task will have `true` returned from their result, all other tasks will have
`false` returned.

```rust
fn is_leader(self: &Self) -> bool
```



