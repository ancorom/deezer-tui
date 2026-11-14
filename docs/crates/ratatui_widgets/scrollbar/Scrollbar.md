**ratatui_widgets > scrollbar > Scrollbar**

# Module: scrollbar::Scrollbar

## Contents

**Functions**

- [`orientation`](#orientation) - Sets the position of the scrollbar.
- [`symbols`](#symbols) - Sets the symbols used for the various parts of the scrollbar from a [`Set`].

---

## ratatui_widgets::scrollbar::Scrollbar::orientation

*Function*

Sets the position of the scrollbar.

The orientation of the scrollbar is the position it will take around a [`Rect`]. See
[`ScrollbarOrientation`] for more details.

Resets the symbols to [`DOUBLE_VERTICAL`] or [`DOUBLE_HORIZONTAL`] based on orientation.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn orientation(self: Self, orientation: ScrollbarOrientation) -> Self
```



## ratatui_widgets::scrollbar::Scrollbar::symbols

*Function*

Sets the symbols used for the various parts of the scrollbar from a [`Set`].

```text
<--▮------->
^  ^   ^   ^
│  │   │   └ end
│  │   └──── track
│  └──────── thumb
└─────────── begin
```

Only sets `begin_symbol`, `end_symbol` and `track_symbol` if they already contain a value.
If they were set to `None` explicitly, this function will respect that choice. Use their
respective setters to change their value.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn symbols(self: Self, symbols: Set<'a>) -> Self
```



