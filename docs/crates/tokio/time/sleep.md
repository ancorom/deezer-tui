**tokio > time > sleep**

# Module: time::sleep

## Contents

**Structs**

- [`Sleep`](#sleep) - Future returned by [`sleep`](sleep) and [`sleep_until`](sleep_until).

**Functions**

- [`sleep`](#sleep) - Waits until `duration` has elapsed.
- [`sleep_until`](#sleep_until) - Waits until `deadline` is reached.

---

## tokio::time::sleep::Sleep

*Struct*

Future returned by [`sleep`](sleep) and [`sleep_until`](sleep_until).

This type does not implement the `Unpin` trait, which means that if you
use it with [`select!`] or by calling `poll`, you have to pin it first.
If you use it with `.await`, this does not apply.

# Examples

Wait 100ms and print "100 ms have elapsed".

```
use tokio::time::{sleep, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
# }
```

Use with [`select!`]. Pinning the `Sleep` with [`tokio::pin!`] is
necessary when the same `Sleep` is selected on multiple times.
```no_run
use tokio::time::{self, Duration, Instant};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let sleep = time::sleep(Duration::from_millis(10));
tokio::pin!(sleep);

loop {
    tokio::select! {
        () = &mut sleep => {
            println!("timer elapsed");
            sleep.as_mut().reset(Instant::now() + Duration::from_millis(50));
        },
    }
}
# }
```
Use in a struct with boxing. By pinning the `Sleep` with a `Box`, the
`HasSleep` struct implements `Unpin`, even though `Sleep` does not.
```
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Sleep;

struct HasSleep {
    sleep: Pin<Box<Sleep>>,
}

impl Future for HasSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.sleep.as_mut().poll(cx)
    }
}
```
Use in a struct with pin projection. This method avoids the `Box`, but
the `HasSleep` struct will not be `Unpin` as a consequence.
```
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Sleep;
use pin_project_lite::pin_project;

pin_project! {
    struct HasSleep {
        #[pin]
        sleep: Sleep,
    }
}

impl Future for HasSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.project().sleep.poll(cx)
    }
}
```

[`select!`]: ../macro.select.html
[`tokio::pin!`]: ../macro.pin.html

**Methods:**

- `fn deadline(self: &Self) -> Instant` - Returns the instant at which the future will complete.
- `fn is_elapsed(self: &Self) -> bool` - Returns `true` if `Sleep` has elapsed.
- `fn reset(self: Pin<& mut Self>, deadline: Instant)` - Resets the `Sleep` instance to a new deadline.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<<Self as >::Output>`



## tokio::time::sleep::sleep

*Function*

Waits until `duration` has elapsed.

Equivalent to `sleep_until(Instant::now() + duration)`. An asynchronous
analog to `std::thread::sleep`.

No work is performed while awaiting on the sleep future to complete. `Sleep`
operates at millisecond granularity and should not be used for tasks that
require high-resolution timers. The implementation is platform specific,
and some platforms (specifically Windows) will provide timers with a
larger resolution than 1 ms.

To run something regularly on a schedule, see [`interval`].

# Cancellation

Canceling a sleep instance is done by dropping the returned future. No additional
cleanup work is required.

# Examples

Wait 100ms and print "100 ms have elapsed".

```
use tokio::time::{sleep, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
# }
```

See the documentation for the [`Sleep`] type for more examples.

# Panics

This function panics if there is no current timer set.

It can be triggered when [`Builder::enable_time`] or
[`Builder::enable_all`] are not included in the builder.

It can also panic whenever a timer is created outside of a
Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
since the function is executed outside of the runtime.
Whereas `rt.block_on(async {sleep(...).await})` doesn't panic.
And this is because wrapping the function on an async makes it lazy,
and so gets executed inside the runtime successfully without
panicking.

[`Sleep`]: struct@crate::time::Sleep
[`interval`]: crate::time::interval()
[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

```rust
fn sleep(duration: crate::time::Duration) -> Sleep
```



## tokio::time::sleep::sleep_until

*Function*

Waits until `deadline` is reached.

No work is performed while awaiting on the sleep future to complete. `Sleep`
operates at millisecond granularity and should not be used for tasks that
require high-resolution timers.

To run something regularly on a schedule, see [`interval`].

# Cancellation

Canceling a sleep instance is done by dropping the returned future. No additional
cleanup work is required.

# Examples

Wait 100ms and print "100 ms have elapsed".

```
use tokio::time::{sleep_until, Instant, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
sleep_until(Instant::now() + Duration::from_millis(100)).await;
println!("100 ms have elapsed");
# }
```

See the documentation for the [`Sleep`] type for more examples.

# Panics

This function panics if there is no current timer set.

It can be triggered when [`Builder::enable_time`] or
[`Builder::enable_all`] are not included in the builder.

It can also panic whenever a timer is created outside of a
Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
since the function is executed outside of the runtime.
Whereas `rt.block_on(async {sleep(...).await})` doesn't panic.
And this is because wrapping the function on an async makes it lazy,
and so gets executed inside the runtime successfully without
panicking.

[`Sleep`]: struct@crate::time::Sleep
[`interval`]: crate::time::interval()
[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

```rust
fn sleep_until(deadline: crate::time::Instant) -> Sleep
```



