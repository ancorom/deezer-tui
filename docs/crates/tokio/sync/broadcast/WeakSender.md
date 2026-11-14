**tokio > sync > broadcast > WeakSender**

# Module: sync::broadcast::WeakSender

## Contents

**Functions**

- [`upgrade`](#upgrade) - Tries to convert a `WeakSender` into a [`Sender`].

---

## tokio::sync::broadcast::WeakSender::upgrade

*Function*

Tries to convert a `WeakSender` into a [`Sender`].

This will return `Some` if there are other `Sender` instances alive and
the channel wasn't previously dropped, otherwise `None` is returned.

```rust
fn upgrade(self: &Self) -> Option<Sender<T>>
```



