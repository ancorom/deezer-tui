**ratatui_core > text > grapheme**

# Module: text::grapheme

## Contents

**Structs**

- [`StyledGrapheme`](#styledgrapheme) - A grapheme associated to a style.

---

## ratatui_core::text::grapheme::StyledGrapheme

*Struct*

A grapheme associated to a style.
Note that, although `StyledGrapheme` is the smallest divisible unit of text,
it actually is not a member of the text type hierarchy (`Text` -> `Line` -> `Span`).
It is a separate type used mostly for rendering purposes. A `Span` consists of components that
can be split into `StyledGrapheme`s, but it does not contain a collection of `StyledGrapheme`s.

**Generic Parameters:**
- 'a

**Fields:**
- `symbol: &'a str`
- `style: crate::style::Style`

**Methods:**

- `fn new<S>(symbol: &'a str, style: S) -> Self` - Creates a new `StyledGrapheme` with the given symbol and style.
- `fn is_whitespace(self: &Self) -> bool`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



