**crossterm > style > attributes**

# Module: style::attributes

## Contents

**Structs**

- [`Attributes`](#attributes) - a bitset for all possible attributes

---

## crossterm::style::attributes::Attributes

*Struct*

a bitset for all possible attributes

**Tuple Struct**: `()`

**Methods:**

- `fn none() -> Self` - Returns the empty bitset.
- `fn with(self: Self, attribute: Attribute) -> Self` - Returns a copy of the bitset with the given attribute set.
- `fn without(self: Self, attribute: Attribute) -> Self` - Returns a copy of the bitset with the given attribute unset.
- `fn set(self: & mut Self, attribute: Attribute)` - Sets the attribute.
- `fn unset(self: & mut Self, attribute: Attribute)` - Unsets the attribute.
- `fn toggle(self: & mut Self, attribute: Attribute)` - Sets the attribute if it's unset, unset it
- `fn has(self: Self, attribute: Attribute) -> bool` - Returns whether the attribute is set.
- `fn extend(self: & mut Self, attributes: Attributes)` - Sets all the passed attributes. Removes none.
- `fn is_empty(self: Self) -> bool` - Returns whether there is no attribute set.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(arr: &[Attribute]) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, rhs: Attribute) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAnd**
  - `fn bitand(self: Self, rhs: Self) -> Self`
- **From**
  - `fn from(attribute: Attribute) -> Self`
- **BitOr**
  - `fn bitor(self: Self, rhs: Attribute) -> Self`
- **BitOr**
  - `fn bitor(self: Self, rhs: Self) -> Self`
- **BitXor**
  - `fn bitxor(self: Self, rhs: Attribute) -> Self`
- **Default**
  - `fn default() -> Self`
- **BitXor**
  - `fn bitxor(self: Self, rhs: Self) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



