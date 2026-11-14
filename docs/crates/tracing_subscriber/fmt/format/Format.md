**tracing_subscriber > fmt > format > Format**

# Module: fmt::format::Format

## Contents

**Functions**

- [`with_ansi`](#with_ansi) - Enable ANSI terminal colors for formatted output.
- [`with_file`](#with_file) - Sets whether or not an event's [source code file path][file] is
- [`with_line_number`](#with_line_number) - Sets whether or not an event's [source code line number][line] is

---

## tracing_subscriber::fmt::format::Format::with_ansi

*Function*

Enable ANSI terminal colors for formatted output.

```rust
fn with_ansi(self: Self, ansi: bool) -> Format<F, T>
```



## tracing_subscriber::fmt::format::Format::with_file

*Function*

Sets whether or not an event's [source code file path][file] is
displayed.

[file]: tracing_core::Metadata::file

```rust
fn with_file(self: Self, display_filename: bool) -> Format<F, T>
```



## tracing_subscriber::fmt::format::Format::with_line_number

*Function*

Sets whether or not an event's [source code line number][line] is
displayed.

[line]: tracing_core::Metadata::line

```rust
fn with_line_number(self: Self, display_line_number: bool) -> Format<F, T>
```



