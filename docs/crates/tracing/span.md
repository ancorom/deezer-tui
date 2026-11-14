**tracing > span**

# Module: span

## Contents

**Structs**

- [`Entered`](#entered) - A guard representing a span which has been entered and is currently
- [`EnteredSpan`](#enteredspan) - An owned version of [`Entered`], a guard representing a span which has been
- [`Span`](#span) - A handle representing a span, with the capability to enter the span if it

**Traits**

- [`AsId`](#asid) - Trait implemented by types which have a span `Id`.

---

## tracing::span::AsId

*Trait*

Trait implemented by types which have a span `Id`.

**Methods:**

- `as_id`: Returns the `Id` of the span that `self` corresponds to, or `None` if



## tracing::span::Entered

*Struct*

A guard representing a span which has been entered and is currently
executing.

When the guard is dropped, the span will be exited.

This is returned by the [`Span::enter`] function.

[`Span::enter`]: super::Span::enter

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing::span::EnteredSpan

*Struct*

An owned version of [`Entered`], a guard representing a span which has been
entered and is currently executing.

When the guard is dropped, the span will be exited.

This is returned by the [`Span::entered`] function.

[`Span::entered`]: super::Span::entered()

**Methods:**

- `fn id(self: &Self) -> Option<Id>` - Returns this span's `Id`, if it is enabled.
- `fn exit(self: Self) -> Span` - Exits this span, returning the underlying [`Span`].

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &Span`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## tracing::span::Span

*Struct*

A handle representing a span, with the capability to enter the span if it
exists.

If the span was rejected by the current `Subscriber`'s filter, entering the
span will silently do nothing. Thus, the handle can be used in the same
manner regardless of whether or not the trace is currently being collected.

**Methods:**

- `fn new(meta: &'static Metadata<'static>, values: &field::ValueSet) -> Span` - Constructs a new `Span` with the given [metadata] and set of
- `fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet) -> Span` - Constructs a new `Span` as the root of its own trace tree, with the
- `fn child_of<impl Into<Option<Id>>>(parent: impl Trait, meta: &'static Metadata<'static>, values: &field::ValueSet) -> Span` - Constructs a new `Span` as child of the given parent span, with the
- `fn new_disabled(meta: &'static Metadata<'static>) -> Span` - Constructs a new disabled span with the given `Metadata`.
- `fn none() -> Span` - Constructs a new span that is *completely disabled*.
- `fn current() -> Span` - Returns a handle to the span [considered by the `Subscriber`] to be the
- `fn enter(self: &Self) -> Entered` - Enters this span, returning a guard that will exit the span when dropped.
- `fn entered(self: Self) -> EnteredSpan` - Enters this span, consuming it and returning a [guard][`EnteredSpan`]
- `fn or_current(self: Self) -> Self` - Returns this span, if it was [enabled] by the current [`Subscriber`], or
- `fn in_scope<F, T>(self: &Self, f: F) -> T` - Executes the given function in the context of this span.
- `fn field<Q>(self: &Self, field: &Q) -> Option<field::Field>` - Returns a [`Field`][super::field::Field] for the field with the
- `fn has_field<Q>(self: &Self, field: &Q) -> bool` - Returns true if this `Span` has a field for the given
- `fn record<Q, V>(self: &Self, field: &Q, value: V) -> &Self` - Records that the field described by `field` has the value `value`.
- `fn is_disabled(self: &Self) -> bool` - Returns `true` if this span was disabled by the subscriber and does not
- `fn is_none(self: &Self) -> bool` - Returns `true` if this span was constructed by [`Span::none`] and is
- `fn follows_from<impl Into<Option<Id>>>(self: &Self, from: impl Trait) -> &Self` - Indicates that the span with the given ID has an indirect causal
- `fn id(self: &Self) -> Option<Id>` - Returns this span's `Id`, if it is enabled.
- `fn metadata(self: &Self) -> Option<&'static Metadata<'static>>` - Returns this span's `Metadata`, if it is enabled.
- `fn with_subscriber<T, impl FnOnce((&Id, &Dispatch)) -> T>(self: &Self, f: impl Trait) -> Option<T>` - Invokes a function with a reference to this span's ID and subscriber.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, hasher: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



