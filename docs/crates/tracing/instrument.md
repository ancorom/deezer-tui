**tracing > instrument**

# Module: instrument

## Contents

**Structs**

- [`Instrumented`](#instrumented) - A [`Future`] that has been instrumented with a `tracing` [`Span`].
- [`WithDispatch`](#withdispatch) - A [`Future`] that has been instrumented with a `tracing` [`Subscriber`].

**Traits**

- [`Instrument`](#instrument) - Attaches spans to a [`std::future::Future`].
- [`WithSubscriber`](#withsubscriber) - Extension trait allowing futures to be instrumented with

---

## tracing::instrument::Instrument

*Trait*

Attaches spans to a [`std::future::Future`].

Extension trait allowing futures to be
instrumented with a `tracing` [span].

[span]: super::Span

**Methods:**

- `instrument`: Instruments this type with the provided [`Span`], returning an
- `in_current_span`: Instruments this type with the [current] [`Span`], returning an



## tracing::instrument::Instrumented

*Struct*

A [`Future`] that has been instrumented with a `tracing` [`Span`].

This type is returned by the [`Instrument`] extension trait. See that
trait's documentation for details.

[`Future`]: std::future::Future
[`Span`]: crate::Span

**Generic Parameters:**
- T

**Methods:**

- `fn span(self: &Self) -> &Span` - Borrows the `Span` that this type is instrumented by.
- `fn span_mut(self: & mut Self) -> & mut Span` - Mutably borrows the `Span` that this type is instrumented by.
- `fn inner(self: &Self) -> &T` - Borrows the wrapped type.
- `fn inner_mut(self: & mut Self) -> & mut T` - Mutably borrows the wrapped type.
- `fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>` - Get a pinned reference to the wrapped type.
- `fn inner_pin_mut(self: Pin<& mut Self>) -> Pin<& mut T>` - Get a pinned mutable reference to the wrapped type.
- `fn into_inner(self: Self) -> T` - Consumes the `Instrumented`, returning the wrapped type.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing::instrument::WithDispatch

*Struct*

A [`Future`] that has been instrumented with a `tracing` [`Subscriber`].

This type is returned by the [`WithSubscriber`] extension trait. See that
trait's documentation for details.

[`Future`]: std::future::Future
[`Subscriber`]: crate::Subscriber

**Generic Parameters:**
- T

**Methods:**

- `fn dispatcher(self: &Self) -> &Dispatch` - Borrows the [`Dispatch`] that is entered when this type is polled.
- `fn inner(self: &Self) -> &T` - Borrows the wrapped type.
- `fn inner_mut(self: & mut Self) -> & mut T` - Mutably borrows the wrapped type.
- `fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>` - Get a pinned reference to the wrapped type.
- `fn inner_pin_mut(self: Pin<& mut Self>) -> Pin<& mut T>` - Get a pinned mutable reference to the wrapped type.
- `fn into_inner(self: Self) -> T` - Consumes the `Instrumented`, returning the wrapped type.

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing::instrument::WithSubscriber

*Trait*

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

**Methods:**

- `with_subscriber`: Attaches the provided [`Subscriber`] to this type, returning a
- `with_current_subscriber`: Attaches the current [default] [`Subscriber`] to this type, returning a



