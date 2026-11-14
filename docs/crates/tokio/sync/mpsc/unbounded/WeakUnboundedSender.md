**tokio > sync > mpsc > unbounded > WeakUnboundedSender**

# Module: sync::mpsc::unbounded::WeakUnboundedSender

## Contents

**Functions**

- [`upgrade`](#upgrade) - Tries to convert a `WeakUnboundedSender` into an [`UnboundedSender`].

---

## tokio::sync::mpsc::unbounded::WeakUnboundedSender::upgrade

*Function*

Tries to convert a `WeakUnboundedSender` into an [`UnboundedSender`].
This will return `Some` if there are other `Sender` instances alive and
the channel wasn't previously dropped, otherwise `None` is returned.

```rust
fn upgrade(self: &Self) -> Option<UnboundedSender<T>>
```



