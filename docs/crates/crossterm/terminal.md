**crossterm > terminal**

# Module: terminal

## Contents

**Structs**

- [`BeginSynchronizedUpdate`](#beginsynchronizedupdate) - A command that instructs the terminal emulator to begin a synchronized frame.
- [`Clear`](#clear) - A command that clears the terminal screen buffer.
- [`DisableLineWrap`](#disablelinewrap) - Disables line wrapping.
- [`EnableLineWrap`](#enablelinewrap) - Enable line wrapping.
- [`EndSynchronizedUpdate`](#endsynchronizedupdate) - A command that instructs the terminal to end a synchronized frame.
- [`EnterAlternateScreen`](#enteralternatescreen) - A command that switches to alternate screen.
- [`LeaveAlternateScreen`](#leavealternatescreen) - A command that switches back to the main screen.
- [`ScrollDown`](#scrolldown) - A command that scrolls the terminal screen a given number of rows down.
- [`ScrollUp`](#scrollup) - A command that scrolls the terminal screen a given number of rows up.
- [`SetSize`](#setsize) - A command that sets the terminal buffer size `(columns, rows)`.
- [`SetTitle`](#settitle) - A command that sets the terminal title
- [`WindowSize`](#windowsize)

**Enums**

- [`ClearType`](#cleartype) - Different ways to clear the terminal buffer.

**Functions**

- [`disable_raw_mode`](#disable_raw_mode) - Disables raw mode.
- [`enable_raw_mode`](#enable_raw_mode) - Enables raw mode.
- [`is_raw_mode_enabled`](#is_raw_mode_enabled) - Tells whether the raw mode is enabled.
- [`size`](#size) - Returns the terminal size `(columns, rows)`.
- [`window_size`](#window_size) - Returns the terminal size `[WindowSize]`.

---

## crossterm::terminal::BeginSynchronizedUpdate

*Struct*

A command that instructs the terminal emulator to begin a synchronized frame.

# Notes

* Commands must be executed/queued for execution otherwise they do nothing.
* Use [EndSynchronizedUpdate](./struct.EndSynchronizedUpdate.html) command to leave the entered alternate screen.

When rendering the screen of the terminal, the Emulator usually iterates through each visible grid cell and
renders its current state. With applications updating the screen at a higher frequency this can cause tearing.

This mode attempts to mitigate that.

When the synchronization mode is enabled following render calls will keep rendering the last rendered state.
The terminal Emulator keeps processing incoming text and sequences. When the synchronized update mode is disabled
again the renderer may fetch the latest screen buffer state again, effectively avoiding the tearing effect
by unintentionally rendering in the middle a of an application screen update.

# Examples

```no_run
use std::io::{self, Write};
use crossterm::{execute, terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate}};

fn main() -> io::Result<()> {
    execute!(io::stdout(), BeginSynchronizedUpdate)?;

    // Anything performed here will not be rendered until EndSynchronizedUpdate is called.

    execute!(io::stdout(), EndSynchronizedUpdate)?;
    Ok(())
}
```


**Unit Struct**

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



## crossterm::terminal::Clear

*Struct*

A command that clears the terminal screen buffer.

See the [`ClearType`](enum.ClearType.html) enum.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(ClearType)`

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



## crossterm::terminal::ClearType

*Enum*

Different ways to clear the terminal buffer.

**Variants:**
- `All` - All cells.
- `Purge` - All plus history
- `FromCursorDown` - All cells from the cursor position downwards.
- `FromCursorUp` - All cells from the cursor position upwards.
- `CurrentLine` - All cells at the cursor row.
- `UntilNewLine` - All cells from the cursor position until the new line.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::terminal::DisableLineWrap

*Struct*

Disables line wrapping.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## crossterm::terminal::EnableLineWrap

*Struct*

Enable line wrapping.

**Unit Struct**

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



## crossterm::terminal::EndSynchronizedUpdate

*Struct*

A command that instructs the terminal to end a synchronized frame.

# Notes

* Commands must be executed/queued for execution otherwise they do nothing.
* Use [BeginSynchronizedUpdate](./struct.BeginSynchronizedUpdate.html) to enter the alternate screen.

When rendering the screen of the terminal, the Emulator usually iterates through each visible grid cell and
renders its current state. With applications updating the screen a at higher frequency this can cause tearing.

This mode attempts to mitigate that.

When the synchronization mode is enabled following render calls will keep rendering the last rendered state.
The terminal Emulator keeps processing incoming text and sequences. When the synchronized update mode is disabled
again the renderer may fetch the latest screen buffer state again, effectively avoiding the tearing effect
by unintentionally rendering in the middle a of an application screen update.

# Examples

```no_run
use std::io::{self, Write};
use crossterm::{execute, terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate}};

fn main() -> io::Result<()> {
    execute!(io::stdout(), BeginSynchronizedUpdate)?;

    // Anything performed here will not be rendered until EndSynchronizedUpdate is called.

    execute!(io::stdout(), EndSynchronizedUpdate)?;
    Ok(())
}
```


**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## crossterm::terminal::EnterAlternateScreen

*Struct*

A command that switches to alternate screen.

# Notes

* Commands must be executed/queued for execution otherwise they do nothing.
* Use [LeaveAlternateScreen](./struct.LeaveAlternateScreen.html) command to leave the entered alternate screen.

# Examples

```no_run
use std::io::{self, Write};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;

    // Do anything on the alternate screen

    execute!(io::stdout(), LeaveAlternateScreen)
}
```


**Unit Struct**

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



## crossterm::terminal::LeaveAlternateScreen

*Struct*

A command that switches back to the main screen.

# Notes

* Commands must be executed/queued for execution otherwise they do nothing.
* Use [EnterAlternateScreen](./struct.EnterAlternateScreen.html) to enter the alternate screen.

# Examples

```no_run
use std::io::{self, Write};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;

    // Do anything on the alternate screen

    execute!(io::stdout(), LeaveAlternateScreen)
}
```


**Unit Struct**

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



## crossterm::terminal::ScrollDown

*Struct*

A command that scrolls the terminal screen a given number of rows down.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

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



## crossterm::terminal::ScrollUp

*Struct*

A command that scrolls the terminal screen a given number of rows up.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

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



## crossterm::terminal::SetSize

*Struct*

A command that sets the terminal buffer size `(columns, rows)`.

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Tuple Struct**: `(u16, u16)`

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



## crossterm::terminal::SetTitle

*Struct*

A command that sets the terminal title

# Notes

Commands must be executed/queued for execution otherwise they do nothing.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

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



## crossterm::terminal::WindowSize

*Struct*

**Fields:**
- `rows: u16`
- `columns: u16`
- `width: u16`
- `height: u16`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(size: Winsize) -> WindowSize`



## crossterm::terminal::disable_raw_mode

*Function*

Disables raw mode.

Please have a look at the [raw mode](./index.html#raw-mode) section.

```rust
fn disable_raw_mode() -> io::Result<()>
```



## crossterm::terminal::enable_raw_mode

*Function*

Enables raw mode.

Please have a look at the [raw mode](./index.html#raw-mode) section.

```rust
fn enable_raw_mode() -> io::Result<()>
```



## crossterm::terminal::is_raw_mode_enabled

*Function*

Tells whether the raw mode is enabled.

Please have a look at the [raw mode](./index.html#raw-mode) section.

```rust
fn is_raw_mode_enabled() -> io::Result<bool>
```



## crossterm::terminal::size

*Function*

Returns the terminal size `(columns, rows)`.

The top left cell is represented `(1, 1)`.

```rust
fn size() -> io::Result<(u16, u16)>
```



## crossterm::terminal::window_size

*Function*

Returns the terminal size `[WindowSize]`.

The width and height in pixels may not be reliably implemented or default to 0.
For unix, <https://man7.org/linux/man-pages/man4/tty_ioctl.4.html> documents them as "unused".
For windows it is not implemented.

```rust
fn window_size() -> io::Result<WindowSize>
```



