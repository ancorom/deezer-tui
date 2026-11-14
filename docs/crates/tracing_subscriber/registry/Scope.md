**tracing_subscriber > registry > Scope**

# Module: registry::Scope

## Contents

**Functions**

- [`from_root`](#from_root) - Flips the order of the iterator, so that it is ordered from root to leaf.

---

## tracing_subscriber::registry::Scope::from_root

*Function*

Flips the order of the iterator, so that it is ordered from root to leaf.

The iterator will first return the root span, then that span's immediate child,
and so on until it finally returns the span that [`SpanRef::scope`] was called on.

If any items were consumed from the [`Scope`] before calling this method then they
will *not* be returned from the [`ScopeFromRoot`].

**Note**: this will allocate if there are many spans remaining, or if the
"smallvec" feature flag is not enabled.

```rust
fn from_root(self: Self) -> ScopeFromRoot<'a, R>
```



