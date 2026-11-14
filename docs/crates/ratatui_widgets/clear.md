**ratatui_widgets > clear**

# Module: clear

## Contents

**Structs**

- [`Clear`](#clear) - A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).

---

## ratatui_widgets::clear::Clear

*Struct*

A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).

This widget **cannot be used to clear the terminal on the first render** as `ratatui` assumes
the render area is empty. Use `Terminal::clear` instead.

# Examples

```
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Clear};

fn draw_on_clear(f: &mut Frame, area: Rect) {
    let block = Block::bordered().title("Block");
    f.render_widget(Clear, area); // <- this will clear/reset the area first
    f.render_widget(block, area); // now render the block widget
}
```

# Popup Example

For a more complete example how to utilize `Clear` to realize popups see
the example `examples/popup.rs`

**Unit Struct**

**Traits:** Eq

**Trait Implementations:**

- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::clear::Clear`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



