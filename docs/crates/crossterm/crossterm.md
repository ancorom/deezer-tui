**crossterm**

# Module: crossterm

## Contents

**Modules**

- [`cursor`](#cursor) - A module to work with the terminal cursor
- [`event`](#event) - A module to read events.
- [`style`](#style) - A module to apply attributes and colors on your text.
- [`terminal`](#terminal) - A module to work with the terminal.
- [`tty`](#tty) - A module to query if the current instance is a tty.

**Macros**

- [`execute`](#execute) - Executes one or more command(s).
- [`queue`](#queue) - Queues one or more command(s) for further execution.

---

## Module: cursor

A module to work with the terminal cursor
# Cursor

The `cursor` module provides functionality to work with the terminal cursor.

This documentation does not contain a lot of examples. The reason is that it's fairly
obvious how to use this crate. Although, we do provide
[examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) repository
to demonstrate the capabilities.

## Examples

Cursor actions can be performed with commands.
Please have a look at [command documentation](../index.html#command-api) for a more detailed documentation.

```no_run
use std::io::{self, Write};

use crossterm::{
    ExecutableCommand, execute,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}
};

fn main() -> io::Result<()> {
    // with macro
    execute!(
        io::stdout(),
        SavePosition,
        MoveTo(10, 10),
        EnableBlinking,
        DisableBlinking,
        RestorePosition
    );

  // with function
  io::stdout()
    .execute(MoveTo(11,11))?
    .execute(RestorePosition);

 Ok(())
}
```

For manual execution control check out [crossterm::queue](../macro.queue.html).



## Module: event

A module to read events.
# Event

The `event` module provides the functionality to read keyboard, mouse and terminal resize events.

* The [`read`](fn.read.html) function returns an [`Event`](enum.Event.html) immediately
(if available) or blocks until an [`Event`](enum.Event.html) is available.

* The [`poll`](fn.poll.html) function allows you to check if there is or isn't an [`Event`](enum.Event.html) available
within the given period of time. In other words - if subsequent call to the [`read`](fn.read.html)
function will block or not.

It's **not allowed** to call these functions from different threads or combine them with the
[`EventStream`](struct.EventStream.html). You're allowed to either:

* use the [`read`](fn.read.html) & [`poll`](fn.poll.html) functions on any, but same, thread
* or the [`EventStream`](struct.EventStream.html).

**Make sure to enable [raw mode](../terminal/index.html#raw-mode) in order for keyboard events to work properly**

## Mouse and Focus Events

Mouse and focus events are not enabled by default. You have to enable them with the
[`EnableMouseCapture`](struct.EnableMouseCapture.html) / [`EnableFocusChange`](struct.EnableFocusChange.html) command.
See [Command API](../index.html#command-api) for more information.

## Examples

Blocking read:

```no_run
#![cfg(feature = "bracketed-paste")]
use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};

fn print_events() -> std::io::Result<()> {
    execute!(
         std::io::stdout(),
         EnableBracketedPaste,
         EnableFocusChange,
         EnableMouseCapture
    )?;
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
    execute!(
        std::io::stdout(),
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture
    )?;
    Ok(())
}
```

Non-blocking read:

```no_run
#![cfg(feature = "bracketed-paste")]
use std::{time::Duration, io};

use crossterm::{
    event::{
        poll, read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
        EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};

fn print_events() -> io::Result<()> {
    execute!(
         std::io::stdout(),
         EnableBracketedPaste,
         EnableFocusChange,
         EnableMouseCapture
    )?;
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                #[cfg(feature = "bracketed-paste")]
                Event::Paste(data) => println!("Pasted {:?}", data),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    execute!(
        std::io::stdout(),
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture
    )?;
    Ok(())
}
```

Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder for more of
them (`event-*`).



## crossterm::execute

*Declarative Macro*

Executes one or more command(s).

# Arguments

- [std::io::Writer](std::io::Write)

  ANSI escape codes are written on the given 'writer', after which they are flushed.

- [Command](./trait.Command.html)

  One or more commands

# Examples

```rust
use std::io::{Write, stdout};
use crossterm::{execute, style::Print};

// will be executed directly
execute!(stdout(), Print("sum:\n".to_string()));

// will be executed directly
execute!(stdout(), Print("1 + 1 = ".to_string()), Print((1+1).to_string()));

// ==== Output ====
// sum:
// 1 + 1 = 2
```

Have a look over at the [Command API](./index.html#command-api) for more details.

# Notes

* In the case of UNIX and Windows 10, ANSI codes are written to the given 'writer'.
* In case of Windows versions lower than 10, a direct WinAPI call will be made.
  The reason for this is that Windows versions lower than 10 do not support ANSI codes,
  and can therefore not be written to the given `writer`.
  Therefore, there is no difference between [execute](macro.execute.html)
  and [queue](macro.queue.html) for those old Windows versions.

```rust
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => { ... };
}
```



## crossterm::queue

*Declarative Macro*

Queues one or more command(s) for further execution.

Queued commands must be flushed to the underlying device to be executed.
This generally happens in the following cases:

* When `flush` is called manually on the given type implementing `io::Write`.
* The terminal will `flush` automatically if the buffer is full.
* Each line is flushed in case of `stdout`, because it is line buffered.

# Arguments

- [std::io::Writer](std::io::Write)

    ANSI escape codes are written on the given 'writer', after which they are flushed.

- [Command](./trait.Command.html)

    One or more commands

# Examples

```rust
use std::io::{Write, stdout};
use crossterm::{queue, style::Print};

let mut stdout = stdout();

// `Print` will executed executed when `flush` is called.
queue!(stdout, Print("foo".to_string()));

// some other code (no execution happening here) ...

// when calling `flush` on `stdout`, all commands will be written to the stdout and therefore executed.
stdout.flush();

// ==== Output ====
// foo
```

Have a look over at the [Command API](./index.html#command-api) for more details.

# Notes

In case of Windows versions lower than 10, a direct WinAPI call will be made.
The reason for this is that Windows versions lower than 10 do not support ANSI codes,
and can therefore not be written to the given `writer`.
Therefore, there is no difference between [execute](macro.execute.html)
and [queue](macro.queue.html) for those old Windows versions.


```rust
macro_rules! queue {
    ($writer:expr $(, $command:expr)* $(,)?) => { ... };
}
```



## Module: style

A module to apply attributes and colors on your text.
# Style

The `style` module provides a functionality to apply attributes and colors on your text.

This documentation does not contain a lot of examples. The reason is that it's fairly
obvious how to use this crate. Although, we do provide
[examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) repository
to demonstrate the capabilities.

## Platform-specific Notes

Not all features are supported on all terminals/platforms. You should always consult
platform-specific notes of the following types:

* [Color](enum.Color.html#platform-specific-notes)
* [Attribute](enum.Attribute.html#platform-specific-notes)

## Examples

A few examples of how to use the style module.

### Colors

How to change the terminal text color.

Command API:

Using the Command API to color text.

```no_run
use std::io::{self, Write};
use crossterm::execute;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};

fn main() -> io::Result<()> {
    execute!(
        io::stdout(),
        // Blue foreground
        SetForegroundColor(Color::Blue),
        // Red background
        SetBackgroundColor(Color::Red),
        // Print text
        Print("Blue text on Red.".to_string()),
        // Reset to default colors
        ResetColor
    )
}
```

Functions:

Using functions from [`Stylize`](crate::style::Stylize) on a `String` or `&'static str` to color
it.

```no_run
use crossterm::style::Stylize;

println!("{}", "Red foreground color & blue background.".red().on_blue());
```

### Attributes

How to apply terminal attributes to text.

Command API:

Using the Command API to set attributes.

```no_run
use std::io::{self, Write};

use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute};

fn main() -> io::Result<()> {
    execute!(
        io::stdout(),
        // Set to bold
        SetAttribute(Attribute::Bold),
        Print("Bold text here.".to_string()),
        // Reset all attributes
        SetAttribute(Attribute::Reset)
    )
}
```

Functions:

Using [`Stylize`](crate::style::Stylize) functions on a `String` or `&'static str` to set
attributes to it.

```no_run
use crossterm::style::Stylize;

println!("{}", "Bold".bold());
println!("{}", "Underlined".underlined());
println!("{}", "Negative".negative());
```

Displayable:

[`Attribute`](enum.Attribute.html) implements [Display](https://doc.rust-lang.org/beta/std/fmt/trait.Display.html) and therefore it can be formatted like:

```no_run
use crossterm::style::Attribute;

println!(
    "{} Underlined {} No Underline",
    Attribute::Underlined,
    Attribute::NoUnderline
);
```



## Module: terminal

A module to work with the terminal.
# Terminal

The `terminal` module provides functionality to work with the terminal.

This documentation does not contain a lot of examples. The reason is that it's fairly
obvious how to use this crate. Although, we do provide
[examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) repository
to demonstrate the capabilities.

Most terminal actions can be performed with commands.
Please have a look at [command documentation](../index.html#command-api) for a more detailed documentation.

## Screen Buffer

A screen buffer is a two-dimensional array of character
and color data which is displayed in a terminal screen.

The terminal has several of those buffers and is able to switch between them.
The default screen in which you work is called the 'main screen'.
The other screens are called the 'alternative screen'.

It is important to understand that crossterm does not yet support creating screens,
or switch between more than two buffers, and only offers the ability to change
between the 'alternate' and 'main screen'.

### Alternate Screen

By default, you will be working on the main screen.
There is also another screen called the 'alternative' screen.
This screen is slightly different from the main screen.
For example, it has the exact dimensions of the terminal window,
without any scroll-back area.

Crossterm offers the possibility to switch to the 'alternative' screen,
make some modifications, and move back to the 'main' screen again.
The main screen will stay intact and will have the original data as we performed all
operations on the alternative screen.

An good example of this is Vim.
When it is launched from bash, a whole new buffer is used to modify a file.
Then, when the modification is finished, it closes again and continues on the main screen.

### Raw Mode

By default, the terminal functions in a certain way.
For example, it will move the cursor to the beginning of the next line when the input hits the end of a line.
Or that the backspace is interpreted for character removal.

Sometimes these default modes are irrelevant,
and in this case, we can turn them off.
This is what happens when you enable raw modes.

Those modes will be set when enabling raw modes:

- Input will not be forwarded to screen
- Input will not be processed on enter press
- Input will not be line buffered (input sent byte-by-byte to input buffer)
- Special keys like backspace and CTRL+C will not be processed by terminal driver
- New line character will not be processed therefore `println!` can't be used, use `write!` instead

Raw mode can be enabled/disabled with the [enable_raw_mode](terminal::enable_raw_mode) and [disable_raw_mode](terminal::disable_raw_mode) functions.

## Examples

```no_run
use std::io::{self, Write};
use crossterm::{execute, terminal::{ScrollUp, SetSize, size}};

fn main() -> io::Result<()> {
    let (cols, rows) = size()?;
    // Resize terminal and scroll up.
    execute!(
        io::stdout(),
        SetSize(10, 10),
        ScrollUp(5)
    )?;

    // Be a good citizen, cleanup
    execute!(io::stdout(), SetSize(cols, rows))?;
    Ok(())
}
```

For manual execution control check out [crossterm::queue](../macro.queue.html).



## Module: tty

A module to query if the current instance is a tty.
Making it a little more convenient and safe to query whether
something is a terminal teletype or not.
This module defines the IsTty trait and the is_tty method to
return true if the item represents a terminal.



