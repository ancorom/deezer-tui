**tracing_subscriber > reload**

# Module: reload

## Contents

**Structs**

- [`Error`](#error) - Indicates that an error occurred when reloading a layer.
- [`Handle`](#handle) - Allows reloading the state of an associated [`Layer`](crate::layer::Layer).
- [`Layer`](#layer) - Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.

---

## tracing_subscriber::reload::Error

*Struct*

Indicates that an error occurred when reloading a layer.

**Methods:**

- `fn is_poisoned(self: &Self) -> bool` - Returns `true` if this error occurred because the layer was poisoned by
- `fn is_dropped(self: &Self) -> bool` - Returns `true` if this error occurred because the `Subscriber`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::reload::Handle

*Struct*

Allows reloading the state of an associated [`Layer`](crate::layer::Layer).

**Generic Parameters:**
- L
- S

**Methods:**

- `fn reload<impl Into<L>>(self: &Self, new_value: impl Trait) -> Result<(), Error>` - Replace the current [`Layer`] or [`Filter`] with the provided `new_value`.
- `fn modify<impl FnOnce(&mut L)>(self: &Self, f: impl Trait) -> Result<(), Error>` - Invokes a closure with a mutable reference to the current layer or filter,
- `fn clone_current(self: &Self) -> Option<L>` - Returns a clone of the layer or filter's current value if it still exists.
- `fn with_current<T, impl FnOnce(&L) -> T>(self: &Self, f: impl Trait) -> Result<T, Error>` - Invokes a closure with a borrowed reference to the current layer or filter,

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::reload::Layer

*Struct*

Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.

**Generic Parameters:**
- L
- S

**Methods:**

- `fn new(inner: L) -> (Self, Handle<L, S>)` - Wraps the given [`Layer`] or [`Filter`], returning a `reload::Layer`
- `fn handle(self: &Self) -> Handle<L, S>` - Returns a `Handle` that can be used to reload the wrapped [`Layer`] or [`Filter`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Layer**
  - `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`
  - `fn on_layer(self: & mut Self, subscriber: & mut S)`
  - `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn enabled(self: &Self, metadata: &Metadata, ctx: layer::Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &span::Attributes, id: &span::Id, ctx: layer::Context<S>)`
  - `fn on_record(self: &Self, span: &span::Id, values: &span::Record, ctx: layer::Context<S>)`
  - `fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, ctx: layer::Context<S>)`
  - `fn event_enabled(self: &Self, event: &Event, ctx: layer::Context<S>) -> bool`
  - `fn on_event(self: &Self, event: &Event, ctx: layer::Context<S>)`
  - `fn on_enter(self: &Self, id: &span::Id, ctx: layer::Context<S>)`
  - `fn on_exit(self: &Self, id: &span::Id, ctx: layer::Context<S>)`
  - `fn on_close(self: &Self, id: span::Id, ctx: layer::Context<S>)`
  - `fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, ctx: layer::Context<S>)`
- **Filter**
  - `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn enabled(self: &Self, metadata: &Metadata, ctx: &layer::Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &span::Attributes, id: &span::Id, ctx: layer::Context<S>)`
  - `fn on_record(self: &Self, span: &span::Id, values: &span::Record, ctx: layer::Context<S>)`
  - `fn on_enter(self: &Self, id: &span::Id, ctx: layer::Context<S>)`
  - `fn on_exit(self: &Self, id: &span::Id, ctx: layer::Context<S>)`
  - `fn on_close(self: &Self, id: span::Id, ctx: layer::Context<S>)`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`



