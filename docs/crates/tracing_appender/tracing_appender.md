**tracing_appender**

# Module: tracing_appender

## Contents

**Modules**

- [`non_blocking`](#non_blocking) - A non-blocking, off-thread writer.
- [`rolling`](#rolling) - A rolling file appender.

**Functions**

- [`non_blocking`](#non_blocking) - Convenience function for creating a non-blocking, off-thread writer.

---

## Module: non_blocking

A non-blocking, off-thread writer.

This spawns a dedicated worker thread which is responsible for writing log
lines to the provided writer. When a line is written using the returned
`NonBlocking` struct's `make_writer` method, it will be enqueued to be
written by the worker thread.

The queue has a fixed capacity, and if it becomes full, any logs written
to it will be dropped until capacity is once again available. This may
occur if logs are consistently produced faster than the worker thread can
output them. The queue capacity and behavior when full (i.e., whether to
drop logs or to exert backpressure to slow down senders) can be configured
using [`NonBlockingBuilder::default()`][builder].
This function returns the default configuration. It is equivalent to:

```rust
# use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
# fn doc() -> (NonBlocking, WorkerGuard) {
tracing_appender::non_blocking(std::io::stdout())
# }
```
[builder]: NonBlockingBuilder::default

<br/> This function returns a tuple of `NonBlocking` and `WorkerGuard`.
`NonBlocking` implements [`MakeWriter`] which integrates with `tracing_subscriber`.
`WorkerGuard` is a drop guard that is responsible for flushing any remaining logs when
the program terminates.

Note that the `WorkerGuard` returned by `non_blocking` _must_ be assigned to a binding that
is not `_`, as `_` will result in the `WorkerGuard` being dropped immediately.
Unintentional drops of `WorkerGuard` remove the guarantee that logs will be flushed
during a program's termination, in a panic or otherwise.

See [`WorkerGuard`] for examples of using the guard.

# Examples

``` rust
# fn docs() {
let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
let subscriber = tracing_subscriber::fmt().with_writer(non_blocking);
tracing::subscriber::with_default(subscriber.finish(), || {
   tracing::event!(tracing::Level::INFO, "Hello");
});
# }
```



## tracing_appender::non_blocking

*Function*

Convenience function for creating a non-blocking, off-thread writer.

See the [`non_blocking` module's docs][non_blocking]'s for more details.

[non_blocking]: mod@non_blocking

# Examples

``` rust
# fn docs() {
let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
let subscriber = tracing_subscriber::fmt().with_writer(non_blocking);
tracing::subscriber::with_default(subscriber.finish(), || {
   tracing::event!(tracing::Level::INFO, "Hello");
});
# }
```

```rust
fn non_blocking<T>(writer: T) -> (crate::non_blocking::NonBlocking, crate::non_blocking::WorkerGuard)
```



## Module: rolling

A rolling file appender.

Creates a new log file at a fixed frequency as defined by [`Rotation`].
Logs will be written to this file for the duration of the period and will automatically roll over
to the newly created log file once the time period has elapsed.

The log file is created at the specified directory and file name prefix which *may* be appended with
the date and time.

The following helpers are available for creating a rolling file appender.

- [`Rotation::minutely()`][minutely]: A new log file in the format of `some_directory/log_file_name_prefix.yyyy-MM-dd-HH-mm`
  will be created minutely (once per minute)
- [`Rotation::hourly()`][hourly]: A new log file in the format of `some_directory/log_file_name_prefix.yyyy-MM-dd-HH`
  will be created hourly
- [`Rotation::daily()`][daily]: A new log file in the format of `some_directory/log_file_name_prefix.yyyy-MM-dd`
  will be created daily
- [`Rotation::never()`][never()]: This will result in log file located at `some_directory/log_file_name`


# Examples

```rust
# fn docs() {
use tracing_appender::rolling::{RollingFileAppender, Rotation};
let file_appender = RollingFileAppender::new(Rotation::HOURLY, "/some/directory", "prefix.log");
# }
```



