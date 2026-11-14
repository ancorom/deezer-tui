**tokio > task > coop > unconstrained**

# Module: task::coop::unconstrained

## Contents

**Structs**

- [`Unconstrained`](#unconstrained) - Future for the [`unconstrained`](unconstrained) method.

**Functions**

- [`unconstrained`](#unconstrained) - Turn off cooperative scheduling for a future. The future will never be forced to yield by

---

## tokio::task::coop::unconstrained::Unconstrained

*Struct*

Future for the [`unconstrained`](unconstrained) method.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## tokio::task::coop::unconstrained::unconstrained

*Function*

Turn off cooperative scheduling for a future. The future will never be forced to yield by
Tokio. Using this exposes your service to starvation if the unconstrained future never yields
otherwise.

See also the usage example in the [task module](index.html#unconstrained).

```rust
fn unconstrained<F>(inner: F) -> Unconstrained<F>
```



