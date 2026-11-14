**ratatui_widgets > table > highlight_spacing**

# Module: table::highlight_spacing

## Contents

**Enums**

- [`HighlightSpacing`](#highlightspacing) - This option allows the user to configure the "highlight symbol" column width spacing

---

## ratatui_widgets::table::highlight_spacing::HighlightSpacing

*Enum*

This option allows the user to configure the "highlight symbol" column width spacing

**Variants:**
- `Always` - Always add spacing for the selection symbol column
- `WhenSelected` - Only add spacing for the selection symbol column if a row is selected
- `Never` - Never add spacing to the selection symbol column, regardless of whether something is

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<HighlightSpacing, <Self as ::core::str::FromStr>::Err>`
- **Default**
  - `fn default() -> Self`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<HighlightSpacing, <Self as ::core::convert::TryFrom>::Error>`



