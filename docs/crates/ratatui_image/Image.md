**ratatui_image > Image**

# Module: Image

## Contents

**Functions**

- [`allow_clipping`](#allow_clipping) - Allow clipping the image if the render area is smaller than the image, and if the protocol

---

## ratatui_image::Image::allow_clipping

*Function*

Allow clipping the image if the render area is smaller than the image, and if the protocol
supports it ([`protocol::kitty`] and [`protocol::halfblocks`]).

This is disabled by default to make the behavior consistent.

See also [`protocol::Protocol::needs_placeholder`], which is an excellent complement if you
need to render *something* when the image couldn't.

```rust
# use ratatui_image::picker::Picker;
# use ratatui::layout::Size;
# use ratatui_image::{*, sliced::*};
# let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24))?;
# let picker = Picker::halfblocks();
# let dyn_img = image::ImageReader::open("./assets/NixOS.png")?.decode()?;
# let proto = picker.new_protocol(dyn_img, (20, 10).into(), Resize::Fit(None))?;
terminal.draw(|f| {
    if let Some(placeholder_area) = proto.needs_placeholder(f.area()) {
        // Render `Box` or something with placeholder_area.
    } else {
        f.render_widget(Image::new(&proto).allow_clipping(true), f.area());
    }
});
# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
fn allow_clipping(self: Self, allow: bool) -> Self
```



