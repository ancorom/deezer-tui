**crossterm > style > types > color > Color**

# Module: style::types::color::Color

## Contents

**Functions**

- [`parse_ansi`](#parse_ansi) - Parses an ANSI color sequence.

---

## crossterm::style::types::color::Color::parse_ansi

*Function*

Parses an ANSI color sequence.

# Examples

```
use crossterm::style::Color;

assert_eq!(Color::parse_ansi("5;0"), Some(Color::Black));
assert_eq!(Color::parse_ansi("5;26"), Some(Color::AnsiValue(26)));
assert_eq!(Color::parse_ansi("2;50;60;70"), Some(Color::Rgb { r: 50, g: 60, b: 70 }));
assert_eq!(Color::parse_ansi("invalid color"), None);
```

Currently, 3/4 bit color values aren't supported so return `None`.

See also: [`Colored::parse_ansi`](crate::style::Colored::parse_ansi).

```rust
fn parse_ansi(ansi: &str) -> Option<Self>
```



