**tracing_subscriber > fmt > fmt_layer > FmtContext**

# Module: fmt::fmt_layer::FmtContext

## Contents

**Functions**

- [`field_format`](#field_format) - Returns the [field formatter] configured by the subscriber invoking

---

## tracing_subscriber::fmt::fmt_layer::FmtContext::field_format

*Function*

Returns the [field formatter] configured by the subscriber invoking
`format_event`.

The event formatter may use the returned field formatter to format the
fields of any events it records.

[field formatter]: FormatFields

```rust
fn field_format(self: &Self) -> &N
```



