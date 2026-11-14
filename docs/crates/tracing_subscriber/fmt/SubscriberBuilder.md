**tracing_subscriber > fmt > SubscriberBuilder**

# Module: fmt::SubscriberBuilder

## Contents

**Functions**

- [`finish`](#finish) - Finish the builder, returning a new `FmtSubscriber`.
- [`init`](#init) - Install this Subscriber as the global default.
- [`try_init`](#try_init) - Install this Subscriber as the global default if one is
- [`with_max_level`](#with_max_level) - Sets the maximum [verbosity level] that will be enabled by the
- [`with_span_events`](#with_span_events) - Configures how synthesized events are emitted at points in the [span
- [`without_time`](#without_time) - Do not emit timestamps with log messages.

---

## tracing_subscriber::fmt::SubscriberBuilder::finish

*Function*

Finish the builder, returning a new `FmtSubscriber`.

```rust
fn finish(self: Self) -> Subscriber<N, E, F, W>
```



## tracing_subscriber::fmt::SubscriberBuilder::init

*Function*

Install this Subscriber as the global default.

If the `tracing-log` feature is enabled, this will also install
the LogTracer to convert `Log` records into `tracing` `Event`s.

# Panics
Panics if the initialization was unsuccessful, likely because a
global subscriber was already installed by another call to `try_init`.

```rust
fn init(self: Self)
```



## tracing_subscriber::fmt::SubscriberBuilder::try_init

*Function*

Install this Subscriber as the global default if one is
not already set.

If the `tracing-log` feature is enabled, this will also install
the LogTracer to convert `Log` records into `tracing` `Event`s.

# Errors
Returns an Error if the initialization was unsuccessful, likely
because a global subscriber was already installed by another
call to `try_init`.

```rust
fn try_init(self: Self) -> Result<(), Box<dyn Error>>
```



## tracing_subscriber::fmt::SubscriberBuilder::with_max_level

*Function*

Sets the maximum [verbosity level] that will be enabled by the
subscriber.

If the max level has already been set, or a [`EnvFilter`] was added by
[`with_env_filter`], this replaces that configuration with the new
maximum level.

# Examples

Enable up to the `DEBUG` verbosity level:
```rust
use tracing_subscriber::fmt;
use tracing::Level;

fmt()
    .with_max_level(Level::DEBUG)
    .init();
```
This subscriber won't record any spans or events!
```rust
use tracing_subscriber::{fmt, filter::LevelFilter};

let subscriber = fmt()
    .with_max_level(LevelFilter::OFF)
    .finish();
```
[verbosity level]: tracing_core::Level
[`EnvFilter`]: struct@crate::filter::EnvFilter
[`with_env_filter`]: fn@Self::with_env_filter

```rust
fn with_max_level<impl Into<LevelFilter>>(self: Self, filter: impl Trait) -> SubscriberBuilder<N, E, LevelFilter, W>
```



## tracing_subscriber::fmt::SubscriberBuilder::with_span_events

*Function*

Configures how synthesized events are emitted at points in the [span
lifecycle][lifecycle].

The following options are available:

- `FmtSpan::NONE`: No events will be synthesized when spans are
  created, entered, exited, or closed. Data from spans will still be
  included as the context for formatted events. This is the default.
- `FmtSpan::NEW`: An event will be synthesized when spans are created.
- `FmtSpan::ENTER`: An event will be synthesized when spans are entered.
- `FmtSpan::EXIT`: An event will be synthesized when spans are exited.
- `FmtSpan::CLOSE`: An event will be synthesized when a span closes. If
  [timestamps are enabled][time] for this formatter, the generated
  event will contain fields with the span's _busy time_ (the total
  time for which it was entered) and _idle time_ (the total time that
  the span existed but was not entered).
- `FmtSpan::ACTIVE`: An event will be synthesized when spans are entered
  or exited.
- `FmtSpan::FULL`: Events will be synthesized whenever a span is
  created, entered, exited, or closed. If timestamps are enabled, the
  close event will contain the span's busy and idle time, as
  described above.

The options can be enabled in any combination. For instance, the following
will synthesize events whenever spans are created and closed:

```rust
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt;

let subscriber = fmt()
    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    .finish();
```

Note that the generated events will only be part of the log output by
this formatter; they will not be recorded by other `Subscriber`s or by
`Layer`s added to this subscriber.

[lifecycle]: https://docs.rs/tracing/latest/tracing/span/index.html#the-span-lifecycle
[time]: SubscriberBuilder::without_time()

```rust
fn with_span_events(self: Self, kind: format::FmtSpan) -> Self
```



## tracing_subscriber::fmt::SubscriberBuilder::without_time

*Function*

Do not emit timestamps with log messages.

```rust
fn without_time(self: Self) -> SubscriberBuilder<N, format::Format<L, ()>, F, W>
```



