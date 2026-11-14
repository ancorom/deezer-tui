**crossterm > terminal > sys > unix**

# Module: terminal::sys::unix

## Contents

**Functions**

- [`supports_keyboard_enhancement`](#supports_keyboard_enhancement) - Queries the terminal's support for progressive keyboard enhancement.

---

## crossterm::terminal::sys::unix::supports_keyboard_enhancement

*Function*

Queries the terminal's support for progressive keyboard enhancement.

On unix systems, this function will block and possibly time out while
[`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.

```rust
fn supports_keyboard_enhancement() -> io::Result<bool>
```



