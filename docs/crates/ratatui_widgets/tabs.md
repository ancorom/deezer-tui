**ratatui_widgets > tabs**

# Module: tabs

## Contents

**Structs**

- [`Tabs`](#tabs) - A widget that displays a horizontal set of Tabs with a single tab selected.

---

## ratatui_widgets::tabs::Tabs

*Struct*

A widget that displays a horizontal set of Tabs with a single tab selected.

Each tab title is stored as a [`Line`] which can be individually styled. The selected tab is set
using [`Tabs::select`] and styled using [`Tabs::highlight_style`]. The divider can be customized
with [`Tabs::divider`]. Padding can be set with [`Tabs::padding`] or [`Tabs::padding_left`] and
[`Tabs::padding_right`].

The divider defaults to |, and padding defaults to a singular space on each side.

# Example

```
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::widgets::{Block, Tabs};

Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
    .block(Block::bordered().title("Tabs"))
    .style(Style::default().white())
    .highlight_style(Style::default().yellow())
    .select(2)
    .divider(symbols::DOT)
    .padding("->", "<-");
```

In addition to `Tabs::new`, any iterator whose element is convertible to `Line` can be collected
into `Tabs`.

```
use ratatui::widgets::Tabs;

(0..5).map(|i| format!("Tab{i}")).collect::<Tabs>();
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<Iter>(titles: Iter) -> Self` - Creates new `Tabs` from their titles.
- `fn titles<Iter>(self: Self, titles: Iter) -> Self` - Sets the titles of the tabs.
- `fn block(self: Self, block: Block<'a>) -> Self` - Surrounds the `Tabs` with a [`Block`].
- `fn select<T>(self: Self, selected: T) -> Self` - Sets the selected tab.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of the tabs.
- `fn highlight_style<S>(self: Self, style: S) -> Self` - Sets the style for the highlighted tab.
- `fn divider<T>(self: Self, divider: T) -> Self` - Sets the string to use as tab divider.
- `fn padding<T, U>(self: Self, left: T, right: U) -> Self` - Sets the padding between tabs.
- `fn padding_left<T>(self: Self, padding: T) -> Self` - Sets the left side padding between tabs.
- `fn padding_right<T>(self: Self, padding: T) -> Self` - Sets the right side padding between tabs.

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::tabs::Tabs<'a>`
- **Default**
  - `fn default() -> Self` - Returns a default `Tabs` widget.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **FromIterator**
  - `fn from_iter<Iter>(iter: Iter) -> Self`
- **UnicodeWidthStr**
  - `fn width(self: &Self) -> usize` - Returns the width of the rendered tabs.
  - `fn width_cjk(self: &Self) -> usize` - Returns the width of the rendered tabs, accounting for CJK characters.



