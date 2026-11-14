**tokio > time > timeout**

# Module: time::timeout

## Contents

**Structs**

- [`Timeout`](#timeout) - Future returned by [`timeout`](timeout) and [`timeout_at`](timeout_at).

**Functions**

- [`timeout`](#timeout) - Requires a `Future` to complete before the specified duration has elapsed.
- [`timeout_at`](#timeout_at) - Requires a `Future` to complete before the specified instant in time.

---

## tokio::time::timeout::Timeout

*Struct*

Future returned by [`timeout`](timeout) and [`timeout_at`](timeout_at).

**Generic Parameters:**
- T

**Methods:**

- `fn get_ref(self: &Self) -> &T` - Gets a reference to the underlying value in this timeout.
- `fn get_mut(self: & mut Self) -> & mut T` - Gets a mutable reference to the underlying value in this timeout.
- `fn into_inner(self: Self) -> T` - Consumes this timeout, returning the underlying value.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<<Self as >::Output>`



## tokio::time::timeout::timeout

*Function*

Requires a `Future` to complete before the specified duration has elapsed.

If the future completes before the duration has elapsed, then the completed
value is returned. Otherwise, an error is returned and the future is
canceled.

Note that the future is polled before the timeout is checked, so if the future
does not yield during execution then it is possible for the future to complete
and exceed the timeout _without_ returning an error.

This function returns a future whose return type is [`Result`]`<T,`[`Elapsed`]`>`, where `T` is the
return type of the provided future.

If the provided future completes immediately, then the future returned from
this function is guaranteed to complete immediately with an [`Ok`] variant
no matter the provided duration.

[`Ok`]: std::result::Result::Ok
[`Result`]: std::result::Result
[`Elapsed`]: crate::time::error::Elapsed

# Cancellation

Cancelling a timeout is done by dropping the future. No additional cleanup
or other work is required.

The original future may be obtained by calling [`Timeout::into_inner`]. This
consumes the `Timeout`.

# Examples

Create a new `Timeout` set to expire in 10 milliseconds.

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;

use std::time::Duration;

# async fn dox() {
let (tx, rx) = oneshot::channel();
# tx.send(()).unwrap();

// Wrap the future with a `Timeout` set to expire in 10 milliseconds.
if let Err(_) = timeout(Duration::from_millis(10), rx).await {
    println!("did not receive value within 10 ms");
}
# }
```

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

[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

```rust
fn timeout<F>(duration: crate::Duration, future: F) -> Timeout<<F as >::IntoFuture>
```



## tokio::time::timeout::timeout_at

*Function*

Requires a `Future` to complete before the specified instant in time.

If the future completes before the instant is reached, then the completed
value is returned. Otherwise, an error is returned.

This function returns a future whose return type is [`Result`]`<T,`[`Elapsed`]`>`, where `T` is the
return type of the provided future.

If the provided future completes immediately, then the future returned from
this function is guaranteed to complete immediately with an [`Ok`] variant
no matter the provided deadline.

[`Ok`]: std::result::Result::Ok
[`Result`]: std::result::Result
[`Elapsed`]: crate::time::error::Elapsed

# Cancellation

Cancelling a timeout is done by dropping the future. No additional cleanup
or other work is required.

The original future may be obtained by calling [`Timeout::into_inner`]. This
consumes the `Timeout`.

# Examples

Create a new `Timeout` set to expire in 10 milliseconds.

```rust
use tokio::time::{Instant, timeout_at};
use tokio::sync::oneshot;

use std::time::Duration;

# async fn dox() {
let (tx, rx) = oneshot::channel();
# tx.send(()).unwrap();

// Wrap the future with a `Timeout` set to expire 10 milliseconds into the
// future.
if let Err(_) = timeout_at(Instant::now() + Duration::from_millis(10), rx).await {
    println!("did not receive value within 10 ms");
}
# }
```

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

[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

```rust
fn timeout_at<F>(deadline: crate::Instant, future: F) -> Timeout<<F as >::IntoFuture>
```



