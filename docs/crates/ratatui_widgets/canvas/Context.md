**ratatui_widgets > canvas > Context**

# Module: canvas::Context

## Contents

**Functions**

- [`draw`](#draw) - Draw the given [`Shape`] in this context
- [`layer`](#layer) - Save the existing state of the grid as a layer.
- [`print`](#print) - Print a [`Text`] on the [`Canvas`] at the given position.

---

## ratatui_widgets::canvas::Context::draw

*Function*

Draw the given [`Shape`] in this context

```rust
fn draw<S>(self: & mut Self, shape: &S)
```



## ratatui_widgets::canvas::Context::layer

*Function*

Save the existing state of the grid as a layer.

Save the existing state as a layer to be rendered and reset the grid to its initial
state for the next layer.

This allows the canvas to be drawn in multiple layers. This is useful if you want to
draw multiple shapes on the [`Canvas`] in specific order.

```rust
fn layer(self: & mut Self)
```



## ratatui_widgets::canvas::Context::print

*Function*

Print a [`Text`] on the [`Canvas`] at the given position.

Note that the text is always printed on top of the canvas and is **not** affected by the
layers.

[`Text`]: ratatui_core::text::Text

```rust
fn print<T>(self: & mut Self, x: f64, y: f64, line: T)
```



