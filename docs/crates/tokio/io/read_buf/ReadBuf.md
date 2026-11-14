**tokio > io > read_buf > ReadBuf**

# Module: io::read_buf::ReadBuf

## Contents

**Functions**

- [`assume_init`](#assume_init) - Asserts that the first `n` unfilled bytes of the buffer are initialized.

---

## tokio::io::read_buf::ReadBuf::assume_init

*Function*

Asserts that the first `n` unfilled bytes of the buffer are initialized.

`ReadBuf` assumes that bytes are never de-initialized, so this method does nothing when called with fewer
bytes than are already known to be initialized.

# Safety

The caller must ensure that `n` unfilled bytes of the buffer have already been initialized.

```rust
fn assume_init(self: & mut Self, n: usize)
```



