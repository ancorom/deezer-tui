**ratatui_image > picker > Picker**

# Module: picker::Picker

## Contents

**Functions**

- [`capabilities`](#capabilities) - Returns the capabilities detected by [Picker::from_query_stdio].
- [`from_query_stdio`](#from_query_stdio) - Query terminal stdio for graphics capabilities and font-size with some escape sequences.

---

## ratatui_image::picker::Picker::capabilities

*Function*

Returns the capabilities detected by [Picker::from_query_stdio].

```rust
fn capabilities(self: &Self) -> &Vec<Capability>
```



## ratatui_image::picker::Picker::from_query_stdio

*Function*

Query terminal stdio for graphics capabilities and font-size with some escape sequences.

This writes and reads from stdio momentarily. WARNING: this method should be called after
entering alternate screen but before reading terminal events.

# Example
```rust
use ratatui_image::picker::Picker;
let mut picker = Picker::from_query_stdio();
```


```rust
fn from_query_stdio() -> std::result::Result<Self, errors::Errors>
```



