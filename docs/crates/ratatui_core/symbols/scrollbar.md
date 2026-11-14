**ratatui_core > symbols > scrollbar**

# Module: symbols::scrollbar

## Contents

**Structs**

- [`Set`](#set) - Scrollbar Set

**Constants**

- [`DOUBLE_HORIZONTAL`](#double_horizontal)
- [`DOUBLE_VERTICAL`](#double_vertical)
- [`HORIZONTAL`](#horizontal)
- [`VERTICAL`](#vertical)

---

## ratatui_core::symbols::scrollbar::DOUBLE_HORIZONTAL

*Constant*: `Set`



## ratatui_core::symbols::scrollbar::DOUBLE_VERTICAL

*Constant*: `Set`



## ratatui_core::symbols::scrollbar::HORIZONTAL

*Constant*: `Set`



## ratatui_core::symbols::scrollbar::Set

*Struct*

Scrollbar Set
```text
<--▮------->
^  ^   ^   ^
│  │   │   └ end
│  │   └──── track
│  └──────── thumb
└─────────── begin
```

**Generic Parameters:**
- 'a

**Fields:**
- `track: &'a str`
- `thumb: &'a str`
- `begin: &'a str`
- `end: &'a str`

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
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_core::symbols::scrollbar::VERTICAL

*Constant*: `Set`



