**tracing_subscriber > filter > layer_filters > combinator**

# Module: filter::layer_filters::combinator

## Contents

**Structs**

- [`And`](#and) - Combines two [`Filter`]s so that spans and events are enabled if and only if
- [`Not`](#not) - Inverts the result of a [`Filter`].
- [`Or`](#or) - Combines two [`Filter`]s so that spans and events are enabled if *either* filter

---

## tracing_subscriber::filter::layer_filters::combinator::And

*Struct*

Combines two [`Filter`]s so that spans and events are enabled if and only if
*both* filters return `true`.

This type is typically returned by the [`FilterExt::and`] method. See that
method's documentation for details.

[`Filter`]: crate::layer::Filter
[`FilterExt::and`]: crate::filter::FilterExt::and

**Generic Parameters:**
- A
- B
- S

**Trait Implementations:**

- **Filter**
  - `fn enabled(self: &Self, meta: &Metadata, cx: &Context<S>) -> bool`
  - `fn callsite_enabled(self: &Self, meta: &'static Metadata<'static>) -> Interest`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`
  - `fn event_enabled(self: &Self, event: &tracing_core::Event, cx: &Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &Attributes, id: &Id, ctx: Context<S>)`
  - `fn on_record(self: &Self, id: &Id, values: &Record, ctx: Context<S>)`
  - `fn on_enter(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_exit(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_close(self: &Self, id: Id, ctx: Context<S>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::filter::layer_filters::combinator::Not

*Struct*

Inverts the result of a [`Filter`].

If the wrapped filter would enable a span or event, it will be disabled. If
it would disable a span or event, that span or event will be enabled.

This type is typically returned by the [`FilterExt::not`] method. See that
method's documentation for details.

[`Filter`]: crate::layer::Filter
[`FilterExt::not`]: crate::filter::FilterExt::not

**Generic Parameters:**
- A
- S

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Filter**
  - `fn enabled(self: &Self, meta: &Metadata, cx: &Context<S>) -> bool`
  - `fn callsite_enabled(self: &Self, meta: &'static Metadata<'static>) -> Interest`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`
  - `fn event_enabled(self: &Self, event: &tracing_core::Event, cx: &Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &Attributes, id: &Id, ctx: Context<S>)`
  - `fn on_record(self: &Self, id: &Id, values: &Record, ctx: Context<S>)`
  - `fn on_enter(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_exit(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_close(self: &Self, id: Id, ctx: Context<S>)`



## tracing_subscriber::filter::layer_filters::combinator::Or

*Struct*

Combines two [`Filter`]s so that spans and events are enabled if *either* filter
returns `true`.

This type is typically returned by the [`FilterExt::or`] method. See that
method's documentation for details.

[`Filter`]: crate::layer::Filter
[`FilterExt::or`]: crate::filter::FilterExt::or

**Generic Parameters:**
- A
- B
- S

**Trait Implementations:**

- **Filter**
  - `fn enabled(self: &Self, meta: &Metadata, cx: &Context<S>) -> bool`
  - `fn callsite_enabled(self: &Self, meta: &'static Metadata<'static>) -> Interest`
  - `fn max_level_hint(self: &Self) -> Option<LevelFilter>`
  - `fn event_enabled(self: &Self, event: &tracing_core::Event, cx: &Context<S>) -> bool`
  - `fn on_new_span(self: &Self, attrs: &Attributes, id: &Id, ctx: Context<S>)`
  - `fn on_record(self: &Self, id: &Id, values: &Record, ctx: Context<S>)`
  - `fn on_enter(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_exit(self: &Self, id: &Id, ctx: Context<S>)`
  - `fn on_close(self: &Self, id: Id, ctx: Context<S>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



