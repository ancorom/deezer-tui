**symphonia_core > units**

# Module: units

## Contents

**Structs**

- [`Delta`](#delta) - A `Delta` is a signed difference between timestamps.
- [`Duration`](#duration) - A `Duration` is a positive span of time expressed in stream timebase units.
- [`Time`](#time) - `Time` represents an instant in time from an arbitrary epoch with nanosecond precision.
- [`TimeBase`](#timebase) - A `TimeBase` is the conversion factor between time, expressed in seconds, and a `TimeStamp` or
- [`Timestamp`](#timestamp) - A `Timestamp` is an instant in time expressed in stream timebase units.

---

## symphonia_core::units::Delta

*Struct*

A `Delta` is a signed difference between timestamps.

**Tuple Struct**: `()`

**Methods:**

- `fn get(self: Self) -> i64` - Get the underlying unsigned integer representing the duration.
- `fn is_zero(self: Self) -> bool` - Returns `true` if `self` is 0.
- `fn is_negative(self: Self) -> bool` - Returns `true` if `self` is negative and `false` if the delta is zero or positive.
- `fn is_positive(self: Self) -> bool` - Returns `true` if `self` is positive and `false` if the delta is zero or negative.
- `fn checked_abs(self: Self) -> Option<Self>` - Compute the absolute value, returning `None` if it is not representable.
- `fn unsigned_abs(self: Self) -> u64` - Get the delta as an unsigned absolute value.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## symphonia_core::units::Duration

*Struct*

A `Duration` is a positive span of time expressed in stream timebase units.

One duration "tick" is equal to the stream's `TimeBase` in seconds.

**Tuple Struct**: `()`

**Methods:**

- `fn new(dur: u64) -> Self` - Create a duration.
- `fn get(self: Self) -> u64` - Get the underlying unsigned integer representing the duration.
- `fn is_zero(self: Self) -> bool` - Returns `true` if `self` is 0.
- `fn checked_add(self: Self, other: Self) -> Option<Self>` - Add the provided duration to `self`, returning `None` if an overflow occurred.
- `fn checked_div(self: Self, rhs: u64) -> Option<Self>` - Divide `self` by the provided value, returning `None` if `rhs == 0`.
- `fn checked_div_euclid(self: Self, rhs: u64) -> Option<Self>` - Calculate the quotient of Euclidean division of `self` by `rhs`, returning `None` if
- `fn checked_mul(self: Self, rhs: u64) -> Option<Self>` - Multiply `self` by the provided value, returning `None` if an overflow occurred.
- `fn checked_sub(self: Self, other: Self) -> Option<Self>` - Subtract the provided duration from `self`, returning `None` if an overflow occurred.
- `fn saturating_add(self: Self, other: Self) -> Self` - Add the provided duration to `self`, saturating at the numeric bounds instead of
- `fn saturating_div(self: Self, rhs: u64) -> Self` - Divide `self` by the provided value, saturating at the numeric bounds instead of
- `fn saturating_mul(self: Self, rhs: u64) -> Self` - Multiply `self` by the provided value, saturating at the numeric bounds instead of
- `fn saturating_sub(self: Self, other: Self) -> Self` - Subtract the provided duration from `self`, saturating at the numeric bounds instead of
- `fn align_towards_zero(self: Self, align_to: Self) -> Option<Self>` - Align `self` to the nearest multiple of the provided duration that is closest to 0.
- `fn align_down(self: Self, align_to: Self) -> Option<Self>` - Align `self` to the nearest multiple of the provided duration that is closest to negative
- `fn timestamp_from(self: Self, from: Timestamp) -> Option<Timestamp>` - Compute a `Timestamp` from the duration by adding `self` to `from`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(value: u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **TryFrom**
  - `fn try_from(value: i16) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(value: u64) -> Self`
- **TryFrom**
  - `fn try_from(value: i64) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **From**
  - `fn from(value: u8) -> Self`
- **TryFrom**
  - `fn try_from(value: i8) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: u32) -> Self`
- **TryFrom**
  - `fn try_from(value: i32) -> Result<Self, <Self as >::Error>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



## symphonia_core::units::Time

*Struct*

`Time` represents an instant in time from an arbitrary epoch with nanosecond precision.

The duration of time from the epoch is stored as a signed-integer number of seconds and an
unsigned number of nanoseconds. The exact duration from the epoch is the sum of both.

**Methods:**

- `fn try_new(s: i64, ns: u32) -> Option<Self>` - Try to create from a count of seconds and nanoseconds representing the fractional portion of
- `fn try_from_nanos_i128(total_ns: i128) -> Option<Time>` - Try to create from a 128-bit count of total nanoseconds.
- `fn try_from_nanos_u128(total_ns: u128) -> Option<Time>` - Try to create from an unsigned 128-bit count of total nanoseconds.
- `fn try_from_secs_f64(total_secs: f64) -> Option<Time>` - Try to instantiate from a floating-point count of total seconds.
- `fn from_nanos(total_ns: i64) -> Time` - Create from a count of total nanoseconds.
- `fn from_nanos_u64(total_ns: u64) -> Time` - Create from an unsigned count of total nanoseconds.
- `fn from_micros(total_us: i64) -> Time` - Create from a count of total microseconds.
- `fn from_micros_u64(total_us: u64) -> Time` - Create from an unsigned count of total microseconds.
- `fn from_millis(total_ms: i64) -> Time` - Create from a count of total milliseconds.
- `fn from_millis_u64(total_ms: u64) -> Time` - Create from an unsigned count of total milliseconds.
- `fn from_ss(s: u8, ns: u32) -> Option<Time>` - Try to create from a clock time consisting of seconds and nanoseconds (i.e.,
- `fn from_mmss(m: u8, s: u8, ns: u32) -> Option<Time>` - Try to create from a clock time consisting of minute, seconds, and nanoseconds (i.e.,
- `fn from_hhmmss(h: u32, m: u8, s: u8, ns: u32) -> Option<Time>` - Try to create from a clock time consisting of hours, minute, seconds, and nanoseconds (i.e.,
- `fn is_zero(self: &Self) -> bool` - Returns true if zero.
- `fn is_positive(self: &Self) -> bool` - Returns `true` if self is positive and `false` if the number is zero or negative.
- `fn is_negative(self: &Self) -> bool` - Returns `true` if self is positive and `false` if the number is zero or negative.
- `fn as_nanos(self: &Self) -> i128` - Get the time in nanoseconds.
- `fn as_micros(self: &Self) -> i128` - Get the time in whole microseconds.
- `fn as_millis(self: &Self) -> i128` - Get the time in whole milliseconds.
- `fn as_secs(self: &Self) -> i64` - Get the time in whole seconds.
- `fn as_mins(self: &Self) -> i64` - Get the time in whole minutes.
- `fn as_hours(self: &Self) -> i64` - Get the time in whole hours.
- `fn as_secs_f64(self: &Self) -> f64` - Get the time in floating-point seconds.
- `fn parts(self: &Self) -> (i64, u32)` - Decomposes the time into two parts: total whole seconds and a fractional sub-second
- `fn checked_neg(self: Self) -> Option<Self>` - Negate the time, returning `None` if an overflow occurs.
- `fn checked_add_nanos(self: Self, nanos: i64) -> Option<Self>` - Add the provided number of nanoseconds to `self`, returning `None` if an overflow occurred.
- `fn checked_add_micros(self: Self, micros: i64) -> Option<Self>` - Add the provided number of microseconds to `self`, returning `None` if an overflow occurred.
- `fn checked_add_millis(self: Self, millis: i64) -> Option<Self>` - Add the provided number of milliseconds to `self`, returning `None` if an overflow occurred.
- `fn checked_add_secs(self: Self, secs: i64) -> Option<Self>` - Add the provided number of seconds to `self`, returning `None` if an overflow occurred.
- `fn checked_add_mins(self: Self, mins: i64) -> Option<Self>` - Add the provided number of minutes to `self`, returning `None` if an overflow occurred.
- `fn checked_add_hours(self: Self, hours: i64) -> Option<Self>` - Add the provided number of hours to `self`, returning `None` if an overflow occurred.
- `fn checked_sub_nanos(self: Self, nanos: i64) -> Option<Self>` - Subtract the provided number of nanoseconds to `self`, returning `None` if an overflow
- `fn checked_sub_micros(self: Self, micros: i64) -> Option<Self>` - Subtract the provided number of microseconds to `self`, returning `None` if an overflow
- `fn checked_sub_millis(self: Self, millis: i64) -> Option<Self>` - Subtract the provided number of milliseconds to `self`, returning `None` if an overflow
- `fn checked_sub_secs(self: Self, secs: i64) -> Option<Self>` - Subtract the provided number of seconds to `self`, returning `None` if an overflow occurred.
- `fn checked_sub_mins(self: Self, mins: i64) -> Option<Self>` - Subtract the provided number of minutes to `self`, returning `None` if an overflow occurred.
- `fn checked_sub_hours(self: Self, hours: i64) -> Option<Self>` - Subtract the provided number of hours to `self`, returning `None` if an overflow occurred.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(seconds: u8) -> Self`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(seconds: u16) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **From**
  - `fn from(seconds: u32) -> Self`



## symphonia_core::units::TimeBase

*Struct*

A `TimeBase` is the conversion factor between time, expressed in seconds, and a `TimeStamp` or
`Duration`.

In other words, a `TimeBase` is the length in seconds of one tick of a `TimeStamp` or
`Duration`.

**Fields:**
- `numer: core::num::NonZero<u32>` - The numerator.
- `denom: core::num::NonZero<u32>` - The denominator.

**Methods:**

- `fn new(numer: NonZero<u32>, denom: NonZero<u32>) -> Self` - Create a new timebase.
- `fn try_new(numer: u32, denom: u32) -> Option<Self>` - Try to create a new timebase. Returns `None` if either the numerator or denominator are 0.
- `fn from_recip(rate: NonZero<u32>) -> Self` - Create a timebase from the reciprocal of the provided rate.
- `fn try_from_recip(rate: u32) -> Option<Self>` - Try to create a timebase from the reciprocal of the provided rate. Returns `None` if the
- `fn calc_time(self: &Self, ts: Timestamp) -> Option<Time>` - Calculate a `Time` upto nanosecond precision from the provided `TimeStamp` using `self` as
- `fn calc_time_saturating(self: &Self, ts: Timestamp) -> Time` - Calculate a `Time` upto nanosecond precision from the provided `TimeStamp` using `self` as
- `fn calc_duration(self: &Self, dur: Duration) -> Option<Time>` - Calculate a `Time` upto nanosecond precision from the provided `Duration` using `self` as
- `fn calc_duration_saturating(self: &Self, dur: Duration) -> Time` - Calculate a `Time` upto nanosecond precision from the provided `Duration` using `self` as
- `fn calc_timestamp(self: &Self, time: Time) -> Option<Timestamp>` - Calculate a `TimeStamp` from the provided `Time` using `self` as the conversion factor.
- `fn scale(self: Self, factor: f64) -> Option<Self>` - Scale the timebase by a positive, non-zero, floating-point scale factor.
- `fn reduce(self: Self) -> Self` - Reduce the timebase.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## symphonia_core::units::Timestamp

*Struct*

A `Timestamp` is an instant in time expressed in stream timebase units.

One timestamp "tick" is equal to the stream's `TimeBase` in seconds.

**Tuple Struct**: `()`

**Methods:**

- `fn new(ts: i64) -> Self` - Create a timestamp.
- `fn get(self: Self) -> i64` - Get the underlying integer representing the timestamp.
- `fn is_zero(self: Self) -> bool` - Returns `true` if `self` is 0.
- `fn is_negative(self: Self) -> bool` - Returns `true` if `self` is negative and `false` if the timestamp is zero or positive.
- `fn is_positive(self: Self) -> bool` - Returns `true` if `self` is positive and `false` if the timestamp is zero or negative.
- `fn checked_add(self: Self, dur: Duration) -> Option<Self>` - Add the provided duration to `self`, returning `None` if an overflow occurred.
- `fn checked_sub(self: Self, dur: Duration) -> Option<Self>` - Subtract the provided duration from `self`, returning `None` if an overflow occurred.
- `fn saturating_add(self: Self, dur: Duration) -> Self` - Add the provided duration to `self`, saturating at the numeric bounds instead of
- `fn saturating_sub(self: Self, dur: Duration) -> Self` - Subtract the provided duration from `self`, saturating at the numeric bounds instead of
- `fn checked_delta(self: Self, other: Self) -> Option<Delta>` - Computes the difference between `self` and `other`, returning `None` if an overflow
- `fn saturating_delta(self: Self, other: Self) -> Delta` - Computes the difference between `self` and `other`, saturating at the numeric bounds instead
- `fn checked_add_delta(self: Self, delta: Delta) -> Option<Self>` - Add the provided delta to `self`, returning `None` if an overflow occurred.
- `fn checked_sub_delta(self: Self, delta: Delta) -> Option<Self>` - Subtract the provided delta from `self`, returning `None` if an overflow occurred.
- `fn saturating_add_delta(self: Self, delta: Delta) -> Self` - Add the provided delta to `self`, saturating at the numeric bounds instead of
- `fn saturating_sub_delta(self: Self, delta: Delta) -> Self` - Subtract the provided delta from `self`, saturating at the numeric bounds instead of
- `fn abs_delta(self: Self, other: Self) -> Duration` - Calculate the absolute difference in duration between `self` and `other`.
- `fn duration_from(self: Self, other: Self) -> Option<Duration>` - Calculate the duration from `other` to `self`. If `other` is greater-than `self`, returns
- `fn duration_to(self: Self, other: Self) -> Option<Duration>` - Calculate the duration from `self` to `other`. If `other` is less-than `self`, returns
- `fn align_towards_zero(self: Self, align_to: Duration) -> Option<Self>` - Align `self` to the nearest multiple of the provided duration that is closest to 0.
- `fn align_down(self: Self, align_to: Duration) -> Option<Self>` - Align `self` to the nearest multiple of the provided duration that is closest to negative

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: i8) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **TryFrom**
  - `fn try_from(value: u64) -> Result<Self, <Self as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(value: i16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **From**
  - `fn from(value: i64) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: i32) -> Self`
- **From**
  - `fn from(value: u16) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: u32) -> Self`



