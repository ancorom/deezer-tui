**tracing_subscriber > fmt > format > Writer**

# Module: fmt::format::Writer

## Contents

**Functions**

- [`has_ansi_escapes`](#has_ansi_escapes) - Returns `true` if [ANSI escape codes] may be used to add colors

---

## tracing_subscriber::fmt::format::Writer::has_ansi_escapes

*Function*

Returns `true` if [ANSI escape codes] may be used to add colors
and other formatting when writing to this `Writer`.

If this returns `false`, formatters should not emit ANSI escape codes.

[ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

```rust
fn has_ansi_escapes(self: &Self) -> bool
```



