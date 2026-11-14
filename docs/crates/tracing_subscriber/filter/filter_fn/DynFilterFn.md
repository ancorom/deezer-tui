**tracing_subscriber > filter > filter_fn > DynFilterFn**

# Module: filter::filter_fn::DynFilterFn

## Contents

**Functions**

- [`new`](#new) - Constructs a [`Filter`] from a function or closure that returns `true`

---

## tracing_subscriber::filter::filter_fn::DynFilterFn::new

*Function*

Constructs a [`Filter`] from a function or closure that returns `true`
if a span or event should be enabled in the current [span
context][`Context`].

Unlike [`FilterFn`], a `DynFilterFn` is constructed from a closure or
function pointer that takes both the [`Metadata`] for a span or event
*and* the current [`Context`]. This means that a [`DynFilterFn`] can
choose whether to enable spans or events based on information about the
_current_ span (or its parents).

If this is *not* necessary, use [`FilterFn`] instead.

See the [documentation on per-layer filtering][plf] for details on using
[`Filter`]s.

[`Filter`]: crate::layer::Filter
[plf]: crate::layer#per-layer-filtering
[`Context`]: crate::layer::Context
[`Metadata`]: tracing_core::Metadata

# Examples

```
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter::DynFilterFn,
    util::SubscriberInitExt,
};

// Only enable spans or events within a span named "interesting_span".
let my_filter = DynFilterFn::new(|metadata, cx| {
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

```rust
fn new(enabled: F) -> Self
```



