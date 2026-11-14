**tokio > runtime > handle**

# Module: runtime::handle

## Contents

**Structs**

- [`EnterGuard`](#enterguard) - Runtime context guard.
- [`Handle`](#handle) - Handle to the runtime.
- [`TryCurrentError`](#trycurrenterror) - Error returned by `try_current` when no Runtime has been started

---

## tokio::runtime::handle::EnterGuard

*Struct*

Runtime context guard.

Returned by [`Runtime::enter`] and [`Handle::enter`], the context guard exits
the runtime context on drop.

[`Runtime::enter`]: fn@crate::runtime::Runtime::enter

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::runtime::handle::Handle

*Struct*

Handle to the runtime.

The handle is internally reference-counted and can be freely cloned. A handle can be
obtained using the [`Runtime::handle`] method.

[`Runtime::handle`]: crate::runtime::Runtime::handle()

**Methods:**

- `fn enter(self: &Self) -> EnterGuard` - Enters the runtime context. This allows you to construct types that must
- `fn current() -> Self` - Returns a `Handle` view over the currently running `Runtime`.
- `fn try_current() -> Result<Self, TryCurrentError>` - Returns a Handle view over the currently running Runtime
- `fn spawn<F>(self: &Self, future: F) -> JoinHandle<<F as >::Output>` - Spawns a future onto the Tokio runtime.
- `fn spawn_blocking<F, R>(self: &Self, func: F) -> JoinHandle<R>` - Runs the provided function on an executor dedicated to blocking
- `fn block_on<F>(self: &Self, future: F) -> <F as >::Output` - Runs a future to completion on this `Handle`'s associated `Runtime`.
- `fn runtime_flavor(self: &Self) -> RuntimeFlavor` - Returns the flavor of the current `Runtime`.
- `fn id(self: &Self) -> runtime::Id` - Returns the [`Id`] of the current `Runtime`.
- `fn name(self: &Self) -> Option<&str>` - Returns the name of the current `Runtime`.
- `fn metrics(self: &Self) -> RuntimeMetrics` - Returns a view that lets you get information about how the runtime

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## tokio::runtime::handle::TryCurrentError

*Struct*

Error returned by `try_current` when no Runtime has been started

**Methods:**

- `fn is_missing_context(self: &Self) -> bool` - Returns true if the call failed because there is currently no runtime in
- `fn is_thread_local_destroyed(self: &Self) -> bool` - Returns true if the call failed because the Tokio context thread-local

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



