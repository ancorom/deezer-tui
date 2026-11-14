**reqwest > retry**

# Module: retry

## Contents

**Structs**

- [`Builder`](#builder) - Builder to configure retries

**Functions**

- [`for_host`](#for_host) - Create a retry builder with a request scope.
- [`never`](#never) - Create a retry policy that will never retry any request.

---

## reqwest::retry::Builder

*Struct*

Builder to configure retries

Construct with [`for_host()`].

**Methods:**

- `fn scoped<impl scope::Scope>(scope: impl Trait) -> Self` - Create a scoped retry policy.
- `fn no_budget(self: Self) -> Self` - Set no retry budget.
- `fn max_extra_load(self: Self, extra_percent: f32) -> Self` - Sets the max extra load the budget will allow.
- `fn max_retries_per_request(self: Self, max: u32) -> Self` - Set the max retries allowed per request.
- `fn classify_fn<F>(self: Self, func: F) -> Self` - Provide a classifier to determine if a request should be retried.
- `fn classify<impl classify::Classify>(self: Self, classifier: impl Trait) -> Self` - Provide a classifier to determine if a request should be retried.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## reqwest::retry::for_host

*Function*

Create a retry builder with a request scope.

To provide a scope that isn't a closure, use the more general
[`Builder::scoped()`].

```rust
fn for_host<S>(host: S) -> Builder
```



## reqwest::retry::never

*Function*

Create a retry policy that will never retry any request.

This is useful for disabling the `Client`s default behavior of retrying
protocol nacks.

```rust
fn never() -> Builder
```



