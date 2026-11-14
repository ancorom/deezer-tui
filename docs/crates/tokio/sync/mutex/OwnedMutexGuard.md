**tokio > sync > mutex > OwnedMutexGuard**

# Module: sync::mutex::OwnedMutexGuard

## Contents

**Functions**

- [`map`](#map) - Makes a new [`OwnedMappedMutexGuard`] for a component of the locked data.

---

## tokio::sync::mutex::OwnedMutexGuard::map

*Function*

Makes a new [`OwnedMappedMutexGuard`] for a component of the locked data.

This operation cannot fail as the [`OwnedMutexGuard`] passed in already locked the mutex.

This is an associated function that needs to be used as `OwnedMutexGuard::map(...)`. A method
would interfere with methods of the same name on the contents of the locked data.

# Examples

```
use tokio::sync::{Mutex, OwnedMutexGuard};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Foo(u32);

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let foo = Arc::new(Mutex::new(Foo(1)));

{
    let mut mapped = OwnedMutexGuard::map(foo.clone().lock_owned().await, |f| &mut f.0);
    *mapped = 2;
}

assert_eq!(Foo(2), *foo.lock().await);
# }
```

[`OwnedMutexGuard`]: struct@OwnedMutexGuard
[`OwnedMappedMutexGuard`]: struct@OwnedMappedMutexGuard

```rust
fn map<U, F>(this: Self, f: F) -> OwnedMappedMutexGuard<T, U>
```



