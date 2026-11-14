**tracing > subscriber**

# Module: subscriber

## Contents

**Functions**

- [`set_default`](#set_default) - Sets the [`Subscriber`] as the default for the current thread for the
- [`set_global_default`](#set_global_default) - Sets this subscriber as the global default for the duration of the entire program.
- [`with_default`](#with_default) - Sets this [`Subscriber`] as the default for the current thread for the

---

## tracing::subscriber::set_default

*Function*

Sets the [`Subscriber`] as the default for the current thread for the
duration of the lifetime of the returned [`DefaultGuard`].

The default subscriber is used when creating a new [`Span`] or [`Event`].

[`Span`]: super::span::Span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event
[`DefaultGuard`]: super::dispatcher::DefaultGuard

```rust
fn set_default<S>(subscriber: S) -> DefaultGuard
```



## tracing::subscriber::set_global_default

*Function*

Sets this subscriber as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns whether the initialization was successful.

Note: Libraries should *NOT* call `set_global_default()`! That will cause conflicts when
executables try to set them later.

[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

```rust
fn set_global_default<S>(subscriber: S) -> Result<(), SetGlobalDefaultError>
```



## tracing::subscriber::with_default

*Function*

Sets this [`Subscriber`] as the default for the current thread for the
duration of a closure.

The default subscriber is used when creating a new [`Span`] or
[`Event`].


[`Span`]: super::span::Span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

```rust
fn with_default<T, S, impl FnOnce() -> T>(subscriber: S, f: impl Trait) -> T
```



