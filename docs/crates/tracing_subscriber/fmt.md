**tracing_subscriber > fmt**

# Module: fmt

## Contents

**Modules**

- [`format`](#format) - Formatters for logging [`tracing`] events.
- [`time`](#time) - Formatters for event timestamps.
- [`writer`](#writer) - Abstractions for creating [`io::Write`] instances.

**Structs**

- [`Subscriber`](#subscriber) - A `Subscriber` that logs formatted representations of `tracing` events.
- [`SubscriberBuilder`](#subscriberbuilder) - Configures and constructs `Subscriber`s.

**Functions**

- [`fmt`](#fmt) - Returns a new [`SubscriberBuilder`] for configuring a [formatting subscriber].
- [`init`](#init) - Install a global tracing subscriber that listens for events and
- [`layer`](#layer) - Returns a new [formatting layer] that can be [composed] with other layers to
- [`try_init`](#try_init) - Install a global tracing subscriber that listens for events and

**Type Aliases**

- [`Formatter`](#formatter) - A `Subscriber` that logs formatted representations of `tracing` events.

---

## tracing_subscriber::fmt::Formatter

*Type Alias*: `layer::Layered<fmt_layer::Layer<crate::Registry, N, E, W>, crate::Registry>`

A `Subscriber` that logs formatted representations of `tracing` events.
This type only logs formatted events; it does not perform any filtering.



## tracing_subscriber::fmt::Subscriber

*Struct*

A `Subscriber` that logs formatted representations of `tracing` events.

This consists of an inner `Formatter` wrapped in a layer that performs filtering.

**Generic Parameters:**
- N
- E
- F
- W

**Methods:**

- `fn builder() -> SubscriberBuilder` - Returns a new `SubscriberBuilder` for configuring a format subscriber.
- `fn new() -> Self` - Returns a new format subscriber with the default configuration.

**Trait Implementations:**

- **Subscriber**
  - `fn register_callsite(self: &Self, meta: &'static Metadata<'static>) -> Interest`
  - `fn enabled(self: &Self, meta: &Metadata) -> bool`
  - `fn new_span(self: &Self, attrs: &span::Attributes) -> span::Id`
  - `fn record(self: &Self, span: &span::Id, values: &span::Record)`
  - `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`
  - `fn event_enabled(self: &Self, event: &Event) -> bool`
  - `fn event(self: &Self, event: &Event)`
  - `fn enter(self: &Self, id: &span::Id)`
  - `fn exit(self: &Self, id: &span::Id)`
  - `fn current_span(self: &Self) -> span::Current`
  - `fn clone_span(self: &Self, id: &span::Id) -> span::Id`
  - `fn try_close(self: &Self, id: span::Id) -> bool`
  - `fn max_level_hint(self: &Self) -> Option<tracing_core::LevelFilter>`
  - `fn downcast_raw(self: &Self, id: TypeId) -> Option<*const ()>`
- **LookupSpan**
  - `fn span_data(self: &'a Self, id: &span::Id) -> Option<<Self as >::Data>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::fmt::SubscriberBuilder

*Struct*

Configures and constructs `Subscriber`s.

**Generic Parameters:**
- N
- E
- F
- W

**Methods:**

- `fn finish(self: Self) -> Subscriber<N, E, F, W>` - Finish the builder, returning a new `FmtSubscriber`.
- `fn try_init(self: Self) -> Result<(), Box<dyn Error>>` - Install this Subscriber as the global default if one is
- `fn init(self: Self)` - Install this Subscriber as the global default.
- `fn fmt_fields<N2>(self: Self, fmt_fields: N2) -> SubscriberBuilder<N2, E, F, W>` - Sets the field formatter that the subscriber being built will use to record
- `fn with_max_level<impl Into<LevelFilter>>(self: Self, filter: impl Trait) -> SubscriberBuilder<N, E, LevelFilter, W>` - Sets the maximum [verbosity level] that will be enabled by the
- `fn event_format<E2>(self: Self, fmt_event: E2) -> SubscriberBuilder<N, E2, F, W>` - Sets the [event formatter][`FormatEvent`] that the subscriber being built
- `fn with_writer<W2>(self: Self, make_writer: W2) -> SubscriberBuilder<N, E, F, W2>` - Sets the [`MakeWriter`] that the subscriber being built will use to write events.
- `fn with_test_writer(self: Self) -> SubscriberBuilder<N, E, F, TestWriter>` - Configures the subscriber to support [`libtest`'s output capturing][capturing] when used in
- `fn map_event_format<E2, impl FnOnce(E) -> E2>(self: Self, f: impl Trait) -> SubscriberBuilder<N, E2, F, W>` - Updates the event formatter by applying a function to the existing event formatter.
- `fn map_fmt_fields<N2, impl FnOnce(N) -> N2>(self: Self, f: impl Trait) -> SubscriberBuilder<N2, E, F, W>` - Updates the field formatter by applying a function to the existing field formatter.
- `fn map_writer<W2, impl FnOnce(W) -> W2>(self: Self, f: impl Trait) -> SubscriberBuilder<N, E, F, W2>` - Updates the [`MakeWriter`] by applying a function to the existing [`MakeWriter`].
- `fn with_timer<T2>(self: Self, timer: T2) -> SubscriberBuilder<N, format::Format<L, T2>, F, W>` - Use the given [`timer`] for log message timestamps.
- `fn without_time(self: Self) -> SubscriberBuilder<N, format::Format<L, ()>, F, W>` - Do not emit timestamps with log messages.
- `fn with_span_events(self: Self, kind: format::FmtSpan) -> Self` - Configures how synthesized events are emitted at points in the [span
- `fn with_ansi(self: Self, ansi: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not the formatter emits ANSI terminal escape codes
- `fn with_ansi_sanitization(self: Self, ansi_sanitization: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether ANSI control character sanitization is enabled.
- `fn log_internal_errors(self: Self, log_internal_errors: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether to write errors from [`FormatEvent`] to the writer.
- `fn with_target(self: Self, display_target: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not an event's target is displayed.
- `fn with_file(self: Self, display_filename: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not an event's [source code file path][file] is
- `fn with_line_number(self: Self, display_line_number: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not an event's [source code line number][line] is
- `fn with_level(self: Self, display_level: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not an event's level is displayed.
- `fn with_thread_names(self: Self, display_thread_names: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not the [name] of the current thread is displayed
- `fn with_thread_ids(self: Self, display_thread_ids: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>` - Sets whether or not the [thread ID] of the current thread is displayed
- `fn compact(self: Self) -> SubscriberBuilder<N, format::Format<format::Compact, T>, F, W>` - Sets the subscriber being built to use a less verbose formatter.
- `fn pretty(self: Self) -> SubscriberBuilder<format::Pretty, format::Format<format::Pretty, T>, F, W>` - Sets the subscriber being built to use an [excessively pretty, human-readable formatter](crate::fmt::format::Pretty).

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::fmt::fmt

*Function*

Returns a new [`SubscriberBuilder`] for configuring a [formatting subscriber].

This is essentially shorthand for [`SubscriberBuilder::default()]`.

# Examples

Using [`init`] to set the default subscriber:

```rust
tracing_subscriber::fmt().init();
```

Configuring the output format:

```rust

tracing_subscriber::fmt()
    // Configure formatting settings.
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    // Set the subscriber as the default.
    .init();
```

[`try_init`] returns an error if the default subscriber could not be set:

```rust
use std::error::Error;

fn init_subscriber() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        // Configure the subscriber to emit logs in JSON format.
        .json()
        // Configure the subscriber to flatten event fields in the output JSON objects.
        .flatten_event(true)
        // Set the subscriber as the default, returning an error if this fails.
        .try_init()?;

    Ok(())
}
```

Rather than setting the subscriber as the default, [`finish`] _returns_ the
constructed subscriber, which may then be passed to other functions:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .compact()
    .finish();

tracing::subscriber::with_default(subscriber, || {
    // the subscriber will only be set as the default
    // inside this closure...
})
```

[formatting subscriber]: Subscriber
[`SubscriberBuilder::default()`]: SubscriberBuilder::default
[`init`]: SubscriberBuilder::init()
[`try_init`]: SubscriberBuilder::try_init()
[`finish`]: SubscriberBuilder::finish()

```rust
fn fmt() -> SubscriberBuilder
```



## Module: format

Formatters for logging [`tracing`] events.

This module provides several formatter implementations, as well as utilities
for implementing custom formatters.

# Formatters
This module provides a number of formatter implementations:

* [`Full`]: The default formatter. This emits human-readable,
  single-line logs for each event that occurs, with the current span context
  displayed before the formatted representation of the event. See
  [here](Full#example-output) for sample output.

* [`Compact`]: A variant of the default formatter, optimized for
  short line lengths. Fields from the current span context are appended to
  the fields of the formatted event, and span names are not shown; the
  verbosity level is abbreviated to a single character. See
  [here](Compact#example-output) for sample output.

* [`Pretty`]: Emits excessively pretty, multi-line logs, optimized
  for human readability. This is primarily intended to be used in local
  development and debugging, or for command-line applications, where
  automated analysis and compact storage of logs is less of a priority than
  readability and visual appeal. See [here](Pretty#example-output)
  for sample output.

* [`Json`]: Outputs newline-delimited JSON logs. This is intended
  for production use with systems where structured logs are consumed as JSON
  by analysis and viewing tools. The JSON output is not optimized for human
  readability. See [here](Json#example-output) for sample output.



## tracing_subscriber::fmt::init

*Function*

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable].

The configuration of the subscriber initialized by this function
depends on what [feature flags](crate#feature-flags) are enabled.

If the `tracing-log` feature is enabled, this will also install
the LogTracer to convert `Log` records into `tracing` `Event`s.

If the `env-filter` feature is enabled, this is shorthand for

```rust
# use tracing_subscriber::EnvFilter;
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
```

# Panics
Panics if the initialization was unsuccessful, likely because a
global subscriber was already installed by another call to `try_init`.

[`RUST_LOG` environment variable]: crate::filter::EnvFilter::DEFAULT_ENV

```rust
fn init()
```



## tracing_subscriber::fmt::layer

*Function*

Returns a new [formatting layer] that can be [composed] with other layers to
construct a [`Subscriber`].

This is a shorthand for the equivalent [`Layer::default()`] function.

[formatting layer]: Layer
[composed]: crate::layer
[`Layer::default()`]: Layer::default

```rust
fn layer<S>() -> Layer<S>
```



## Module: time

Formatters for event timestamps.



## tracing_subscriber::fmt::try_init

*Function*

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable],
if one is not already set.

If the `tracing-log` feature is enabled, this will also install
the [`LogTracer`] to convert `log` records into `tracing` `Event`s.

This is shorthand for

```rust
# fn doc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
tracing_subscriber::fmt().try_init()
# }
```


# Errors

Returns an Error if the initialization was unsuccessful,
likely because a global subscriber was already installed by another
call to `try_init`.

[`LogTracer`]:
    https://docs.rs/tracing-log/0.1.0/tracing_log/struct.LogTracer.html
[`RUST_LOG` environment variable]: crate::filter::EnvFilter::DEFAULT_ENV

```rust
fn try_init() -> Result<(), alloc::boxed::Box<dyn Error>>
```



## Module: writer

Abstractions for creating [`io::Write`] instances.

[`io::Write`]: std::io::Write



