**ratatui_core > layout > constraint > Constraint**

# Module: layout::constraint::Constraint

## Contents

**Functions**

- [`apply`](#apply)
- [`from_fills`](#from_fills) - Convert an iterator of proportional factors into a vector of constraints
- [`from_lengths`](#from_lengths) - Convert an iterator of lengths into a vector of constraints
- [`from_maxes`](#from_maxes) - Convert an iterator of maxes into a vector of constraints
- [`from_mins`](#from_mins) - Convert an iterator of mins into a vector of constraints
- [`from_percentages`](#from_percentages) - Convert an iterator of percentages into a vector of constraints
- [`from_ratios`](#from_ratios) - Convert an iterator of ratios into a vector of constraints

---

## ratatui_core::layout::constraint::Constraint::apply

*Function*

```rust
fn apply(self: &Self, length: u16) -> u16
```



## ratatui_core::layout::constraint::Constraint::from_fills

*Function*

Convert an iterator of proportional factors into a vector of constraints

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

# let area = Rect::default();
let constraints = Constraint::from_fills([1, 2, 3]);
let layout = Layout::default().constraints(constraints).split(area);
```

```rust
fn from_fills<T>(proportional_factors: T) -> Vec<Self>
```



## ratatui_core::layout::constraint::Constraint::from_lengths

*Function*

Convert an iterator of lengths into a vector of constraints

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

# let area = Rect::default();
let constraints = Constraint::from_lengths([1, 2, 3]);
let layout = Layout::default().constraints(constraints).split(area);
```

```rust
fn from_lengths<T>(lengths: T) -> Vec<Self>
```



## ratatui_core::layout::constraint::Constraint::from_maxes

*Function*

Convert an iterator of maxes into a vector of constraints

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

# let area = Rect::default();
let constraints = Constraint::from_maxes([1, 2, 3]);
let layout = Layout::default().constraints(constraints).split(area);
```

```rust
fn from_maxes<T>(maxes: T) -> Vec<Self>
```



## ratatui_core::layout::constraint::Constraint::from_mins

*Function*

Convert an iterator of mins into a vector of constraints

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

# let area = Rect::default();
let constraints = Constraint::from_mins([1, 2, 3]);
let layout = Layout::default().constraints(constraints).split(area);
```

```rust
fn from_mins<T>(mins: T) -> Vec<Self>
```



## ratatui_core::layout::constraint::Constraint::from_percentages

*Function*

Convert an iterator of percentages into a vector of constraints

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

# let area = Rect::default();
let constraints = Constraint::from_percentages([25, 50, 25]);
let layout = Layout::default().constraints(constraints).split(area);
```

```rust
fn from_percentages<T>(percentages: T) -> Vec<Self>
```



## ratatui_core::layout::constraint::Constraint::from_ratios

*Function*

Convert an iterator of ratios into a vector of constraints

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

# let area = Rect::default();
let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
let layout = Layout::default().constraints(constraints).split(area);
```

```rust
fn from_ratios<T>(ratios: T) -> Vec<Self>
```



