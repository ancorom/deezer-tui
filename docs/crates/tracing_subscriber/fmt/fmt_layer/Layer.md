**tracing_subscriber > fmt > fmt_layer > Layer**

# Module: fmt::fmt_layer::Layer

## Contents

**Functions**

- [`set_ansi`](#set_ansi) - Sets whether this layer should use ANSI terminal formatting
- [`with_ansi`](#with_ansi) - Sets whether or not the formatter emits ANSI terminal escape codes
- [`with_span_events`](#with_span_events) - Configures how synthesized events are emitted at points in the [span
- [`without_time`](#without_time) - Do not emit timestamps with spans and event.

---

## tracing_subscriber::fmt::fmt_layer::Layer::set_ansi

*Function*

Sets whether this layer should use ANSI terminal formatting
escape codes (such as colors).

This method is primarily expected to be used with the
[`reload::Handle::modify`](crate::reload::Handle::modify) method when changing
the writer.

```rust
fn set_ansi(self: & mut Self, ansi: bool)
```



## tracing_subscriber::fmt::fmt_layer::Layer::with_ansi

*Function*

Sets whether or not the formatter emits ANSI terminal escape codes
for colors and other text formatting.

When the "ansi" crate feature flag is enabled, ANSI colors are enabled
by default unless the [`NO_COLOR`] environment variable is set to
a non-empty value.  If the [`NO_COLOR`] environment variable is set to
any non-empty value, then ANSI colors will be suppressed by default.
The [`with_ansi`] and [`set_ansi`] methods can be used to forcibly
enable ANSI colors, overriding any [`NO_COLOR`] environment variable.

[`NO_COLOR`]: https://no-color.org/

Enabling ANSI escapes (calling `with_ansi(true)`) requires the "ansi"
crate feature flag. Calling `with_ansi(true)` without the "ansi"
feature flag enabled will panic if debug assertions are enabled, or
print a warning otherwise.

This method itself is still available without the feature flag. This
is to allow ANSI escape codes to be explicitly *disabled* without
having to opt-in to the dependencies required to emit ANSI formatting.
This way, code which constructs a formatter that should never emit
ANSI escape codes can ensure that they are not used, regardless of
whether or not other crates in the dependency graph enable the "ansi"
feature flag.

[`with_ansi`]: Layer::with_ansi
[`set_ansi`]: Layer::set_ansi

```rust
fn with_ansi(self: Self, ansi: bool) -> Self
```



## tracing_subscriber::fmt::fmt_layer::Layer::with_span_events

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
- `FmtSpan::ACTIVE`: Events will be synthesized when spans are entered
  or exited.
- `FmtSpan::FULL`: Events will be synthesized whenever a span is
  created, entered, exited, or closed. If timestamps are enabled, the
  close event will contain the span's busy and idle time, as
  described above.

The options can be enabled in any combination. For instance, the following
will synthesize events whenever spans are created and closed:

```rust
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

let subscriber = fmt()
    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    .finish();
```

Note that the generated events will only be part of the log output by
this formatter; they will not be recorded by other `Subscriber`s or by
`Layer`s added to this subscriber.

[lifecycle]: https://docs.rs/tracing/latest/tracing/span/index.html#the-span-lifecycle
[time]: Layer::without_time()

```rust
fn with_span_events(self: Self, kind: FmtSpan) -> Self
```



## tracing_subscriber::fmt::fmt_layer::Layer::without_time

*Function*

Do not emit timestamps with spans and event.

```rust
fn without_time(self: Self) -> Layer<S, N, format::Format<L, ()>, W>
```



