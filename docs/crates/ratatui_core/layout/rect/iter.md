**ratatui_core > layout > rect > iter**

# Module: layout::rect::iter

## Contents

**Structs**

- [`Columns`](#columns) - An iterator over columns within a `Rect`.
- [`Positions`](#positions) - An iterator over positions within a `Rect`.
- [`Rows`](#rows) - An iterator over rows within a `Rect`.

---

## ratatui_core::layout::rect::iter::Columns

*Struct*

An iterator over columns within a `Rect`.

**Methods:**

- `fn new(rect: Rect) -> Self` - Creates a new `Columns` iterator.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>` - Retrieves the next column within the `Rect`.
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>` - Retrieves the previous column within the `Rect`.



## ratatui_core::layout::rect::iter::Positions

*Struct*

An iterator over positions within a `Rect`.

The iterator will yield all positions within the `Rect` in a row-major order.

**Methods:**

- `fn new(rect: Rect) -> Self` - Creates a new `Positions` iterator.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>` - Retrieves the next position within the `Rect`.
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## ratatui_core::layout::rect::iter::Rows

*Struct*

An iterator over rows within a `Rect`.

**Methods:**

- `fn new(rect: Rect) -> Self` - Creates a new `Rows` iterator.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>` - Retrieves the next row within the `Rect`.
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>` - Retrieves the previous row within the `Rect`.



