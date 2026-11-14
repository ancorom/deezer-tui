**ratatui_widgets > barchart > bar_group**

# Module: barchart::bar_group

## Contents

**Structs**

- [`BarGroup`](#bargroup) - A group of bars to be shown by the Barchart.

---

## ratatui_widgets::barchart::bar_group::BarGroup

*Struct*

A group of bars to be shown by the Barchart.

# Examples

```
use ratatui::widgets::{Bar, BarGroup};

let group = BarGroup::new([Bar::with_label("Red", 20), Bar::with_label("Blue", 15)]);
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(bars: T) -> Self` - Creates a new `BarGroup` with the given bars.
- `fn with_label<T, B>(label: T, bars: B) -> Self` - Creates a new `BarGroup` with the given bars and label.
- `fn label<T>(self: Self, label: T) -> Self` - Set the group label
- `fn bars(self: Self, bars: &[Bar<'a>]) -> Self` - Set the bars of the group to be shown

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: &Vec<(&'a str, u64)>) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: &[(&'a str, u64); N]) -> Self`
- **From**
  - `fn from(value: &[(&'a str, u64)]) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



