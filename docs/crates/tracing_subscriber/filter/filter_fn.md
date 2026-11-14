**tracing_subscriber > filter > filter_fn**

# Module: filter::filter_fn

## Contents

**Structs**

- [`DynFilterFn`](#dynfilterfn) - A filter implemented by a closure or function pointer that
- [`FilterFn`](#filterfn) - A filter implemented by a closure or function pointer that

**Functions**

- [`dynamic_filter_fn`](#dynamic_filter_fn) - Constructs a [`DynFilterFn`] from a function or closure that returns `true`
- [`filter_fn`](#filter_fn) - Constructs a [`FilterFn`], from a function or closure that returns `true` if

---

## tracing_subscriber::filter::filter_fn::DynFilterFn

*Struct*

A filter implemented by a closure or function pointer that
determines whether a given span or event is enabled _dynamically_,
potentially based on the current [span context].

This type can be used for both [per-layer filtering][plf] (using its
[`Filter`] implementation) and [global filtering][global] (using its
[`Layer`] implementation).

See the [documentation on filtering with layers][filtering] for details.

[span context]: crate::layer::Context
[`Filter`]: crate::layer::Filter
[`Layer`]: crate::layer::Layer
[plf]: crate::layer#per-layer-filtering
[global]: crate::layer#global-filtering
[filtering]: crate::layer#filtering-with-layers

**Generic Parameters:**
- S
- F
- R

**Methods:**

- `fn with_max_level_hint<impl Into<LevelFilter>>(self: Self, max_level_hint: impl Trait) -> Self` - Sets the highest verbosity [`Level`] the filter function will enable.
- `fn with_callsite_filter<R2>(self: Self, callsite_enabled: R2) -> DynFilterFn<S, F, R2>` - Adds a function for filtering callsites to this filter.
- `fn new(enabled: F) -> Self` - Constructs a [`Filter`] from a function or closure that returns `true`

**Trait Implementations:**

- **From**
  - `fn from(f: F) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Filter**
  - `fn enabled(self: &Self, metadata: &Metadata, cx: &Context<S>) -> bool`
  - `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Layer**
  - `fn enabled(self: &Self, metadata: &Metadata, cx: Context<S>) -> bool`
  - `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`



## tracing_subscriber::filter::filter_fn::FilterFn

*Struct*

A filter implemented by a closure or function pointer that
determines whether a given span or event is enabled, based on its
[`Metadata`].

This type can be used for both [per-layer filtering][plf] (using its
[`Filter`] implementation) and [global filtering][global] (using its
[`Layer`] implementation).

See the [documentation on filtering with layers][filtering] for details.

[`Metadata`]: tracing_core::Metadata
[`Filter`]: crate::layer::Filter
[`Layer`]: crate::layer::Layer
[plf]: crate::layer#per-layer-filtering
[global]: crate::layer#global-filtering
[filtering]: crate::layer#filtering-with-layers

**Generic Parameters:**
- F

**Methods:**

- `fn new(enabled: F) -> Self` - Constructs a [`FilterFn`] from a function or closure that returns `true`
- `fn with_max_level_hint<impl Into<LevelFilter>>(self: Self, max_level_hint: impl Trait) -> Self` - Sets the highest verbosity [`Level`] the filter function will enable.

**Trait Implementations:**

- **Filter**
  - `fn enabled(self: &Self, metadata: &Metadata, _: &Context<S>) -> bool`
  - `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Layer**
  - `fn enabled(self: &Self, metadata: &Metadata, _: Context<S>) -> bool`
  - `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`
- **From**
  - `fn from(enabled: F) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::filter::filter_fn::dynamic_filter_fn

*Function*

Constructs a [`DynFilterFn`] from a function or closure that returns `true`
if a span or event should be enabled within a particular [span context][`Context`].

This is equivalent to calling [`DynFilterFn::new`].

Unlike [`filter_fn`], this function takes a closure or function pointer
taking the [`Metadata`] for a span or event *and* the current [`Context`].
This means that a [`DynFilterFn`] can choose whether to enable spans or
events based on information about the _current_ span (or its parents).

If this is *not* necessary, use [`filter_fn`] instead.

The returned [`DynFilterFn`] can be used for both [per-layer filtering][plf]
(using its [`Filter`] implementation) and [global filtering][global] (using
its  [`Layer`] implementation).

See the [documentation on filtering with layers][filtering] for details.

# Examples

```
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter,
    util::SubscriberInitExt,
};

// Only enable spans or events within a span named "interesting_span".
let my_filter = filter::dynamic_filter_fn(|metadata, cx| {
    // If this *is* "interesting_span", make sure to enable it.
    if metadata.is_span() && metadata.name() == "interesting_span" {
        return true;
    }

    // Otherwise, are we in an interesting span?
    if let Some(current_span) = cx.lookup_current() {
        return current_span.name() == "interesting_span";
    }

    false
});

let my_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    .with(my_layer.with_filter(my_filter))
    .init();

// This event will not be enabled.
tracing::info!("something happened");

tracing::info_span!("interesting_span").in_scope(|| {
    // This event will be enabled.
    tracing::debug!("something else happened");
});
```

[`Filter`]: crate::layer::Filter
[`Layer`]: crate::layer::Layer
[plf]: crate::layer#per-layer-filtering
[global]: crate::layer#global-filtering
[filtering]: crate::layer#filtering-with-layers
[`Context`]: crate::layer::Context
[`Metadata`]: tracing_core::Metadata

```rust
fn dynamic_filter_fn<S, F>(f: F) -> DynFilterFn<S, F>
```



## tracing_subscriber::filter::filter_fn::filter_fn

*Function*

Constructs a [`FilterFn`], from a function or closure that returns `true` if
a span or event should be enabled, based on its [`Metadata`].

The returned [`FilterFn`] can be used for both [per-layer filtering][plf]
(using its [`Filter`] implementation) and [global filtering][global] (using
its  [`Layer`] implementation).

See the [documentation on filtering with layers][filtering] for details.

This is equivalent to calling [`FilterFn::new`].

[`Metadata`]: tracing_core::Metadata
[`Filter`]: crate::layer::Filter
[`Layer`]: crate::layer::Layer
[plf]: crate::layer#per-layer-filtering
[global]: crate::layer#global-filtering
[filtering]: crate::layer#filtering-with-layers

# Examples

```
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter,
    util::SubscriberInitExt,
};

let my_filter = filter::filter_fn(|metadata| {
    // Only enable spans or events with the target "interesting_things"
    metadata.target() == "interesting_things"
});

let my_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    .with(my_layer.with_filter(my_filter))
    .init();

// This event will not be enabled.
tracing::warn!("something important but uninteresting happened!");

// This event will be enabled.
tracing::debug!(target: "interesting_things", "an interesting minor detail...");
```

```rust
fn filter_fn<F>(f: F) -> FilterFn<F>
```



