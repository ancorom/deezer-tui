**tokio > sync > rwlock > write_guard**

# Module: sync::rwlock::write_guard

## Contents

**Structs**

- [`RwLockWriteGuard`](#rwlockwriteguard) - RAII structure used to release the exclusive write access of a lock when

---

## tokio::sync::rwlock::write_guard::RwLockWriteGuard

*Struct*

RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by the [`write`] method
on [`RwLock`].

[`write`]: method@crate::sync::RwLock::write
[`RwLock`]: struct@crate::sync::RwLock

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn map<F, U>(this: Self, f: F) -> RwLockMappedWriteGuard<'a, U>` - Makes a new [`RwLockMappedWriteGuard`] for a component of the locked data.
- `fn downgrade_map<F, U>(this: Self, f: F) -> RwLockReadGuard<'a, U>` - Makes a new [`RwLockReadGuard`] for a component of the locked data.
- `fn try_map<F, U>(this: Self, f: F) -> Result<RwLockMappedWriteGuard<'a, U>, Self>` - Attempts to make a new [`RwLockMappedWriteGuard`] for a component of
- `fn try_downgrade_map<F, U>(this: Self, f: F) -> Result<RwLockReadGuard<'a, U>, Self>` - Attempts to make a new [`RwLockReadGuard`] for a component of
- `fn into_mapped(this: Self) -> RwLockMappedWriteGuard<'a, T>` - Converts this `RwLockWriteGuard` into an `RwLockMappedWriteGuard`. This
- `fn downgrade(self: Self) -> RwLockReadGuard<'a, T>` - Atomically downgrades a write lock into a read lock without allowing

**Traits:** Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Drop**
  - `fn drop(self: & mut Self)`



