**tracing_subscriber > layer > layered**

# Module: layer::layered

## Contents

**Structs**

- [`Layered`](#layered) - A [`Subscriber`] composed of a `Subscriber` wrapped by one or more

---

## tracing_subscriber::layer::layered::Layered

*Struct*

A [`Subscriber`] composed of a `Subscriber` wrapped by one or more
[`Layer`]s.

[`Layer`]: crate::Layer
[`Subscriber`]: tracing_core::Subscriber

**Generic Parameters:**
- L
- I
- S

**Methods:**

- `fn is<T>(self: &Self) -> bool` - Returns `true` if this [`Subscriber`] is the same type as `T`.
- `fn downcast_ref<T>(self: &Self) -> Option<&T>` - Returns some reference to this [`Subscriber`] value if it is of type `T`,

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Subscriber**
  - `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`
  - `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn enabled(self: &Self, metadata: &Metadata) -> bool`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`
  - `fn new_span(self: &Self, span: &span::Attributes) -> span::Id`
  - `fn record(self: &Self, span: &span::Id, values: &span::Record)`
  - `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`
  - `fn event_enabled(self: &Self, event: &Event) -> bool`
  - `fn event(self: &Self, event: &Event)`
  - `fn enter(self: &Self, span: &span::Id)`
  - `fn exit(self: &Self, span: &span::Id)`
  - `fn clone_span(self: &Self, old: &span::Id) -> span::Id`
  - `fn drop_span(self: &Self, id: span::Id)`
  - `fn try_close(self: &Self, id: span::Id) -> bool`
  - `fn current_span(self: &Self) -> span::Current`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Layer**
  - `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`
  - `fn on_layer(self: & mut Self, subscriber: & mut S)`
  - `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn enabled(self: &Self, metadata: &Metadata, ctx: Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &span::Attributes, id: &span::Id, ctx: Context<S>)`
  - `fn on_record(self: &Self, span: &span::Id, values: &span::Record, ctx: Context<S>)`
  - `fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, ctx: Context<S>)`
  - `fn event_enabled(self: &Self, event: &Event, ctx: Context<S>) -> bool`
  - `fn on_event(self: &Self, event: &Event, ctx: Context<S>)`
  - `fn on_enter(self: &Self, id: &span::Id, ctx: Context<S>)`
  - `fn on_exit(self: &Self, id: &span::Id, ctx: Context<S>)`
  - `fn on_close(self: &Self, id: span::Id, ctx: Context<S>)`
  - `fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, ctx: Context<S>)`
- **LookupSpan**
  - `fn span_data(self: &'a Self, id: &span::Id) -> Option<<Self as >::Data>`
  - `fn register_filter(self: & mut Self) -> FilterId`



