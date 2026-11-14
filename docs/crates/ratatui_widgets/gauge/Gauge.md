**ratatui_widgets > gauge > Gauge**

# Module: gauge::Gauge

## Contents

**Functions**

- [`block`](#block) - Surrounds the `Gauge` with a [`Block`].
- [`label`](#label) - Sets the label to display in the center of the bar.
- [`percent`](#percent) - Sets the bar progression from a percentage.
- [`ratio`](#ratio) - Sets the bar progression from a ratio (float).
- [`use_unicode`](#use_unicode) - Sets whether to use unicode characters to display the progress bar.

---

## ratatui_widgets::gauge::Gauge::block

*Function*

Surrounds the `Gauge` with a [`Block`].

The gauge is rendered in the inner portion of the block once space for borders and padding
is reserved. Styles set on the block do **not** affect the bar itself.

```rust
fn block(self: Self, block: Block<'a>) -> Self
```



## ratatui_widgets::gauge::Gauge::label

*Function*

Sets the label to display in the center of the bar.

For a left-aligned label, see [`LineGauge`].
If the label is not defined, it is the percentage filled.

```rust
fn label<T>(self: Self, label: T) -> Self
```



## ratatui_widgets::gauge::Gauge::percent

*Function*

Sets the bar progression from a percentage.

# Panics

This method panics if `percent` is **not** between 0 and 100 inclusively.

# See also

See [`Gauge::ratio`] to set from a float.

```rust
fn percent(self: Self, percent: u16) -> Self
```



## ratatui_widgets::gauge::Gauge::ratio

*Function*

Sets the bar progression from a ratio (float).

`ratio` is the ratio between filled bar over empty bar (i.e. `3/4` completion is `0.75`).
This is more easily seen as a floating point percentage (e.g. 42% = `0.42`).

# Panics

This method panics if `ratio` is **not** between 0 and 1 inclusively.

# See also

See [`Gauge::percent`] to set from a percentage.

```rust
fn ratio(self: Self, ratio: f64) -> Self
```



## ratatui_widgets::gauge::Gauge::use_unicode

*Function*

Sets whether to use unicode characters to display the progress bar.

This enables the use of
[unicode block characters](https://en.wikipedia.org/wiki/Block_Elements).
This is useful to display a higher precision bar (8 extra fractional parts per cell).

```rust
fn use_unicode(self: Self, unicode: bool) -> Self
```



