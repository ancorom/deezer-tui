**ratatui_core > layout > flex**

# Module: layout::flex

## Contents

**Enums**

- [`Flex`](#flex) - Defines the options for layout flex justify content in a container.

---

## ratatui_core::layout::flex::Flex

*Enum*

Defines the options for layout flex justify content in a container.

This enumeration controls the distribution of space when layout constraints are met and there
is excess space available. The `Flex` option is used with [`Layout`](crate::layout::Layout) to
control how extra space is distributed among layout segments, which is particularly useful for
creating responsive layouts that adapt to different terminal sizes.

Available options:

- `Legacy`: Fills the available space within the container, putting excess space into the last
  element.
- `Start`: Aligns items to the start of the container.
- `End`: Aligns items to the end of the container.
- `Center`: Centers items within the container.
- `SpaceBetween`: Adds excess space between each element.
- `SpaceAround`: Adds excess space around each element.

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Variants:**
- `Legacy` - Fills the available space within the container, putting excess space into the last
- `Start` - Aligns items to the start of the container.
- `End` - Aligns items to the end of the container.
- `Center` - Centers items within the container.
- `SpaceBetween` - Adds excess space between each element.
- `SpaceEvenly` - Evenly distributes excess space between all elements, including before the first and after
- `SpaceAround` - Adds excess space around each element.

**Methods:**

- `fn is_legacy(self: &Self) -> bool` - Returns [true] if the enum is [Flex::Legacy] otherwise [false]
- `fn is_start(self: &Self) -> bool` - Returns [true] if the enum is [Flex::Start] otherwise [false]
- `fn is_end(self: &Self) -> bool` - Returns [true] if the enum is [Flex::End] otherwise [false]
- `fn is_center(self: &Self) -> bool` - Returns [true] if the enum is [Flex::Center] otherwise [false]
- `fn is_space_between(self: &Self) -> bool` - Returns [true] if the enum is [Flex::SpaceBetween] otherwise [false]
- `fn is_space_evenly(self: &Self) -> bool` - Returns [true] if the enum is [Flex::SpaceEvenly] otherwise [false]
- `fn is_space_around(self: &Self) -> bool` - Returns [true] if the enum is [Flex::SpaceAround] otherwise [false]

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<Flex, <Self as ::core::str::FromStr>::Err>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<Flex, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



