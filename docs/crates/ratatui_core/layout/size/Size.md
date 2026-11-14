**ratatui_core > layout > size > Size**

# Module: layout::size::Size

## Contents

**Functions**

- [`area`](#area) - Compute the total area of the size as a `u32`.
- [`new`](#new) - Create a new `Size` struct

---

## ratatui_core::layout::size::Size::area

*Function*

Compute the total area of the size as a `u32`.

The multiplication uses `u32` to avoid overflow when the width and height are at their
`u16` maximum values.

```rust
fn area(self: Self) -> u32
```



## ratatui_core::layout::size::Size::new

*Function*

Create a new `Size` struct

```rust
fn new(width: u16, height: u16) -> Self
```



