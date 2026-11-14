**ratatui_widgets > block > shadow > Shadow**

# Module: block::shadow::Shadow

## Contents

**Functions**

- [`block`](#block) - Creates a shadow filled with full block symbols.
- [`custom`](#custom) - Creates a new shadow from a custom cell effect.
- [`dark_shade`](#dark_shade) - Creates a shadow filled with dark shade symbols.
- [`light_shade`](#light_shade) - Creates a shadow filled with light shade symbols.
- [`medium_shade`](#medium_shade) - Creates a shadow filled with medium shade symbols.
- [`overlay`](#overlay) - Creates a shadow that only applies style to the offset area.

---

## ratatui_widgets::block::shadow::Shadow::block

*Function*

Creates a shadow filled with full block symbols.

# Example

```
use ratatui::widgets::{Block, Shadow};

let block = Block::bordered().shadow(Shadow::block());
```

```rust
fn block() -> Self
```



## ratatui_widgets::block::shadow::Shadow::custom

*Function*

Creates a new shadow from a custom cell effect.

The effect receives the shadow area, the original block area, and the target buffer. It is
called after the shadow style has been applied.

```rust
fn custom<F>(effect: F) -> Self
```



## ratatui_widgets::block::shadow::Shadow::dark_shade

*Function*

Creates a shadow filled with dark shade symbols.

# Example

```
use ratatui::layout::Offset;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Shadow};

let block = Block::bordered().shadow(
    Shadow::dark_shade()
        .black()
        .on_white()
        .offset(Offset::new(2, 1)),
);
```

```rust
fn dark_shade() -> Self
```



## ratatui_widgets::block::shadow::Shadow::light_shade

*Function*

Creates a shadow filled with light shade symbols.

# Example

```
use ratatui::widgets::{Block, Shadow};

let block = Block::bordered().shadow(Shadow::light_shade());
```

```rust
fn light_shade() -> Self
```



## ratatui_widgets::block::shadow::Shadow::medium_shade

*Function*

Creates a shadow filled with medium shade symbols.

# Example

```
use ratatui::widgets::{Block, Shadow};

let block = Block::bordered().shadow(Shadow::medium_shade());
```

```rust
fn medium_shade() -> Self
```



## ratatui_widgets::block::shadow::Shadow::overlay

*Function*

Creates a shadow that only applies style to the offset area.

This leaves the existing cell symbols unchanged.

# Example

```
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Shadow};

let shadow = Shadow::overlay().black().on_white();
let block = Block::bordered().shadow(shadow);
```

```rust
fn overlay() -> Self
```



