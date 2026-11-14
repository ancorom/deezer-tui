**tokio > sync > rwlock > owned_write_guard**

# Module: sync::rwlock::owned_write_guard

## Contents

**Structs**

- [`OwnedRwLockWriteGuard`](#ownedrwlockwriteguard) - Owned RAII structure used to release the exclusive write access of a lock when

---

## tokio::sync::rwlock::owned_write_guard::OwnedRwLockWriteGuard

*Struct*

Owned RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by the [`write_owned`] method
on [`RwLock`].

[`write_owned`]: method@crate::sync::RwLock::write_owned
[`RwLock`]: struct@crate::sync::RwLock

**Generic Parameters:**
- T

**Methods:**

- `fn map<F, U>(this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, U>` - Makes a new [`OwnedRwLockMappedWriteGuard`] for a component of the locked
- `fn downgrade_map<F, U>(this: Self, f: F) -> OwnedRwLockReadGuard<T, U>` - Makes a new [`OwnedRwLockReadGuard`] for a component of the locked data.
- `fn try_map<F, U>(this: Self, f: F) -> Result<OwnedRwLockMappedWriteGuard<T, U>, Self>` - Attempts to make a new [`OwnedRwLockMappedWriteGuard`] for a component
- `fn try_downgrade_map<F, U>(this: Self, f: F) -> Result<OwnedRwLockReadGuard<T, U>, Self>` - Attempts to make a new [`OwnedRwLockReadGuard`] for a component of
- `fn into_mapped(this: Self) -> OwnedRwLockMappedWriteGuard<T>` - Converts this `OwnedRwLockWriteGuard` into an
- `fn downgrade(self: Self) -> OwnedRwLockReadGuard<T>` - Atomically downgrades a write lock into a read lock without allowing
- `fn rwlock(this: &Self) -> &Arc<RwLock<T>>` - Returns a reference to the original `Arc<RwLock>`.

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



