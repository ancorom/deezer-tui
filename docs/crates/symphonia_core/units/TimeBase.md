**symphonia_core > units > TimeBase**

# Module: units::TimeBase

## Contents

**Functions**

- [`calc_duration`](#calc_duration) - Calculate a `Time` upto nanosecond precision from the provided `Duration` using `self` as
- [`calc_time`](#calc_time) - Calculate a `Time` upto nanosecond precision from the provided `TimeStamp` using `self` as

---

## symphonia_core::units::TimeBase::calc_duration

*Function*

Calculate a `Time` upto nanosecond precision from the provided `Duration` using `self` as
the conversion factor. Returns `None` if an overflow occurs.

```rust
fn calc_duration(self: &Self, dur: Duration) -> Option<Time>
```



## symphonia_core::units::TimeBase::calc_time

*Function*

Calculate a `Time` upto nanosecond precision from the provided `TimeStamp` using `self` as
the conversion factor. Returns `None` if an overflow occurs.

```rust
fn calc_time(self: &Self, ts: Timestamp) -> Option<Time>
```



