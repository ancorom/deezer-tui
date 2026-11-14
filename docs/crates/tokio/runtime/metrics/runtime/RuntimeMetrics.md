**tokio > runtime > metrics > runtime > RuntimeMetrics**

# Module: runtime::metrics::runtime::RuntimeMetrics

## Contents

**Functions**

- [`worker_total_busy_duration`](#worker_total_busy_duration) - Returns the amount of time the given worker thread has been busy.

---

## tokio::runtime::metrics::runtime::RuntimeMetrics::worker_total_busy_duration

*Function*

Returns the amount of time the given worker thread has been busy.

The worker busy duration starts at zero when the runtime is created and
increases whenever the worker is spending time processing work. Using
this value can indicate the load of the given worker. If a lot of time
is spent busy, then the worker is under load and will check for inbound
events less often.

The timer is monotonically increasing. It is never decremented or reset
to zero.

# Arguments

`worker` is the index of the worker being queried. The given value must
be between 0 and `num_workers()`. The index uniquely identifies a single
worker and will continue to identify the worker throughout the lifetime
of the runtime instance.

# Panics

The method panics when `worker` represents an invalid worker, i.e. is
greater than or equal to `num_workers()`.

# Examples

```
use tokio::runtime::Handle;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let metrics = Handle::current().metrics();

let n = metrics.worker_total_busy_duration(0);
println!("worker 0 was busy for a total of {:?}", n);
# }
```

```rust
fn worker_total_busy_duration(self: &Self, worker: usize) -> Duration
```



