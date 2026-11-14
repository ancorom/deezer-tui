**ratatui_core > layout > constraint**

# Module: layout::constraint

## Contents

**Enums**

- [`Constraint`](#constraint) - A constraint that defines the size of a layout element.

---

## ratatui_core::layout::constraint::Constraint

*Enum*

A constraint that defines the size of a layout element.

Constraints are the core mechanism for defining how space should be allocated within a
[`Layout`](crate::layout::Layout). They can specify fixed sizes (length), proportional sizes
(percentage, ratio), size limits (min, max), or proportional fill values for layout elements.
Relative constraints (percentage, ratio) are calculated relative to the entire space being
divided, rather than the space available after applying more fixed constraints (min, max,
length).

Constraints are prioritized in the following order:

1. [`Constraint::Min`]
2. [`Constraint::Max`]
3. [`Constraint::Length`]
4. [`Constraint::Percentage`]
5. [`Constraint::Ratio`]
6. [`Constraint::Fill`]

# Size Calculation

- [`apply`](Self::apply) - Apply the constraint to a length and return the resulting size

# Collection Creation

- [`from_lengths`](Self::from_lengths) - Create a collection of length constraints
- [`from_ratios`](Self::from_ratios) - Create a collection of ratio constraints
- [`from_percentages`](Self::from_percentages) - Create a collection of percentage constraints
- [`from_maxes`](Self::from_maxes) - Create a collection of maximum constraints
- [`from_mins`](Self::from_mins) - Create a collection of minimum constraints
- [`from_fills`](Self::from_fills) - Create a collection of fill constraints

# Conversion and Construction

- [`from(u16)`](Self::from) - Create a [`Length`](Self::Length) constraint from `u16`
- [`from(&Constraint)`](Self::from) - Create from `&Constraint` (copy)
- [`as_ref()`](Self::as_ref) - Get a reference to self
- [`default()`](Self::default) - Create default constraint
  ([`Percentage(100)`](Self::Percentage))

# Examples

`Constraint` provides helper methods to create lists of constraints from various input formats.

```rust
use ratatui_core::layout::Constraint;

// Create a layout with specified lengths for each element
let constraints = Constraint::from_lengths([10, 20, 10]);

// Create a centered layout using ratio or percentage constraints
let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
let constraints = Constraint::from_percentages([25, 50, 25]);

// Create a centered layout with a minimum size constraint for specific elements
let constraints = Constraint::from_mins([0, 100, 0]);

// Create a sidebar layout specifying maximum sizes for the columns
let constraints = Constraint::from_maxes([30, 170]);

// Create a layout with fill proportional sizes for each element
let constraints = Constraint::from_fills([1, 2, 1]);
```

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Variants:**
- `Min(u16)` - Applies a minimum size constraint to the element
- `Max(u16)` - Applies a maximum size constraint to the element
- `Length(u16)` - Applies a length constraint to the element
- `Percentage(u16)` - Applies a percentage of the available space to the element
- `Ratio(u32, u32)` - Applies a ratio of the available space to the element
- `Fill(u16)` - Applies the scaling factor proportional to all other [`Constraint::Fill`] elements

**Methods:**

- `fn is_min(self: &Self) -> bool` - Returns [true] if the enum is [Constraint::Min] otherwise [false]
- `fn is_max(self: &Self) -> bool` - Returns [true] if the enum is [Constraint::Max] otherwise [false]
- `fn is_length(self: &Self) -> bool` - Returns [true] if the enum is [Constraint::Length] otherwise [false]
- `fn is_percentage(self: &Self) -> bool` - Returns [true] if the enum is [Constraint::Percentage] otherwise [false]
- `fn is_ratio(self: &Self) -> bool` - Returns [true] if the enum is [Constraint::Ratio] otherwise [false]
- `fn is_fill(self: &Self) -> bool` - Returns [true] if the enum is [Constraint::Fill] otherwise [false]
- `fn apply(self: &Self, length: u16) -> u16`
- `fn from_lengths<T>(lengths: T) -> Vec<Self>` - Convert an iterator of lengths into a vector of constraints
- `fn from_ratios<T>(ratios: T) -> Vec<Self>` - Convert an iterator of ratios into a vector of constraints
- `fn from_percentages<T>(percentages: T) -> Vec<Self>` - Convert an iterator of percentages into a vector of constraints
- `fn from_maxes<T>(maxes: T) -> Vec<Self>` - Convert an iterator of maxes into a vector of constraints
- `fn from_mins<T>(mins: T) -> Vec<Self>` - Convert an iterator of mins into a vector of constraints
- `fn from_fills<T>(proportional_factors: T) -> Vec<Self>` - Convert an iterator of proportional factors into a vector of constraints

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(constraint: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(length: u16) -> Self` - Convert a `u16` into a [`Constraint::Length`]
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



