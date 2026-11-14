**ratatui_image > picker > cap_parser**

# Module: picker::cap_parser

## Contents

**Structs**

- [`Parser`](#parser)
- [`QueryStdioOptions`](#querystdiooptions) - Extra query options

**Enums**

- [`Response`](#response)
- [`ResponseParseState`](#responseparsestate)

---

## ratatui_image::picker::cap_parser::Parser

*Struct*

**Methods:**

- `fn new() -> Self`
- `fn tmux_start_escape_end(is_tmux: bool) -> (&'static str, &'static str, &'static str)` - Tmux requires escapes to be escaped, and some special start/end sequences.
- `fn query(is_tmux: bool, options: QueryStdioOptions) -> String`
- `fn push(self: & mut Self, next: char) -> Vec<Response>`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## ratatui_image::picker::cap_parser::QueryStdioOptions

*Struct*

Extra query options

**Fields:**
- `timeout: std::time::Duration` - Timeout for the stdio query.
- `text_sizing_protocol: bool` - Query for [Text Sizing Protocol]. The result can be checked by searching for
- `terminal_background_color_osc: bool` - Query the terminal background color. The result will be
- `blacklist_protocols: Vec<crate::picker::ProtocolType>` - Blacklist protocols from the detection query. Currently only kitty can be detected, so that
- `kitty_compression: bool` - Probe for, and use, kitty's `o=z` zlib transmission compression.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## ratatui_image::picker::cap_parser::Response

*Enum*

**Variants:**
- `Kitty`
- `Sixel`
- `RectangularOps`
- `KittyCompression`
- `CellSize(Option<(u16, u16)>)`
- `CursorPositionReport(u16, u16)`
- `Background(u8, u8, u8)`
- `Status`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_image::picker::cap_parser::ResponseParseState

*Enum*

**Variants:**
- `Unknown`
- `CSIResponse`
- `OSCResponse`
- `KittyResponse`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



