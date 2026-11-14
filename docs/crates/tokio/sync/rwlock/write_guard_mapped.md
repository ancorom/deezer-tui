**tokio > sync > rwlock > write_guard_mapped**

# Module: sync::rwlock::write_guard_mapped

## Contents

**Structs**

- [`RwLockMappedWriteGuard`](#rwlockmappedwriteguard) - RAII structure used to release the exclusive write access of a lock when

---

## tokio::sync::rwlock::write_guard_mapped::RwLockMappedWriteGuard

*Struct*

RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by [mapping] an [`RwLockWriteGuard`]. It is a
separate type from `RwLockWriteGuard` to disallow downgrading a mapped
guard, since doing so can cause undefined behavior.

[mapping]: method@crate::sync::RwLockWriteGuard::map
[`RwLockWriteGuard`]: struct@crate::sync::RwLockWriteGuard

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn map<F, U>(this: Self, f: F) -> RwLockMappedWriteGuard<'a, U>` - Makes a new `RwLockMappedWriteGuard` for a component of the locked data.
- `fn try_map<F, U>(this: Self, f: F) -> Result<RwLockMappedWriteGuard<'a, U>, Self>` - Attempts to make a new [`RwLockMappedWriteGuard`] for a component of

**Traits:** Send, Sync

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



