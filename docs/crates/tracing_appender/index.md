# tracing_appender

Writers for logging events and spans

# Overview

[`tracing`][tracing] is a framework for structured, event-based diagnostic information.
`tracing-appender` allows events and spans to be recorded in a non-blocking manner through
a dedicated logging thread. It also provides a [`RollingFileAppender`] that can
be used with _or_ without the non-blocking writer.

*Compiler support: [requires `rustc` 1.63+][msrv]*

[msrv]: #supported-rust-versions
[`RollingFileAppender`]: rolling::RollingFileAppender
[tracing]: https://docs.rs/tracing/

# Usage

Add the following to your `Cargo.toml`:
```toml
tracing-appender = "0.2"
```

This crate can be used in a few ways to record spans/events:
 - Using a [`RollingFileAppender`] to perform writes to a log file. This will block on writes.
 - Using *any* type implementing [`std::io::Write`] in a non-blocking fashion.
 - Using a combination of [`NonBlocking`] and [`RollingFileAppender`] to allow writes to a log file
   without blocking.

## File Appender

The [`rolling` module][rolling] provides functions to create rolling and non-rolling file
appenders.

Rolling file appender rotation options are [`Rotation::MINUTELY`](rolling::Rotation::MINUTELY),
[`Rotation::HOURLY`](rolling::Rotation::HOURLY), and
[`Rotation::DAILY`](rolling::Rotation::DAILY).

To create a non-rolling file appender, use
[`tracing_appender::rolling::never(/*...*/)`](rolling::never) or
[`Rotation::NEVER`](rolling::Rotation::NEVER).

The following example creates an hourly rotating file appender that writes to
`/some/directory/prefix.log.YYYY-MM-DD-HH`:

```rust
# fn docs() {
let file_appender = tracing_appender::rolling::hourly("/some/directory", "prefix.log");
# }
```

The file appender implements [`std::io::Write`]. To be used with
[`tracing_subscriber::FmtSubscriber`][fmt_subscriber], it must be combined with a
[`MakeWriter`][make_writer] implementation to be able to record tracing spans/event.

See the [`rolling` module][rolling]'s documentation for more detail on how to use this file
appender.

## Non-Blocking Writer

The example below demonstrates the construction of a `non_blocking` writer with `std::io::stdout()`,
which implements [`MakeWriter`][make_writer].

```rust
# fn doc() {
let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .init();
# }
```
**Note:** `_guard` is a [`WorkerGuard`] which is returned by [`tracing_appender::non_blocking`][non_blocking]
to ensure buffered logs are flushed to their output in the case of abrupt terminations of a process.
See [`WorkerGuard` module][guard] for more details.

The example below demonstrates the construction of a [`tracing_appender::non_blocking`][non_blocking]
writer constructed with a [`std::io::Write`]:

```rust
use std::io::Error;

struct TestWriter;

impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        println!("{:?}", buf);
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

# fn doc() {
let (non_blocking, _guard) = tracing_appender::non_blocking(TestWriter);
tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .init();
# }
```

The [`non_blocking` module][non_blocking]'s documentation provides more detail on how to use `non_blocking`.

[non_blocking]: mod@non_blocking
[write]: std::io::Write
[guard]: non_blocking::WorkerGuard
[make_writer]: tracing_subscriber::fmt::MakeWriter
[rolling_struct]: rolling::RollingFileAppender
[fmt_subscriber]: tracing_subscriber::fmt::Subscriber

## Non-Blocking Rolling File Appender

```rust
# fn docs() {
let file_appender = tracing_appender::rolling::hourly("/some/directory", "prefix.log");
let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .init();
# }
```

## Supported Rust Versions

`tracing-appender` is built against the latest stable release. The minimum supported
version is 1.63. The current `tracing-appender` version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.


## Modules

### [`tracing_appender`](tracing_appender.md)

*1 function, 2 modules*

### [`non_blocking`](non_blocking.md)

*1 constant, 4 structs*

### [`non_blocking::NonBlockingBuilder`](non_blocking/NonBlockingBuilder.md)

*1 function*

### [`rolling`](rolling.md)

*3 structs, 5 functions*

### [`rolling::RollingFileAppender`](rolling/RollingFileAppender.md)

*2 functions*

### [`rolling::builder`](rolling/builder.md)

*2 structs*

### [`rolling::builder::Builder`](rolling/builder/Builder.md)

*5 functions*

