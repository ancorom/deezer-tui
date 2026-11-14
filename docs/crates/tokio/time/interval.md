**tokio > time > interval**

# Module: time::interval

## Contents

**Structs**

- [`Interval`](#interval) - Interval returned by [`interval`] and [`interval_at`].

**Enums**

- [`MissedTickBehavior`](#missedtickbehavior) - Defines the behavior of an [`Interval`] when it misses a tick.

**Functions**

- [`interval`](#interval) - Creates new [`Interval`] that yields with interval of `period`. The first
- [`interval_at`](#interval_at) - Creates new [`Interval`] that yields with interval of `period` with the

---

## tokio::time::interval::Interval

*Struct*

Interval returned by [`interval`] and [`interval_at`].

This type allows you to wait on a sequence of instants with a certain
duration between each instant. Unlike calling [`sleep`] in a loop, this lets
you count the time spent between the calls to [`sleep`] as well.

An `Interval` can be turned into a `Stream` with [`IntervalStream`].

[`IntervalStream`]: https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.IntervalStream.html
[`sleep`]: crate::time::sleep()

**Methods:**

- `fn tick(self: & mut Self) -> Instant` - Completes when the next instant in the interval has been reached.
- `fn poll_tick(self: & mut Self, cx: & mut Context) -> Poll<Instant>` - Polls for the next instant in the interval to be reached.
- `fn reset(self: & mut Self)` - Resets the interval to complete one period after the current time.
- `fn reset_immediately(self: & mut Self)` - Resets the interval immediately.
- `fn reset_after(self: & mut Self, after: Duration)` - Resets the interval after the specified [`std::time::Duration`].
- `fn reset_at(self: & mut Self, deadline: Instant)` - Resets the interval to a [`crate::time::Instant`] deadline.
- `fn missed_tick_behavior(self: &Self) -> MissedTickBehavior` - Returns the [`MissedTickBehavior`] strategy currently being used.
- `fn set_missed_tick_behavior(self: & mut Self, behavior: MissedTickBehavior)` - Sets the [`MissedTickBehavior`] strategy that should be used.
- `fn period(self: &Self) -> Duration` - Returns the period of the interval.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::time::interval::MissedTickBehavior

*Enum*

Defines the behavior of an [`Interval`] when it misses a tick.

Sometimes, an [`Interval`]'s tick is missed. For example, consider the
following:

```
use tokio::time::{self, Duration};
# async fn task_that_takes_one_to_three_millis() {}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// ticks every 2 milliseconds
let mut interval = time::interval(Duration::from_millis(2));
for _ in 0..5 {
    interval.tick().await;
    // if this takes more than 2 milliseconds, a tick will be delayed
    task_that_takes_one_to_three_millis().await;
}
# }
```

Generally, a tick is missed if too much time is spent without calling
[`Interval::tick()`].

By default, when a tick is missed, [`Interval`] fires ticks as quickly as it
can until it is "caught up" in time to where it should be.
`MissedTickBehavior` can be used to specify a different behavior for
[`Interval`] to exhibit. Each variant represents a different strategy.

Note that because the executor cannot guarantee exact precision with timers,
these strategies will only apply when the delay is greater than 5
milliseconds.

**Variants:**
- `Burst` - Ticks as fast as possible until caught up.
- `Delay` - Tick at multiples of `period` from when [`tick`] was called, rather than
- `Skip` - Skips missed ticks and tick on the next multiple of `period` from

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self` - Returns [`MissedTickBehavior::Burst`].
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tokio::time::interval::interval

*Function*

Creates new [`Interval`] that yields with interval of `period`. The first
tick completes immediately. The default [`MissedTickBehavior`] is
[`Burst`](MissedTickBehavior::Burst), but this can be configured
by calling [`set_missed_tick_behavior`](Interval::set_missed_tick_behavior).

An interval will tick indefinitely. At any time, the [`Interval`] value can
be dropped. This cancels the interval.

This function is equivalent to
[`interval_at(Instant::now(), period)`](interval_at).

# Panics

This function panics if `period` is zero.

# Examples

```
use tokio::time::{self, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut interval = time::interval(Duration::from_millis(10));

interval.tick().await; // ticks immediately
interval.tick().await; // ticks after 10ms
interval.tick().await; // ticks after 10ms

// approximately 20ms have elapsed.
# }
```

A simple example using `interval` to execute a task every two seconds.

The difference between `interval` and [`sleep`] is that an [`Interval`]
measures the time since the last tick, which means that [`.tick().await`]
may wait for a shorter time than the duration specified for the interval
if some time has passed between calls to [`.tick().await`].

If the tick in the example below was replaced with [`sleep`], the task
would only be executed once every three seconds, and not every two
seconds.

```
use tokio::time;

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut interval = time::interval(time::Duration::from_secs(2));
for _i in 0..5 {
    interval.tick().await;
    task_that_takes_a_second().await;
}
# }
```

[`sleep`]: crate::time::sleep()
[`.tick().await`]: Interval::tick

```rust
fn interval(period: crate::time::Duration) -> Interval
```



## tokio::time::interval::interval_at

*Function*

Creates new [`Interval`] that yields with interval of `period` with the
first tick completing at `start`. The default [`MissedTickBehavior`] is
[`Burst`](MissedTickBehavior::Burst), but this can be configured
by calling [`set_missed_tick_behavior`](Interval::set_missed_tick_behavior).

An interval will tick indefinitely. At any time, the [`Interval`] value can
be dropped. This cancels the interval.

# Panics

This function panics if `period` is zero.

# Examples

```
use tokio::time::{interval_at, Duration, Instant};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let start = Instant::now() + Duration::from_millis(50);
let mut interval = interval_at(start, Duration::from_millis(10));

interval.tick().await; // ticks after 50ms
interval.tick().await; // ticks after 10ms
interval.tick().await; // ticks after 10ms

// approximately 70ms have elapsed.
# }
```

```rust
fn interval_at(start: crate::time::Instant, period: crate::time::Duration) -> Interval
```



