**ratatui > init**

# Module: init

## Contents

**Functions**

- [`init`](#init) - Initialize a terminal with reasonable defaults for most applications.
- [`init_with_options`](#init_with_options) - Initialize a terminal with the given options and reasonable defaults.
- [`restore`](#restore) - Restores the terminal to its original state.
- [`run`](#run) - Run a closure with a terminal initialized with reasonable defaults for most applications.
- [`try_init`](#try_init) - Try to initialize a terminal using reasonable defaults for most applications.
- [`try_init_with_options`](#try_init_with_options) - Try to initialize a terminal with the given options and reasonable defaults.
- [`try_restore`](#try_restore) - Restore the terminal to its original state.

**Type Aliases**

- [`DefaultTerminal`](#defaultterminal) - A type alias for the default terminal type.

---

## ratatui::init::DefaultTerminal

*Type Alias*: `ratatui_core::terminal::Terminal<ratatui_crossterm::CrosstermBackend<std::io::Stdout>>`

A type alias for the default terminal type.

This is a [`Terminal`] using the [`CrosstermBackend`] which writes to [`Stdout`]. This is a
reasonable default for most applications. To use a different backend or output stream, instead
use [`Terminal`] and a [backend][`crate::backend`] of your choice directly.



## ratatui::init::init

*Function*

Initialize a terminal with reasonable defaults for most applications.

This will create a new [`DefaultTerminal`] and initialize it with the following defaults:

- Backend: [`CrosstermBackend`] writing to [`Stdout`]
- Raw mode is enabled
- Alternate screen buffer enabled
- A panic hook is installed that restores the terminal before panicking. Ensure that this method
  is called after any other panic hooks that may be installed to ensure that the terminal is
  restored before those hooks are called.

For more control over the terminal initialization, use [`Terminal::new`] or
[`Terminal::with_options`].

Ensure that this method is called *after* your app installs any other panic hooks to ensure the
terminal is restored before the other hooks are called.

Generally, use this function instead of [`try_init`] to ensure that the terminal is restored
correctly if any of the initialization steps fail. If you need to handle the error yourself, use
[`try_init`] instead.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Panics

This function will panic if any of the following steps fail:

- Enabling raw mode
- Entering the alternate screen buffer
- Creating the terminal fails due to being unable to calculate the terminal size

# Examples

```rust,no_run
let terminal = ratatui::init();
```

```rust
fn init() -> DefaultTerminal
```



## ratatui::init::init_with_options

*Function*

Initialize a terminal with the given options and reasonable defaults.

This function allows the caller to specify a custom [`Viewport`] via the [`TerminalOptions`]. It
will create a new [`DefaultTerminal`] and initialize it with the given options and the following
defaults:

[`Viewport`]: crate::Viewport

- Raw mode is enabled
- A panic hook is installed that restores the terminal before panicking.

Unlike [`init`], this function does not enter the alternate screen buffer as this may not be
desired in all cases. If you need the alternate screen buffer, you should enable it manually
after calling this function.

For more control over the terminal initialization, use [`Terminal::with_options`].

Ensure that this method is called *after* your app installs any other panic hooks to ensure the
terminal is restored before the other hooks are called.

Generally, use this function instead of [`try_init_with_options`] to ensure that the terminal is
restored correctly if any of the initialization steps fail. If you need to handle the error
yourself, use [`try_init_with_options`] instead.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Panics

This function will panic if any of the following steps fail:

- Enabling raw mode
- Creating the terminal fails due to being unable to calculate the terminal size

# Examples

```rust,no_run
use ratatui::{TerminalOptions, Viewport};

let options = TerminalOptions {
    viewport: Viewport::Inline(5),
};
let terminal = ratatui::init_with_options(options);
```

```rust
fn init_with_options(options: ratatui_core::terminal::TerminalOptions) -> DefaultTerminal
```



## ratatui::init::restore

*Function*

Restores the terminal to its original state.

This function should be called before the program exits to ensure that the terminal is
restored to its original state.

This function will attempt to restore the terminal to its original state by performing the
following steps:

1. Raw mode is disabled.
2. The alternate screen buffer is left.

If either of these steps fail, the error is printed to stderr and ignored.

Use this function over [`try_restore`] when you don't need to handle the error yourself, as
ignoring the error is generally the correct behavior when cleaning up before exiting. If you
need to handle the error yourself, use [`try_restore`] instead.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Examples

```rust,no_run
ratatui::restore();
```

```rust
fn restore()
```



## ratatui::init::run

*Function*

Run a closure with a terminal initialized with reasonable defaults for most applications.

This function creates a new [`DefaultTerminal`] with [`init`] and then runs the given closure
with a mutable reference to the terminal. After the closure completes, the terminal is restored
to its original state with [`restore`].

This function is a convenience wrapper around [`init`] and [`restore`], and is useful for simple
applications that need a terminal with reasonable defaults for the entire lifetime of the
application.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Examples

A simple example where the app logic is contained in the closure:

```rust,no_run
use crossterm::event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello, world!", frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
```

A more complex example where the app logic is contained in a separate function:

```rust,no_run
use crossterm::event;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    ratatui::run(app)
}

fn app(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    const GREETING: &str = "Hello, world!";
    loop {
        terminal.draw(|frame| frame.render_widget(format!("{GREETING}"), frame.area()))?;
        if matches!(event::read()?, event::Event::Key(_)) {
            break Ok(());
        }
    }
}
```

Once the app logic becomes more complex, it may be beneficial to move the app logic into a
separate struct. This allows the app logic to be split into multiple methods with each having
access to the state of the app. This can make the app logic easier to understand and maintain.

```rust,no_run
use crossterm::event;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))
}

struct App {
    should_quit: bool,
    name: String,
}

impl App {
    fn new() -> Self {
        Self {
            should_quit: false,
            name: "world".to_string(),
        }
    }

    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| frame.render_widget("Hello, world!", frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        let greeting = format!("Hello, {}!", self.name);
        frame.render_widget(greeting, frame.area());
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }
}
```

```rust
fn run<F, R>(f: F) -> R
```



## ratatui::init::try_init

*Function*

Try to initialize a terminal using reasonable defaults for most applications.

This function will attempt to create a [`DefaultTerminal`] and initialize it with the following
defaults:

- Raw mode is enabled
- Alternate screen buffer enabled
- A panic hook is installed that restores the terminal before panicking.
- A [`Terminal`] is created using [`CrosstermBackend`] writing to [`Stdout`]

If any of these steps fail, the error is returned.

Ensure that this method is called *after* your app installs any other panic hooks to ensure the
terminal is restored before the other hooks are called.

Generally, you should use [`init`] instead of this function, as the panic hook installed by this
function will ensure that any failures during initialization will restore the terminal before
panicking. This function is provided for cases where you need to handle the error yourself.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Examples

```no_run
let terminal = ratatui::try_init()?;
# Ok::<(), std::io::Error>(())
```

```rust
fn try_init() -> io::Result<DefaultTerminal>
```



## ratatui::init::try_init_with_options

*Function*

Try to initialize a terminal with the given options and reasonable defaults.

This function allows the caller to specify a custom [`Viewport`] via the [`TerminalOptions`]. It
will attempt to create a [`DefaultTerminal`] and initialize it with the given options and the
following defaults:

[`Viewport`]: crate::Viewport

- Raw mode is enabled
- A panic hook is installed that restores the terminal before panicking.

Unlike [`try_init`], this function does not enter the alternate screen buffer as this may not be
desired in all cases. If you need the alternate screen buffer, you should enable it manually
after calling this function.

If any of these steps fail, the error is returned.

Ensure that this method is called *after* your app installs any other panic hooks to ensure the
terminal is restored before the other hooks are called.

Generally, you should use [`init_with_options`] instead of this function, as the panic hook
installed by this function will ensure that any failures during initialization will restore the
terminal before panicking. This function is provided for cases where you need to handle the
error yourself.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Examples

```no_run
use ratatui::{TerminalOptions, Viewport};

let options = TerminalOptions {
    viewport: Viewport::Inline(5),
};
let terminal = ratatui::try_init_with_options(options)?;
# Ok::<(), std::io::Error>(())
```

```rust
fn try_init_with_options(options: ratatui_core::terminal::TerminalOptions) -> io::Result<DefaultTerminal>
```



## ratatui::init::try_restore

*Function*

Restore the terminal to its original state.

This function will attempt to restore the terminal to its original state by performing the
following steps:

1. Raw mode is disabled.
2. The alternate screen buffer is left.

If either of these steps fail, the error is returned.

Use [`restore`] instead of this function when you don't need to handle the error yourself, as
ignoring the error is generally the correct behavior when cleaning up before exiting. If you
need to handle the error yourself, use this function instead.

See the [module-level documentation](mod@crate::init) for a comparison of all initialization
functions and guidance on when to use each one.

# Examples

```no_run
ratatui::try_restore()?;
# Ok::<(), std::io::Error>(())
```

```rust
fn try_restore() -> io::Result<()>
```



