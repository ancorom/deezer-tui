**tracing_subscriber > fmt > fmt_layer**

# Module: fmt::fmt_layer

## Contents

**Structs**

- [`FmtContext`](#fmtcontext) - Provides the current span context to a formatter.
- [`FormattedFields`](#formattedfields) - A formatted representation of a span's fields stored in its [extensions].
- [`Layer`](#layer) - A [`Layer`] that logs formatted representations of `tracing` events.

---

## tracing_subscriber::fmt::fmt_layer::FmtContext

*Struct*

Provides the current span context to a formatter.

**Generic Parameters:**
- 'a
- S
- N

**Methods:**

- `fn visit_spans<E, F>(self: &Self, f: F) -> Result<(), E>` - Visits every span in the current context with a closure.
- `fn metadata(self: &Self, id: &Id) -> Option<&'static Metadata<'static>>` - Returns metadata for the span with the given `id`, if it exists.
- `fn span(self: &Self, id: &Id) -> Option<SpanRef<S>>` - Returns [stored data] for the span with the given `id`, if it exists.
- `fn exists(self: &Self, id: &Id) -> bool` - Returns `true` if an active span exists for the given `Id`.
- `fn lookup_current(self: &Self) -> Option<SpanRef<S>>` - Returns [stored data] for the span that the wrapped subscriber considers
- `fn current_span(self: &Self) -> Current` - Returns the current span for this formatter.
- `fn parent_span(self: &Self) -> Option<SpanRef<S>>` - Returns [stored data] for the parent span of the event currently being
- `fn span_scope(self: &Self, id: &Id) -> Option<registry::Scope<S>>` - Returns an iterator over the [stored data] for all the spans in the
- `fn event_scope(self: &Self) -> Option<registry::Scope<S>>` - Returns an iterator over the [stored data] for all the spans in the
- `fn field_format(self: &Self) -> &N` - Returns the [field formatter] configured by the subscriber invoking

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FormatFields**
  - `fn format_fields<R>(self: &Self, writer: format::Writer<'writer>, fields: R) -> fmt::Result`



## tracing_subscriber::fmt::fmt_layer::FormattedFields

*Struct*

A formatted representation of a span's fields stored in its [extensions].

Because `FormattedFields` is generic over the type of the formatter that
produced it, multiple versions of a span's formatted fields can be stored in
the [`Extensions`][extensions] type-map. This means that when multiple
formatters are in use, each can store its own formatted representation
without conflicting.

[extensions]: crate::registry::Extensions

**Generic Parameters:**
- E

**Fields:**
- `fields: alloc::string::String` - The formatted fields of a span.

**Methods:**

- `fn new(fields: String) -> Self` - Returns a new `FormattedFields`.
- `fn as_writer(self: & mut Self) -> format::Writer` - Returns a new [`format::Writer`] for writing to this `FormattedFields`.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tracing_subscriber::fmt::fmt_layer::Layer

*Struct*

A [`Layer`] that logs formatted representations of `tracing` events.

## Examples

Constructing a layer with the default configuration:

```rust
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::prelude::*;

let subscriber = Registry::default()
    .with(fmt::Layer::default());

tracing::subscriber::set_global_default(subscriber).unwrap();
```

Overriding the layer's behavior:

```rust
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::prelude::*;

let fmt_layer = fmt::layer()
   .with_target(false) // don't include event targets when logging
   .with_level(false); // don't include event levels when logging

let subscriber = Registry::default().with(fmt_layer);
# tracing::subscriber::set_global_default(subscriber).unwrap();
```

Setting a custom event formatter:

```rust
use tracing_subscriber::fmt::{self, format, time};
use tracing_subscriber::prelude::*;

let fmt = format().with_timer(time::Uptime::default());
let fmt_layer = fmt::layer()
    .event_format(fmt)
    .with_target(false);
# let subscriber = fmt_layer.with_subscriber(tracing_subscriber::registry::Registry::default());
# tracing::subscriber::set_global_default(subscriber).unwrap();
```

[`Layer`]: super::layer::Layer

**Generic Parameters:**
- S
- N
- E
- W

**Methods:**

- `fn new() -> Self` - Returns a new [`Layer`][self::Layer] with the default configuration.
- `fn event_format<E2>(self: Self, e: E2) -> Layer<S, N, E2, W>` - Sets the [event formatter][`FormatEvent`] that the layer being built will
- `fn map_event_format<E2, impl FnOnce(E) -> E2>(self: Self, f: impl Trait) -> Layer<S, N, E2, W>` - Updates the event formatter by applying a function to the existing event formatter.
- `fn with_writer<W2>(self: Self, make_writer: W2) -> Layer<S, N, E, W2>` - Sets the [`MakeWriter`] that the layer being built will use to write events.
- `fn writer(self: &Self) -> &W` - Borrows the [writer] for this [`Layer`].
- `fn writer_mut(self: & mut Self) -> & mut W` - Mutably borrows the [writer] for this [`Layer`].
- `fn set_ansi(self: & mut Self, ansi: bool)` - Sets whether this layer should use ANSI terminal formatting
- `fn set_span_events(self: & mut Self, kind: FmtSpan)` - Modifies how synthesized events are emitted at points in the [span
- `fn with_test_writer(self: Self) -> Layer<S, N, E, TestWriter>` - Configures the layer to support [`libtest`'s output capturing][capturing] when used in
- `fn with_ansi(self: Self, ansi: bool) -> Self` - Sets whether or not the formatter emits ANSI terminal escape codes
- `fn with_ansi_sanitization(self: Self, ansi_sanitization: bool) -> Self` - Sets whether ANSI control character sanitization is enabled.
- `fn log_internal_errors(self: Self, log_internal_errors: bool) -> Self` - Sets whether to write errors from [`FormatEvent`] to the writer.
- `fn map_writer<W2, impl FnOnce(W) -> W2>(self: Self, f: impl Trait) -> Layer<S, N, E, W2>` - Updates the [`MakeWriter`] by applying a function to the existing [`MakeWriter`].
- `fn with_timer<T2>(self: Self, timer: T2) -> Layer<S, N, format::Format<L, T2>, W>` - Use the given [`timer`] for span and event timestamps.
- `fn without_time(self: Self) -> Layer<S, N, format::Format<L, ()>, W>` - Do not emit timestamps with spans and event.
- `fn with_span_events(self: Self, kind: FmtSpan) -> Self` - Configures how synthesized events are emitted at points in the [span
- `fn with_target(self: Self, display_target: bool) -> Layer<S, N, format::Format<L, T>, W>` - Sets whether or not an event's target is displayed.
- `fn with_file(self: Self, display_filename: bool) -> Layer<S, N, format::Format<L, T>, W>` - Sets whether or not an event's [source code file path][file] is
- `fn with_line_number(self: Self, display_line_number: bool) -> Layer<S, N, format::Format<L, T>, W>` - Sets whether or not an event's [source code line number][line] is
- `fn with_level(self: Self, display_level: bool) -> Layer<S, N, format::Format<L, T>, W>` - Sets whether or not an event's level is displayed.
- `fn with_thread_ids(self: Self, display_thread_ids: bool) -> Layer<S, N, format::Format<L, T>, W>` - Sets whether or not the [thread ID] of the current thread is displayed
- `fn with_thread_names(self: Self, display_thread_names: bool) -> Layer<S, N, format::Format<L, T>, W>` - Sets whether or not the [name] of the current thread is displayed
- `fn compact(self: Self) -> Layer<S, N, format::Format<format::Compact, T>, W>` - Sets the layer being built to use a [less verbose formatter][super::format::Compact].
- `fn pretty(self: Self) -> Layer<S, format::Pretty, format::Format<format::Pretty, T>, W>` - Sets the layer being built to use an [excessively pretty, human-readable formatter](crate::fmt::format::Pretty).
- `fn fmt_fields<N2>(self: Self, fmt_fields: N2) -> Layer<S, N2, E, W>` - Sets the field formatter that the layer being built will use to record
- `fn map_fmt_fields<N2, impl FnOnce(N) -> N2>(self: Self, f: impl Trait) -> Layer<S, N2, E, W>` - Updates the field formatter by applying a function to the existing field formatter.

**Trait Implementations:**

- **Layer**
  - `fn on_new_span(self: &Self, attrs: &Attributes, id: &Id, ctx: Context<S>)`
  - `fn on_record(self: &Self, id: &Id, values: &Record, ctx: Context<S>)`
  - `fn on_enter(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_exit(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_close(self: &Self, id: Id, ctx: Context<S>)`
  - `fn on_event(self: &Self, event: &Event, ctx: Context<S>)`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



