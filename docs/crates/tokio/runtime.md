**tokio > runtime**

# Module: runtime

## Contents

**Functions**

- [`worker_index`](#worker_index) - Returns the index of the current worker thread, if called from a

---

## tokio::runtime::worker_index

*Function*

Returns the index of the current worker thread, if called from a
runtime worker thread.

The returned value is a 0-based index matching the worker indices
used by [`RuntimeMetrics`] methods such as
[`worker_total_busy_duration`](RuntimeMetrics::worker_total_busy_duration).

Returns `None` when called from outside a runtime worker thread
(for example, from a blocking thread or a non-Tokio thread). On the
multi-thread runtime, the thread that calls [`Runtime::block_on`] is
not a worker thread, so this also returns `None` there.

For the current-thread runtime, this returns `Some(0)` when called from
the thread that currently owns the runtime driver. If multiple threads
call [`Runtime::block_on`] concurrently, calls on threads that do not
own the driver return `None`. A [`LocalRuntime`] can only be driven from
its owning thread, so calls inside `block_on` always return `Some(0)`.

Note that the result may change across `.await` points, as the
task may be moved to a different worker thread by the scheduler.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let index = tokio::spawn(async {
        tokio::runtime::worker_index()
    }).await.unwrap();
    println!("Task ran on worker {:?}", index);
}
# }
```

```rust
fn worker_index() -> Option<usize>
```



