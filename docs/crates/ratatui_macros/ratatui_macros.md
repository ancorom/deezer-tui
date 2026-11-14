**ratatui_macros**

# Module: ratatui_macros

## Contents

**Macros**

- [`constraint`](#constraint) - Creates a single constraint.
- [`constraints`](#constraints) - Creates an array of constraints.
- [`horizontal`](#horizontal) - Creates a horizontal layout with specified constraints.
- [`line`](#line) - A macro for creating a [`Line`] using vec! syntax.
- [`row`](#row) - A macro for creating a [`Row`] using vec! syntax.
- [`span`](#span) - A macro for creating a [`Span`] using formatting syntax.
- [`text`](#text) - A macro for creating a [`Text`] using vec! syntax.
- [`vertical`](#vertical) - Creates a vertical layout with specified constraints.

---

## ratatui_macros::constraint

*Declarative Macro*

Creates a single constraint.

If creating an array of constraints, you probably want to use
[`constraints!`] instead.

# Examples

```
# use ratatui_core::layout::Constraint;
use ratatui_macros::constraint;
assert_eq!(constraint!(>= 3 + 4), Constraint::Min(7));
assert_eq!(constraint!(<= 3 + 4), Constraint::Max(7));
assert_eq!(constraint!(== 1 / 3), Constraint::Ratio(1, 3));
assert_eq!(constraint!(== 3), Constraint::Length(3));
assert_eq!(constraint!(== 10 %), Constraint::Percentage(10));
assert_eq!(constraint!(*= 1), Constraint::Fill(1));
```

[`constraints!`]: crate::constraints

```rust
macro_rules! constraint {
    (== $token:tt %) => { ... };
    (>= $expr:expr) => { ... };
    (<= $expr:expr) => { ... };
    (== $num:tt / $denom:tt) => { ... };
    (== $expr:expr) => { ... };
    (*= $expr:expr) => { ... };
}
```



## ratatui_macros::constraints

*Declarative Macro*

Creates an array of constraints.

See [`constraint!`] for more information.

If you want to solve the constraints, see
[`vertical!`] and [`horizontal!`] macros.

# Examples

```rust
use ratatui_macros::constraints;
assert_eq!(constraints![==5, ==30%, >=3, <=1, ==1/2].len(), 5);
assert_eq!(constraints![==5; 5].len(), 5);
```

```rust
# use ratatui_core::layout::Constraint;
# use ratatui_macros::constraints;
assert_eq!(
    constraints![==50, ==30%, >=3, <=1, ==1/2, *=1],
    [
        Constraint::Length(50),
        Constraint::Percentage(30),
        Constraint::Min(3),
        Constraint::Max(1),
        Constraint::Ratio(1, 2),
        Constraint::Fill(1),
    ]
)
```

[`constraint!`]: crate::constraint
[`vertical!`]: crate::vertical
[`horizontal!`]: crate::horizontal

```rust
macro_rules! constraints {
    ([ , $($rest:tt)* ] -> () []) => { ... };
    ([ , $($rest:tt)* ] -> ($($partial:tt)*) [ $($parsed:tt)* ]) => { ... };
    ([ ; $count:expr , ] -> ($($partial:tt)*) []) => { ... };
    ([ $head:tt $($rest:tt)* ] -> ($($partial:tt)*) [ $($parsed:tt)* ]) => { ... };
    ([$(,)?]  -> () [ $( $parsed:tt )* ]) => { ... };
    ($( $constraint:tt )+) => { ... };
}
```



## ratatui_macros::horizontal

*Declarative Macro*

Creates a horizontal layout with specified constraints.

It takes a series of constraints and applies them to create a horizontal layout. The constraints
can include fixed sizes, minimum and maximum sizes, percentages, and ratios.

See [`constraint!`]  or [`constraints!`] for more information.

# Examples

```
// Horizontal layout with a ratio constraint and a minimum size constraint
use ratatui_macros::horizontal;
horizontal![== 1/3, >= 100];
```

```rust
macro_rules! horizontal {
    ($( $constraint:tt )+) => { ... };
}
```



## ratatui_macros::line

*Declarative Macro*

A macro for creating a [`Line`] using vec! syntax.

`line!` is similar to the [`vec!`] macro, but it returns a [`Line`] instead of a `Vec`.

# Examples

* Create a [`Line`] containing a vector of [`Span`]s:

```rust
# use ratatui_core::style::Stylize;
use ratatui_macros::line;

let line = line!["hello", "world"];
let line = line!["hello".red(), "world".red().bold()];
```

* Create a [`Line`] from a given [`Span`] repeated some amount of times:

```rust
# use ratatui_macros::line;
let line = line!["hello"; 2];
```

* Use [`span!`] macro inside [`line!`] macro for formatting.

```rust
# use ratatui_core::style::Modifier;
use ratatui_macros::{line, span};

let line = line![span!("hello {}", "world"), span!(Modifier::BOLD; "goodbye {}", "world")];
```

[`span!`]: crate::span
[`Line`]: ratatui_core::text::Line
[`Span`]: ratatui_core::text::Span
[`vec!`]: alloc::vec!

```rust
macro_rules! line {
    () => { ... };
    ($span:expr; $n:expr) => { ... };
    ($($span:expr),+ $(,)?) => { ... };
}
```



## ratatui_macros::row

*Declarative Macro*

A macro for creating a [`Row`] using vec! syntax.

`row!` is similar to the [`vec!`] macro, but it returns a [`Row`] instead of a `Vec`.

# Examples

* Create a [`Row`] containing a vector of [`Cell`]s:

```rust
# use ratatui_core::style::Stylize;
use ratatui_macros::row;

let row = row!["hello", "world"];
let row = row!["hello".red(), "world".red().bold()];
```

* Create an empty [`Row`]:

```rust
# use ratatui_macros::row;
let empty_row = row![];
```

* Create a [`Row`] from a given [`Cell`] repeated some amount of times:

```rust
# use ratatui_macros::row;
let row = row!["hello"; 2];
```

* Use [`text!`], [`line!`] or [`span!`] macro inside [`row!`] macro.

```rust
# use ratatui_core::style::{Modifier};
use ratatui_macros::{row, line, text, span};

let row = row![
    line!["hello", "world"], span!(Modifier::BOLD; "goodbye {}", "world"),
    text!["hello", "world"],
];
```

[`text!`]: crate::text
[`line!`]: crate::line
[`span!`]: crate::span
[`row!`]: crate::row
[`Row`]: ratatui_widgets::table::Row
[`Cell`]: ratatui_widgets::table::Cell
[`vec!`]: alloc::vec!

```rust
macro_rules! row {
    () => { ... };
    ($cell:expr; $n:expr) => { ... };
    ($($cell:expr),+ $(,)?) => { ... };
}
```



## ratatui_macros::span

*Declarative Macro*

A macro for creating a [`Span`] using formatting syntax.

`span!` is similar to the [`format!`] macro, but it returns a [`Span`] instead of a `String`. In
addition, it also accepts an expression for the first argument, which will be converted to a
string using the [`format!`] macro.

If semicolon follows the first argument, then the first argument is a [`Style`] and a styled
[`Span`] will be created. Otherwise, the [`Span`] will be created as a raw span (i.e. with style
set to `Style::default()`).

# Examples

```rust
# use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_macros::span;

let content = "content";

// expression
let span = span!(content);

// format string
let span = span!("test content");
let span = span!("test {}", "content");
let span = span!("{} {}", "test", "content");
let span = span!("test {content}");
let span = span!("test {content}", content = "content");

// with format specifiers
let span = span!("test {:4}", 123);
let span = span!("test {:04}", 123);

let style = Style::new().green();

// styled expression
let span = span!(style; content);

// styled format string
let span = span!(style; "test content");
let span = span!(style; "test {}", "content");
let span = span!(style; "{} {}", "test", "content");
let span = span!(style; "test {content}");
let span = span!(style; "test {content}", content = "content");

// accepts any type that is convertible to Style
let span = span!(Style::new().green(); "test {content}");
let span = span!(Color::Green; "test {content}");
let span = span!(Modifier::BOLD; "test {content}");

// with format specifiers
let span = span!(style; "test {:4}", 123);
let span = span!(style; "test {:04}", 123);
```

# Note

The first parameter must be a formatting specifier followed by a comma OR anything that can be
converted into a [`Style`] followed by a semicolon.

For example, the following will fail to compile:

```compile_fail
# use ratatui::prelude::*;
# use ratatui_macros::span;
let span = span!(Modifier::BOLD, "hello world");
```

But this will work:

```rust
# use ratatui_core::style::{Modifier};
# use ratatui_macros::span;
let span = span!(Modifier::BOLD; "hello world");
```

The following will fail to compile:

```compile_fail
# use ratatui::prelude::*;
# use ratatui_macros::span;
let span = span!("hello", "world");
```

But this will work:

```rust
# use ratatui_macros::span;
let span = span!("hello {}", "world");
```

[`Color`]: ratatui_core::style::Color
[`Span`]: ratatui_core::text::Span
[`Style`]: ratatui_core::style::Style
[`format!`]: alloc::format!

```rust
macro_rules! span {
    ($string:literal) => { ... };
    ($string:literal, $($arg:tt)*) => { ... };
    ($expr:expr) => { ... };
    ($style:expr, $($arg:tt)*) => { ... };
    ($style:expr; $string:literal) => { ... };
    ($style:expr; $string:literal, $($arg:tt)*) => { ... };
    ($style:expr; $expr:expr) => { ... };
}
```



## ratatui_macros::text

*Declarative Macro*

A macro for creating a [`Text`] using vec! syntax.

`text!` is similar to the [`vec!`] macro, but it returns a [`Text`] instead of a `Vec`.

# Examples

* Create a [`Text`] containing a vector of [`Line`]s:

```rust
# use ratatui_core::style::Stylize;
use ratatui_macros::text;

let text = text!["hello", "world"];
let text = text!["hello".red(), "world".red().bold()];
```

* Create a [`text`] from a given [`Line`] repeated some amount of times:

```rust
# use ratatui_macros::text;
let text = text!["hello"; 2];
```

* Use [`line!`] or [`span!`] macro inside [`text!`] macro.

```rust
# use ratatui_core::style::{Modifier};
use ratatui_macros::{line, text, span};

let text = text![line!["hello", "world"], span!(Modifier::BOLD; "goodbye {}", "world")];
```

[`span!`]: crate::span
[`text!`]: crate::text
[`Text`]: ratatui_core::text::Text
[`Line`]: ratatui_core::text::Line
[`Span`]: ratatui_core::text::Span
[`vec!`]: alloc::vec!

```rust
macro_rules! text {
    () => { ... };
    ($line:expr; $n:expr) => { ... };
    ($($line:expr),+ $(,)?) => { ... };
}
```



## ratatui_macros::vertical

*Declarative Macro*

Creates a vertical layout with specified constraints.

It accepts a series of constraints and applies them to create a vertical layout. The constraints
can include fixed sizes, minimum and maximum sizes, percentages, and ratios.

See [`constraint!`]  or [`constraints!`] for more information.

# Examples

```
// Vertical layout with a fixed size and a percentage constraint
use ratatui_macros::vertical;
vertical![== 50, == 30%];
```

```rust
macro_rules! vertical {
    ($( $constraint:tt )+) => { ... };
}
```



