**tracing_subscriber > field > debug**

# Module: field::debug

## Contents

**Structs**

- [`Alt`](#alt) - A visitor wrapper that ensures any `fmt::Debug` fields are formatted using

---

## tracing_subscriber::field::debug::Alt

*Struct*

A visitor wrapper that ensures any `fmt::Debug` fields are formatted using
the alternate (`:#`) formatter.

**Generic Parameters:**
- V

**Tuple Struct**: `()`

**Methods:**

- `fn new(inner: V) -> Self` - Wraps the provided visitor so that any `fmt::Debug` fields are formatted

**Trait Implementations:**

- **VisitFmt**
  - `fn writer(self: & mut Self) -> & mut dyn fmt::Write`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Visit**
  - `fn record_f64(self: & mut Self, field: &Field, value: f64)`
  - `fn record_i64(self: & mut Self, field: &Field, value: i64)`
  - `fn record_u64(self: & mut Self, field: &Field, value: u64)`
  - `fn record_bool(self: & mut Self, field: &Field, value: bool)`
  - `fn record_str(self: & mut Self, field: &Field, value: &str)` - Visit a string value.
  - `fn record_debug(self: & mut Self, field: &Field, value: &dyn fmt::Debug)`
- **MakeVisitor**
  - `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor`
- **VisitOutput**
  - `fn finish(self: Self) -> O`
- **VisitWrite**
  - `fn writer(self: & mut Self) -> & mut dyn io::Write`



