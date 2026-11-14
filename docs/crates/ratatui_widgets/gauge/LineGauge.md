**ratatui_widgets > gauge > LineGauge**

# Module: gauge::LineGauge

## Contents

**Functions**

- [`filled_style`](#filled_style) - Sets the style of filled part of the bar.
- [`filled_symbol`](#filled_symbol) - Sets the symbol for the filled part of the gauge.
- [`label`](#label) - Sets the label to display.
- [`line_set`](#line_set) - Sets the characters to use for the line.
- [`ratio`](#ratio) - Sets the bar progression from a ratio (float).
- [`unfilled_style`](#unfilled_style) - Sets the style of the unfilled part of the bar.
- [`unfilled_symbol`](#unfilled_symbol) - Sets the symbol for the unfilled part of the gauge.

---

## ratatui_widgets::gauge::LineGauge::filled_style

*Function*

Sets the style of filled part of the bar.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

```rust
fn filled_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::gauge::LineGauge::filled_symbol

*Function*

Sets the symbol for the filled part of the gauge.

```rust
fn filled_symbol(self: Self, symbol: &'a str) -> Self
```



## ratatui_widgets::gauge::LineGauge::label

*Function*

Sets the label to display.

With `LineGauge`, labels are only on the left, see [`Gauge`] for a centered label.
If the label is not defined, it is the percentage filled.

```rust
fn label<T>(self: Self, label: T) -> Self
```



## ratatui_widgets::gauge::LineGauge::line_set

*Function*

Sets the characters to use for the line.

# See also

See [`symbols::line::Set`] for more information. Predefined sets are also available, see
[`NORMAL`](symbols::line::NORMAL), [`DOUBLE`](symbols::line::DOUBLE) and
[`THICK`](symbols::line::THICK).

```rust
fn line_set(self: Self, set: symbols::line::Set<'a>) -> Self
```



## ratatui_widgets::gauge::LineGauge::ratio

*Function*

Sets the bar progression from a ratio (float).

`ratio` is the ratio between filled bar over empty bar (i.e. `3/4` completion is `0.75`).
This is more easily seen as a floating point percentage (e.g. 42% = `0.42`).

# Panics

This method panics if `ratio` is **not** between 0 and 1 inclusively.

```rust
fn ratio(self: Self, ratio: f64) -> Self
```



## ratatui_widgets::gauge::LineGauge::unfilled_style

*Function*

Sets the style of the unfilled part of the bar.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

```rust
fn unfilled_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::gauge::LineGauge::unfilled_symbol

*Function*

Sets the symbol for the unfilled part of the gauge.

```rust
fn unfilled_symbol(self: Self, symbol: &'a str) -> Self
```



