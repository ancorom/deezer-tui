**tracing_subscriber > fmt > writer**

# Module: fmt::writer

## Contents

**Structs**

- [`BoxMakeWriter`](#boxmakewriter) - A writer that erases the specific [`io::Write`] and [`MakeWriter`] types being used.
- [`MutexGuardWriter`](#mutexguardwriter) - A type implementing [`io::Write`] for a [`MutexGuard`] where the type
- [`OrElse`](#orelse) - Combines a [`MakeWriter`] that returns an [`OptionalWriter`] with another
- [`Tee`](#tee) - Combines two types implementing [`MakeWriter`] (or [`std::io::Write`]) to
- [`TestWriter`](#testwriter) - A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.
- [`WithFilter`](#withfilter) - A [`MakeWriter`] combinator that wraps a [`MakeWriter`] with a predicate for
- [`WithMaxLevel`](#withmaxlevel) - A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
- [`WithMinLevel`](#withminlevel) - A [`MakeWriter`] combinator that only returns an enabled [writer] for spans

**Enums**

- [`EitherWriter`](#eitherwriter) - A [writer] that is one of two types implementing [`io::Write`].

**Traits**

- [`MakeWriter`](#makewriter) - A type that can create [`io::Write`] instances.
- [`MakeWriterExt`](#makewriterext) - Extension trait adding combinators for working with types implementing

**Type Aliases**

- [`OptionalWriter`](#optionalwriter) - A [writer] which may or may not be enabled.

---

## tracing_subscriber::fmt::writer::BoxMakeWriter

*Struct*

A writer that erases the specific [`io::Write`] and [`MakeWriter`] types being used.

This is useful in cases where the concrete type of the writer cannot be known
until runtime.

# Examples

A function that returns a [`Subscriber`] that will write to either stdout or stderr:

```rust
# use tracing::Subscriber;
# use tracing_subscriber::fmt::writer::BoxMakeWriter;

fn dynamic_writer(use_stderr: bool) -> impl Subscriber {
    let writer = if use_stderr {
        BoxMakeWriter::new(std::io::stderr)
    } else {
        BoxMakeWriter::new(std::io::stdout)
    };

    tracing_subscriber::fmt().with_writer(writer).finish()
}
```

[`Subscriber`]: tracing::Subscriber
[`io::Write`]: std::io::Write

**Methods:**

- `fn new<M>(make_writer: M) -> Self` - Constructs a `BoxMakeWriter` wrapping a type implementing [`MakeWriter`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
  - `fn make_writer_for(self: &'a Self, meta: &Metadata) -> <Self as >::Writer`



## tracing_subscriber::fmt::writer::EitherWriter

*Enum*

A [writer] that is one of two types implementing [`io::Write`].

This may be used by [`MakeWriter`] implementations that may conditionally
return one of two writers.

[writer]: std::io::Write

**Generic Parameters:**
- A
- B

**Variants:**
- `A(A)` - A writer of type `A`.
- `B(B)` - A writer of type `B`.

**Methods:**

- `fn none() -> Self` - Returns a [disabled writer].
- `fn some(t: T) -> Self` - Returns an enabled writer of type `T`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
  - `fn write_vectored(self: & mut Self, bufs: &[io::IoSlice]) -> io::Result<usize>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> io::Result<()>`
  - `fn write_fmt(self: & mut Self, fmt: std::fmt::Arguments) -> io::Result<()>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## tracing_subscriber::fmt::writer::MakeWriter

*Trait*

A type that can create [`io::Write`] instances.

`MakeWriter` is used by [`fmt::Layer`] or [`fmt::Subscriber`] to print
formatted text representations of [`Event`]s.

This trait is already implemented for function pointers and
immutably-borrowing closures that return an instance of [`io::Write`], such
as [`io::stdout`] and [`io::stderr`]. Additionally, it is implemented for
[`std::sync::Mutex`] when the type inside the mutex implements
[`io::Write`].

# Examples

The simplest usage is to pass in a named function that returns a writer. For
example, to log all events to stderr, we could write:
```
let subscriber = tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .finish();
# drop(subscriber);
```

Any function that returns a writer can be used:

```
fn make_my_great_writer() -> impl std::io::Write {
    // ...
    # std::io::stdout()
}

let subscriber = tracing_subscriber::fmt()
    .with_writer(make_my_great_writer)
    .finish();
# drop(subscriber);
```

A closure can be used to introduce arbitrary logic into how the writer is
created. Consider the (admittedly rather silly) example of sending every 5th
event to stderr, and all other events to stdout:

```
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

let n = AtomicUsize::new(0);
let subscriber = tracing_subscriber::fmt()
    .with_writer(move || -> Box<dyn io::Write> {
        if n.fetch_add(1, Relaxed) % 5 == 0 {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
       }
    })
    .finish();
# drop(subscriber);
```

A single instance of a type implementing [`io::Write`] may be used as a
`MakeWriter` by wrapping it in a [`Mutex`]. For example, we could
write to a file like so:

```
use std::{fs::File, sync::Mutex};

# fn docs() -> Result<(), Box<dyn std::error::Error>> {
let log_file = File::create("my_cool_trace.log")?;
let subscriber = tracing_subscriber::fmt()
    .with_writer(Mutex::new(log_file))
    .finish();
# drop(subscriber);
# Ok(())
# }
```

[`io::Write`]: std::io::Write
[`fmt::Layer`]: crate::fmt::Layer
[`fmt::Subscriber`]: crate::fmt::Subscriber
[`Event`]: tracing_core::event::Event
[`io::stdout`]: std::io::stdout()
[`io::stderr`]: std::io::stderr()
[`MakeWriter::make_writer_for`]: MakeWriter::make_writer_for
[`Metadata`]: tracing_core::Metadata
[levels]: tracing_core::Level
[targets]: tracing_core::Metadata::target

**Methods:**

- `Writer`: The concrete [`io::Write`] implementation returned by [`make_writer`].
- `make_writer`: Returns an instance of [`Writer`].
- `make_writer_for`: Returns a [`Writer`] for writing data from the span or event described



## tracing_subscriber::fmt::writer::MakeWriterExt

*Trait*

Extension trait adding combinators for working with types implementing
[`MakeWriter`].

This is not intended to be implemented directly for user-defined
[`MakeWriter`]s; instead, it should be imported when the desired methods are
used.

**Methods:**

- `with_max_level`: Wraps `self` and returns a [`MakeWriter`] that will only write output
- `with_min_level`: Wraps `self` and returns a [`MakeWriter`] that will only write output
- `with_filter`: Wraps `self` with a predicate that takes a span or event's [`Metadata`]
- `and`: Combines `self` with another type implementing [`MakeWriter`], returning
- `or_else`: Combines `self` with another type implementing [`MakeWriter`], returning



## tracing_subscriber::fmt::writer::MutexGuardWriter

*Struct*

A type implementing [`io::Write`] for a [`MutexGuard`] where the type
inside the [`Mutex`] implements [`io::Write`].

This is used by the [`MakeWriter`] implementation for [`Mutex`], because
[`MutexGuard`] itself will not implement [`io::Write`] — instead, it
_dereferences_ to a type implementing [`io::Write`]. Because [`MakeWriter`]
requires the `Writer` type to implement [`io::Write`], it's necessary to add
a newtype that forwards the trait implementation.

[`io::Write`]: std::io::Write
[`MutexGuard`]: std::sync::MutexGuard
[`Mutex`]: std::sync::Mutex

**Generic Parameters:**
- 'a
- W

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
  - `fn write_vectored(self: & mut Self, bufs: &[io::IoSlice]) -> io::Result<usize>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> io::Result<()>`
  - `fn write_fmt(self: & mut Self, fmt: std::fmt::Arguments) -> io::Result<()>`



## tracing_subscriber::fmt::writer::OptionalWriter

*Type Alias*: `EitherWriter<T, std::io::Sink>`

A [writer] which may or may not be enabled.

This may be used by [`MakeWriter`] implementations that wish to
conditionally enable or disable the returned writer based on a span or
event's [`Metadata`].

[writer]: std::io::Write



## tracing_subscriber::fmt::writer::OrElse

*Struct*

Combines a [`MakeWriter`] that returns an [`OptionalWriter`] with another
[`MakeWriter`], so that the second [`MakeWriter`] is used when the first
[`MakeWriter`] returns [`OptionalWriter::none`][own].

This is returned by the [`MakeWriterExt::or_else] method. See the
method documentation for details.

[own]: EitherWriter::none

**Generic Parameters:**
- A
- B

**Methods:**

- `fn new<'a, W>(inner: A, or_else: B) -> Self` - Combines

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
  - `fn make_writer_for(self: &'a Self, meta: &Metadata) -> <Self as >::Writer`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## tracing_subscriber::fmt::writer::Tee

*Struct*

Combines two types implementing [`MakeWriter`] (or [`std::io::Write`]) to
produce a writer that writes to both [`MakeWriter`]'s returned writers.

This is returned by the [`MakeWriterExt::and`] method. See the method
documentation for details.

**Generic Parameters:**
- A
- B

**Methods:**

- `fn new(a: A, b: B) -> Self` - Combines two types implementing [`MakeWriter`], returning

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
  - `fn write_vectored(self: & mut Self, bufs: &[io::IoSlice]) -> io::Result<usize>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> io::Result<()>`
  - `fn write_fmt(self: & mut Self, fmt: std::fmt::Arguments) -> io::Result<()>`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
  - `fn make_writer_for(self: &'a Self, meta: &Metadata) -> <Self as >::Writer`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::fmt::writer::TestWriter

*Struct*

A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.

`TestWriter` is used by [`fmt::Subscriber`] or [`fmt::Layer`] to enable capturing support.

`cargo test` can only capture output from the standard library's [`print!`] and [`eprint!`]
macros. See [`libtest`'s output capturing][capturing] and
[rust-lang/rust#90785](https://github.com/rust-lang/rust/issues/90785) for more details about
output capturing.

Writing to [`io::stdout`] and [`io::stderr`] produces the same results as using
[`libtest`'s `--nocapture` option][nocapture] which may make the results look unreadable.

[`fmt::Subscriber`]: super::Subscriber
[`fmt::Layer`]: super::Layer
[capturing]: https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output
[nocapture]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
[`io::stdout`]: std::io::stdout
[`io::stderr`]: std::io::stderr
[`print!`]: std::print!

**Methods:**

- `fn new() -> Self` - Returns a new `TestWriter` with the default configuration.
- `fn with_stderr() -> Self` - Returns a new `TestWriter` that writes to `stderr` instead of `stdout`.

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## tracing_subscriber::fmt::writer::WithFilter

*Struct*

A [`MakeWriter`] combinator that wraps a [`MakeWriter`] with a predicate for
span and event [`Metadata`], so that the [`MakeWriter::make_writer_for`]
method returns [`OptionalWriter::some`][ows] when the predicate returns `true`,
and [`OptionalWriter::none`][own] when the predicate returns `false`.

This is returned by the [`MakeWriterExt::with_filter`] method. See the
method documentation for details.

[`Metadata`]: tracing_core::Metadata
[ows]: EitherWriter::some
[own]: EitherWriter::none

**Generic Parameters:**
- M
- F

**Methods:**

- `fn new(make: M, filter: F) -> Self` - Wraps `make` with the provided `filter`, returning a [`MakeWriter`] that

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
  - `fn make_writer_for(self: &'a Self, meta: &Metadata) -> <Self as >::Writer`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## tracing_subscriber::fmt::writer::WithMaxLevel

*Struct*

A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
and events with metadata at or below a specified verbosity [`Level`].

This is returned by the [`MakeWriterExt::with_max_level`] method. See the
method documentation for details.

[writer]: std::io::Write
[`Level`]: tracing_core::Level

**Generic Parameters:**
- M

**Methods:**

- `fn new(make: M, level: tracing_core::Level) -> Self` - Wraps the provided [`MakeWriter`] with a maximum [`Level`], so that it

**Traits:** Copy, Eq

**Trait Implementations:**

- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
  - `fn make_writer_for(self: &'a Self, meta: &Metadata) -> <Self as >::Writer`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::fmt::writer::WithMinLevel

*Struct*

A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
and events with metadata at or above a specified verbosity [`Level`].

This is returned by the [`MakeWriterExt::with_min_level`] method. See the
method documentation for details.

[writer]: std::io::Write
[`Level`]: tracing_core::Level

**Generic Parameters:**
- M

**Methods:**

- `fn new(make: M, level: tracing_core::Level) -> Self` - Wraps the provided [`MakeWriter`] with a minimum [`Level`], so that it

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **MakeWriter**
  - `fn make_writer(self: &'a Self) -> <Self as >::Writer`
  - `fn make_writer_for(self: &'a Self, meta: &Metadata) -> <Self as >::Writer`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



