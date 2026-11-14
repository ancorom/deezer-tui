**crossterm > tty**

# Module: tty

## Contents

**Traits**

- [`IsTty`](#istty) - Adds the `is_tty` method to types that might represent a terminal

---

## crossterm::tty::IsTty

*Trait*

Adds the `is_tty` method to types that might represent a terminal

```rust
use std::io::stdout;
use crossterm::tty::IsTty;

let is_tty: bool = stdout().is_tty();
```

**Methods:**

- `is_tty`: Returns true when an instance is a terminal teletype, otherwise false.



