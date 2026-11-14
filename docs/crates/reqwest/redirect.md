**reqwest > redirect**

# Module: redirect

## Contents

**Structs**

- [`Action`](#action) - An action to perform when a redirect status code is found.
- [`Attempt`](#attempt) - A type that holds information on the next request and previous requests
- [`Policy`](#policy) - A type that controls the policy on how to handle the following of redirects.

---

## reqwest::redirect::Action

*Struct*

An action to perform when a redirect status code is found.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## reqwest::redirect::Attempt

*Struct*

A type that holds information on the next request and previous requests
in redirect chain.

**Generic Parameters:**
- 'a

**Methods:**

- `fn status(self: &Self) -> StatusCode` - Get the type of redirect.
- `fn url(self: &Self) -> &Url` - Get the next URL to redirect to.
- `fn previous(self: &Self) -> &[Url]` - Get the list of previous URLs that have already been requested in this chain.
- `fn follow(self: Self) -> Action` - Returns an action meaning reqwest should follow the next URL.
- `fn stop(self: Self) -> Action` - Returns an action meaning reqwest should not follow the next URL.
- `fn error<E>(self: Self, error: E) -> Action` - Returns an action failing the redirect with an error.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## reqwest::redirect::Policy

*Struct*

A type that controls the policy on how to handle the following of redirects.

The default value will catch redirect loops, and has a maximum of 10
redirects it will follow in a chain before returning an error.

- `limited` can be used have the same as the default behavior, but adjust
  the allowed maximum redirect hops in a chain.
- `none` can be used to disable all redirect behavior.
- `custom` can be used to create a customized policy.

**Methods:**

- `fn limited(max: usize) -> Self` - Create a `Policy` with a maximum number of redirects.
- `fn none() -> Self` - Create a `Policy` that does not follow any redirect.
- `fn custom<T>(policy: T) -> Self` - Create a custom `Policy` using the passed function.
- `fn redirect(self: &Self, attempt: Attempt) -> Action` - Apply this policy to a given [`Attempt`] to produce a [`Action`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Policy`



