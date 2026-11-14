**tokio > sync > barrier**

# Module: sync::barrier

## Contents

**Structs**

- [`Barrier`](#barrier) - A barrier enables multiple tasks to synchronize the beginning of some computation.
- [`BarrierWaitResult`](#barrierwaitresult) - A `BarrierWaitResult` is returned by `wait` when all tasks in the `Barrier` have rendezvoused.

---

## tokio::sync::barrier::Barrier

*Struct*

A barrier enables multiple tasks to synchronize the beginning of some computation.

```
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
use tokio::sync::Barrier;
use std::sync::Arc;

let mut handles = Vec::with_capacity(10);
let barrier = Arc::new(Barrier::new(10));
for _ in 0..10 {
    let c = barrier.clone();
    // The same messages will be printed together.
    // You will NOT see any interleaving.
    handles.push(tokio::spawn(async move {
        println!("before wait");
        let wait_result = c.wait().await;
        println!("after wait");
        wait_result
    }));
}

// Will not resolve until all "after wait" messages have been printed
let mut num_leaders = 0;
for handle in handles {
    let wait_result = handle.await.unwrap();
    if wait_result.is_leader() {
        num_leaders += 1;
    }
}

// Exactly one barrier will resolve as the "leader"
assert_eq!(num_leaders, 1);
# }
```

**Methods:**

- `fn new(n: usize) -> Barrier` - Creates a new barrier that can block a given number of tasks.
- `fn wait(self: &Self) -> BarrierWaitResult` - Does not resolve until all tasks have rendezvoused here.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::barrier::BarrierWaitResult

*Struct*

A `BarrierWaitResult` is returned by `wait` when all tasks in the `Barrier` have rendezvoused.

**Tuple Struct**: `()`

**Methods:**

- `fn is_leader(self: &Self) -> bool` - Returns `true` if this task from wait is the "leader task".

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



