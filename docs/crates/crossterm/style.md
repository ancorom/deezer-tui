**crossterm > style**

# Module: style

## Contents

**Structs**

- [`Print`](#print) - A command that prints the given displayable type.
- [`PrintStyledContent`](#printstyledcontent) - A command that prints styled content.
- [`ResetColor`](#resetcolor) - A command that resets the colors back to default.
- [`SetAttribute`](#setattribute) - A command that sets an attribute.
- [`SetAttributes`](#setattributes) - A command that sets several attributes.
- [`SetBackgroundColor`](#setbackgroundcolor) - A command that sets the the background color.
- [`SetColors`](#setcolors) - A command that optionally sets the foreground and/or background color.
- [`SetForegroundColor`](#setforegroundcolor) - A command that sets the the foreground color.
- [`SetStyle`](#setstyle) - A command that sets a style (colors and attributes).
- [`SetUnderlineColor`](#setunderlinecolor) - A command that sets the the underline color.

**Functions**

- [`available_color_count`](#available_color_count) - Returns available color count.
- [`force_color_output`](#force_color_output) - Forces colored output on or off globally, overriding NO_COLOR.
- [`style`](#style) - Creates a `StyledContent`.

---

## crossterm::style::Print

*Struct*

A command that prints the given displayable type.

Commands must be executed/queued for execution otherwise they do nothing.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::style::PrintStyledContent

*Struct*

A command that prints styled content.

See [`StyledContent`](struct.StyledContent.html) for more info.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Generic Parameters:**
- D

**Tuple Struct**: `(StyledContent<D>)`

**Traits:** Copy

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`



## crossterm::style::ResetColor

*Struct*

A command that resets the colors back to default.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## crossterm::style::SetAttribute

*Struct*

A command that sets an attribute.

See [`Attribute`](enum.Attribute.html) for more info.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(Attribute)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## crossterm::style::SetAttributes

*Struct*

A command that sets several attributes.

See [`Attributes`](struct.Attributes.html) for more info.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(Attributes)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::style::SetBackgroundColor

*Struct*

A command that sets the the background color.

See [`Color`](enum.Color.html) for more info.

[`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
color with one command.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(Color)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## crossterm::style::SetColors

*Struct*

A command that optionally sets the foreground and/or background color.

For example:
```no_run
use std::io::{stdout, Write};

use crossterm::execute;
use crossterm::style::{Color::{Green, Black}, Colors, Print, SetColors};

execute!(
    stdout(),
    SetColors(Colors::new(Green, Black)),
    Print("Hello, world!".to_string()),
).unwrap();
```

See [`Colors`](struct.Colors.html) for more info.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(Colors)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## crossterm::style::SetForegroundColor

*Struct*

A command that sets the the foreground color.

See [`Color`](enum.Color.html) for more info.

[`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
color in one command.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(Color)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::style::SetStyle

*Struct*

A command that sets a style (colors and attributes).

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(ContentStyle)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::style::SetUnderlineColor

*Struct*

A command that sets the the underline color.

See [`Color`](enum.Color.html) for more info.

[`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
color with one command.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(Color)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::style::available_color_count

*Function*

Returns available color count.

# Notes

This does not always provide a good result.

```rust
fn available_color_count() -> u16
```



## crossterm::style::force_color_output

*Function*

Forces colored output on or off globally, overriding NO_COLOR.

# Notes

crossterm supports NO_COLOR (<https://no-color.org/>) to disabled colored output.

This API allows applications to override that behavior and force colorized output
even if NO_COLOR is set.

```rust
fn force_color_output(enabled: bool)
```



## crossterm::style::style

*Function*

Creates a `StyledContent`.

This could be used to style any type that implements `Display` with colors and text attributes.

See [`StyledContent`](struct.StyledContent.html) for more info.

# Examples

```no_run
use crossterm::style::{style, Stylize, Color};

let styled_content = style("Blue colored text on yellow background")
    .with(Color::Blue)
    .on(Color::Yellow);

println!("{}", styled_content);
```

```rust
fn style<D>(val: D) -> StyledContent<D>
```



