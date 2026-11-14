**tracing_subscriber > field**

# Module: field

## Contents

**Modules**

- [`debug`](#debug) - `MakeVisitor` wrappers for working with `fmt::Debug` fields.
- [`delimited`](#delimited) - A `MakeVisitor` wrapper that separates formatted fields with a delimiter.
- [`display`](#display) - `MakeVisitor` wrappers for working with `fmt::Display` fields.

**Traits**

- [`MakeExt`](#makeext) - Extension trait providing `MakeVisitor` combinators.
- [`MakeOutput`](#makeoutput) - Extension trait implemented for all `MakeVisitor` implementations that
- [`MakeVisitor`](#makevisitor) - Creates new [visitors].
- [`RecordFields`](#recordfields) - Extension trait implemented by types which can be recorded by a [visitor].
- [`VisitFmt`](#visitfmt) - Extension trait implemented by visitors to indicate that they write to a
- [`VisitOutput`](#visitoutput) - A [visitor] that produces output once it has visited a set of fields.
- [`VisitWrite`](#visitwrite) - Extension trait implemented by visitors to indicate that they write to an

---

## tracing_subscriber::field::MakeExt

*Trait*

Extension trait providing `MakeVisitor` combinators.

**Methods:**

- `debug_alt`: Wraps `self` so that any `fmt::Debug` fields are recorded using the
- `display_messages`: Wraps `self` so that any string fields named "message" are recorded
- `delimited`: Wraps `self` so that when fields are formatted to a writer, they are



## tracing_subscriber::field::MakeOutput

*Trait*

Extension trait implemented for all `MakeVisitor` implementations that
produce a visitor implementing `VisitOutput`.

**Methods:**

- `visit_with`: Visits all fields in `fields` with a new visitor constructed from



## tracing_subscriber::field::MakeVisitor

*Trait*

Creates new [visitors].

A type implementing `MakeVisitor` represents a composable factory for types
implementing the [`Visit` trait][visitors]. The `MakeVisitor` trait defines
a single function, `make_visitor`, which takes in a `T`-typed `target` and
returns a type implementing `Visit` configured for that target. A target may
be a string, output stream, or data structure that the visitor will record
data to, configuration variables that determine the visitor's behavior, or
`()` when no input is required to produce a visitor.

[visitors]: tracing_core::field::Visit

**Methods:**

- `Visitor`: The visitor type produced by this `MakeVisitor`.
- `make_visitor`: Make a new visitor for the provided `target`.



## tracing_subscriber::field::RecordFields

*Trait*

Extension trait implemented by types which can be recorded by a [visitor].

This allows writing code that is generic over `tracing_core`'s
[`span::Attributes`][attr], [`span::Record`][rec], and [`Event`]
types. These types all provide inherent `record` methods that allow a
visitor to record their fields, but there is no common trait representing this.

With `RecordFields`, we can write code like this:
```
use tracing_core::field::Visit;
# use tracing_core::field::Field;
use tracing_subscriber::field::RecordFields;

struct MyVisitor {
    // ...
}
# impl MyVisitor { fn new() -> Self { Self{} } }
impl Visit for MyVisitor {
    // ...
# fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}
}

fn record_with_my_visitor<R>(r: R)
where
    R: RecordFields,
{
    let mut visitor = MyVisitor::new();
    r.record(&mut visitor);
}
```
[visitor]: tracing_core::field::Visit
[attr]: tracing_core::span::Attributes
[rec]: tracing_core::span::Record

**Methods:**

- `record`: Record all the fields in `self` with the provided `visitor`.



## tracing_subscriber::field::VisitFmt

*Trait*

Extension trait implemented by visitors to indicate that they write to a
`fmt::Write` instance, and allow access to that writer.

**Methods:**

- `writer`: Returns the formatter that this visitor writes to.



## tracing_subscriber::field::VisitOutput

*Trait*

A [visitor] that produces output once it has visited a set of fields.

[visitor]: tracing_core::field::Visit

**Methods:**

- `finish`: Completes the visitor, returning any output.
- `visit`: Visit a set of fields, and return the output of finishing the visitor



## tracing_subscriber::field::VisitWrite

*Trait*

Extension trait implemented by visitors to indicate that they write to an
`io::Write` instance, and allow access to that writer.

**Methods:**

- `writer`: Returns the writer that this visitor writes to.



## Module: debug

`MakeVisitor` wrappers for working with `fmt::Debug` fields.



## Module: delimited

A `MakeVisitor` wrapper that separates formatted fields with a delimiter.



## Module: display

`MakeVisitor` wrappers for working with `fmt::Display` fields.



