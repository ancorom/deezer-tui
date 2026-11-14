**ratatui_core > symbols > merge**

# Module: symbols::merge

## Contents

**Enums**

- [`MergeStrategy`](#mergestrategy) - A strategy for merging two symbols into one.

---

## ratatui_core::symbols::merge::MergeStrategy

*Enum*

A strategy for merging two symbols into one.

This enum defines how two symbols should be merged together, allowing for different behaviors
when combining symbols, such as replacing the previous symbol, merging them if an exact match
exists, or using a fuzzy match to find the closest representation.

This is useful for [collapsing borders] in layouts, where multiple symbols may need to be
combined to create a single, coherent border representation.

Not all combinations of box drawing symbols can be represented as a single unicode character, as
many of them are not defined in the [Box Drawing Unicode block]. This means that some merging
strategies will not yield a valid unicode character. The [`MergeStrategy::Replace`] strategy
will be used as a fallback in such cases, replacing the previous symbol with the next one.

Specifically, the following combinations of box drawing symbols are not defined in the [Box
Drawing Unicode block]:

- Combining any dashed segments with any non dashed segments (e.g. `╎` with `─` or `━`).
- Combining any rounded segments with any other segments (e.g. `╯` with `─` or `━`).
- Combining any double segments with any thick segments (e.g. `═` with `┃` or `━`).
- Combining some double segments with some plain segments (e.g. `┐` with `╔`).

The merging strategies include:

- [`Self::Replace`]: Replaces the previous symbol with the next one.
- [`Self::Exact`]: Merges symbols only if an exact composite unicode character exists, falling
  back to [`Self::Replace`] if not.
- [`Self::Fuzzy`]: Merges symbols even if an exact composite unicode character doesn't exist,
  using the closest match, and falling back to [`Self::Exact`] if necessary.

See [`Cell::merge_symbol`] for how to use this strategy in practice, and
[`Block::merge_borders`] for a more concrete example of merging borders in a layout.

# Examples

```
# use ratatui_core::symbols::merge::MergeStrategy;

assert_eq!(MergeStrategy::Replace.merge("│", "━"), "━");
assert_eq!(MergeStrategy::Exact.merge("│", "─"), "┼");
assert_eq!(MergeStrategy::Fuzzy.merge("┘", "╔"), "╬");
```

[Box Drawing Unicode block]: https://en.wikipedia.org/wiki/Box_Drawing
[collapsing borders]: https://ratatui.rs/recipes/layout/collapse-borders
[`Block::merge_borders`]:
    https://docs.rs/ratatui/latest/ratatui/widgets/block/struct.Block.html#method.merge_borders
[`Cell::merge_symbol`]: crate::buffer::Cell::merge_symbol

**Variants:**
- `Replace` - Replaces the previous symbol with the next one.
- `Exact` - Merges symbols only if an exact composite unicode character exists.
- `Fuzzy` - Merges symbols even if an exact composite unicode character doesn't exist, using the closest

**Methods:**

- `fn merge<'a>(self: Self, prev: &'a str, next: &'a str) -> &'a str` - Merges two symbols using this merge strategy.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



