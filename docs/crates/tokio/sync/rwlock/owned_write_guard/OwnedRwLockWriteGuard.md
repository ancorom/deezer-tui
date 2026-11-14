**tokio > sync > rwlock > owned_write_guard > OwnedRwLockWriteGuard**

# Module: sync::rwlock::owned_write_guard::OwnedRwLockWriteGuard

## Contents

**Functions**

- [`map`](#map) - Makes a new [`OwnedRwLockMappedWriteGuard`] for a component of the locked

---

## tokio::sync::rwlock::owned_write_guard::OwnedRwLockWriteGuard::map

*Function*

Makes a new [`OwnedRwLockMappedWriteGuard`] for a component of the locked
data.

This operation cannot fail as the `OwnedRwLockWriteGuard` passed in
already locked the data.

This is an associated function that needs to be used as
`OwnedRwLockWriteGuard::map(..)`. A method would interfere with methods
of the same name on the contents of the locked data.

# Examples

```
use std::sync::Arc;
use tokio::sync::{RwLock, OwnedRwLockWriteGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Foo(u32);

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = Arc::new(RwLock::new(Foo(1)));

{
    let lock = Arc::clone(&lock);
    let mut mapped = OwnedRwLockWriteGuard::map(lock.write_owned().await, |f| &mut f.0);
    *mapped = 2;
}

assert_eq!(Foo(2), *lock.read().await);
# }
```

```rust
fn map<F, U>(this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, U>
```



