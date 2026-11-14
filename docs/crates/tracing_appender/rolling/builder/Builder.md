**tracing_appender > rolling > builder > Builder**

# Module: rolling::builder::Builder

## Contents

**Functions**

- [`build`](#build) - Builds a new [`RollingFileAppender`] with the configured parameters,
- [`filename_prefix`](#filename_prefix) - Sets the prefix for log filenames. The prefix is output before the
- [`filename_suffix`](#filename_suffix) - Sets the suffix for log filenames. The suffix is output after the
- [`max_log_files`](#max_log_files) - Keeps the last `n` log files on disk.
- [`rotation`](#rotation) - Sets the [rotation strategy] for log files.

---

## tracing_appender::rolling::builder::Builder::build

*Function*

Builds a new [`RollingFileAppender`] with the configured parameters,
emitting log files to the provided directory.

Unlike [`RollingFileAppender::new`], this returns a `Result` rather than
panicking when the appender cannot be initialized.

# Examples

```
use tracing_appender::rolling::{Rotation, RollingFileAppender};

# fn docs() {
let appender = RollingFileAppender::builder()
    .rotation(Rotation::DAILY) // rotate log files once per day
    .filename_prefix("myapp.log") // log files will have names like "myapp.log.2019-01-01"
    .build("/var/log/myapp") // write log files to the '/var/log/myapp' directory
    .expect("failed to initialize rolling file appender");
# drop(appender);
# }
```

This is equivalent to
```
# fn docs() {
let appender = tracing_appender::rolling::daily("myapp.log", "/var/log/myapp");
# drop(appender);
# }
```

```rust
fn build<impl AsRef<Path>>(self: &Self, directory: impl Trait) -> Result<RollingFileAppender, InitError>
```



## tracing_appender::rolling::builder::Builder::filename_prefix

*Function*

Sets the prefix for log filenames. The prefix is output before the
timestamp in the file name, and if it is non-empty, it is followed by a
dot (`.`).

By default, log files do not have a prefix.

# Examples

Setting a prefix:

```
use tracing_appender::rolling::RollingFileAppender;

# fn docs() {
let appender = RollingFileAppender::builder()
    .filename_prefix("myapp.log") // log files will have names like "myapp.log.2019-01-01"
    // ...
    .build("/var/log")
    .expect("failed to initialize rolling file appender");
# drop(appender)
# }
```

No prefix:

```
use tracing_appender::rolling::RollingFileAppender;

# fn docs() {
let appender = RollingFileAppender::builder()
    .filename_prefix("") // log files will have names like "2019-01-01"
    // ...
    .build("/var/log")
    .expect("failed to initialize rolling file appender");
# drop(appender)
# }
```

[rotation strategy]: Rotation

```rust
fn filename_prefix<impl Into<String>>(self: Self, prefix: impl Trait) -> Self
```



## tracing_appender::rolling::builder::Builder::filename_suffix

*Function*

Sets the suffix for log filenames. The suffix is output after the
timestamp in the file name, and if it is non-empty, it is preceded by a
dot (`.`).

By default, log files do not have a suffix.

# Examples

Setting a suffix:

```
use tracing_appender::rolling::RollingFileAppender;

# fn docs() {
let appender = RollingFileAppender::builder()
    .filename_suffix("myapp.log") // log files will have names like "2019-01-01.myapp.log"
    // ...
    .build("/var/log")
    .expect("failed to initialize rolling file appender");
# drop(appender)
# }
```

No suffix:

```
use tracing_appender::rolling::RollingFileAppender;

# fn docs() {
let appender = RollingFileAppender::builder()
    .filename_suffix("") // log files will have names like "2019-01-01"
    // ...
    .build("/var/log")
    .expect("failed to initialize rolling file appender");
# drop(appender)
# }
```

[rotation strategy]: Rotation

```rust
fn filename_suffix<impl Into<String>>(self: Self, suffix: impl Trait) -> Self
```



## tracing_appender::rolling::builder::Builder::max_log_files

*Function*

Keeps the last `n` log files on disk.

When constructing a [`RollingFileAppender`] or starting a new log file,
the appender will delete the oldest matching log files until at most `n`
files remain. The exact number of retained files can sometimes dip below
the maximum, so if you need to retain `m` log files, specify a max of
`m + 1`.

If `0` is supplied, the [`RollingFileAppender`] will not remove any files.

Files are considered candidates for deletion based on the following
criteria:

* The file must not be a directory or symbolic link.
* If the appender is configured with a [`filename_prefix`], the file
  name must start with that prefix.
* If the appender is configured with a [`filename_suffix`], the file
  name must end with that suffix.
* If the appender has neither a filename prefix nor a suffix, then the
  file name must parse as a valid date based on the appender's date
  format.

Files matching these criteria may be deleted if the maximum number of
log files in the directory has been reached.

[`filename_prefix`]: Self::filename_prefix
[`filename_suffix`]: Self::filename_suffix

# Examples

```
use tracing_appender::rolling::RollingFileAppender;

# fn docs() {
let appender = RollingFileAppender::builder()
    .max_log_files(5) // only the most recent 5 log files will be kept
    // ...
    .build("/var/log")
    .expect("failed to initialize rolling file appender");
# drop(appender)
# }
```

```rust
fn max_log_files(self: Self, n: usize) -> Self
```



## tracing_appender::rolling::builder::Builder::rotation

*Function*

Sets the [rotation strategy] for log files.

By default, this is [`Rotation::NEVER`].

# Examples

```
# fn docs() {
use tracing_appender::rolling::{Rotation, RollingFileAppender};

let appender = RollingFileAppender::builder()
    .rotation(Rotation::HOURLY) // rotate log files once every hour
    // ...
    .build("/var/log")
    .expect("failed to initialize rolling file appender");

# drop(appender)
# }
```

[rotation strategy]: Rotation

```rust
fn rotation(self: Self, rotation: Rotation) -> Self
```



