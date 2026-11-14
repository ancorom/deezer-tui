**tracing_subscriber > fmt > format > pretty**

# Module: fmt::format::pretty

## Contents

**Structs**

- [`Pretty`](#pretty) - An excessively pretty, human-readable event formatter.
- [`PrettyFields`](#prettyfields) - An excessively pretty, human-readable [`MakeVisitor`] implementation.
- [`PrettyVisitor`](#prettyvisitor) - The [visitor] produced by [`Pretty`]'s [`MakeVisitor`] implementation.

---

## tracing_subscriber::fmt::format::pretty::Pretty

*Struct*

An excessively pretty, human-readable event formatter.

Unlike the [`Full`], [`Compact`], and [`Json`] formatters, this is a
multi-line output format. Each individual event may output multiple lines of
text.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-pretty
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-pretty`
  2022-02-15T18:44:24.535324Z <font color="#4E9A06"> INFO</font> <font color="#4E9A06"><b>fmt_pretty</b></font><font color="#4E9A06">: preparing to shave yaks, </font><font color="#4E9A06"><b>number_of_yaks</b></font><font color="#4E9A06">: 3</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt-pretty.rs:16 <font color="#AAAAAA"><i>on</i></font> main

  2022-02-15T18:44:24.535403Z <font color="#4E9A06"> INFO</font> <font color="#4E9A06"><b>fmt_pretty::yak_shave</b></font><font color="#4E9A06">: shaving yaks</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:41 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535442Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: hello! I&apos;m gonna shave a yak, </font><font color="#75507B"><b>excitement</b></font><font color="#75507B">: &quot;yay!&quot;</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:16 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 1
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535469Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: yak shaved successfully</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:25 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 1
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535502Z <font color="#3465A4">DEBUG</font> <font color="#3465A4"><b>yak_events</b></font><font color="#3465A4">: </font><font color="#3465A4"><b>yak</b></font><font color="#3465A4">: 1, </font><font color="#3465A4"><b>shaved</b></font><font color="#3465A4">: true</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:46 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535524Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: </font><font color="#75507B"><b>yaks_shaved</b></font><font color="#75507B">: 1</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:55 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535551Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: hello! I&apos;m gonna shave a yak, </font><font color="#75507B"><b>excitement</b></font><font color="#75507B">: &quot;yay!&quot;</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:16 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 2
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535573Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: yak shaved successfully</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:25 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 2
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535600Z <font color="#3465A4">DEBUG</font> <font color="#3465A4"><b>yak_events</b></font><font color="#3465A4">: </font><font color="#3465A4"><b>yak</b></font><font color="#3465A4">: 2, </font><font color="#3465A4"><b>shaved</b></font><font color="#3465A4">: true</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:46 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535618Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: </font><font color="#75507B"><b>yaks_shaved</b></font><font color="#75507B">: 2</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:55 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535644Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: hello! I&apos;m gonna shave a yak, </font><font color="#75507B"><b>excitement</b></font><font color="#75507B">: &quot;yay!&quot;</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:16 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 3
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535670Z <font color="#C4A000"> WARN</font> <font color="#C4A000"><b>fmt_pretty::yak_shave</b></font><font color="#C4A000">: could not locate yak</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:18 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 3
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535698Z <font color="#3465A4">DEBUG</font> <font color="#3465A4"><b>yak_events</b></font><font color="#3465A4">: </font><font color="#3465A4"><b>yak</b></font><font color="#3465A4">: 3, </font><font color="#3465A4"><b>shaved</b></font><font color="#3465A4">: false</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:46 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535720Z <font color="#CC0000">ERROR</font> <font color="#CC0000"><b>fmt_pretty::yak_shave</b></font><font color="#CC0000">: failed to shave yak, </font><font color="#CC0000"><b>yak</b></font><font color="#CC0000">: 3, </font><font color="#CC0000"><b>error</b></font><font color="#CC0000">: missing yak, </font><font color="#CC0000"><b>error.sources</b></font><font color="#CC0000">: [out of space, out of cash]</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:51 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535742Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: </font><font color="#75507B"><b>yaks_shaved</b></font><font color="#75507B">: 2</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:55 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535765Z <font color="#4E9A06"> INFO</font> <font color="#4E9A06"><b>fmt_pretty</b></font><font color="#4E9A06">: yak shaving completed, </font><font color="#4E9A06"><b>all_yaks_shaved</b></font><font color="#4E9A06">: false</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt-pretty.rs:19 <font color="#AAAAAA"><i>on</i></font> main
</pre>

**Methods:**

- `fn with_source_location(self: Self, display_location: bool) -> Self` - Sets whether the event's source code location is displayed.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **FormatFields**
  - `fn format_fields<R>(self: &Self, writer: Writer<'writer>, fields: R) -> fmt::Result`
  - `fn add_fields(self: &Self, current: &'writer  mut FormattedFields<Self>, fields: &span::Record) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::fmt::format::pretty::PrettyFields

*Struct*

An excessively pretty, human-readable [`MakeVisitor`] implementation.

[`MakeVisitor`]: crate::field::MakeVisitor

**Methods:**

- `fn new() -> Self` - Returns a new default [`PrettyFields`] implementation.
- `fn with_ansi(self: Self, ansi: bool) -> Self` - Enable ANSI encoding for formatted fields.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **MakeVisitor**
  - `fn make_visitor(self: &Self, target: Writer<'a>) -> <Self as >::Visitor`
- **Default**
  - `fn default() -> Self`



## tracing_subscriber::fmt::format::pretty::PrettyVisitor

*Struct*

The [visitor] produced by [`Pretty`]'s [`MakeVisitor`] implementation.

[visitor]: field::Visit
[`MakeVisitor`]: crate::field::MakeVisitor

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(writer: Writer<'a>, is_empty: bool) -> Self` - Returns a new default visitor that formats to the provided `writer`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **VisitFmt**
  - `fn writer(self: & mut Self) -> & mut dyn fmt::Write`
- **Visit**
  - `fn record_str(self: & mut Self, field: &Field, value: &str)`
  - `fn record_error(self: & mut Self, field: &Field, value: &dyn std::error::Error)`
  - `fn record_debug(self: & mut Self, field: &Field, value: &dyn fmt::Debug)`
- **VisitOutput**
  - `fn finish(self: Self) -> fmt::Result`



