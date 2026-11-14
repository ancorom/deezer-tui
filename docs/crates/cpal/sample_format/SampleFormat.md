**cpal > sample_format > SampleFormat**

# Module: sample_format::SampleFormat

## Contents

**Functions**

- [`bits_per_sample`](#bits_per_sample) - Returns the number of bits of a sample of this format. Note that this is
- [`is_float`](#is_float)
- [`sample_size`](#sample_size) - Returns the size in bytes of a sample of this format. This corresponds to

---

## cpal::sample_format::SampleFormat::bits_per_sample

*Function*

Returns the number of bits of a sample of this format. Note that this is
not necessarily the same as the size of the primitive used to represent
this sample format (e.g., I24 has size of i32 but 24 bits per sample).

```rust
fn bits_per_sample(self: &Self) -> u32
```



## cpal::sample_format::SampleFormat::is_float

*Function*

```rust
fn is_float(self: &Self) -> bool
```



## cpal::sample_format::SampleFormat::sample_size

*Function*

Returns the size in bytes of a sample of this format. This corresponds to
the internal size of the rust primitives that are used to represent this
sample format (e.g., i24 has size of i32).

```rust
fn sample_size(self: &Self) -> usize
```



