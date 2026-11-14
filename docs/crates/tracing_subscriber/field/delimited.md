**tracing_subscriber > field > delimited**

# Module: field::delimited

## Contents

**Structs**

- [`Delimited`](#delimited) - A `MakeVisitor` wrapper that wraps a visitor that writes formatted output so
- [`VisitDelimited`](#visitdelimited) - A visitor wrapper that inserts a delimiter after the wrapped visitor formats

---

## tracing_subscriber::field::delimited::Delimited

*Struct*

A `MakeVisitor` wrapper that wraps a visitor that writes formatted output so
that a delimiter is inserted between writing formatted field values.

**Generic Parameters:**
- D
- V

**Methods:**

- `fn new(delimiter: D, inner: V) -> Self` - Returns a new [`MakeVisitor`] implementation that wraps `inner` so that

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **MakeVisitor**
  - `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor`



## tracing_subscriber::field::delimited::VisitDelimited

*Struct*

A visitor wrapper that inserts a delimiter after the wrapped visitor formats
a field value.

**Generic Parameters:**
- D
- V

**Methods:**

- `fn new(delimiter: D, inner: V) -> Self` - Returns a new [`Visit`] implementation that wraps `inner` so that

**Trait Implementations:**

- **VisitOutput**
  - `fn finish(self: Self) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Visit**
  - `fn record_i64(self: & mut Self, field: &Field, value: i64)`
  - `fn record_u64(self: & mut Self, field: &Field, value: u64)`
  - `fn record_bool(self: & mut Self, field: &Field, value: bool)`
  - `fn record_str(self: & mut Self, field: &Field, value: &str)`
  - `fn record_debug(self: & mut Self, field: &Field, value: &dyn fmt::Debug)`
- **VisitFmt**
  - `fn writer(self: & mut Self) -> & mut dyn fmt::Write`



