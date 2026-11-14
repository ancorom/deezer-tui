**tracing_subscriber > filter > directive**

# Module: filter::directive

## Contents

**Structs**

- [`ParseError`](#parseerror) - Indicates that a string could not be parsed as a filtering directive.

---

## tracing_subscriber::filter::directive::ParseError

*Struct*

Indicates that a string could not be parsed as a filtering directive.

**Trait Implementations:**

- **From**
  - `fn from(l: level::ParseError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(e: Box<dyn std::error::Error>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn description(self: &Self) -> &str`
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`



