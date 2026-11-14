**tokio > sync > rwlock > read_guard**

# Module: sync::rwlock::read_guard

## Contents

**Structs**

- [`RwLockReadGuard`](#rwlockreadguard) - RAII structure used to release the shared read access of a lock when

---

## tokio::sync::rwlock::read_guard::RwLockReadGuard

*Struct*

RAII structure used to release the shared read access of a lock when
dropped.

This structure is created by the [`read`] method on
[`RwLock`].

[`read`]: method@crate::sync::RwLock::read
[`RwLock`]: struct@crate::sync::RwLock

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn map<F, U>(this: Self, f: F) -> RwLockReadGuard<'a, U>` - Makes a new `RwLockReadGuard` for a component of the locked data.
- `fn try_map<F, U>(this: Self, f: F) -> Result<RwLockReadGuard<'a, U>, Self>` - Attempts to make a new [`RwLockReadGuard`] for a component of the

**Traits:** Send, Sync

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



