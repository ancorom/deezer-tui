**ratatui_core > layout > position > Position**

# Module: layout::position::Position

## Contents

**Functions**

- [`new`](#new) - Create a new position
- [`offset`](#offset) - Moves the position by the given offset.

---

## ratatui_core::layout::position::Position::new

*Function*

Create a new position

```rust
fn new(x: u16, y: u16) -> Self
```



## ratatui_core::layout::position::Position::offset

*Function*

Moves the position by the given offset.

Positive offsets move right and down, negative offsets move left and up. Values that would
move the position outside the `u16` range are clamped to the nearest edge.

```rust
fn offset(self: Self, offset: Offset) -> Self
```



