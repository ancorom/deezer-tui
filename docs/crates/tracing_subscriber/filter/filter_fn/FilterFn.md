**tracing_subscriber > filter > filter_fn > FilterFn**

# Module: filter::filter_fn::FilterFn

## Contents

**Functions**

- [`new`](#new) - Constructs a [`FilterFn`] from a function or closure that returns `true`

---

## tracing_subscriber::filter::filter_fn::FilterFn::new

*Function*

Constructs a [`FilterFn`] from a function or closure that returns `true`
if a span or event should be enabled, based on its [`Metadata`].

If determining whether a span or event should be enabled also requires
information about the current span context, use [`DynFilterFn`] instead.

See the [documentation on per-layer filtering][plf] for details on using
[`Filter`]s.

[`Filter`]: crate::layer::Filter
[plf]: crate::layer#per-layer-filtering
[`Metadata`]: tracing_core::Metadata

# Examples

```
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter::FilterFn,
    util::SubscriberInitExt,
};

let my_filter = FilterFn::new(|metadata| {
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
fn new(enabled: F) -> Self
```



