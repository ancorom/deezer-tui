**ratatui_image**

# Module: ratatui_image

## Contents

**Modules**

- [`errors`](#errors)
- [`picker`](#picker) - Helper module to build a protocol, and swap protocols at runtime
- [`protocol`](#protocol) - Protocol backends for the widgets
- [`sliced`](#sliced) - Sliced image widget and protocol wrapper.
- [`thread`](#thread) - Widget that separates resize+encode from rendering.

**Structs**

- [`CropOptions`](#cropoptions) - Specifies which sides to be clipped when cropping an image.
- [`FontSize`](#fontsize) - The terminal's font size in `(width, height)`
- [`Image`](#image) - Fixed size image widget that uses [Protocol].
- [`StatefulImage`](#statefulimage) - Resizeable image widget that uses a [protocol::StatefulProtocol] state.

**Enums**

- [`Resize`](#resize) - Resize accounting for terminal [`FontSize`].

**Traits**

- [`ResizeEncodeRender`](#resizeencoderender)

---

## ratatui_image::CropOptions

*Struct*

Specifies which sides to be clipped when cropping an image.

**Fields:**
- `clip_top: bool` - If `true`, the top side should be clipped.
- `clip_left: bool` - If `true`, the left side should be clipped.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_image::FontSize

*Struct*

The terminal's font size in `(width, height)`

**Fields:**
- `width: u16`
- `height: u16`

**Methods:**

- `fn new(width: u16, height: u16) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from((width, height): (u16, u16)) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_image::Image

*Struct*

Fixed size image widget that uses [Protocol].

The widget does **not** react to area resizes.
Its advantage lies in that the [Protocol] needs only one initial resize.

The image won't render if it doesn't fit, unless [`Image::allow_clipping`] has been set.
```rust
# use ratatui_image::picker::Picker;
# use ratatui::layout::Size;
# use ratatui_image::{*, sliced::{SlicedProtocol, SlicedImage}};
# let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24))?;
# let picker = Picker::halfblocks(); // Note: use from_query_studio
// let picker = Picker::from_query_studio()?;
let image = image::ImageReader::open("./assets/NixOS.png")?.decode()?;
let proto = picker.new_protocol(image, Size::new(20, 10), Resize::Fit(None))?;

terminal.draw(|f| {
    f.render_widget(Image::new(&proto), f.area());
});
# Ok::<(), Box<dyn std::error::Error>>(())
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(image: &'a Protocol) -> Self`
- `fn allow_clipping(self: Self, allow: bool) -> Self` - Allow clipping the image if the render area is smaller than the image, and if the protocol

**Trait Implementations:**

- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`



## ratatui_image::Resize

*Enum*

Resize accounting for terminal [`FontSize`].

Resizes images with [`FontSize`] grid boundaries.

**Variants:**
- `Fit(Option<FilterType>)` - Fit to a [`Size`].
- `Crop(Option<CropOptions>)` - Crop to size.
- `Scale(Option<FilterType>)` - Scale the image

**Methods:**

- `fn resize(self: &Self, image: &DynamicImage, font_size: FontSize, size: Size, background_color: Option<Rgba<u8>>) -> DynamicImage` - Resize [`image::DynamicImage`] to fit into the [`Size`] or smaller.
- `fn size_for(self: &Self, image: &DynamicImage, font_size: FontSize, available: Size) -> Size` - Calculate the [`Size`] for the [`DynamicImage`] for `available` size after resizing.
- `fn natural_size(image: &DynamicImage, font_size: FontSize) -> Size` - Calculate the "natural" [`Size`] needed to render the [`DynamicImage`] at the [`FontSize`],

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_image::ResizeEncodeRender

*Trait*

**Methods:**

- `resize_encode_render`: Resize and encode if necessary, and render immediately.
- `resize_encode`: Resize the image and encode it for rendering. The result should be stored statefully so
- `render`: Render the currently resized and encoded data to the buffer.
- `needs_resize`: Check if the current image state would need resizing (grow or shrink) for the given area.



## ratatui_image::StatefulImage

*Struct*

Resizeable image widget that uses a [protocol::StatefulProtocol] state.

This stateful widget resizes the image at render time.

**Do not use it withou [`thread::ThreadProtocol`] in a reactive UI**. Rendering the widget
**will** block the UI thread if the image has not been resized by another thread.

```rust
# use ratatui::Frame;
# use ratatui_image::{Resize, StatefulImage, protocol::{StatefulProtocol}};
struct App {
    image_state: StatefulProtocol,
}
fn ui(f: &mut Frame<'_>, app: &mut App) {
    let image = StatefulImage::default().resize(Resize::Crop(None));
    f.render_stateful_widget(
        image,
        f.area(),
        &mut app.image_state,
    );
}
```

**Generic Parameters:**
- T

**Methods:**

- `fn resize(self: Self, resize: Resize) -> Self`
- `fn new() -> Self`

**Trait Implementations:**

- **StatefulWidget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer, state: & mut <Self as >::State)`
- **Default**
  - `fn default() -> Self`



## Module: errors



## Module: picker

Helper module to build a protocol, and swap protocols at runtime



## Module: protocol

Protocol backends for the widgets



## Module: sliced

Sliced image widget and protocol wrapper.



## Module: thread

Widget that separates resize+encode from rendering.
This allows for rendering to be non-blocking, offloading resize+encode into another thread.
See examples/thread.rs and examples/tokio.rs for how to setup the threads and channels.
At least one worker thread for resize+encode is required, the example shows how to combine
the needs-resize-polling with other terminal events into one event loop.



