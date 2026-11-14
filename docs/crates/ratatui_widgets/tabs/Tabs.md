**ratatui_widgets > tabs > Tabs**

# Module: tabs::Tabs

## Contents

**Functions**

- [`block`](#block) - Surrounds the `Tabs` with a [`Block`].
- [`divider`](#divider) - Sets the string to use as tab divider.
- [`highlight_style`](#highlight_style) - Sets the style for the highlighted tab.
- [`new`](#new) - Creates new `Tabs` from their titles.
- [`padding`](#padding) - Sets the padding between tabs.
- [`padding_left`](#padding_left) - Sets the left side padding between tabs.
- [`padding_right`](#padding_right) - Sets the right side padding between tabs.
- [`select`](#select) - Sets the selected tab.
- [`style`](#style) - Sets the style of the tabs.
- [`titles`](#titles) - Sets the titles of the tabs.

---

## ratatui_widgets::tabs::Tabs::block

*Function*

Surrounds the `Tabs` with a [`Block`].

```rust
fn block(self: Self, block: Block<'a>) -> Self
```



## ratatui_widgets::tabs::Tabs::divider

*Function*

Sets the string to use as tab divider.

By default, the divider is a pipe (`|`).

# Examples

Use a dot (`•`) as separator.
```
use ratatui::symbols;
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider(symbols::DOT);
```
Use dash (`-`) as separator.
```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider("-");
```

```rust
fn divider<T>(self: Self, divider: T) -> Self
```



## ratatui_widgets::tabs::Tabs::highlight_style

*Function*

Sets the style for the highlighted tab.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

Highlighted tab can be selected with [`Tabs::select`].

[`Color`]: ratatui_core::style::Color

```rust
fn highlight_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::tabs::Tabs::new

*Function*

Creates new `Tabs` from their titles.

`titles` can be a [`Vec`] of [`&str`], [`String`] or anything that can be converted into
[`Line`]. As such, titles can be styled independently.

The selected tab can be set with [`Tabs::select`]. The first tab has index 0 (this is also
the default index).

The selected tab can have a different style with [`Tabs::highlight_style`]. This defaults to
a style with the [`Modifier::REVERSED`] modifier added.

The default divider is a pipe (`|`), but it can be customized with [`Tabs::divider`].

The entire widget can be styled with [`Tabs::style`].

The widget can be wrapped in a [`Block`] using [`Tabs::block`].

# Examples

Basic titles.
```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]);
```

Styled titles
```
use ratatui::style::Stylize;
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1".red(), "Tab 2".blue()]);
```
[`String`]: alloc::string::String
[`Modifier::REVERSED`]: ratatui_core::style::Modifier

```rust
fn new<Iter>(titles: Iter) -> Self
```



## ratatui_widgets::tabs::Tabs::padding

*Function*

Sets the padding between tabs.

Both default to space.

# Examples

A space on either side of the tabs.
```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding(" ", " ");
```
Nothing on either side of the tabs.
```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding("", "");
```

```rust
fn padding<T, U>(self: Self, left: T, right: U) -> Self
```



## ratatui_widgets::tabs::Tabs::padding_left

*Function*

Sets the left side padding between tabs.

Defaults to a space.

# Example

An arrow on the left of tabs.
```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding_left("->");
```

```rust
fn padding_left<T>(self: Self, padding: T) -> Self
```



## ratatui_widgets::tabs::Tabs::padding_right

*Function*

Sets the right side padding between tabs.

Defaults to a space.

# Example

An arrow on the right of tabs.
```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding_right("<-");
```

```rust
fn padding_right<T>(self: Self, padding: T) -> Self
```



## ratatui_widgets::tabs::Tabs::select

*Function*

Sets the selected tab.

The first tab has index 0 (this is also the default index).
The selected tab can have a different style with [`Tabs::highlight_style`].

# Examples

Select the second tab.

```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).select(1);
```

Deselect the selected tab.

```
use ratatui::widgets::Tabs;

let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).select(None);
```

```rust
fn select<T>(self: Self, selected: T) -> Self
```



## ratatui_widgets::tabs::Tabs::style

*Function*

Sets the style of the tabs.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This will set the given style on the entire render area.
More precise style can be applied to the titles by styling the ones given to [`Tabs::new`].
The selected tab can be styled differently using [`Tabs::highlight_style`].

[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::tabs::Tabs::titles

*Function*

Sets the titles of the tabs.

`titles` is an iterator whose elements can be converted into `Line`.

The selected tab can be set with [`Tabs::select`]. The first tab has index 0 (this is also
the default index).

# Examples

Basic titles.

```
use ratatui::widgets::Tabs;

let tabs = Tabs::default().titles(vec!["Tab 1", "Tab 2"]);
```

Styled titles.

```
use ratatui::style::Stylize;
use ratatui::widgets::Tabs;

let tabs = Tabs::default().titles(vec!["Tab 1".red(), "Tab 2".blue()]);
```

```rust
fn titles<Iter>(self: Self, titles: Iter) -> Self
```



