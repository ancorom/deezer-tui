**tokio > sync > mutex > MutexGuard**

# Module: sync::mutex::MutexGuard

## Contents

**Functions**

- [`map`](#map) - Makes a new [`MappedMutexGuard`] for a component of the locked data.

---

## tokio::sync::mutex::MutexGuard::map

*Function*

Makes a new [`MappedMutexGuard`] for a component of the locked data.

This operation cannot fail as the [`MutexGuard`] passed in already locked the mutex.

This is an associated function that needs to be used as `MutexGuard::map(...)`. A method
would interfere with methods of the same name on the contents of the locked data.

# Examples

```
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Foo(u32);

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let foo = Mutex::new(Foo(1));

{
    let mut mapped = MutexGuard::map(foo.lock().await, |f| &mut f.0);
    *mapped = 2;
}

assert_eq!(Foo(2), *foo.lock().await);
# }
```

[`MutexGuard`]: struct@MutexGuard
[`MappedMutexGuard`]: struct@MappedMutexGuard

```rust
fn map<U, F>(this: Self, f: F) -> MappedMutexGuard<'a, U>
```



