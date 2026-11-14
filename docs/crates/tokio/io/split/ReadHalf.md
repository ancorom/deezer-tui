**tokio > io > split > ReadHalf**

# Module: io::split::ReadHalf

## Contents

**Functions**

- [`is_pair_of`](#is_pair_of) - Checks if this `ReadHalf` and some `WriteHalf` were split from the same
- [`unsplit`](#unsplit) - Reunites with a previously split `WriteHalf`.

---

## tokio::io::split::ReadHalf::is_pair_of

*Function*

Checks if this `ReadHalf` and some `WriteHalf` were split from the same
stream.

```rust
fn is_pair_of(self: &Self, other: &WriteHalf<T>) -> bool
```



## tokio::io::split::ReadHalf::unsplit

*Function*

Reunites with a previously split `WriteHalf`.

# Panics

If this `ReadHalf` and the given `WriteHalf` do not originate from the
same `split` operation this method will panic.
This can be checked ahead of time by calling [`is_pair_of()`](Self::is_pair_of).

```rust
fn unsplit(self: Self, wr: WriteHalf<T>) -> T
```



