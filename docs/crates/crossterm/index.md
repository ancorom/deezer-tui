# crossterm

 # Cross-platform Terminal Manipulation Library

 Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces.

 This crate supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested
 see [Tested Terminals](https://github.com/crossterm-rs/crossterm#tested-terminals)
 for more info).

 ## Command API

 The command API makes the use of `crossterm` much easier and offers more control over when and how a
 command is executed. A command is just an action you can perform on the terminal e.g. cursor movement.

 The command API offers:

 * Better Performance.
 * Complete control over when to flush.
 * Complete control over where the ANSI escape commands are executed to.
 * Way easier and nicer API.

 There are two ways to use the API command:

 * Functions can execute commands on types that implement Write. Functions are easier to use and debug.
   There is a disadvantage, and that is that there is a boilerplate code involved.
 * Macros are generally seen as more difficult and aren't always well supported by editors but offer an API with less boilerplate code. If you are
   not afraid of macros, this is a recommendation.

 Linux and Windows 10 systems support ANSI escape codes. Those ANSI escape codes are strings or rather a
 byte sequence. When we `write` and `flush` those to the terminal we can perform some action.
 For older windows systems a WinAPI call is made.

 ### Supported Commands

 - Module [`cursor`](cursor/index.html)
   - Visibility - [`Show`](cursor/struct.Show.html), [`Hide`](cursor/struct.Hide.html)
   - Appearance - [`EnableBlinking`](cursor/struct.EnableBlinking.html),
     [`DisableBlinking`](cursor/struct.DisableBlinking.html),
     [`SetCursorStyle`](cursor/enum.SetCursorStyle.html)
   - Position -
     [`SavePosition`](cursor/struct.SavePosition.html), [`RestorePosition`](cursor/struct.RestorePosition.html),
     [`MoveUp`](cursor/struct.MoveUp.html), [`MoveDown`](cursor/struct.MoveDown.html),
     [`MoveLeft`](cursor/struct.MoveLeft.html), [`MoveRight`](cursor/struct.MoveRight.html),
     [`MoveTo`](cursor/struct.MoveTo.html), [`MoveToColumn`](cursor/struct.MoveToColumn.html),[`MoveToRow`](cursor/struct.MoveToRow.html),
     [`MoveToNextLine`](cursor/struct.MoveToNextLine.html), [`MoveToPreviousLine`](cursor/struct.MoveToPreviousLine.html)
 - Module [`event`](event/index.html)
   - Keyboard events -
     [`PushKeyboardEnhancementFlags`](event/struct.PushKeyboardEnhancementFlags.html),
     [`PopKeyboardEnhancementFlags`](event/struct.PopKeyboardEnhancementFlags.html)
   - Mouse events - [`EnableMouseCapture`](event/struct.EnableMouseCapture.html),
     [`DisableMouseCapture`](event/struct.DisableMouseCapture.html)
 - Module [`style`](style/index.html)
   - Colors - [`SetForegroundColor`](style/struct.SetForegroundColor.html),
     [`SetBackgroundColor`](style/struct.SetBackgroundColor.html),
     [`ResetColor`](style/struct.ResetColor.html), [`SetColors`](style/struct.SetColors.html)
   - Attributes - [`SetAttribute`](style/struct.SetAttribute.html), [`SetAttributes`](style/struct.SetAttributes.html),
     [`PrintStyledContent`](style/struct.PrintStyledContent.html)
 - Module [`terminal`](terminal/index.html)
   - Scrolling - [`ScrollUp`](terminal/struct.ScrollUp.html),
     [`ScrollDown`](terminal/struct.ScrollDown.html)
   - Miscellaneous - [`Clear`](terminal/struct.Clear.html),
     [`SetSize`](terminal/struct.SetSize.html),
     [`SetTitle`](terminal/struct.SetTitle.html),
     [`DisableLineWrap`](terminal/struct.DisableLineWrap.html),
     [`EnableLineWrap`](terminal/struct.EnableLineWrap.html)
   - Alternate screen - [`EnterAlternateScreen`](terminal/struct.EnterAlternateScreen.html),
     [`LeaveAlternateScreen`](terminal/struct.LeaveAlternateScreen.html)
 - Module [`clipboard`](clipboard/index.html) (requires
     [`feature = "osc52"`](#optional-features))
   - Clipboard - [`CopyToClipboard`](clipboard/struct.CopyToClipboard.html)

 ### Command Execution

 There are two different ways to execute commands:

 * [Lazy Execution](#lazy-execution)
 * [Direct Execution](#direct-execution)

 #### Lazy Execution

 Flushing bytes to the terminal buffer is a heavy system call. If we perform a lot of actions with the terminal,
 we want to do this periodically - like with a TUI editor - so that we can flush more data to the terminal buffer
 at the same time.

 Crossterm offers the possibility to do this with `queue`.
 With `queue` you can queue commands, and when you call [Write::flush][flush] these commands will be executed.

 You can pass a custom buffer implementing [std::io::Write][write] to this `queue` operation.
 The commands will be executed on that buffer.
 The most common buffer is [std::io::stdout][stdout] however, [std::io::stderr][stderr] is used sometimes as well.

 ##### Examples

 A simple demonstration that shows the command API in action with cursor commands.

 Functions:

 ```no_run
 use std::io::{Write, stdout};
 use crossterm::{QueueableCommand, cursor};

 let mut stdout = stdout();
 stdout.queue(cursor::MoveTo(5,5));

 // some other code ...

 stdout.flush();
 ```

 The [queue](./trait.QueueableCommand.html) function returns itself, therefore you can use this to queue another
 command. Like `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.

 Macros:

 ```no_run
 use std::io::{Write, stdout};
 use crossterm::{queue, QueueableCommand, cursor};

 let mut stdout = stdout();
 queue!(stdout,  cursor::MoveTo(5, 5));

 // some other code ...

 // move operation is performed only if we flush the buffer.
 stdout.flush();
 ```

 You can pass more than one command into the [queue](./macro.queue.html) macro like
 `queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and
 they will be executed in the given order from left to right.

 #### Direct Execution

 For many applications it is not at all important to be efficient with 'flush' operations.
 For this use case there is the `execute` operation.
 This operation executes the command immediately, and calls the `flush` under water.

 You can pass a custom buffer implementing [std::io::Write][write] to this `execute` operation.
 The commands will be executed on that buffer.
 The most common buffer is [std::io::stdout][stdout] however, [std::io::stderr][stderr] is used sometimes as well.

 ##### Examples

 Functions:

 ```no_run
 use std::io::{Write, stdout};
 use crossterm::{ExecutableCommand, cursor};

 let mut stdout = stdout();
 stdout.execute(cursor::MoveTo(5,5));
 ```
 The [execute](./trait.ExecutableCommand.html) function returns itself, therefore you can use this to queue
 another command. Like `stdout.execute(Goto(5,5))?.execute(Clear(ClearType::All))`.

 Macros:

 ```no_run
 use std::io::{stdout, Write};
 use crossterm::{execute, ExecutableCommand, cursor};

 let mut stdout = stdout();
 execute!(stdout, cursor::MoveTo(5, 5));
 ```
 You can pass more than one command into the [execute](./macro.execute.html) macro like
 `execute!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and they will be executed in the given order from
 left to right.

 ## Examples

 Print a rectangle colored with magenta and use both direct execution and lazy execution.

 Functions:

 ```no_run
 use std::io::{self, Write};
 use crossterm::{
     ExecutableCommand, QueueableCommand,
     terminal, cursor, style::{self, Stylize}
 };

 fn main() -> io::Result<()> {
   let mut stdout = io::stdout();

   stdout.execute(terminal::Clear(terminal::ClearType::All))?;

   for y in 0..40 {
     for x in 0..150 {
       if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
         // in this loop we are more efficient by not flushing the buffer.
         stdout
           .queue(cursor::MoveTo(x,y))?
           .queue(style::PrintStyledContent( "█".magenta()))?;
       }
     }
   }
   stdout.flush()?;
   Ok(())
 }
 ```

 Macros:

 ```no_run
 use std::io::{self, Write};
 use crossterm::{
     execute, queue,
     style::{self, Stylize}, cursor, terminal
 };

 fn main() -> io::Result<()> {
   let mut stdout = io::stdout();

   execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

   for y in 0..40 {
     for x in 0..150 {
       if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
         // in this loop we are more efficient by not flushing the buffer.
         queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
       }
     }
   }
   stdout.flush()?;
   Ok(())
 }
```
 ## Feature Flags
 ### Default features
* **`bracketed-paste`** *(enabled by default)* —  Enables triggering [`Event::Paste`](event::Event::Paste) when pasting text into the terminal.
* **`events`** *(enabled by default)* —  Enables reading input/events from the system using the [`event`] module.
* **`windows`** *(enabled by default)* —  Enables windows specific crates.

 ### Optional Features
* **`event-stream`** —  Enables the [EventStream](event::EventStream) struct for async event reading.
* **`serde`** —  Enables [`serde`] for various types.
* **`use-dev-tty`** —  Enables raw file descriptor polling / selecting instead of mio.
* **`derive-more`** *(enabled by default)* —  Enables `is_*` helper functions for event enums.
* **`osc52`** —  Enables interacting with a host clipboard via [`clipboard`](clipboard/index.html)

 [write]: https://doc.rust-lang.org/std/io/trait.Write.html
 [stdout]: https://doc.rust-lang.org/std/io/fn.stdout.html
 [stderr]: https://doc.rust-lang.org/std/io/fn.stderr.html
 [flush]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush

## Modules

### [`crossterm`](crossterm.md)

*2 macros, 5 modules*

### [`command`](command.md)

*4 traits*

### [`cursor`](cursor.md)

*1 enum, 15 structs*

### [`cursor::sys::unix`](cursor/sys/unix.md)

*1 function*

### [`event`](event.md)

*13 structs, 2 functions, 7 enums*

### [`style`](style.md)

*10 structs, 3 functions*

### [`style::attributes`](style/attributes.md)

*1 struct*

### [`style::content_style`](style/content_style.md)

*1 struct*

### [`style::styled_content`](style/styled_content.md)

*1 struct*

### [`style::stylize`](style/stylize.md)

*1 trait*

### [`style::types::attribute`](style/types/attribute.md)

*1 enum*

### [`style::types::color`](style/types/color.md)

*1 enum*

### [`style::types::color::Color`](style/types/color/Color.md)

*1 function*

### [`style::types::colored`](style/types/colored.md)

*1 enum*

### [`style::types::colored::Colored`](style/types/colored/Colored.md)

*1 function*

### [`style::types::colors`](style/types/colors.md)

*1 struct*

### [`terminal`](terminal.md)

*1 enum, 12 structs, 5 functions*

### [`terminal::sys::unix`](terminal/sys/unix.md)

*1 function*

### [`tty`](tty.md)

*1 trait*

