**tracing_subscriber > fmt > format**

# Module: fmt::format

## Contents

**Structs**

- [`Compact`](#compact) - Marker for [`Format`] that indicates that the compact log format should be used.
- [`DefaultFields`](#defaultfields) - The default [`FormatFields`] implementation.
- [`DefaultVisitor`](#defaultvisitor) - The [visitor] produced by [`DefaultFields`]'s [`MakeVisitor`] implementation.
- [`FieldFn`](#fieldfn) - A [`FormatFields`] implementation that formats fields by calling a function
- [`FieldFnVisitor`](#fieldfnvisitor) - The [visitor] produced by [`FieldFn`]'s [`MakeVisitor`] implementation.
- [`FmtSpan`](#fmtspan) - Configures what points in the span lifecycle are logged as events.
- [`Format`](#format) - A pre-configured event formatter.
- [`Full`](#full) - Marker for [`Format`] that indicates that the default log format should be used.
- [`Writer`](#writer) - A writer to which formatted representations of spans and events are written.

**Functions**

- [`debug_fn`](#debug_fn) - Returns a [`FormatFields`] implementation that formats fields using the
- [`format`](#format) - Returns the default configuration for an event formatter.

**Traits**

- [`FormatEvent`](#formatevent) - A type that can format a tracing [`Event`] to a [`Writer`].
- [`FormatFields`](#formatfields) - A type that can format a [set of fields] to a [`Writer`].

---

## tracing_subscriber::fmt::format::Compact

*Struct*

Marker for [`Format`] that indicates that the compact log format should be used.

The compact format includes fields from all currently entered spans, after
the event's fields. Span fields are ordered (but not grouped) by
span, and span names are not shown. A more compact representation of the
event's [`Level`] is used, and additional information—such as the event's
target—is disabled by default.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-compact
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-compact`
<font color="#AAAAAA">2022-02-17T19:51:05.809287Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: preparing to shave yaks </font><i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809367Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: shaving yaks </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809414Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809443Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809477Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809500Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=1 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809531Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809554Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809581Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809606Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809635Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809664Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: could not locate yak </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809693Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809717Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash] </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809743Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809768Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: yak shaving completed </font><i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>

</pre>

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## tracing_subscriber::fmt::format::DefaultFields

*Struct*

The default [`FormatFields`] implementation.


**Methods:**

- `fn new() -> Self` - Returns a new default [`FormatFields`] implementation.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **MakeVisitor**
  - `fn make_visitor(self: &Self, target: Writer<'a>) -> <Self as >::Visitor`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_subscriber::fmt::format::DefaultVisitor

*Struct*

The [visitor] produced by [`DefaultFields`]'s [`MakeVisitor`] implementation.

[visitor]: super::super::field::Visit
[`MakeVisitor`]: super::super::field::MakeVisitor

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(writer: Writer<'a>, is_empty: bool) -> Self` - Returns a new default visitor that formats to the provided `writer`.

**Trait Implementations:**

- **Visit**
  - `fn record_str(self: & mut Self, field: &Field, value: &str)`
  - `fn record_error(self: & mut Self, field: &Field, value: &dyn std::error::Error)`
  - `fn record_debug(self: & mut Self, field: &Field, value: &dyn fmt::Debug)`
- **VisitFmt**
  - `fn writer(self: & mut Self) -> & mut dyn fmt::Write`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **VisitOutput**
  - `fn finish(self: Self) -> fmt::Result`



## tracing_subscriber::fmt::format::FieldFn

*Struct*

A [`FormatFields`] implementation that formats fields by calling a function
or closure.


**Generic Parameters:**
- F

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **MakeVisitor**
  - `fn make_visitor(self: &Self, writer: Writer<'a>) -> <Self as >::Visitor`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::fmt::format::FieldFnVisitor

*Struct*

The [visitor] produced by [`FieldFn`]'s [`MakeVisitor`] implementation.

[visitor]: super::super::field::Visit
[`MakeVisitor`]: super::super::field::MakeVisitor

**Generic Parameters:**
- 'a
- F

**Trait Implementations:**

- **VisitFmt**
  - `fn writer(self: & mut Self) -> & mut dyn fmt::Write`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **VisitOutput**
  - `fn finish(self: Self) -> fmt::Result`
- **Visit**
  - `fn record_debug(self: & mut Self, field: &Field, value: &dyn fmt::Debug)`



## tracing_subscriber::fmt::format::FmtSpan

*Struct*

Configures what points in the span lifecycle are logged as events.

See also [`with_span_events`].

[`with_span_events`]: super::SubscriberBuilder::with_span_events

**Tuple Struct**: `()`

**Methods:**


**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **BitOr**
  - `fn bitor(self: Self, rhs: Self) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: Self)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, rhs: Self)`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, rhs: Self)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitXor**
  - `fn bitxor(self: Self, rhs: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, rhs: Self) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`



## tracing_subscriber::fmt::format::Format

*Struct*

A pre-configured event formatter.

You will usually want to use this as the [`FormatEvent`] for a [`FmtSubscriber`].

The default logging format, [`Full`] includes all fields in each event and its containing
spans. The [`Compact`] logging format is intended to produce shorter log
lines; it displays each event's fields, along with fields from the current
span context, but other information is abbreviated. The [`Pretty`] logging
format is an extra-verbose, multi-line human-readable logging format
intended for use in development.

[`FmtSubscriber`]: super::Subscriber

**Generic Parameters:**
- F
- T

**Methods:**

- `fn compact(self: Self) -> Format<Compact, T>` - Use a less verbose output format.
- `fn pretty(self: Self) -> Format<Pretty, T>` - Use an excessively pretty, human-readable output format.
- `fn with_timer<T2>(self: Self, timer: T2) -> Format<F, T2>` - Use the given [`timer`] for log message timestamps.
- `fn without_time(self: Self) -> Format<F, ()>` - Do not emit timestamps with log messages.
- `fn with_ansi(self: Self, ansi: bool) -> Format<F, T>` - Enable ANSI terminal colors for formatted output.
- `fn with_target(self: Self, display_target: bool) -> Format<F, T>` - Sets whether or not an event's target is displayed.
- `fn with_level(self: Self, display_level: bool) -> Format<F, T>` - Sets whether or not an event's level is displayed.
- `fn with_thread_ids(self: Self, display_thread_id: bool) -> Format<F, T>` - Sets whether or not the [thread ID] of the current thread is displayed
- `fn with_thread_names(self: Self, display_thread_name: bool) -> Format<F, T>` - Sets whether or not the [name] of the current thread is displayed
- `fn with_file(self: Self, display_filename: bool) -> Format<F, T>` - Sets whether or not an event's [source code file path][file] is
- `fn with_line_number(self: Self, display_line_number: bool) -> Format<F, T>` - Sets whether or not an event's [source code line number][line] is
- `fn with_source_location(self: Self, display_location: bool) -> Self` - Sets whether or not the source code location from which an event

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FormatEvent**
  - `fn format_event(self: &Self, ctx: &FmtContext<S, N>, writer: Writer, event: &Event) -> fmt::Result`
- **Default**
  - `fn default() -> Self`
- **FormatEvent**
  - `fn format_event(self: &Self, ctx: &FmtContext<C, N>, writer: Writer, event: &Event) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FormatEvent**
  - `fn format_event(self: &Self, ctx: &FmtContext<S, N>, writer: Writer, event: &Event) -> fmt::Result`



## tracing_subscriber::fmt::format::FormatEvent

*Trait*

A type that can format a tracing [`Event`] to a [`Writer`].

[`FormatEvent`] is primarily used in the context of [`fmt::Subscriber`] or
[`fmt::Layer`]. Each time an event is dispatched to [`fmt::Subscriber`] or
[`fmt::Layer`], the subscriber or layer
forwards it to its associated [`FormatEvent`] to emit a log message.

This trait is already implemented for function pointers with the same
signature as `format_event`.

# Arguments

The following arguments are passed to [`FormatEvent::format_event`]:

* A [`FmtContext`]. This is an extension of the [`layer::Context`] type,
  which can be used for accessing stored information such as the current
  span context an event occurred in.

  In addition, [`FmtContext`] exposes access to the [`FormatFields`]
  implementation that the subscriber was configured to use via the
  [`FmtContext::field_format`] method. This can be used when the
  [`FormatEvent`] implementation needs to format the event's fields.

  For convenience, [`FmtContext`] also implements [`FormatFields`],
  forwarding to the configured [`FormatFields`] type.

* A [`Writer`] to which the formatted representation of the event is
  written. This type implements the [`std::fmt::Write`] trait, and therefore
  can be used with the [`std::write!`] and [`std::writeln!`] macros, as well
  as calling [`std::fmt::Write`] methods directly.

  The [`Writer`] type also implements additional methods that provide
  information about how the event should be formatted. The
  [`Writer::has_ansi_escapes`] method indicates whether [ANSI terminal
  escape codes] are supported by the underlying I/O writer that the event
  will be written to. If this returns `true`, the formatter is permitted to
  use ANSI escape codes to add colors and other text formatting to its
  output. If it returns `false`, the event will be written to an output that
  does not support ANSI escape codes (such as a log file), and they should
  not be emitted.

  Crates like [`nu_ansi_term`] and [`owo-colors`] can be used to add ANSI
  escape codes to formatted output.

* The actual [`Event`] to be formatted.

# Examples

This example re-implements a simplified version of this crate's [default
formatter]:

```rust
use std::fmt;
use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

let _subscriber = tracing_subscriber::fmt()
    .event_format(MyFormatter)
    .init();

let _span = tracing::info_span!("my_span", answer = 42).entered();
tracing::info!(question = "life, the universe, and everything", "hello world");
```

This formatter will print events like this:

```text
DEBUG yak_shaving::shaver: some-span{field-on-span=foo}: started shaving yak
```

[`layer::Context`]: crate::layer::Context
[`fmt::Layer`]: super::Layer
[`fmt::Subscriber`]: super::Subscriber
[`Event`]: tracing::Event
[implements `FormatFields`]: super::FmtContext#impl-FormatFields<'writer>
[ANSI terminal escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
[`Writer::has_ansi_escapes`]: Writer::has_ansi_escapes
[`nu_ansi_term`]: https://crates.io/crates/nu_ansi_term
[`owo-colors`]: https://crates.io/crates/owo-colors
[default formatter]: Full

**Methods:**

- `format_event`: Write a log message for [`Event`] in [`FmtContext`] to the given [`Writer`].



## tracing_subscriber::fmt::format::FormatFields

*Trait*

A type that can format a [set of fields] to a [`Writer`].

[`FormatFields`] is primarily used in the context of [`FmtSubscriber`]. Each
time a span or event with fields is recorded, the subscriber will format
those fields with its associated [`FormatFields`] implementation.

[set of fields]: crate::field::RecordFields
[`FmtSubscriber`]: super::Subscriber

**Methods:**

- `format_fields`: Format the provided `fields` to the provided [`Writer`], returning a result.
- `add_fields`: Record additional field(s) on an existing span.



## tracing_subscriber::fmt::format::Full

*Struct*

Marker for [`Format`] that indicates that the default log format should be used.

This formatter shows the span context before printing event data. Spans are
displayed including their names and fields.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt`
<font color="#AAAAAA">2022-02-15T18:40:14.289898Z </font><font color="#4E9A06"> INFO</font> fmt: preparing to shave yaks <i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-15T18:40:14.289974Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: shaving yaks</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290011Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290038Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290070Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290089Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290114Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290134Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290157Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290174Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290198Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290222Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: could not locate yak</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290247Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290268Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash]</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290287Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290309Z </font><font color="#4E9A06"> INFO</font> fmt: yak shaving completed. <i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>
</pre>

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## tracing_subscriber::fmt::format::Writer

*Struct*

A writer to which formatted representations of spans and events are written.

This type is provided as input to the [`FormatEvent::format_event`] and
[`FormatFields::format_fields`] methods, which will write formatted
representations of [`Event`]s and [fields] to the [`Writer`].

This type implements the [`std::fmt::Write`] trait, allowing it to be used
with any function that takes an instance of [`std::fmt::Write`].
Additionally, it can be used with the standard library's [`std::write!`] and
[`std::writeln!`] macros.

Additionally, a [`Writer`] may expose additional [`tracing`]-specific
information to the formatter implementation.

[fields]: tracing_core::field

**Generic Parameters:**
- 'writer

**Methods:**

- `fn new<impl fmt::Write>(writer: &'writer  mut impl Trait) -> Self` - Create a new [`Writer`] from any type that implements [`fmt::Write`].
- `fn by_ref(self: & mut Self) -> Writer` - Return a new [`Writer`] that mutably borrows [`self`].
- `fn write_str(self: & mut Self, s: &str) -> fmt::Result` - Writes a string slice into this [`Writer`], returning whether the write succeeded.
- `fn write_char(self: & mut Self, c: char) -> fmt::Result` - Writes a [`char`] into this writer, returning whether the write succeeded.
- `fn write_fmt(self: & mut Self, args: fmt::Arguments) -> fmt::Result` - Glue for usage of the [`write!`] macro with [`Writer`]s.
- `fn has_ansi_escapes(self: &Self) -> bool` - Returns `true` if [ANSI escape codes] may be used to add colors
- `fn sanitizes_ansi_escapes(self: &Self) -> bool` - Returns `true` if ANSI escape codes should be sanitized.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> fmt::Result`
  - `fn write_char(self: & mut Self, c: char) -> fmt::Result`
  - `fn write_fmt(self: & mut Self, args: fmt::Arguments) -> fmt::Result`



## tracing_subscriber::fmt::format::debug_fn

*Function*

Returns a [`FormatFields`] implementation that formats fields using the
provided function or closure.


```rust
fn debug_fn<F>(f: F) -> FieldFn<F>
```



## tracing_subscriber::fmt::format::format

*Function*

Returns the default configuration for an event formatter.

Methods on the returned event formatter can be used for further
configuration. For example:

```rust
let format = tracing_subscriber::fmt::format()
    .without_time()         // Don't include timestamps
    .with_target(false)     // Don't include event targets.
    .with_level(false)      // Don't include event levels.
    .compact();             // Use a more compact, abbreviated format.

// Use the configured formatter when building a new subscriber.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

```rust
fn format() -> Format
```



