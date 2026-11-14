**crossterm > cursor**

# Module: cursor

## Contents

**Structs**

- [`DisableBlinking`](#disableblinking) - A command that disables blinking of the terminal cursor.
- [`EnableBlinking`](#enableblinking) - A command that enables blinking of the terminal cursor.
- [`Hide`](#hide) - A command that hides the terminal cursor.
- [`MoveDown`](#movedown) - A command that moves the terminal cursor a given number of rows down.
- [`MoveLeft`](#moveleft) - A command that moves the terminal cursor a given number of columns to the left.
- [`MoveRight`](#moveright) - A command that moves the terminal cursor a given number of columns to the right.
- [`MoveTo`](#moveto) - A command that moves the terminal cursor to the given position (column, row).
- [`MoveToColumn`](#movetocolumn) - A command that moves the terminal cursor to the given column on the current row.
- [`MoveToNextLine`](#movetonextline) - A command that moves the terminal cursor down the given number of lines,
- [`MoveToPreviousLine`](#movetopreviousline) - A command that moves the terminal cursor up the given number of lines,
- [`MoveToRow`](#movetorow) - A command that moves the terminal cursor to the given row on the current column.
- [`MoveUp`](#moveup) - A command that moves the terminal cursor a given number of rows up.
- [`RestorePosition`](#restoreposition) - A command that restores the saved terminal cursor position.
- [`SavePosition`](#saveposition) - A command that saves the current terminal cursor position.
- [`Show`](#show) - A command that shows the terminal cursor.

**Enums**

- [`SetCursorStyle`](#setcursorstyle) - A command that sets the style of the cursor.

---

## crossterm::cursor::DisableBlinking

*Struct*

A command that disables blinking of the terminal cursor.

# Notes

- Some Unix terminals (ex: GNOME and Konsole) as well as Windows versions lower than Windows 10 do not support this functionality.
  Use `SetCursorStyle` for better cross-compatibility.
- Commands must be executed/queued for execution otherwise they do nothing.

**Unit Struct**

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



## crossterm::cursor::EnableBlinking

*Struct*

A command that enables blinking of the terminal cursor.

# Notes

- Some Unix terminals (ex: GNOME and Konsole) as well as Windows versions lower than Windows 10 do not support this functionality.
  Use `SetCursorStyle` for better cross-compatibility.
- Commands must be executed/queued for execution otherwise they do nothing.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::cursor::Hide

*Struct*

A command that hides the terminal cursor.

# Notes

- Commands must be executed/queued for execution otherwise they do nothing.

**Unit Struct**

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



## crossterm::cursor::MoveDown

*Struct*

A command that moves the terminal cursor a given number of rows down.

# Notes
* This command is 1 based, meaning `MoveDown(1)` moves the cursor down one cell.
* Most terminals default 0 argument to 1.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

**Traits:** Copy, Eq

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



## crossterm::cursor::MoveLeft

*Struct*

A command that moves the terminal cursor a given number of columns to the left.

# Notes
* This command is 1 based, meaning `MoveLeft(1)` moves the cursor left one cell.
* Most terminals default 0 argument to 1.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`



## crossterm::cursor::MoveRight

*Struct*

A command that moves the terminal cursor a given number of columns to the right.

# Notes
* This command is 1 based, meaning `MoveRight(1)` moves the cursor right one cell.
* Most terminals default 0 argument to 1.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

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



## crossterm::cursor::MoveTo

*Struct*

A command that moves the terminal cursor to the given position (column, row).

# Notes
* Top left cell is represented as `0,0`.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16, u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## crossterm::cursor::MoveToColumn

*Struct*

A command that moves the terminal cursor to the given column on the current row.

# Notes
* This command is 0 based, meaning 0 is the leftmost column.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::cursor::MoveToNextLine

*Struct*

A command that moves the terminal cursor down the given number of lines,
and moves it to the first column.

# Notes
* This command is 1 based, meaning `MoveToNextLine(1)` moves to the next line.
* Most terminals default 0 argument to 1.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`



## crossterm::cursor::MoveToPreviousLine

*Struct*

A command that moves the terminal cursor up the given number of lines,
and moves it to the first column.

# Notes
* This command is 1 based, meaning `MoveToPreviousLine(1)` moves to the previous line.
* Most terminals default 0 argument to 1.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

**Traits:** Copy, Eq

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



## crossterm::cursor::MoveToRow

*Struct*

A command that moves the terminal cursor to the given row on the current column.

# Notes
* This command is 0 based, meaning 0 is the topmost row.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

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



## crossterm::cursor::MoveUp

*Struct*

A command that moves the terminal cursor a given number of rows up.

# Notes
* This command is 1 based, meaning `MoveUp(1)` moves the cursor up one cell.
* Most terminals default 0 argument to 1.
* Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16)`

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



## crossterm::cursor::RestorePosition

*Struct*

A command that restores the saved terminal cursor position.

See the [SavePosition](./struct.SavePosition.html) command.

# Notes

- The cursor position is stored globally.
- Commands must be executed/queued for execution otherwise they do nothing.

**Unit Struct**

**Traits:** Copy, Eq

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



## crossterm::cursor::SavePosition

*Struct*

A command that saves the current terminal cursor position.

See the [RestorePosition](./struct.RestorePosition.html) command.

# Notes

- The cursor position is stored globally.
- Commands must be executed/queued for execution otherwise they do nothing.

**Unit Struct**

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



## crossterm::cursor::SetCursorStyle

*Enum*

A command that sets the style of the cursor.
It uses two types of escape codes, one to control blinking, and the other the shape.

# Note

- Commands must be executed/queued for execution otherwise they do nothing.

**Variants:**
- `DefaultUserShape` - Default cursor shape configured by the user.
- `BlinkingBlock` - A blinking block cursor shape (■).
- `SteadyBlock` - A non blinking block cursor shape (inverse of `BlinkingBlock`).
- `BlinkingUnderScore` - A blinking underscore cursor shape(_).
- `SteadyUnderScore` - A non blinking underscore cursor shape (inverse of `BlinkingUnderScore`).
- `BlinkingBar` - A blinking cursor bar shape (|)
- `SteadyBar` - A steady cursor bar shape (inverse of `BlinkingBar`).

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> ::std::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::cursor::Show

*Struct*

A command that shows the terminal cursor.

# Notes

- Commands must be executed/queued for execution otherwise they do nothing.

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



