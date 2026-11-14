**tracing_subscriber > layer > context**

# Module: layer::context

## Contents

**Structs**

- [`Context`](#context) - Represents information about the current context provided to [`Layer`]s by the

---

## tracing_subscriber::layer::context::Context

*Struct*

Represents information about the current context provided to [`Layer`]s by the
wrapped [`Subscriber`].

To access [stored data] keyed by a span ID, implementors of the `Layer`
trait should ensure that the `Subscriber` type parameter is *also* bound by the
[`LookupSpan`]:

```rust
use tracing::Subscriber;
use tracing_subscriber::{Layer, registry::LookupSpan};

pub struct MyLayer;

impl<S> Layer<S> for MyLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    // ...
}
```

[`Layer`]: super::Layer
[`Subscriber`]: tracing_core::Subscriber
[stored data]: crate::registry::SpanRef
[`LookupSpan`]: crate::registry::LookupSpan

**Generic Parameters:**
- 'a
- S

**Methods:**

- `fn current_span(self: &Self) -> span::Current` - Returns the wrapped subscriber's view of the current span.
- `fn enabled(self: &Self, metadata: &Metadata) -> bool` - Returns whether the wrapped subscriber would enable the current span.
- `fn event(self: &Self, event: &Event)` - Records the provided `event` with the wrapped subscriber.
- `fn event_span(self: &Self, event: &Event) -> Option<SpanRef<S>>` - Returns a [`SpanRef`] for the parent span of the given [`Event`], if
- `fn metadata(self: &Self, id: &span::Id) -> Option<&'static Metadata<'static>>` - Returns metadata for the span with the given `id`, if it exists.
- `fn span(self: &Self, id: &span::Id) -> Option<registry::SpanRef<S>>` - Returns [stored data] for the span with the given `id`, if it exists.
- `fn exists(self: &Self, id: &span::Id) -> bool` - Returns `true` if an active span exists for the given `Id`.
- `fn lookup_current(self: &Self) -> Option<registry::SpanRef<S>>` - Returns [stored data] for the span that the wrapped subscriber considers
- `fn span_scope(self: &Self, id: &span::Id) -> Option<registry::Scope<S>>` - Returns an iterator over the [stored data] for all the spans in the
- `fn event_scope(self: &Self, event: &Event) -> Option<registry::Scope<S>>` - Returns an iterator over the [stored data] for all the spans in the

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



