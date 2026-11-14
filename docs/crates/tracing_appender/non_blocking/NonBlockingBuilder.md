**tracing_appender > non_blocking > NonBlockingBuilder**

# Module: non_blocking::NonBlockingBuilder

## Contents

**Functions**

- [`lossy`](#lossy) - Sets whether `NonBlocking` should be lossy or not.

---

## tracing_appender::non_blocking::NonBlockingBuilder::lossy

*Function*

Sets whether `NonBlocking` should be lossy or not.

If set to `true`, logs will be dropped when the buffered limit is reached. If `false`, backpressure
will be exerted on senders, blocking them until the buffer has capacity again.

By default, the built `NonBlocking` will be lossy.

```rust
fn lossy(self: Self, is_lossy: bool) -> NonBlockingBuilder
```



