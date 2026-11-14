**tracing_subscriber > field > display**

# Module: field::display

## Contents

**Structs**

- [`Messages`](#messages) - A visitor wrapper that ensures any strings named "message" are formatted

---

## tracing_subscriber::field::display::Messages

*Struct*

A visitor wrapper that ensures any strings named "message" are formatted
using `fmt::Display`

**Generic Parameters:**
- V

**Tuple Struct**: `()`

**Methods:**

- `fn new(inner: V) -> Self` - Returns a new [`MakeVisitor`] implementation that will wrap `inner` so

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **MakeVisitor**
  - `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor`
- **VisitWrite**
  - `fn writer(self: & mut Self) -> & mut dyn io::Write`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **VisitOutput**
  - `fn finish(self: Self) -> O`
- **Visit**
  - `fn record_f64(self: & mut Self, field: &Field, value: f64)`
  - `fn record_i64(self: & mut Self, field: &Field, value: i64)`
  - `fn record_u64(self: & mut Self, field: &Field, value: u64)`
  - `fn record_bool(self: & mut Self, field: &Field, value: bool)`
  - `fn record_str(self: & mut Self, field: &Field, value: &str)` - Visit a string value.
  - `fn record_debug(self: & mut Self, field: &Field, value: &dyn fmt::Debug)`
- **VisitFmt**
  - `fn writer(self: & mut Self) -> & mut dyn fmt::Write`



