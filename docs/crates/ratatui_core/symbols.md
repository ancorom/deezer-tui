**ratatui_core > symbols**

# Module: symbols

## Contents

**Modules**

- [`bar`](#bar)
- [`block`](#block)
- [`border`](#border)
- [`braille`](#braille) - Braille symbols.
- [`half_block`](#half_block)
- [`line`](#line)
- [`marker`](#marker)
- [`merge`](#merge) - This module provides strategies for merging symbols in a layout.
- [`pixel`](#pixel) - Pseudo-pixel symbols: quadrant, sextant and octant characters.
- [`scrollbar`](#scrollbar)
- [`shade`](#shade)

---

## Module: bar



## Module: block



## Module: border



## Module: braille

Braille symbols.

Note that the symbols are not listed according to their unicode codepoint but according to the
corresponding bit pattern in row-major order.



## Module: half_block



## Module: line



## Module: marker



## Module: merge

This module provides strategies for merging symbols in a layout.

It defines the [`MergeStrategy`] enum, which allows for different behaviors when combining
symbols, such as replacing the previous symbol, merging them if an exact match exists, or using
a fuzzy match to find the closest representation.

The merging strategies are useful for [collapsing borders] in layouts, where multiple symbols
may need to be combined to create a single, coherent border representation.

[collapsing borders]: https://ratatui.rs/recipes/layout/collapse-borders



## Module: pixel

Pseudo-pixel symbols: quadrant, sextant and octant characters.

Note that the symbols are not listed according to their unicode codepoint but according to the
corresponding bit pattern in row-major order.



## Module: scrollbar



## Module: shade



