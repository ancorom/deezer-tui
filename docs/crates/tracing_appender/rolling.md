**tracing_appender > rolling**

# Module: rolling

## Contents

**Structs**

- [`RollingFileAppender`](#rollingfileappender) - A file appender with the ability to rotate log files at a fixed schedule.
- [`RollingWriter`](#rollingwriter) - A [writer] that writes to a rolling log file.
- [`Rotation`](#rotation) - Defines a fixed period for rolling of a log file.

**Functions**

- [`daily`](#daily) - Creates a daily-rotating file appender.
- [`hourly`](#hourly) - Creates an hourly-rotating file appender.
- [`minutely`](#minutely) - Creates a minutely-rotating file appender. This will rotate the log file once per minute.
- [`never`](#never) - Creates a non-rolling file appender.
- [`weekly`](#weekly) - Creates a weekly-rotating file appender. The logs will rotate every Sunday at midnight UTC.

---

## tracing_appender::rolling::RollingFileAppender

*Struct*

A file appender with the ability to rotate log files at a fixed schedule.

`RollingFileAppender` implements the [`std:io::Write` trait][write] and will
block on write operations. It may be used with [`NonBlocking`] to perform
writes without blocking the current thread.

Additionally, `RollingFileAppender` also implements the [`MakeWriter`]
trait from `tracing-subscriber`, so it may also be used
directly, without [`NonBlocking`].

[write]: std::io::Write
[`NonBlocking`]: super::non_blocking::NonBlocking

# Examples

Rolling a log file once every hour:

```rust
# fn docs() {
let file_appender = tracing_appender::rolling::hourly("/some/directory", "prefix");
# }
```

Combining a `RollingFileAppender` with another [`MakeWriter`] implementation:

```rust
# fn docs() {
use tracing_subscriber::fmt::writer::MakeWriterExt;

// Log all events to a rolling log file.
let logfile = tracing_appender::rolling::hourly("/logs", "myapp-logs");

// Log `INFO` and above to stdout.
let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

tracing_subscriber::fmt()
    // Combine the stdout and log file `MakeWriter`s into one
    // `MakeWriter` that writes to both
    .with_writer(stdout.and(logfile))
    .init();
# }
```

[`MakeWriter`]: tracing_subscriber::fmt::writer::MakeWriter

**Methods:**

- `fn new<impl AsRef<Path>, impl AsRef<Path>>(rotation: Rotation, directory: impl Trait, filename_prefix: impl Trait) -> RollingFileAppender` - Creates a new `RollingFileAppender`.
- `fn builder() -> Builder` - Returns a new [`Builder`] for configuring a `RollingFileAppender`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



## tracing_appender::rolling::RollingWriter

*Struct*

A [writer] that writes to a rolling log file.

This is returned by the [`MakeWriter`] implementation for [`RollingFileAppender`].

[writer]: std::io::Write
[`MakeWriter`]: tracing_subscriber::fmt::writer::MakeWriter

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_appender::rolling::Rotation

*Struct*

Defines a fixed period for rolling of a log file.

To use a `Rotation`, pick one of the following options:

### Minutely Rotation
```rust
# fn docs() {
use tracing_appender::rolling::Rotation;
let rotation = tracing_appender::rolling::Rotation::MINUTELY;
# }
```

### Hourly Rotation
```rust
# fn docs() {
use tracing_appender::rolling::Rotation;
let rotation = tracing_appender::rolling::Rotation::HOURLY;
# }
```

### Daily Rotation
```rust
# fn docs() {
use tracing_appender::rolling::Rotation;
let rotation = tracing_appender::rolling::Rotation::DAILY;
# }
```

### Weekly Rotation
```rust
# fn docs() {
use tracing_appender::rolling::Rotation;
let rotation = tracing_appender::rolling::Rotation::WEEKLY;
# }
```

### No Rotation
```rust
# fn docs() {
use tracing_appender::rolling::Rotation;
let rotation = tracing_appender::rolling::Rotation::NEVER;
# }
```

**Tuple Struct**: `()`

**Methods:**


**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_appender::rolling::daily

*Function*

Creates a daily-rotating file appender.

The appender returned by `rolling::daily` can be used with `non_blocking` to create
a non-blocking, daily file appender.

A `RollingFileAppender` has a fixed rotation whose frequency is
defined by [`Rotation`]. The `directory` and `file_name_prefix`
arguments determine the location and file name's _prefix_ of the log file.
`RollingFileAppender` automatically appends the current date in UTC.

# Examples

``` rust
# #[clippy::allow(needless_doctest_main)]
fn main () {
# fn doc() {
    let appender = tracing_appender::rolling::daily("/some/path", "rolling.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);

    tracing::subscriber::with_default(subscriber.finish(), || {
        tracing::event!(tracing::Level::INFO, "Hello");
    });
# }
}
```

This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd`.

```rust
fn daily<impl AsRef<Path>, impl AsRef<Path>>(directory: impl Trait, file_name_prefix: impl Trait) -> RollingFileAppender
```



## tracing_appender::rolling::hourly

*Function*

Creates an hourly-rotating file appender.

The appender returned by `rolling::hourly` can be used with `non_blocking` to create
a non-blocking, hourly file appender.

The directory of the log file is specified with the `directory` argument.
`file_name_prefix` specifies the _prefix_ of the log file. `RollingFileAppender`
adds the current date and hour to the log file in UTC.

# Examples

``` rust
# #[clippy::allow(needless_doctest_main)]
fn main () {
# fn doc() {
    let appender = tracing_appender::rolling::hourly("/some/path", "rolling.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);

    tracing::subscriber::with_default(subscriber.finish(), || {
        tracing::event!(tracing::Level::INFO, "Hello");
    });
# }
}
```

This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd-HH`.

```rust
fn hourly<impl AsRef<Path>, impl AsRef<Path>>(directory: impl Trait, file_name_prefix: impl Trait) -> RollingFileAppender
```



## tracing_appender::rolling::minutely

*Function*

Creates a minutely-rotating file appender. This will rotate the log file once per minute.

The appender returned by `rolling::minutely` can be used with `non_blocking` to create
a non-blocking, minutely file appender.

The directory of the log file is specified with the `directory` argument.
`file_name_prefix` specifies the _prefix_ of the log file. `RollingFileAppender`
adds the current date, hour, and minute to the log file in UTC.

# Examples

``` rust
# #[clippy::allow(needless_doctest_main)]
fn main () {
# fn doc() {
    let appender = tracing_appender::rolling::minutely("/some/path", "rolling.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);

    tracing::subscriber::with_default(subscriber.finish(), || {
        tracing::event!(tracing::Level::INFO, "Hello");
    });
# }
}
```

This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd-HH-mm`.

```rust
fn minutely<impl AsRef<Path>, impl AsRef<Path>>(directory: impl Trait, file_name_prefix: impl Trait) -> RollingFileAppender
```



## tracing_appender::rolling::never

*Function*

Creates a non-rolling file appender.

The appender returned by `rolling::never` can be used with `non_blocking` to create
a non-blocking, non-rotating appender.

The location of the log file will be specified the `directory` passed in.
`file_name` specifies the complete name of the log file (no date or time is appended).

# Examples

``` rust
# #[clippy::allow(needless_doctest_main)]
fn main () {
# fn doc() {
    let appender = tracing_appender::rolling::never("/some/path", "non-rolling.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);

    tracing::subscriber::with_default(subscriber.finish(), || {
        tracing::event!(tracing::Level::INFO, "Hello");
    });
# }
}
```

This will result in a log file located at `/some/path/non-rolling.log`.

```rust
fn never<impl AsRef<Path>, impl AsRef<Path>>(directory: impl Trait, file_name: impl Trait) -> RollingFileAppender
```



## tracing_appender::rolling::weekly

*Function*

Creates a weekly-rotating file appender. The logs will rotate every Sunday at midnight UTC.

The appender returned by `rolling::weekly` can be used with `non_blocking` to create
a non-blocking, weekly file appender.

A `RollingFileAppender` has a fixed rotation whose frequency is
defined by [`Rotation`]. The `directory` and `file_name_prefix` arguments
determine the location and file name's _prefix_ of the log file.
`RollingFileAppender` automatically appends the current date in UTC.

# Examples

``` rust
# #[clippy::allow(needless_doctest_main)]
fn main () {
# fn doc() {
    let appender = tracing_appender::rolling::weekly("/some/path", "rolling.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);

    let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);

    tracing::subscriber::with_default(subscriber.finish(), || {
        tracing::event!(tracing::Level::INFO, "Hello");
    });
# }
}
```

This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd`.

```rust
fn weekly<impl AsRef<Path>, impl AsRef<Path>>(directory: impl Trait, file_name_prefix: impl Trait) -> RollingFileAppender
```



