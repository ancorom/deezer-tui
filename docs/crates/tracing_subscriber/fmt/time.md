**tracing_subscriber > fmt > time**

# Module: fmt::time

## Contents

**Structs**

- [`SystemTime`](#systemtime) - Retrieve and print the current wall-clock time.
- [`Uptime`](#uptime) - Retrieve and print the relative elapsed wall-clock time since an epoch.

**Functions**

- [`time`](#time) - Returns a new `SystemTime` timestamp provider.
- [`uptime`](#uptime) - Returns a new `Uptime` timestamp provider.

**Traits**

- [`FormatTime`](#formattime) - A type that can measure and format the current time.

---

## tracing_subscriber::fmt::time::FormatTime

*Trait*

A type that can measure and format the current time.

This trait is used by `Format` to include a timestamp with each `Event` when it is logged.

Notable default implementations of this trait are `SystemTime` and `()`. The former prints the
current time as reported by `std::time::SystemTime`, and the latter does not print the current
time at all. `FormatTime` is also automatically implemented for any function pointer with the
appropriate signature.

The full list of provided implementations can be found in [`time`].

[`time`]: self

**Methods:**

- `format_time`: Measure and write out the current time.



## tracing_subscriber::fmt::time::SystemTime

*Struct*

Retrieve and print the current wall-clock time.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FormatTime**
  - `fn format_time(self: &Self, w: & mut Writer) -> fmt::Result`



## tracing_subscriber::fmt::time::Uptime

*Struct*

Retrieve and print the relative elapsed wall-clock time since an epoch.

The `Default` implementation for `Uptime` makes the epoch the current time.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **FormatTime**
  - `fn format_time(self: &Self, w: & mut Writer) -> fmt::Result`
- **From**
  - `fn from(epoch: Instant) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tracing_subscriber::fmt::time::time

*Function*

Returns a new `SystemTime` timestamp provider.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
# fn timer() -> tracing_subscriber::fmt::time::SystemTime {
tracing_subscriber::fmt::time::SystemTime::default()
# }
```

```rust
fn time() -> SystemTime
```



## tracing_subscriber::fmt::time::uptime

*Function*

Returns a new `Uptime` timestamp provider.

With this timer, timestamps will be formatted with the amount of time
elapsed since the timestamp provider was constructed.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
# fn timer() -> tracing_subscriber::fmt::time::Uptime {
tracing_subscriber::fmt::time::Uptime::default()
# }
```

```rust
fn uptime() -> Uptime
```



