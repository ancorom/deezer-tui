**ratatui_image > thread**

# Module: thread

## Contents

**Structs**

- [`ResizeRequest`](#resizerequest) - The only usage of this struct is to call `perform()` on it and pass the completed resize to `ThreadProtocols` `update_protocol()`
- [`ResizeResponse`](#resizeresponse) - The only usage of this struct is to pass it to `ThreadProtocols` `update_resize_protocol()`
- [`ThreadProtocol`](#threadprotocol) - The state for a threaded [`crate::StatefulImage`].

---

## ratatui_image::thread::ResizeRequest

*Struct*

The only usage of this struct is to call `perform()` on it and pass the completed resize to `ThreadProtocols` `update_protocol()`

**Methods:**

- `fn resize_encode(self: Self) -> Result<ResizeResponse, Errors>`



## ratatui_image::thread::ResizeResponse

*Struct*

The only usage of this struct is to pass it to `ThreadProtocols` `update_resize_protocol()`



## ratatui_image::thread::ThreadProtocol

*Struct*

The state for a threaded [`crate::StatefulImage`].

Has `inner` [StatefulProtocol] and sents requests through the mspc channel to do the
`resize_encode()` work.

**Methods:**

- `fn new(tx: Sender<ResizeRequest>, inner: Option<StatefulProtocol>) -> ThreadProtocol`
- `fn replace_protocol(self: & mut Self, proto: StatefulProtocol)`
- `fn protocol_type(self: &Self) -> Option<&StatefulProtocolType>`
- `fn protocol_type_owned(self: Self) -> Option<StatefulProtocolType>`
- `fn background_color(self: &Self) -> Option<Rgba<u8>>`
- `fn empty_protocol(self: & mut Self)` - This function should be used when an image should be updated but the updated image is not yet available
- `fn update_resized_protocol(self: & mut Self, completed: ResizeResponse) -> bool`
- `fn size_for(self: &Self, resize: Resize, size: Size) -> Option<Size>`

**Trait Implementations:**

- **ResizeEncodeRender**
  - `fn needs_resize(self: &Self, resize: &Resize, size: Size) -> Option<Size>`
  - `fn resize_encode(self: & mut Self, resize: &Resize, size: Size)` - Senda a `ResizeRequest` through the channel if there already isn't a pending `ResizeRequest`
  - `fn render(self: & mut Self, area: Rect, buf: & mut Buffer)` - Render the currently resized and encoded data to the buffer, if there isn't a pending `ResizeRequest`



