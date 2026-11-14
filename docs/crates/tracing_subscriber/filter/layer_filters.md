**tracing_subscriber > filter > layer_filters**

# Module: filter::layer_filters

## Contents

**Modules**

- [`combinator`](#combinator) - Filter combinators

**Structs**

- [`FilterId`](#filterid) - Uniquely identifies an individual [`Filter`] instance in the context of
- [`Filtered`](#filtered) - A [`Layer`] that wraps an inner [`Layer`] and adds a [`Filter`] which

**Traits**

- [`FilterExt`](#filterext) - Extension trait adding [combinators] for combining [`Filter`].

---

## tracing_subscriber::filter::layer_filters::FilterExt

*Trait*

Extension trait adding [combinators] for combining [`Filter`].

[combinators]: crate::filter::combinator
[`Filter`]: crate::layer::Filter

**Methods:**

- `and`: Combines this [`Filter`] with another [`Filter`] s so that spans and
- `or`: Combines two [`Filter`]s so that spans and events are enabled if *either* filter
- `not`: Inverts `self`, returning a filter that enables spans and events only if
- `boxed`: [Boxes] `self`, erasing its concrete type.



## tracing_subscriber::filter::layer_filters::FilterId

*Struct*

Uniquely identifies an individual [`Filter`] instance in the context of
a [`Subscriber`].

When adding a [`Filtered`] [`Layer`] to a [`Subscriber`], the [`Subscriber`]
generates a `FilterId` for that [`Filtered`] layer. The [`Filtered`] layer
will then use the generated ID to query whether a particular span was
previously enabled by that layer's [`Filter`].

**Note**: Currently, the [`Registry`] type provided by this crate is the
**only** [`Subscriber`] implementation capable of participating in per-layer
filtering. Therefore, the `FilterId` type cannot currently be constructed by
code outside of `tracing-subscriber`. In the future, new APIs will be added to `tracing-subscriber` to
allow non-Registry [`Subscriber`]s to also participate in per-layer
filtering. When those APIs are added, subscribers will be responsible
for generating and assigning `FilterId`s.

[`Filter`]: crate::layer::Filter
[`Subscriber`]: tracing_core::Subscriber
[`Layer`]: crate::layer::Layer
[`Registry`]: crate::registry::Registry

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **Binary**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::filter::layer_filters::Filtered

*Struct*

A [`Layer`] that wraps an inner [`Layer`] and adds a [`Filter`] which
controls what spans and events are enabled for that layer.

This is returned by the [`Layer::with_filter`] method. See the
[documentation on per-layer filtering][plf] for details.

[`Filter`]: crate::layer::Filter
[plf]: crate::layer#per-layer-filtering

**Generic Parameters:**
- L
- F
- S

**Methods:**

- `fn new(layer: L, filter: F) -> Self` - Wraps the provided [`Layer`] so that it is filtered by the given
- `fn filter(self: &Self) -> &F` - Borrows the [`Filter`](crate::layer::Filter) used by this layer.
- `fn filter_mut(self: & mut Self) -> & mut F` - Mutably borrows the [`Filter`](crate::layer::Filter) used by this layer.
- `fn inner(self: &Self) -> &L` - Borrows the inner [`Layer`] wrapped by this `Filtered` layer.
- `fn inner_mut(self: & mut Self) -> & mut L` - Mutably borrows the inner [`Layer`] wrapped by this `Filtered` layer.

**Trait Implementations:**

- **Layer**
  - `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`
  - `fn on_layer(self: & mut Self, subscriber: & mut S)`
  - `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn enabled(self: &Self, metadata: &Metadata, cx: Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &span::Attributes, id: &span::Id, cx: Context<S>)`
  - `fn on_record(self: &Self, span: &span::Id, values: &span::Record, cx: Context<S>)`
  - `fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, cx: Context<S>)`
  - `fn event_enabled(self: &Self, event: &Event, cx: Context<S>) -> bool`
  - `fn on_event(self: &Self, event: &Event, cx: Context<S>)`
  - `fn on_enter(self: &Self, id: &span::Id, cx: Context<S>)`
  - `fn on_exit(self: &Self, id: &span::Id, cx: Context<S>)`
  - `fn on_close(self: &Self, id: span::Id, cx: Context<S>)`
  - `fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, cx: Context<S>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: combinator

Filter combinators



