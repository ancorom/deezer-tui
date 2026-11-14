**tokio > sync > rwlock > owned_write_guard_mapped**

# Module: sync::rwlock::owned_write_guard_mapped

## Contents

**Structs**

- [`OwnedRwLockMappedWriteGuard`](#ownedrwlockmappedwriteguard) - Owned RAII structure used to release the exclusive write access of a lock when

---

## tokio::sync::rwlock::owned_write_guard_mapped::OwnedRwLockMappedWriteGuard

*Struct*

Owned RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by [mapping] an [`OwnedRwLockWriteGuard`]. It is a
separate type from `OwnedRwLockWriteGuard` to disallow downgrading a mapped
guard, since doing so can cause undefined behavior.

[mapping]: method@crate::sync::OwnedRwLockWriteGuard::map
[`OwnedRwLockWriteGuard`]: struct@crate::sync::OwnedRwLockWriteGuard

**Generic Parameters:**
- T
- U

**Methods:**

- `fn map<F, V>(this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, V>` - Makes a new `OwnedRwLockMappedWriteGuard` for a component of the locked
- `fn try_map<F, V>(this: Self, f: F) -> Result<OwnedRwLockMappedWriteGuard<T, V>, Self>` - Attempts to make a new `OwnedRwLockMappedWriteGuard` for a component
- `fn rwlock(this: &Self) -> &Arc<RwLock<T>>` - Returns a reference to the original `Arc<RwLock>`.

**Traits:** Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &U`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut U`
- **Drop**
  - `fn drop(self: & mut Self)`



