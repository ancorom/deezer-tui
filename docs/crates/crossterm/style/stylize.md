**crossterm > style > stylize**

# Module: style::stylize

## Contents

**Traits**

- [`Stylize`](#stylize) - Provides a set of methods to set attributes and colors.

---

## crossterm::style::stylize::Stylize

*Trait*

Provides a set of methods to set attributes and colors.

# Examples

```no_run
use crossterm::style::Stylize;

println!("{}", "Bold text".bold());
println!("{}", "Underlined text".underlined());
println!("{}", "Negative text".negative());
println!("{}", "Red on blue".red().on_blue());
```

**Methods:**

- `Styled`: This type with styles applied.
- `stylize`: Styles this type.
- `with`: Sets the foreground color.
- `on`: Sets the background color.
- `underline`: Sets the underline color.
- `attribute`: Styles the content with the attribute.
- `reset`: Applies the [`Reset`](Attribute::Reset) attribute to the text.
- `bold`: Applies the [`Bold`](Attribute::Bold) attribute to the text.
- `underlined`: Applies the [`Underlined`](Attribute::Underlined) attribute to the text.
- `reverse`: Applies the [`Reverse`](Attribute::Reverse) attribute to the text.
- `dim`: Applies the [`Dim`](Attribute::Dim) attribute to the text.
- `italic`: Applies the [`Italic`](Attribute::Italic) attribute to the text.
- `negative`: Applies the [`Reverse`](Attribute::Reverse) attribute to the text.
- `slow_blink`: Applies the [`SlowBlink`](Attribute::SlowBlink) attribute to the text.
- `rapid_blink`: Applies the [`RapidBlink`](Attribute::RapidBlink) attribute to the text.
- `hidden`: Applies the [`Hidden`](Attribute::Hidden) attribute to the text.
- `crossed_out`: Applies the [`CrossedOut`](Attribute::CrossedOut) attribute to the text.
- `black`: Sets the foreground color to [`Black`](Color::Black).
- `on_black`: Sets the background color to [`Black`](Color::Black).
- `underline_black`: Sets the underline color to [`Black`](Color::Black).
- `dark_grey`: Sets the foreground color to [`DarkGrey`](Color::DarkGrey).
- `on_dark_grey`: Sets the background color to [`DarkGrey`](Color::DarkGrey).
- `underline_dark_grey`: Sets the underline color to [`DarkGrey`](Color::DarkGrey).
- `red`: Sets the foreground color to [`Red`](Color::Red).
- `on_red`: Sets the background color to [`Red`](Color::Red).
- `underline_red`: Sets the underline color to [`Red`](Color::Red).
- `dark_red`: Sets the foreground color to [`DarkRed`](Color::DarkRed).
- `on_dark_red`: Sets the background color to [`DarkRed`](Color::DarkRed).
- `underline_dark_red`: Sets the underline color to [`DarkRed`](Color::DarkRed).
- `green`: Sets the foreground color to [`Green`](Color::Green).
- `on_green`: Sets the background color to [`Green`](Color::Green).
- `underline_green`: Sets the underline color to [`Green`](Color::Green).
- `dark_green`: Sets the foreground color to [`DarkGreen`](Color::DarkGreen).
- `on_dark_green`: Sets the background color to [`DarkGreen`](Color::DarkGreen).
- `underline_dark_green`: Sets the underline color to [`DarkGreen`](Color::DarkGreen).
- `yellow`: Sets the foreground color to [`Yellow`](Color::Yellow).
- `on_yellow`: Sets the background color to [`Yellow`](Color::Yellow).
- `underline_yellow`: Sets the underline color to [`Yellow`](Color::Yellow).
- `dark_yellow`: Sets the foreground color to [`DarkYellow`](Color::DarkYellow).
- `on_dark_yellow`: Sets the background color to [`DarkYellow`](Color::DarkYellow).
- `underline_dark_yellow`: Sets the underline color to [`DarkYellow`](Color::DarkYellow).
- `blue`: Sets the foreground color to [`Blue`](Color::Blue).
- `on_blue`: Sets the background color to [`Blue`](Color::Blue).
- `underline_blue`: Sets the underline color to [`Blue`](Color::Blue).
- `dark_blue`: Sets the foreground color to [`DarkBlue`](Color::DarkBlue).
- `on_dark_blue`: Sets the background color to [`DarkBlue`](Color::DarkBlue).
- `underline_dark_blue`: Sets the underline color to [`DarkBlue`](Color::DarkBlue).
- `magenta`: Sets the foreground color to [`Magenta`](Color::Magenta).
- `on_magenta`: Sets the background color to [`Magenta`](Color::Magenta).
- `underline_magenta`: Sets the underline color to [`Magenta`](Color::Magenta).
- `dark_magenta`: Sets the foreground color to [`DarkMagenta`](Color::DarkMagenta).
- `on_dark_magenta`: Sets the background color to [`DarkMagenta`](Color::DarkMagenta).
- `underline_dark_magenta`: Sets the underline color to [`DarkMagenta`](Color::DarkMagenta).
- `cyan`: Sets the foreground color to [`Cyan`](Color::Cyan).
- `on_cyan`: Sets the background color to [`Cyan`](Color::Cyan).
- `underline_cyan`: Sets the underline color to [`Cyan`](Color::Cyan).
- `dark_cyan`: Sets the foreground color to [`DarkCyan`](Color::DarkCyan).
- `on_dark_cyan`: Sets the background color to [`DarkCyan`](Color::DarkCyan).
- `underline_dark_cyan`: Sets the underline color to [`DarkCyan`](Color::DarkCyan).
- `white`: Sets the foreground color to [`White`](Color::White).
- `on_white`: Sets the background color to [`White`](Color::White).
- `underline_white`: Sets the underline color to [`White`](Color::White).
- `grey`: Sets the foreground color to [`Grey`](Color::Grey).
- `on_grey`: Sets the background color to [`Grey`](Color::Grey).
- `underline_grey`: Sets the underline color to [`Grey`](Color::Grey).



