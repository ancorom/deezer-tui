**ratatui_core > buffer > cell > Cell**

# Module: buffer::cell::Cell

## Contents

**Functions**

- [`merge_symbol`](#merge_symbol) - Merges the symbol of the cell with the one already on the cell, using the provided
- [`set_diff_option`](#set_diff_option) - Sets cell [`CellDiffOption`].

---

## ratatui_core::buffer::cell::Cell::merge_symbol

*Function*

Merges the symbol of the cell with the one already on the cell, using the provided
[`MergeStrategy`].

Merges [Box Drawing Unicode block] characters to create a single character representing
their combination, useful for [border collapsing]. Currently limited to box drawing
characters, with potential future support for others.

Merging may not be perfect due to Unicode limitations; some symbol combinations might not
produce a valid character. [`MergeStrategy`] defines how to handle such cases, e.g.,
`Exact` for valid merges only, or `Fuzzy` for close matches.

If the cell has no symbol set, it will set the symbol to the provided one rather than
merging.

# Example

```
# use ratatui_core::buffer::Cell;
use ratatui_core::symbols::merge::MergeStrategy;

assert_eq!(
    Cell::new("┘")
        .merge_symbol("┏", MergeStrategy::Exact)
        .symbol(),
    "╆",
);

assert_eq!(
    Cell::new("╭")
        .merge_symbol("┘", MergeStrategy::Fuzzy)
        .symbol(),
    "┼",
);
```

[border collapsing]: https://ratatui.rs/recipes/layout/collapse-borders/
[Box Drawing Unicode block]: https://en.wikipedia.org/wiki/Box_Drawing

```rust
fn merge_symbol(self: & mut Self, symbol: &str, strategy: MergeStrategy) -> & mut Self
```



## ratatui_core::buffer::cell::Cell::set_diff_option

*Function*

Sets cell [`CellDiffOption`].

The diff options are for dealing with cells that are wider than a unit, that should always
be updated, or that should not be updated at all (skip output due to preceding wider
cells).

```rust
fn set_diff_option(self: & mut Self, diff_option: CellDiffOption) -> & mut Self
```



