**tokio > signal > unix**

# Module: signal::unix

## Contents

**Structs**

- [`Signal`](#signal) - An listener for receiving a particular type of OS signal.
- [`SignalKind`](#signalkind) - Represents the specific kind of signal to listen for.

**Functions**

- [`signal`](#signal) - Creates a new listener which will receive notifications when the current

---

## tokio::signal::unix::Signal

*Struct*

An listener for receiving a particular type of OS signal.

The listener can be turned into a `Stream` using [`SignalStream`].

[`SignalStream`]: https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.SignalStream.html

In general signal handling on Unix is a pretty tricky topic, and this
structure is no exception! There are some important limitations to keep in
mind when using `Signal` streams:

* Signals handling in Unix already necessitates coalescing signals
  together sometimes. This `Signal` stream is also no exception here in
  that it will also coalesce signals. That is, even if the signal handler
  for this process runs multiple times, the `Signal` stream may only return
  one signal notification. Specifically, before `poll` is called, all
  signal notifications are coalesced into one item returned from `poll`.
  Once `poll` has been called, however, a further signal is guaranteed to
  be yielded as an item.

  Put another way, any element pulled off the returned listener corresponds to
  *at least one* signal, but possibly more.

* Signal handling in general is relatively inefficient. Although some
  improvements are possible in this crate, it's recommended to not plan on
  having millions of signal channels open.

If you've got any questions about this feel free to open an issue on the
repo! New approaches to alleviate some of these limitations are always
appreciated!

# Caveats

The first time that a `Signal` instance is registered for a particular
signal kind, an OS signal-handler is installed which replaces the default
platform behavior when that signal is received, **for the duration of the
entire process**.

For example, Unix systems will terminate a process by default when it
receives `SIGINT`. But, when a `Signal` instance is created to listen for
this signal, the next `SIGINT` that arrives will be translated to a stream
event, and the process will continue to execute. **Even if this `Signal`
instance is dropped, subsequent `SIGINT` deliveries will end up captured by
Tokio, and the default platform behavior will NOT be reset**.

Thus, applications should take care to ensure the expected signal behavior
occurs as expected after listening for specific signals.

# Examples

Wait for `SIGHUP`

```rust,no_run
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    let mut sig = signal(SignalKind::hangup())?;

    // Print whenever a HUP signal is received
    loop {
        sig.recv().await;
        println!("got signal HUP");
    }
}
```

**Methods:**

- `fn recv(self: & mut Self) -> Option<()>` - Receives the next signal notification event.
- `fn poll_recv(self: & mut Self, cx: & mut Context) -> Poll<Option<()>>` - Polls to receive the next signal notification event, outside of an

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::signal::unix::SignalKind

*Struct*

Represents the specific kind of signal to listen for.

**Tuple Struct**: `()`

**Methods:**

- `fn from_raw(signum: std::os::raw::c_int) -> Self` - Allows for listening to any valid OS signal.
- `fn as_raw_value(self: &Self) -> std::os::raw::c_int` - Get the signal's numeric value.
- `fn alarm() -> Self` - Represents the `SIGALRM` signal.
- `fn child() -> Self` - Represents the `SIGCHLD` signal.
- `fn hangup() -> Self` - Represents the `SIGHUP` signal.
- `fn interrupt() -> Self` - Represents the `SIGINT` signal.
- `fn io() -> Self` - Represents the `SIGIO` signal.
- `fn pipe() -> Self` - Represents the `SIGPIPE` signal.
- `fn quit() -> Self` - Represents the `SIGQUIT` signal.
- `fn terminate() -> Self` - Represents the `SIGTERM` signal.
- `fn user_defined1() -> Self` - Represents the `SIGUSR1` signal.
- `fn user_defined2() -> Self` - Represents the `SIGUSR2` signal.
- `fn window_change() -> Self` - Represents the `SIGWINCH` signal.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(signum: std::os::raw::c_int) -> Self`



## tokio::signal::unix::signal

*Function*

Creates a new listener which will receive notifications when the current
process receives the specified signal `kind`.

This function will create a new stream which binds to the default reactor.
The `Signal` stream is an infinite stream which will receive
notifications whenever a signal is received. More documentation can be
found on `Signal` itself, but to reiterate:

* Signals may be coalesced beyond what the kernel already does.
* Once a signal handler is registered with the process the underlying
  libc signal handler is never unregistered.

A `Signal` stream can be created for a particular signal number
multiple times. When a signal is received then all the associated
channels will receive the signal notification.

# Errors

* If the lower-level C functions fail for some reason.
* If the previous initialization of this specific signal failed.
* If the signal is one of
  [`signal_hook::FORBIDDEN`](fn@signal_hook_registry::register#panics)

# Panics

This function panics if there is no current reactor set, or if the `rt`
feature flag is not enabled.

```rust
fn signal(kind: SignalKind) -> io::Result<Signal>
```



