**ratatui_image > picker**

# Module: picker

## Contents

**Modules**

- [`cap_parser`](#cap_parser) - Terminal stdio query parser module.

**Structs**

- [`Picker`](#picker)

**Enums**

- [`Capability`](#capability)
- [`ProtocolType`](#protocoltype) - Serde-friendly protocol-type enum for [Picker].

---

## ratatui_image::picker::Capability

*Enum*

**Variants:**
- `Kitty` - Reports supporting kitty graphics protocol.
- `Sixel` - Reports supporting sixel graphics protocol.
- `RectangularOps` - Reports supporting rectangular ops.
- `KittyCompression` - Reports being able to inflate a zlib-compressed kitty transmission
- `CellSize(Option<(u16, u16)>)` - Reports font size in pixels.
- `TextSizingProtocol` - Reports supporting text sizing protocol.
- `Background(u8, u8, u8)` - Reports a background color.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_image::picker::Picker

*Struct*

**Methods:**

- `fn from_query_stdio() -> std::result::Result<Self, errors::Errors>` - Query terminal stdio for graphics capabilities and font-size with some escape sequences.
- `fn from_query_stdio_with_options(options: QueryStdioOptions) -> std::result::Result<Self, errors::Errors>` - This should ONLY be used if [Capability::TextSizingProtocol] is needed for some external
- `fn halfblocks() -> Self` - Create a picker that is guaranteed to only work with Halfblocks.
- `fn from_fontsize(font_size: FontSize) -> Self` - Create a picker from a given terminal [FontSize].
- `fn protocol_type(self: &Self) -> ProtocolType` - Returns the current protocol type.
- `fn set_protocol_type(self: & mut Self, protocol_type: ProtocolType)` - Force a protocol type.
- `fn font_size(self: &Self) -> FontSize` - Returns the [FontSize] detected by [Picker::from_query_stdio].
- `fn tmux_detected(self: &Self) -> bool` - Whether launch-time detection (from `TERM`/`TERM_PROGRAM`) concluded this
- `fn set_background_color<T>(self: & mut Self, background_color: Option<T>)` - Change the default background color (transparent black).
- `fn capabilities(self: &Self) -> &Vec<Capability>` - Returns the capabilities detected by [Picker::from_query_stdio].
- `fn new_protocol(self: &Self, image: DynamicImage, size: Size, resize: Resize) -> std::result::Result<Protocol, errors::Errors>` - Returns a new protocol for [`crate::Image`] widgets that fits into the given size.
- `fn new_resize_protocol(self: &Self, image: DynamicImage) -> StatefulProtocol` - Returns a new *stateful* protocol for [`crate::StatefulImage`] widgets.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_image::picker::ProtocolType

*Enum*

Serde-friendly protocol-type enum for [Picker].

**Variants:**
- `Halfblocks`
- `Sixel`
- `Kitty`
- `Iterm2`

**Methods:**

- `fn next(self: &Self) -> ProtocolType`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: cap_parser

Terminal stdio query parser module.



