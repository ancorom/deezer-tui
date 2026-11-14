**tokio > runtime > task > id**

# Module: runtime::task::id

## Contents

**Structs**

- [`Id`](#id) - An opaque ID that uniquely identifies a task relative to all other currently

**Functions**

- [`id`](#id) - Returns the [`Id`] of the currently running task.
- [`try_id`](#try_id) - Returns the [`Id`] of the currently running task, or `None` if called outside

---

## tokio::runtime::task::id::Id

*Struct*

An opaque ID that uniquely identifies a task relative to all other currently
running tasks.

A task's ID may be re-used for another task only once *both* of the
following happen:
1. The task itself exits.
2. There is no active [`JoinHandle`] associated with this task.

A [`JoinHandle`] is considered active in the following situations:
- You are explicitly holding a [`JoinHandle`], [`AbortHandle`], or
  `tokio_util::task::AbortOnDropHandle`.
- The task is being tracked by a [`JoinSet`] or `tokio_util::task::JoinMap`.

# Notes

- Task IDs are *not* sequential, and do not indicate the order in which
  tasks are spawned, what runtime a task is spawned on, or any other data.
- The task ID of the currently running task can be obtained from inside the
  task via the [`task::try_id()`](crate::task::try_id()) and
  [`task::id()`](crate::task::id()) functions and from outside the task via
  the [`JoinHandle::id()`](crate::task::JoinHandle::id()) function.

[`JoinHandle`]: crate::task::JoinHandle
[`AbortHandle`]: crate::task::AbortHandle
[`JoinSet`]: crate::task::JoinSet

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tokio::runtime::task::id::id

*Function*

Returns the [`Id`] of the currently running task.

# Panics

This function panics if called from outside a task. Please note that calls
to `block_on` do not have task IDs, so the method will panic if called from
within a call to `block_on`. For a version of this function that doesn't
panic, see [`task::try_id()`](crate::runtime::task::try_id()).

[task ID]: crate::task::Id

```rust
fn id() -> Id
```



## tokio::runtime::task::id::try_id

*Function*

Returns the [`Id`] of the currently running task, or `None` if called outside
of a task.

This function is similar to  [`task::id()`](crate::runtime::task::id()), except
that it returns `None` rather than panicking if called outside of a task
context.

[task ID]: crate::task::Id

```rust
fn try_id() -> Option<Id>
```



