**tracing_subscriber > registry**

# Module: registry

## Contents

**Structs**

- [`Scope`](#scope) - An iterator over the parents of a span, ordered from leaf to root.
- [`ScopeFromRoot`](#scopefromroot) - An iterator over the parents of a span, ordered from root to leaf.
- [`SpanRef`](#spanref) - A reference to [span data] and the associated [registry].

**Traits**

- [`LookupSpan`](#lookupspan) - Provides access to stored span data.
- [`SpanData`](#spandata) - A stored representation of data associated with a span.

---

## tracing_subscriber::registry::LookupSpan

*Trait*

Provides access to stored span data.

Subscribers which store span data and associate it with span IDs should
implement this trait; if they do, any [`Layer`]s wrapping them can look up
metadata via the [`Context`] type's [`span()`] method.

[`Layer`]: super::layer::Layer
[`Context`]: super::layer::Context
[`span()`]: super::layer::Context::span

**Methods:**

- `Data`: The type of span data stored in this registry.
- `span_data`: Returns the [`SpanData`] for a given `Id`, if it exists.
- `span`: Returns a [`SpanRef`] for the span with the given `Id`, if it exists.
- `register_filter`: Registers a [`Filter`] for [per-layer filtering] with this



## tracing_subscriber::registry::Scope

*Struct*

An iterator over the parents of a span, ordered from leaf to root.

This is returned by the [`SpanRef::scope`] method.

**Generic Parameters:**
- 'a
- R

**Methods:**

- `fn from_root(self: Self) -> ScopeFromRoot<'a, R>` - Flips the order of the iterator, so that it is ordered from root to leaf.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::registry::ScopeFromRoot

*Struct*

An iterator over the parents of a span, ordered from root to leaf.

This is returned by the [`Scope::from_root`] method.

**Generic Parameters:**
- 'a
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## tracing_subscriber::registry::SpanData

*Trait*

A stored representation of data associated with a span.

**Methods:**

- `id`: Returns this span's ID.
- `metadata`: Returns a reference to the span's `Metadata`.
- `parent`: Returns a reference to the ID
- `extensions`: Returns a reference to this span's `Extensions`.
- `extensions_mut`: Returns a mutable reference to this span's `Extensions`.
- `is_enabled_for`: Returns `true` if this span is enabled for the [per-layer filter][plf]



## tracing_subscriber::registry::SpanRef

*Struct*

A reference to [span data] and the associated [registry].

This type implements all the same methods as [`SpanData`], and provides
additional methods for querying the registry based on values from the span.

[registry]: LookupSpan

**Generic Parameters:**
- 'a
- R

**Methods:**

- `fn id(self: &Self) -> Id` - Returns this span's ID.
- `fn metadata(self: &Self) -> &'static Metadata<'static>` - Returns a static reference to the span's metadata.
- `fn name(self: &Self) -> &'static str` - Returns the span's name,
- `fn fields(self: &Self) -> &FieldSet` - Returns a list of [fields] defined by the span.
- `fn parent(self: &Self) -> Option<Self>` - Returns a `SpanRef` describing this span's parent, or `None` if this
- `fn scope(self: &Self) -> Scope<'a, R>` - Returns an iterator over all parents of this span, starting with this span,
- `fn extensions(self: &Self) -> Extensions` - Returns a reference to this span's `Extensions`.
- `fn extensions_mut(self: &Self) -> ExtensionsMut` - Returns a mutable reference to this span's `Extensions`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



