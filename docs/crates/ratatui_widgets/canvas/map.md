**ratatui_widgets > canvas > map**

# Module: canvas::map

## Contents

**Structs**

- [`Map`](#map) - A world map. It represents the world using the [EPSG:4326 coordinate reference system](https://en.wikipedia.org/wiki/EPSG_Geodetic_Parameter_Dataset).

**Enums**

- [`MapResolution`](#mapresolution) - Defines how many points are going to be used to draw a [`Map`].

---

## ratatui_widgets::canvas::map::Map

*Struct*

A world map. It represents the world using the [EPSG:4326 coordinate reference system](https://en.wikipedia.org/wiki/EPSG_Geodetic_Parameter_Dataset).

A world map can be rendered with different [resolutions](MapResolution) and [colors](Color).

**Fields:**
- `resolution: MapResolution` - The resolution of the map.
- `color: ratatui_core::style::Color` - Map color

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Shape**
  - `fn draw(self: &Self, painter: & mut Painter)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_widgets::canvas::map::MapResolution

*Enum*

Defines how many points are going to be used to draw a [`Map`].

You generally want a [high](MapResolution::High) resolution map.

**Variants:**
- `Low` - A lesser resolution for the [`Map`] [`Shape`].
- `High` - A higher resolution for the [`Map`] [`Shape`].

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<MapResolution, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<MapResolution, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



