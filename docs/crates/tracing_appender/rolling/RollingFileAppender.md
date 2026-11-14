**tracing_appender > rolling > RollingFileAppender**

# Module: rolling::RollingFileAppender

## Contents

**Functions**

- [`builder`](#builder) - Returns a new [`Builder`] for configuring a `RollingFileAppender`.
- [`new`](#new) - Creates a new `RollingFileAppender`.

---

## tracing_appender::rolling::RollingFileAppender::builder

*Function*

Returns a new [`Builder`] for configuring a `RollingFileAppender`.

The builder interface can be used to set additional configuration
parameters when constructing a new appender.

Unlike [`RollingFileAppender::new`], the [`Builder::build`] method
returns a `Result` rather than panicking when the appender cannot be
initialized. Therefore, the builder interface can also be used when
appender initialization errors should be handled gracefully.

# Examples

```rust
# fn docs() {
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let file_appender = RollingFileAppender::builder()
    .rotation(Rotation::HOURLY) // rotate log files once every hour
    .filename_prefix("myapp") // log file names will be prefixed with `myapp.`
    .filename_suffix("log") // log file names will be suffixed with `.log`
    .build("/var/log") // try to build an appender that stores log files in `/var/log`
    .expect("initializing rolling file appender failed");
# drop(file_appender);
# }
```

```rust
fn builder() -> Builder
```



## tracing_appender::rolling::RollingFileAppender::new

*Function*

Creates a new `RollingFileAppender`.

A `RollingFileAppender` will have a fixed rotation whose frequency is
defined by [`Rotation`]. The `directory` and `file_name_prefix`
arguments determine the location and file name's _prefix_ of the log
file. `RollingFileAppender` will automatically append the current date
and hour (UTC format) to the file name.

Alternatively, a `RollingFileAppender` can be constructed using one of the following helpers:

- [`Rotation::minutely()`][minutely],
- [`Rotation::hourly()`][hourly],
- [`Rotation::daily()`][daily],
- [`Rotation::never()`][never()]

Additional parameters can be configured using [`RollingFileAppender::builder`].

# Examples

```rust
# fn docs() {
use tracing_appender::rolling::{RollingFileAppender, Rotation};
let file_appender = RollingFileAppender::new(Rotation::HOURLY, "/some/directory", "prefix.log");
# }
```

```rust
fn new<impl AsRef<Path>, impl AsRef<Path>>(rotation: Rotation, directory: impl Trait, filename_prefix: impl Trait) -> RollingFileAppender
```



