**image > animation > Delay**

# Module: animation::Delay

## Contents

**Functions**

- [`from_saturating_duration`](#from_saturating_duration) - Convert from a duration, clamped between 0 and an implemented defined maximum.

---

## image::animation::Delay::from_saturating_duration

*Function*

Convert from a duration, clamped between 0 and an implemented defined maximum.

The maximum is *at least* `i32::MAX` milliseconds. It should be noted that the accuracy of
the result may be relative and very large delays have a coarse resolution.

# Examples

```
use std::time::Duration;
use image::Delay;

let duration = Duration::from_millis(20);
let delay = Delay::from_saturating_duration(duration);
```

```rust
fn from_saturating_duration(duration: Duration) -> Self
```



