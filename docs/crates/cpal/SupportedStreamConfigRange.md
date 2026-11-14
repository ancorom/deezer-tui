**cpal > SupportedStreamConfigRange**

# Module: SupportedStreamConfigRange

## Contents

**Functions**

- [`try_with_standard_sample_rate`](#try_with_standard_sample_rate) - Retrieve a [`SupportedStreamConfig`] with a standard sample rate.

---

## cpal::SupportedStreamConfigRange::try_with_standard_sample_rate

*Function*

Retrieve a [`SupportedStreamConfig`] with a standard sample rate.

The preferred rate is selected in order:
1. 48 kHz as the dominant native rate on modern hardware.
2. 44.1 kHz as the standard CD/consumer rate.

Returns `None` if neither standard rate falls within the range specified within this
[`SupportedStreamConfigRange`] instance.

```rust
fn try_with_standard_sample_rate(self: Self) -> Option<SupportedStreamConfig>
```



