**tracing_subscriber > registry > SpanRef**

# Module: registry::SpanRef

## Contents

**Functions**

- [`scope`](#scope) - Returns an iterator over all parents of this span, starting with this span,

---

## tracing_subscriber::registry::SpanRef::scope

*Function*

Returns an iterator over all parents of this span, starting with this span,
ordered from leaf to root.

The iterator will first return the span, then the span's immediate parent,
followed by that span's parent, and so on, until it reaches a root span.

```rust
use tracing::{span, Subscriber};
use tracing_subscriber::{
    layer::{Context, Layer},
    prelude::*,
    registry::LookupSpan,
};

struct PrintingLayer;
impl<S> Layer<S> for PrintingLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_enter(&self, id: &span::Id, ctx: Context<S>) {
        let span = ctx.span(id).unwrap();
        let scope = span.scope().map(|span| span.name()).collect::<Vec<_>>();
        println!("Entering span: {:?}", scope);
    }
}

tracing::subscriber::with_default(tracing_subscriber::registry().with(PrintingLayer), || {
    let _root = tracing::info_span!("root").entered();
    // Prints: Entering span: ["root"]
    let _child = tracing::info_span!("child").entered();
    // Prints: Entering span: ["child", "root"]
    let _leaf = tracing::info_span!("leaf").entered();
    // Prints: Entering span: ["leaf", "child", "root"]
});
```

If the opposite order (from the root to this span) is desired, calling [`Scope::from_root`] on
the returned iterator reverses the order.

```rust
# use tracing::{span, Subscriber};
# use tracing_subscriber::{
#     layer::{Context, Layer},
#     prelude::*,
#     registry::LookupSpan,
# };
# struct PrintingLayer;
impl<S> Layer<S> for PrintingLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_enter(&self, id: &span::Id, ctx: Context<S>) {
        let span = ctx.span(id).unwrap();
        let scope = span.scope().from_root().map(|span| span.name()).collect::<Vec<_>>();
        println!("Entering span: {:?}", scope);
    }
}

tracing::subscriber::with_default(tracing_subscriber::registry().with(PrintingLayer), || {
    let _root = tracing::info_span!("root").entered();
    // Prints: Entering span: ["root"]
    let _child = tracing::info_span!("child").entered();
    // Prints: Entering span: ["root", "child"]
    let _leaf = tracing::info_span!("leaf").entered();
    // Prints: Entering span: ["root", "child", "leaf"]
});
```

```rust
fn scope(self: &Self) -> Scope<'a, R>
```



