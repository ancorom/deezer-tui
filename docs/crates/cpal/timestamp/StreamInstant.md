**cpal > timestamp > StreamInstant**

# Module: timestamp::StreamInstant

## Contents

**Functions**

- [`checked_add`](#checked_add) - Returns `Some(t)` where `t` is `self + duration`, or `None` if the result cannot be
- [`checked_sub`](#checked_sub) - Returns `Some(t)` where `t` is `self - duration`, or `None` if the result cannot be

---

## cpal::timestamp::StreamInstant::checked_add

*Function*

Returns `Some(t)` where `t` is `self + duration`, or `None` if the result cannot be
represented as a `StreamInstant`.

```rust
fn checked_add(self: &Self, duration: Duration) -> Option<Self>
```



## cpal::timestamp::StreamInstant::checked_sub

*Function*

Returns `Some(t)` where `t` is `self - duration`, or `None` if the result cannot be
represented as a `StreamInstant` (i.e. would be negative).

```rust
fn checked_sub(self: &Self, duration: Duration) -> Option<Self>
```



