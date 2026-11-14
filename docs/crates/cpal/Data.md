**cpal > Data**

# Module: Data

## Contents

**Functions**

- [`as_slice`](#as_slice) - Access the data as a slice of sample type `T`.
- [`sample_format`](#sample_format) - The sample format of the internal audio data.

---

## cpal::Data::as_slice

*Function*

Access the data as a slice of sample type `T`.

Returns `None` if the sample type does not match the expected sample format.

```rust
fn as_slice<T>(self: &Self) -> Option<&[T]>
```



## cpal::Data::sample_format

*Function*

The sample format of the internal audio data.

```rust
fn sample_format(self: &Self) -> SampleFormat
```



