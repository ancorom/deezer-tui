**tracing_subscriber > util**

# Module: util

## Contents

**Structs**

- [`TryInitError`](#tryiniterror) - Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized.

**Traits**

- [`SubscriberInitExt`](#subscriberinitext) - Extension trait adding utility methods for subscriber initialization.

---

## tracing_subscriber::util::SubscriberInitExt

*Trait*

Extension trait adding utility methods for subscriber initialization.

This trait provides extension methods to make configuring and setting a
[default subscriber] more ergonomic. It is automatically implemented for all
types that can be converted into a [trace dispatcher]. Since `Dispatch`
implements `From<T>` for all `T: Subscriber`, all `Subscriber`
implementations will implement this extension trait as well. Types which
can be converted into `Subscriber`s, such as builders that construct a
`Subscriber`, may implement `Into<Dispatch>`, and will also receive an
implementation of this trait.

[default subscriber]: https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber
[trace dispatcher]: https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html

**Methods:**

- `set_default`: Sets `self` as the [default subscriber] in the current scope, returning a
- `try_init`: Attempts to set `self` as the [global default subscriber] in the current
- `init`: Attempts to set `self` as the [global default subscriber] in the current



## tracing_subscriber::util::TryInitError

*Struct*

Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized.

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



