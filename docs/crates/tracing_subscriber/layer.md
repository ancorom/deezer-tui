**tracing_subscriber > layer**

# Module: layer

## Contents

**Structs**

- [`Identity`](#identity) - A layer that does nothing.

**Traits**

- [`Filter`](#filter) - A per-[`Layer`] filter that determines whether a span or event is enabled
- [`Layer`](#layer) - A composable handler for `tracing` events.
- [`SubscriberExt`](#subscriberext) - Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

---

## tracing_subscriber::layer::Filter

*Trait*

A per-[`Layer`] filter that determines whether a span or event is enabled
for an individual layer.

See [the module-level documentation][plf] for details on using [`Filter`]s.

[plf]: crate::layer#per-layer-filtering

**Methods:**

- `enabled`: Returns `true` if this layer is interested in a span or event with the
- `callsite_enabled`: Returns an [`Interest`] indicating whether this layer will [always],
- `event_enabled`: Called before the filtered [`Layer]'s [`on_event`], to determine if
- `max_level_hint`: Returns an optional hint of the highest [verbosity level][level] that
- `on_new_span`: Notifies this filter that a new span was constructed with the given
- `on_record`: Notifies this filter that a span with the given `Id` recorded the given
- `on_enter`: Notifies this filter that a span with the given ID was entered.
- `on_exit`: Notifies this filter that a span with the given ID was exited.
- `on_close`: Notifies this filter that a span with the given ID has been closed.



## tracing_subscriber::layer::Identity

*Struct*

A layer that does nothing.

**Methods:**

- `fn new() -> Self` - Returns a new `Identity` layer.

**Traits:** Layer

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::layer::Layer

*Trait*

A composable handler for `tracing` events.

A `Layer` implements a behavior for recording or collecting traces that can
be composed together with other `Layer`s to build a [`Subscriber`]. See the
[module-level documentation](crate::layer) for details.

[`Subscriber`]: tracing_core::Subscriber

**Methods:**

- `on_register_dispatch`: Performs late initialization when installing this layer as a
- `on_layer`: Performs late initialization when attaching a `Layer` to a
- `register_callsite`: Registers a new callsite with this layer, returning whether or not
- `enabled`: Returns `true` if this layer is interested in a span or event with the
- `on_new_span`: Notifies this layer that a new span was constructed with the given
- `on_record`: Notifies this layer that a span with the given `Id` recorded the given
- `on_follows_from`: Notifies this layer that a span with the ID `span` recorded that it
- `event_enabled`: Called before [`on_event`], to determine if `on_event` should be called.
- `on_event`: Notifies this layer that an event has occurred.
- `on_enter`: Notifies this layer that a span with the given ID was entered.
- `on_exit`: Notifies this layer that the span with the given ID was exited.
- `on_close`: Notifies this layer that the span with the given ID has been closed.
- `on_id_change`: Notifies this layer that a span ID has been cloned, and that the
- `and_then`: Composes this layer around the given `Layer`, returning a `Layered`
- `with_subscriber`:  Composes this `Layer` with the given [`Subscriber`], returning a
- `with_filter`: Combines `self` with a [`Filter`], returning a [`Filtered`] layer.
- `boxed`: Erases the type of this [`Layer`], returning a [`Box`]ed `dyn



## tracing_subscriber::layer::SubscriberExt

*Trait*

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

**Methods:**

- `with`: Wraps `self` with the provided `layer`.



