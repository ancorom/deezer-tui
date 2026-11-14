**tokio > sync > rwlock > owned_read_guard**

# Module: sync::rwlock::owned_read_guard

## Contents

**Structs**

- [`OwnedRwLockReadGuard`](#ownedrwlockreadguard) - Owned RAII structure used to release the shared read access of a lock when

---

## tokio::sync::rwlock::owned_read_guard::OwnedRwLockReadGuard

*Struct*

Owned RAII structure used to release the shared read access of a lock when
dropped.

This structure is created by the [`read_owned`] method on
[`RwLock`].

[`read_owned`]: method@crate::sync::RwLock::read_owned
[`RwLock`]: struct@crate::sync::RwLock

**Generic Parameters:**
- T
- U

**Methods:**

- `fn map<F, V>(this: Self, f: F) -> OwnedRwLockReadGuard<T, V>` - Makes a new `OwnedRwLockReadGuard` for a component of the locked data.
- `fn try_map<F, V>(this: Self, f: F) -> Result<OwnedRwLockReadGuard<T, V>, Self>` - Attempts to make a new [`OwnedRwLockReadGuard`] for a component of the
- `fn rwlock(this: &Self) -> &Arc<RwLock<T>>` - Returns a reference to the original `Arc<RwLock>`.

**Traits:** Sync, Send

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &U`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



