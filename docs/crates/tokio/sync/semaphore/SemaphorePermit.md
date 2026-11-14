**tokio > sync > semaphore > SemaphorePermit**

# Module: sync::semaphore::SemaphorePermit

## Contents

**Functions**

- [`forget`](#forget) - Forgets the permit **without** releasing it back to the semaphore.

---

## tokio::sync::semaphore::SemaphorePermit::forget

*Function*

Forgets the permit **without** releasing it back to the semaphore.
This can be used to reduce the amount of permits available from a
semaphore.

# Examples

```
use std::sync::Arc;
use tokio::sync::Semaphore;

let sem = Arc::new(Semaphore::new(10));
{
    let permit = sem.try_acquire_many(5).unwrap();
    assert_eq!(sem.available_permits(), 5);
    permit.forget();
}

// Since we forgot the permit, available permits won't go back to its initial value
// even after the permit is dropped.
assert_eq!(sem.available_permits(), 5);
```

```rust
fn forget(self: Self)
```



