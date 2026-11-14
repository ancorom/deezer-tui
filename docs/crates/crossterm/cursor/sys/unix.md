**crossterm > cursor > sys > unix**

# Module: cursor::sys::unix

## Contents

**Functions**

- [`position`](#position) - Returns the cursor position (column, row).

---

## crossterm::cursor::sys::unix::position

*Function*

Returns the cursor position (column, row).

The top left cell is represented as `(0, 0)`.

On unix systems, this function will block and possibly time out while
[`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.

```rust
fn position() -> io::Result<(u16, u16)>
```



