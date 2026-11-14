# ratatui_image

# Image widgets with multiple graphics protocol backends for [ratatui]

**Unify terminal image rendering across Sixels, Kitty, and iTerm2 protocols.**

[ratatui] is an immediate-mode TUI library.
ratatui-image tackles 3 general problems when rendering images with an immediate-mode TUI:

**Query the terminal for available graphics protocols**

Some terminals may implement one or more graphics protocols, such as Sixels, or the iTerm2 or
Kitty graphics protocols. Guess by env vars. If that fails, query the terminal with some
control sequences.
Fallback to "halfblocks" which uses some unicode half-block characters with fore- and
background colors.

**Query the terminal for the font-size in pixels.**

If there is an actual graphics protocol available, it is necessary to know the font-size to
be able to map the image pixels to character cell area.
Query the terminal with some control sequences for either the font-size directly, or the
window-size in pixels and derive the font-size together with row/column count.

**Render the image by the means of the guessed protocol.**

Some protocols, like Sixels, are essentially "immediate-mode", but we still need to avoid the
TUI from overwriting the image area, even with blank characters.
Other protocols, like Kitty, are essentially stateful, but at least provide a way to re-render
an image that has been loaded, at a different or same position.
Since we have the font-size in pixels, we can precisely map the characters/cells/rows-columns
that will be covered by the image and skip drawing over the image.

# Quick start
```rust
use ratatui::{backend::TestBackend, layout::Size, Terminal, Frame};
use ratatui_image::{Image, picker::Picker, protocol::Protocol, Resize};

struct App {
    // We need to hold the image data somewhere.
    image: Protocol,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend)?;

    // Should use `Picker::from_query_stdio()?` to get the font size and protocol,
    // but we can't put that here because that would break doctests!
    let mut picker = Picker::halfblocks();

    // Load an image with the image crate.
    let dyn_img = image::ImageReader::open("./assets/Ada.png")?.decode()?;

    let font_size = picker.font_size();
    let size = Size::new(
        dyn_img.width().div_ceil(font_size.width as u32) as u16,
        dyn_img.height().div_ceil(font_size.height as u32) as u16,
    );

    // Create the Protocol once, or in other words, transform the image data to Sixels, Kitty
    // data, iTerm2 base64 PNG data, or some kind of ASCII-art.
    let image = picker.new_protocol(dyn_img, size, Resize::Fit(None))?;

    let mut app = App { image };

    // This would be your typical `loop {` in a real app:
    terminal.draw(|f| {
        let image = Image::new(&app.image);
        // Rendering the transformed data is now cheap.
        f.render_widget(image, f.area());
    });

    Ok(())
}
```
While this approach is usually sufficient, and leaves a lot of room for customizing where the
image actually gets transformed, for more advanced usage I really recommend using
[`thread::ThreadProtocol`] and looking at `excamples/thread.rs` to get an idea how to
dynamically resize images to fit into some area but without blocking the UI.

The [picker::Picker] helper is there to do all this font-size and graphics-protocol guessing,
and also to map character-cell-size to pixel size so that we can e.g. "fit" an image inside
a desired columns+rows bound, and so on.

# Widget choice
* The [`Image`] widget has a fixed size in rows/columns. If the image pixel size exceeds the
  pixel area of the rows/columns, the image is scaled down proportionally to "fit" once, at the
  creation time of the [`Protocol`].  
  The big upside is that this widget is _stateless_ (in terms of ratatui, i.e. immediate-mode),
  and thus can never block the rendering thread/task. A lot of ratatui apps only use stateless
  widgets, so this factor is also important when chosing.  
  What happens when the image does not fit into the render area can be controlled with
  [`Image::allow_clipping`].
* The [StatefulImage] widget adapts to its render area at render-time. It can be set to fit,
  crop, or scale to the available render area.
  This means the widget must be stateful, i.e. use `render_stateful_widget` which takes a
  mutable state parameter.
  The resizing and encoding is blocking, and since it happens at render-time, it should always
  be offloaded to another thread or async task, to keep the UI responsive (see
  `examples/thread.rs` and `examples/tokio.rs` on how to use [`thread::ThreadProtocol`]).

# Examples

* `examples/demo.rs` is a fully fledged demo.
* `examples/thread.rs` shows how to offload resize and encoding to another thread, to avoid
  blocking the UI thread.
* `examples/tokio.rs` same as `thread.rs` but with tokio.
* `examples/sliced.rs` shows how to use an image that can have "rows" or "horizontal slices"
  partially hidden with any protocol.

The lib also includes a binary that renders an image file, but it is focused on testing.

# Features

### Backend

* `crossterm` (default) if this matches your ratatui backend (most likely).
* `termion` if this matches your ratatui backend.
* `termwiz` is available, but not working correctly with ratatui-image.

### Chafa library

* `chafa-dyn` (default) to use the amazing [chafa](https://hpjansson.org/chafa/) library for
  rendering without image protocols. Dynamically link against libchafa.so at compile time.
  Requires libchafa to be available at runtime in the same way.
* `chafa-static` to statically link against libchafa.a at compile time. The library is embedded
  in the binary.
* If you absolutely don't want to deal with libchafa, then you should use
  `--no-default-features --features image-defaults,crossterm` or a variation thereof.

Note: The chafa features are mutually exclusive - enable only one at a time.
There is *NO* compiler error for enabling both features at the same time, because some tools
(like cargo-semver-checks) need to build with all features enabled. If both features are
enabled, then `chafa-dyn` takes precedence, because that one makes it easier for running such
tools in CI.

### Others

* `image-defaults` (default) just enables `image/defaults` (`image` has `default-features =
  false`). To only support a selection of image formats and cut down dependencies, disable this
  feature, add `image` to your crate, and enable its features/formats as desired. See
  <https://doc.rust-lang.org/cargo/reference/features.html#feature-unification/>.
* `serde` for `#[derive]`s on [picker::ProtocolType] for convenience, because it might be
  useful to save it in some user configuration.
* `tokio` whether to use tokio's `UnboundedSender` in `ThreadProtocol`.


[ratatui]: https://github.com/ratatui-org/ratatui
[sixel]: https://en.wikipedia.org/wiki/Sixel
[`render_stateful_widget`]: https://docs.rs/ratatui/latest/ratatui/terminal/struct.Frame.html#method.render_stateful_widget

## Modules

### [`ratatui_image`](ratatui_image.md)

*1 enum, 1 trait, 4 structs, 5 modules*

### [`Image`](Image.md)

*1 function*

### [`errors`](errors.md)

*1 enum*

### [`picker`](picker.md)

*1 module, 1 struct, 2 enums*

### [`picker::Picker`](picker/Picker.md)

*2 functions*

### [`picker::cap_parser`](picker/cap_parser.md)

*2 enums, 2 structs*

### [`protocol`](protocol.md)

*1 struct, 2 enums, 4 modules*

### [`protocol::Protocol`](protocol/Protocol.md)

*1 function*

### [`protocol::halfblocks`](protocol/halfblocks.md)

*2 structs*

### [`protocol::iterm2`](protocol/iterm2.md)

*1 struct*

### [`protocol::kitty`](protocol/kitty.md)

*2 structs*

### [`protocol::sixel`](protocol/sixel.md)

*1 struct*

### [`sliced`](sliced.md)

*1 enum, 2 structs*

### [`sliced::sixel_slice`](sliced/sixel_slice.md)

*2 structs*

### [`thread`](thread.md)

*3 structs*

