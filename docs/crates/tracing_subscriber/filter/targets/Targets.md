**tracing_subscriber > filter > targets > Targets**

# Module: filter::targets::Targets

## Contents

**Functions**

- [`iter`](#iter) - Returns an iterator over the [target]-[`LevelFilter`] pairs in this filter.
- [`with_default`](#with_default) - Sets the default level to enable for spans and events whose targets did
- [`with_target`](#with_target) - Enables spans and events with [target]s starting with the provided target
- [`with_targets`](#with_targets) - Adds [target]s from an iterator of [target]-[`LevelFilter`] pairs to this filter.

---

## tracing_subscriber::filter::targets::Targets::iter

*Function*

Returns an iterator over the [target]-[`LevelFilter`] pairs in this filter.

The order of iteration is undefined.

# Examples

```
use tracing_subscriber::filter::{Targets, LevelFilter};
use tracing_core::Level;

let filter = Targets::new()
    .with_target("my_crate", Level::INFO)
    .with_target("my_crate::interesting_module", Level::DEBUG);

let mut targets: Vec<_> = filter.iter().collect();
targets.sort();

assert_eq!(targets, vec![
    ("my_crate", LevelFilter::INFO),
    ("my_crate::interesting_module", LevelFilter::DEBUG),
]);
```

[target]: tracing_core::Metadata::target

```rust
fn iter(self: &Self) -> Iter
```



## tracing_subscriber::filter::targets::Targets::with_default

*Function*

Sets the default level to enable for spans and events whose targets did
not match any of the configured prefixes.

By default, this is [`LevelFilter::OFF`]. This means that spans and
events will only be enabled if they match one of the configured target
prefixes. If this is changed to a different [`LevelFilter`], spans and
events with targets that did not match any of the configured prefixes
will be enabled if their level is at or below the provided level.

```rust
fn with_default<impl Into<LevelFilter>>(self: Self, level: impl Trait) -> Self
```



## tracing_subscriber::filter::targets::Targets::with_target

*Function*

Enables spans and events with [target]s starting with the provided target
prefix if they are at or below the provided [`LevelFilter`].

# Examples

```
use tracing_subscriber::filter;
use tracing_core::Level;

let filter = filter::Targets::new()
    // Enable the `INFO` level for anything in `my_crate`
    .with_target("my_crate", Level::INFO)
    // Enable the `DEBUG` level for a specific module.
    .with_target("my_crate::interesting_module", Level::DEBUG);
# drop(filter);
```

[`LevelFilter::OFF`] can be used to disable a particular target:
```
use tracing_subscriber::filter::{Targets, LevelFilter};
use tracing_core::Level;

let filter = Targets::new()
    .with_target("my_crate", Level::INFO)
    // Disable all traces from `annoying_module`.
    .with_target("my_crate::interesting_module", LevelFilter::OFF);
# drop(filter);
```

[target]: tracing_core::Metadata::target

```rust
fn with_target<impl Into<String>, impl Into<LevelFilter>>(self: Self, target: impl Trait, level: impl Trait) -> Self
```



## tracing_subscriber::filter::targets::Targets::with_targets

*Function*

Adds [target]s from an iterator of [target]-[`LevelFilter`] pairs to this filter.

# Examples

```
use tracing_subscriber::filter;
use tracing_core::Level;

let filter = filter::Targets::new()
    .with_targets(vec![
        ("my_crate", Level::INFO),
        ("my_crate::some_module", Level::DEBUG),
        ("my_crate::other_module::cool_stuff", Level::TRACE),
        ("other_crate", Level::WARN)
    ]);
# drop(filter);
```

[`LevelFilter::OFF`] can be used to disable a particular target:
```
use tracing_subscriber::filter::{Targets, LevelFilter};
use tracing_core::Level;

let filter = Targets::new()
    .with_target("my_crate", Level::INFO)
    // Disable all traces from `annoying_module`.
    .with_target("my_crate::interesting_module", LevelFilter::OFF);
# drop(filter);
```

[target]: tracing_core::Metadata::target

```rust
fn with_targets<T, L, impl IntoIterator<Item = (T, L)>>(self: Self, targets: impl Trait) -> Self
```



