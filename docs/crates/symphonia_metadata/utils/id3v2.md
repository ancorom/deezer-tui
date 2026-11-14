**symphonia_metadata > utils > id3v2**

# Module: utils::id3v2

## Contents

**Functions**

- [`get_visual_key_from_picture_type`](#get_visual_key_from_picture_type) - Try to get a `StandardVisualKey` from the APIC picture type identifier.

---

## symphonia_metadata::utils::id3v2::get_visual_key_from_picture_type

*Function*

Try to get a `StandardVisualKey` from the APIC picture type identifier.

```rust
fn get_visual_key_from_picture_type(apic: u32) -> Option<symphonia_core::meta::StandardVisualKey>
```



