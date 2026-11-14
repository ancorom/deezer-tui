**tokio > sync > rwlock > write_guard > RwLockWriteGuard**

# Module: sync::rwlock::write_guard::RwLockWriteGuard

## Contents

**Functions**

- [`map`](#map) - Makes a new [`RwLockMappedWriteGuard`] for a component of the locked data.
- [`try_map`](#try_map) - Attempts to make a new [`RwLockMappedWriteGuard`] for a component of

---

## tokio::sync::rwlock::write_guard::RwLockWriteGuard::map

*Function*

Makes a new [`RwLockMappedWriteGuard`] for a component of the locked data.

This operation cannot fail as the `RwLockWriteGuard` passed in already
locked the data.

This is an associated function that needs to be used as
`RwLockWriteGuard::map(..)`. A method would interfere with methods of
the same name on the contents of the locked data.

This is an asynchronous version of [`RwLockWriteGuard::map`] from the
[`parking_lot` crate].

[`RwLockMappedWriteGuard`]: struct@crate::sync::RwLockMappedWriteGuard
[`RwLockWriteGuard::map`]: https://docs.rs/lock_api/latest/lock_api/struct.RwLockWriteGuard.html#method.map
[`parking_lot` crate]: https://crates.io/crates/parking_lot

# Examples

```
use tokio::sync::{RwLock, RwLockWriteGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Foo(u32);

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = RwLock::new(Foo(1));

{
    let mut mapped = RwLockWriteGuard::map(lock.write().await, |f| &mut f.0);
    *mapped = 2;
}

assert_eq!(Foo(2), *lock.read().await);
# }
```

```rust
fn map<F, U>(this: Self, f: F) -> RwLockMappedWriteGuard<'a, U>
```



## tokio::sync::rwlock::write_guard::RwLockWriteGuard::try_map

*Function*

Attempts to make a new [`RwLockMappedWriteGuard`] for a component of
the locked data. The original guard is returned if the closure returns
`None`.

This operation cannot fail as the `RwLockWriteGuard` passed in already
locked the data.

This is an associated function that needs to be
used as `RwLockWriteGuard::try_map(...)`. A method would interfere with
methods of the same name on the contents of the locked data.

This is an asynchronous version of [`RwLockWriteGuard::try_map`] from
the [`parking_lot` crate].

[`RwLockMappedWriteGuard`]: struct@crate::sync::RwLockMappedWriteGuard
[`RwLockWriteGuard::try_map`]: https://docs.rs/lock_api/latest/lock_api/struct.RwLockWriteGuard.html#method.try_map
[`parking_lot` crate]: https://crates.io/crates/parking_lot

# Examples

```
use tokio::sync::{RwLock, RwLockWriteGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Foo(u32);

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = RwLock::new(Foo(1));

{
    let guard = lock.write().await;
    let mut guard = RwLockWriteGuard::try_map(guard, |f| Some(&mut f.0)).expect("should not fail");
    *guard = 2;
}

assert_eq!(Foo(2), *lock.read().await);
# }
```

```rust
fn try_map<F, U>(this: Self, f: F) -> Result<RwLockMappedWriteGuard<'a, U>, Self>
```



