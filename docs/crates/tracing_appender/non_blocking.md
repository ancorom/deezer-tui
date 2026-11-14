**tracing_appender > non_blocking**

# Module: non_blocking

## Contents

**Structs**

- [`ErrorCounter`](#errorcounter) - Tracks the number of times a log line was dropped by the background thread.
- [`NonBlocking`](#nonblocking) - A non-blocking writer.
- [`NonBlockingBuilder`](#nonblockingbuilder) - A builder for [`NonBlocking`].
- [`WorkerGuard`](#workerguard) - A guard that flushes spans/events associated to a [`NonBlocking`] on a drop

**Constants**

- [`DEFAULT_BUFFERED_LINES_LIMIT`](#default_buffered_lines_limit) - The default maximum number of buffered log lines.

---

## tracing_appender::non_blocking::DEFAULT_BUFFERED_LINES_LIMIT

*Constant*: `usize`

The default maximum number of buffered log lines.

If [`NonBlocking`] is lossy, it will drop spans/events at capacity.
If [`NonBlocking`] is _not_ lossy, backpressure will be exerted on
senders, causing them to block their respective threads until there
is available capacity.

Recommended to be a power of 2.



## tracing_appender::non_blocking::ErrorCounter

*Struct*

Tracks the number of times a log line was dropped by the background thread.

If the non-blocking writer is not configured in [lossy mode], the error
count should always be 0.

[lossy mode]: NonBlockingBuilder::lossy

**Tuple Struct**: `()`

**Methods:**

- `fn dropped_lines(self: &Self) -> usize` - Returns the number of log lines that have been dropped.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_appender::non_blocking::NonBlocking

*Struct*

A non-blocking writer.

While the line between "blocking" and "non-blocking" IO is fuzzy, writing to a file is typically
considered to be a _blocking_ operation. For an application whose `Subscriber` writes spans and events
as they are emitted, an application might find the latency profile to be unacceptable.
`NonBlocking` moves the writing out of an application's data path by sending spans and events
to a dedicated logging thread.

This struct implements [`MakeWriter`] from the `tracing-subscriber`
crate. Therefore, it can be used with the [`tracing_subscriber::fmt`][fmt] module
or with any other subscriber/layer implementation that uses the `MakeWriter` trait.

[fmt]: mod@tracing_subscriber::fmt

**Methods:**

- `fn new<T>(writer: T) -> (NonBlocking, WorkerGuard)` - Returns a new `NonBlocking` writer wrapping the provided `writer`.
- `fn error_counter(self: &Self) -> ErrorCounter` - Returns a counter for the number of times logs where dropped. This will always return zero if

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> io::Result<()>`



## tracing_appender::non_blocking::NonBlockingBuilder

*Struct*

A builder for [`NonBlocking`].

**Methods:**

- `fn buffered_lines_limit(self: Self, buffered_lines_limit: usize) -> NonBlockingBuilder` - Sets the number of lines to buffer before dropping logs or exerting backpressure on senders
- `fn lossy(self: Self, is_lossy: bool) -> NonBlockingBuilder` - Sets whether `NonBlocking` should be lossy or not.
- `fn thread_name(self: Self, name: &str) -> NonBlockingBuilder` - Override the worker thread's name.
- `fn finish<T>(self: Self, writer: T) -> (NonBlocking, WorkerGuard)` - Completes the builder, returning the configured `NonBlocking`.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_appender::non_blocking::WorkerGuard

*Struct*

A guard that flushes spans/events associated to a [`NonBlocking`] on a drop

Writing to a [`NonBlocking`] writer will **not** immediately write a span or event to the underlying
output. Instead, the span or event will be written by a dedicated logging thread at some later point.
To increase throughput, the non-blocking writer will flush to the underlying output on
a periodic basis rather than every time a span or event is written. This means that if the program
terminates abruptly (such as through an uncaught `panic` or a `std::process::exit`), some spans
or events may not be written.

Since spans/events and events recorded near a crash are often necessary for diagnosing the failure,
`WorkerGuard` provides a mechanism to ensure that _all_ buffered logs are flushed to their output.
`WorkerGuard` should be assigned in the `main` function or whatever the entrypoint of the program is.
This will ensure that the guard will be dropped during an unwinding or when `main` exits
successfully.

# Examples

``` rust
# #[clippy::allow(needless_doctest_main)]
fn main () {
# fn doc() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let subscriber = tracing_subscriber::fmt().with_writer(non_blocking);
    tracing::subscriber::with_default(subscriber.finish(), || {
        // Emit some tracing events within context of the non_blocking `_guard` and tracing subscriber
        tracing::event!(tracing::Level::INFO, "Hello");
    });
    // Exiting the context of `main` will drop the `_guard` and any remaining logs should get flushed
# }
}
```

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



