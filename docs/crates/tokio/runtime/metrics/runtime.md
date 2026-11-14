**tokio > runtime > metrics > runtime**

# Module: runtime::metrics::runtime

## Contents

**Structs**

- [`RuntimeMetrics`](#runtimemetrics) - Handle to the runtime's metrics.

---

## tokio::runtime::metrics::runtime::RuntimeMetrics

*Struct*

Handle to the runtime's metrics.

This handle is internally reference-counted and can be freely cloned. A
`RuntimeMetrics` handle is obtained using the [`Runtime::metrics`] method.

[`Runtime::metrics`]: crate::runtime::Runtime::metrics()

**Methods:**

- `fn num_workers(self: &Self) -> usize` - Returns the number of worker threads used by the runtime.
- `fn num_alive_tasks(self: &Self) -> usize` - Returns the current number of alive tasks in the runtime.
- `fn global_queue_depth(self: &Self) -> usize` - Returns the number of tasks currently scheduled in the runtime's
- `fn worker_total_busy_duration(self: &Self, worker: usize) -> Duration` - Returns the amount of time the given worker thread has been busy.
- `fn worker_park_count(self: &Self, worker: usize) -> u64` - Returns the total number of times the given worker thread has parked.
- `fn worker_park_unpark_count(self: &Self, worker: usize) -> u64` - Returns the total number of times the given worker thread has parked

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



