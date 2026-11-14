**tokio > future > maybe_done**

# Module: future::maybe_done

## Contents

**Enums**

- [`MaybeDone`](#maybedone) - A future that may have completed.

**Functions**

- [`maybe_done`](#maybe_done) - Wraps a future into a `MaybeDone`.

---

## tokio::future::maybe_done::MaybeDone

*Enum*

A future that may have completed.

**Generic Parameters:**
- Fut

**Variants:**
- `Future{ future: Fut }` - A not-yet-completed future.
- `Done{ output: <Fut as >::Output }` - The output of the completed future.
- `Gone` - The empty variant after the result of a [`MaybeDone`] has been



## tokio::future::maybe_done::maybe_done

*Function*

Wraps a future into a `MaybeDone`.

```rust
fn maybe_done<F>(future: F) -> MaybeDone<<F as >::IntoFuture>
```



