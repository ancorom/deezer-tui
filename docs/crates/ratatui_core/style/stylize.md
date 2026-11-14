**ratatui_core > style > stylize**

# Module: style::stylize

## Contents

**Traits**

- [`Styled`](#styled) - A trait for objects that have a `Style`.
- [`Stylize`](#stylize) - An extension trait for styling objects.

---

## ratatui_core::style::stylize::Styled

*Trait*

A trait for objects that have a `Style`.

This trait enables generic code to be written that can interact with any object that has a
`Style`. This is used by the `Stylize` trait to allow generic code to be written that can
interact with any object that can be styled.

**Methods:**

- `Item`
- `style`: Returns the style of the object.
- `set_style`: Sets the style of the object.



## ratatui_core::style::stylize::Stylize

*Trait*

An extension trait for styling objects.

For any type that implements `Stylize`, the provided methods in this trait can be used to style
the type further. This trait is automatically implemented for any type that implements the
[`Styled`] trait which e.g.: [`String`], [`&str`], [`Span`], [`Style`] and many Widget types.

This results in much more ergonomic styling of text and widgets. For example, instead of
writing:

```rust,ignore
let text = Span::styled("Hello", Style::default().fg(Color::Red).bg(Color::Blue));
```

You can write:

```rust,ignore
let text = "Hello".red().on_blue();
```

This trait implements a provided method for every color as both foreground and background
(prefixed by `on_`), and all modifiers as both an additive and subtractive modifier (prefixed
by `not_`). The `reset()` method is also provided to reset the style.

# Examples
```ignore
use ratatui_core::{
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
};

let span = "hello".red().on_blue().bold();
let line = Line::from(vec![
    "hello".red().on_blue().bold(),
    "world".green().on_yellow().not_bold(),
]);
let paragraph = Paragraph::new(line).italic().underlined();
let block = Block::bordered().title("Title").on_white().bold();
```

**Methods:**

- `bg`
- `fg`
- `reset`
- `add_modifier`
- `remove_modifier`
- `black`: Sets the foreground color to [`black`](Color::Black).
- `on_black`: Sets the background color to [`black`](Color::Black).
- `red`: Sets the foreground color to [`red`](Color::Red).
- `on_red`: Sets the background color to [`red`](Color::Red).
- `green`: Sets the foreground color to [`green`](Color::Green).
- `on_green`: Sets the background color to [`green`](Color::Green).
- `yellow`: Sets the foreground color to [`yellow`](Color::Yellow).
- `on_yellow`: Sets the background color to [`yellow`](Color::Yellow).
- `blue`: Sets the foreground color to [`blue`](Color::Blue).
- `on_blue`: Sets the background color to [`blue`](Color::Blue).
- `magenta`: Sets the foreground color to [`magenta`](Color::Magenta).
- `on_magenta`: Sets the background color to [`magenta`](Color::Magenta).
- `cyan`: Sets the foreground color to [`cyan`](Color::Cyan).
- `on_cyan`: Sets the background color to [`cyan`](Color::Cyan).
- `gray`: Sets the foreground color to [`gray`](Color::Gray).
- `on_gray`: Sets the background color to [`gray`](Color::Gray).
- `dark_gray`: Sets the foreground color to [`dark_gray`](Color::DarkGray).
- `on_dark_gray`: Sets the background color to [`dark_gray`](Color::DarkGray).
- `light_red`: Sets the foreground color to [`light_red`](Color::LightRed).
- `on_light_red`: Sets the background color to [`light_red`](Color::LightRed).
- `light_green`: Sets the foreground color to [`light_green`](Color::LightGreen).
- `on_light_green`: Sets the background color to [`light_green`](Color::LightGreen).
- `light_yellow`: Sets the foreground color to [`light_yellow`](Color::LightYellow).
- `on_light_yellow`: Sets the background color to [`light_yellow`](Color::LightYellow).
- `light_blue`: Sets the foreground color to [`light_blue`](Color::LightBlue).
- `on_light_blue`: Sets the background color to [`light_blue`](Color::LightBlue).
- `light_magenta`: Sets the foreground color to [`light_magenta`](Color::LightMagenta).
- `on_light_magenta`: Sets the background color to [`light_magenta`](Color::LightMagenta).
- `light_cyan`: Sets the foreground color to [`light_cyan`](Color::LightCyan).
- `on_light_cyan`: Sets the background color to [`light_cyan`](Color::LightCyan).
- `white`: Sets the foreground color to [`white`](Color::White).
- `on_white`: Sets the background color to [`white`](Color::White).
- `bold`: Adds the [`bold`](Modifier::BOLD) modifier.
- `not_bold`: Removes the [`bold`](Modifier::BOLD) modifier.
- `dim`: Adds the [`dim`](Modifier::DIM) modifier.
- `not_dim`: Removes the [`dim`](Modifier::DIM) modifier.
- `italic`: Adds the [`italic`](Modifier::ITALIC) modifier.
- `not_italic`: Removes the [`italic`](Modifier::ITALIC) modifier.
- `underlined`: Adds the [`underlined`](Modifier::UNDERLINED) modifier.
- `not_underlined`: Removes the [`underlined`](Modifier::UNDERLINED) modifier.
- `slow_blink`: Adds the [`slow_blink`](Modifier::SLOW_BLINK) modifier.
- `not_slow_blink`: Removes the [`slow_blink`](Modifier::SLOW_BLINK) modifier.
- `rapid_blink`: Adds the [`rapid_blink`](Modifier::RAPID_BLINK) modifier.
- `not_rapid_blink`: Removes the [`rapid_blink`](Modifier::RAPID_BLINK) modifier.
- `reversed`: Adds the [`reversed`](Modifier::REVERSED) modifier.
- `not_reversed`: Removes the [`reversed`](Modifier::REVERSED) modifier.
- `hidden`: Adds the [`hidden`](Modifier::HIDDEN) modifier.
- `not_hidden`: Removes the [`hidden`](Modifier::HIDDEN) modifier.
- `crossed_out`: Adds the [`crossed_out`](Modifier::CROSSED_OUT) modifier.
- `not_crossed_out`: Removes the [`crossed_out`](Modifier::CROSSED_OUT) modifier.



