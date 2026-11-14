**tokio > time > instant**

# Module: time::instant

## Contents

**Structs**

- [`Instant`](#instant) - A measurement of a monotonically nondecreasing clock.

---

## tokio::time::instant::Instant

*Struct*

A measurement of a monotonically nondecreasing clock.
Opaque and useful only with `Duration`.

Instants are always guaranteed to be no less than any previously measured
instant when created, and are often useful for tasks such as measuring
benchmarks or timing how long an operation takes.

Note, however, that instants are not guaranteed to be **steady**. In other
words, each tick of the underlying clock may not be the same length (e.g.
some seconds may be longer than others). An instant may jump forwards or
experience time dilation (slow down or speed up), but it will never go
backwards.

Instants are opaque types that can only be compared to one another. There is
no method to get "the number of seconds" from an instant. Instead, it only
allows measuring the duration between two instants (or comparing two
instants).

The size of an `Instant` struct may vary depending on the target operating
system.

# Note

This type wraps the inner `std` variant and is used to align the Tokio
clock for uses of `now()`. This can be useful for testing where you can
take advantage of `time::pause()` and `time::advance()`.

**Methods:**

- `fn now() -> Instant` - Returns an instant corresponding to "now".
- `fn from_std(std: std::time::Instant) -> Instant` - Create a `tokio::time::Instant` from a `std::time::Instant`.
- `fn into_std(self: Self) -> std::time::Instant` - Convert the value into a `std::time::Instant`.
- `fn duration_since(self: &Self, earlier: Instant) -> Duration` - Returns the amount of time elapsed from another instant to this one, or
- `fn checked_duration_since(self: &Self, earlier: Instant) -> Option<Duration>` - Returns the amount of time elapsed from another instant to this one, or
- `fn saturating_duration_since(self: &Self, earlier: Instant) -> Duration` - Returns the amount of time elapsed from another instant to this one, or
- `fn elapsed(self: &Self) -> Duration` - Returns the amount of time elapsed since this instant was created,
- `fn checked_add(self: &Self, duration: Duration) -> Option<Instant>` - Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
- `fn checked_sub(self: &Self, duration: Duration) -> Option<Instant>` - Returns `Some(t)` where `t` is the time `self - duration` if `t` can be

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(time: std::time::Instant) -> Instant`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Sub**
  - `fn sub(self: Self, rhs: Instant) -> Duration`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> Instant`
- **Add**
  - `fn add(self: Self, other: Duration) -> Instant`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



