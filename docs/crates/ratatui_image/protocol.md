**ratatui_image > protocol**

# Module: protocol

## Contents

**Modules**

- [`halfblocks`](#halfblocks) - Halfblocks protocol implementations.
- [`iterm2`](#iterm2) - ITerm2 protocol implementation.
- [`kitty`](#kitty) - Kitty protocol.
- [`sixel`](#sixel) - Sixel protocol implementations.

**Structs**

- [`StatefulProtocol`](#statefulprotocol) - A stateful resizing image protocol for the [crate::StatefulImage] widget.

**Enums**

- [`Protocol`](#protocol) - A fixed-size image protocol for the [crate::Image] widget.
- [`StatefulProtocolType`](#statefulprotocoltype)

---

## ratatui_image::protocol::Protocol

*Enum*

A fixed-size image protocol for the [crate::Image] widget.

**Variants:**
- `Halfblocks(self::halfblocks::Halfblocks)`
- `Sixel(self::sixel::Sixel)`
- `Kitty(self::Kitty)`
- `ITerm2(self::iterm2::Iterm2)`

**Methods:**

- `fn size(self: &Self) -> Size`
- `fn needs_placeholder(self: &Self, area: Rect) -> Option<Rect>` - Returns a placeholder area, if the image will not render into the given area, or `None`.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_image::protocol::StatefulProtocol

*Struct*

A stateful resizing image protocol for the [crate::StatefulImage] widget.

The [crate::thread::ThreadProtocol] widget also uses this, and is the reason why resizing is
split from rendering.

**Methods:**

- `fn new(image: DynamicImage, font_size: FontSize, background_color: Option<Rgba<u8>>, protocol_type: StatefulProtocolType) -> Self`
- `fn size_for(self: &Self, resize: Resize, size: Size) -> Size`
- `fn protocol_type(self: &Self) -> &StatefulProtocolType`
- `fn protocol_type_owned(self: Self) -> StatefulProtocolType`
- `fn last_encoding_result(self: & mut Self) -> Option<std::result::Result<(), errors::Errors>>` - This returns the latest Result returned when encoding, and none if there was no encoding since the last result read. It is encouraged but not required to handle it
- `fn background_color(self: &Self) -> Option<Rgba<u8>>`

**Trait Implementations:**

- **ResizeEncodeRender**
  - `fn resize_encode(self: & mut Self, resize: &Resize, size: Size)`
  - `fn render(self: & mut Self, area: Rect, buf: & mut Buffer)`
  - `fn needs_resize(self: &Self, resize: &Resize, size: Size) -> Option<Size>`



## ratatui_image::protocol::StatefulProtocolType

*Enum*

**Variants:**
- `Halfblocks(self::halfblocks::Halfblocks)`
- `Sixel(self::sixel::Sixel)`
- `Kitty(self::StatefulKitty)`
- `ITerm2(self::iterm2::Iterm2)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: halfblocks

Halfblocks protocol implementations.

Uses the unicode character `▀` combined with foreground and background color. Assumes that the
font aspect ratio is roughly 1:2. Should work in all terminals.

If chafa is available, uses chafa for much richer rendering than primitive halfblocks:
- `chafa-static`: statically linked at compile time (requires static libchafa.a)
- `chafa-dyn`: dynamically linked at compile time via pkg-config



## Module: iterm2

ITerm2 protocol implementation.

Delivers the full raw png image on every render.



## Module: kitty

Kitty protocol.

Transmits the image once on first render, tracked via an AtomicBool, and then subsequentially
renders the image with the [unicode-placeholders] feature of the [kitty protocol].

[unicode-placeholders]: https://sw.kovidgoyal.net/kitty/graphics-protocol/#unicode-placeholders
[kitty protocol]: https://sw.kovidgoyal.net/kitty/graphics-protocol



## Module: sixel

Sixel protocol implementations.
Uses [`icy_sixel`] to draw image pixels, if the terminal [supports] the [Sixel] protocol.

Delivers the image on each render as [Sixel]s.

[`icy_sixel`]: https://github.com/mkrueger/icy_sixel
[supports]: https://arewesixelyet.com
[Sixel]: https://en.wikipedia.org/wiki/Sixel



