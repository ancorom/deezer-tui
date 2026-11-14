**tracing_subscriber > reload > Handle**

# Module: reload::Handle

## Contents

**Functions**

- [`modify`](#modify) - Invokes a closure with a mutable reference to the current layer or filter,
- [`reload`](#reload) - Replace the current [`Layer`] or [`Filter`] with the provided `new_value`.

---

## tracing_subscriber::reload::Handle::modify

*Function*

Invokes a closure with a mutable reference to the current layer or filter,
allowing it to be modified in place.

```rust
fn modify<impl FnOnce(&mut L)>(self: &Self, f: impl Trait) -> Result<(), Error>
```



## tracing_subscriber::reload::Handle::reload

*Function*

Replace the current [`Layer`] or [`Filter`] with the provided `new_value`.

[`Handle::reload`] cannot be used with the [`Filtered`] layer; use
[`Handle::modify`] instead (see [this issue] for additional details).

However, if the _only_ the [`Filter`]  needs to be modified, use
`reload::Layer` to wrap the `Filter` directly.

[`Layer`]: crate::layer::Layer
[`Filter`]: crate::layer::Filter
[`Filtered`]: crate::filter::Filtered

[this issue]: https://github.com/tokio-rs/tracing/issues/1629

```rust
fn reload<impl Into<L>>(self: &Self, new_value: impl Trait) -> Result<(), Error>
```



