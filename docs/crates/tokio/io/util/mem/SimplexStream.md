**tokio > io > util > mem > SimplexStream**

# Module: io::util::mem::SimplexStream

## Contents

**Functions**

- [`new_unsplit`](#new_unsplit) - Creates unidirectional buffer that acts like in memory pipe. To create split

---

## tokio::io::util::mem::SimplexStream::new_unsplit

*Function*

Creates unidirectional buffer that acts like in memory pipe. To create split
version with separate reader and writer you can use [`simplex`] function.

The `max_buf_size` argument is the maximum amount of bytes that can be
written to a buffer before it returns `Poll::Pending`.

# Panics

This function panics if `max_buf_size` is 0.

```rust
fn new_unsplit(max_buf_size: usize) -> SimplexStream
```



