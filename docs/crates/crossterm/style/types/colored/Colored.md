**crossterm > style > types > colored > Colored**

# Module: style::types::colored::Colored

## Contents

**Functions**

- [`parse_ansi`](#parse_ansi) - Parse an ANSI foreground or background color.

---

## crossterm::style::types::colored::Colored::parse_ansi

*Function*

Parse an ANSI foreground or background color.
This is the string that would appear within an `ESC [ <str> m` escape sequence, as found in
various configuration files.

# Examples

```
use crossterm::style::{Colored::{self, ForegroundColor, BackgroundColor}, Color};

assert_eq!(Colored::parse_ansi("38;5;0"), Some(ForegroundColor(Color::Black)));
assert_eq!(Colored::parse_ansi("38;5;26"), Some(ForegroundColor(Color::AnsiValue(26))));
assert_eq!(Colored::parse_ansi("48;2;50;60;70"), Some(BackgroundColor(Color::Rgb { r: 50, g: 60, b: 70 })));
assert_eq!(Colored::parse_ansi("49"), Some(BackgroundColor(Color::Reset)));
assert_eq!(Colored::parse_ansi("invalid color"), None);
```

Currently, 3/4 bit color values aren't supported so return `None`.

See also: [`Color::parse_ansi`].

```rust
fn parse_ansi(ansi: &str) -> Option<Self>
```



