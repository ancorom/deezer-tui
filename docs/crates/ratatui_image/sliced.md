**ratatui_image > sliced**

# Module: sliced

## Contents

**Structs**

- [`SignedPosition`](#signedposition)
- [`SlicedImage`](#slicedimage) - An image "sliced" into rows for partially displaying, for example in vertical scrolling.

**Enums**

- [`SlicedProtocol`](#slicedprotocol) - The sliced image for [`SlicedImage`].

---

## ratatui_image::sliced::SignedPosition

*Struct*

**Fields:**
- `x: i16`
- `y: i16`

**Traits:** Copy

**Trait Implementations:**

- **From**
  - `fn from((x, y): (i16, i16)) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_image::sliced::SlicedImage

*Struct*

An image "sliced" into rows for partially displaying, for example in vertical scrolling.

Uses a specialized [`SlicedProtocol`] with specialized operations based on the protocol.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(sliced_protocol: &'a SlicedProtocol, position: SignedPosition) -> SlicedImage<'a>` - Create a sliced image that will render with the given size at the given position.

**Trait Implementations:**

- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut ratatui::prelude::Buffer)`



## ratatui_image::sliced::SlicedProtocol

*Enum*

The sliced image for [`SlicedImage`].

Contains the sliced data specialized for the protocol.

**Variants:**
- `Sliced(Vec<crate::Protocol>)` - Generic, simply a list of image slices (or rows).
- `Kitty(crate::kitty::Kitty)` - Takes full advantage of the unicode-placeholder mechanism.
- `Sixel(crate::sliced::sixel_slice::SlicedSixel)` - Strips sixel "bands" at render time to display only relevant parts, since the sixel format
- `Halfblocks(crate::halfblocks::Halfblocks)` - Renders the full image (with chafa if available) for best ASCII art results, then just

**Methods:**

- `fn new(picker: &Picker, dyn_img: DynamicImage, size: Option<Size>) -> Result<SlicedProtocol, Errors>` - Create a `SlicedProtocol` for the target [`ratatui::layout::Size`].
- `fn new_with_resize(picker: &Picker, dyn_img: DynamicImage, size: Size, resize: Resize) -> Result<SlicedProtocol, Errors>` - Create a `SlicedProtocol` for the target [`ratatui::layout::Size`] with the given
- `fn size(self: &Self) -> Size`



