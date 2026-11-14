**tokio > io > util > copy_bidirectional**

# Module: io::util::copy_bidirectional

## Contents

**Functions**

- [`copy_bidirectional`](#copy_bidirectional) - Copies data in both directions between `a` and `b`.
- [`copy_bidirectional_with_sizes`](#copy_bidirectional_with_sizes) - Copies data in both directions between `a` and `b` using buffers of the specified size.

---

## tokio::io::util::copy_bidirectional::copy_bidirectional

*Function*

Copies data in both directions between `a` and `b`.

This function returns a future that will read from both streams,
writing any data read to the opposing stream.
This happens in both directions concurrently.

If an EOF is observed on one stream, [`shutdown()`] will be invoked on
the other, and reading from that stream will stop. Copying of data in
the other direction will continue.

The future will complete successfully once both directions of communication has been shut down.
A direction is shut down when the reader reports EOF,
at which point [`shutdown()`] is called on the corresponding writer. When finished,
it will return a tuple of the number of bytes copied from a to b
and the number of bytes copied from b to a, in that order.

It uses two 8 KB buffers for transferring bytes between `a` and `b` by default.
To set your own buffers sizes use [`copy_bidirectional_with_sizes()`].

[`shutdown()`]: crate::io::AsyncWriteExt::shutdown

# Errors

Interrupted reads and writes are retried. The future will immediately
return any other read or write error, or any flush or shutdown error. Some
data read from either stream may be lost (not written to the other stream)
in this case.

# Return value

Returns a tuple of bytes copied `a` to `b` and bytes copied `b` to `a`.

```rust
fn copy_bidirectional<A, B>(a: & mut A, b: & mut B) -> io::Result<(u64, u64)>
```



## tokio::io::util::copy_bidirectional::copy_bidirectional_with_sizes

*Function*

Copies data in both directions between `a` and `b` using buffers of the specified size.

This method is the same as the [`copy_bidirectional()`], except that it allows you to set the
size of the internal buffers used when copying data.

# Panics

Panics if either buffer size is zero.

```rust
fn copy_bidirectional_with_sizes<A, B>(a: & mut A, b: & mut B, a_to_b_buf_size: usize, b_to_a_buf_size: usize) -> io::Result<(u64, u64)>
```



