**ratatui_core > buffer > diff**

# Module: buffer::diff

## Contents

**Structs**

- [`BufferDiff`](#bufferdiff) - A zero-allocation iterator over the differences between two buffers of the same width.

---

## ratatui_core::buffer::diff::BufferDiff

*Struct*

A zero-allocation iterator over the differences between two buffers of the same width.

Yields `(x, y, &Cell)` tuples for each cell in `next` that differs from the corresponding cell
in `prev`. Handles multi-width characters (including VS16 emoji trailing cells) and
[`CellDiffOption`] directives.

**Generic Parameters:**
- 'prev
- 'next

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



