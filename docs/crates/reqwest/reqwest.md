**reqwest**

# Module: reqwest

## Contents

**Modules**

- [`cookie`](#cookie) - HTTP Cookies
- [`dns`](#dns) - DNS resolution
- [`redirect`](#redirect) - Redirect Handling
- [`retry`](#retry) - Retry requests
- [`tls`](#tls) - TLS configuration and types

**Functions**

- [`get`](#get) - Shortcut method to quickly make a `GET` request.

---

## Module: cookie

HTTP Cookies



## Module: dns

DNS resolution



## reqwest::get

*Function*

Shortcut method to quickly make a `GET` request.

See also the methods on the [`reqwest::Response`](./struct.Response.html)
type.

**NOTE**: This function creates a new internal `Client` on each call,
and so should not be used if making many requests. Create a
[`Client`](./struct.Client.html) instead.

# Examples

```rust
# async fn run() -> Result<(), reqwest::Error> {
let body = reqwest::get("https://www.rust-lang.org").await?
    .text().await?;
# Ok(())
# }
```

# Errors

This function fails if:

- native TLS backend cannot be initialized
- supplied `Url` cannot be parsed
- there was an error while sending request
- redirect limit was exhausted

```rust
fn get<T>(url: T) -> crate::Result<Response>
```



## Module: redirect

Redirect Handling

By default, a `Client` will automatically handle HTTP redirects, having a
maximum redirect chain of 10 hops. To customize this behavior, a
`redirect::Policy` can be used with a `ClientBuilder`.



## Module: retry

Retry requests

A `Client` has the ability to retry requests, by sending additional copies
to the server if a response is considered retryable.

The [`Builder`] makes it easier to configure what requests to retry, along
with including best practices by default, such as a retry budget.

# Defaults

The default retry behavior of a `Client` is to only retry requests where an
error or low-level protocol NACK is encountered that is known to be safe to
retry. Note however that providing a specific retry policy will override
the default, and you will need to explicitly include that behavior.

All policies default to including a retry budget that permits 20% extra
requests to be sent.

# Scoped

A client's retry policy is scoped. That means that the policy doesn't
apply to all requests, but only those within a user-defined scope.

Since all policies include a budget by default, it doesn't make sense to
apply it on _all_ requests. Rather, the retry history applied by a budget
should likely only be applied to the same host.

# Classifiers

A retry policy needs to be configured with a classifier that determines
if a request should be retried. Knowledge of the destination server's
behavior is required to make a safe classifier. **Requests should not be
retried** if the server cannot safely handle the same request twice, or if
it causes side effects.

Some common properties to check include if the request method is
idempotent, or if the response status code indicates a transient error.



## Module: tls

TLS configuration and types

A `Client` will use transport layer security (TLS) by default to connect to
HTTPS destinations.

# Backends

reqwest supports several TLS backends, enabled with Cargo features.

## default-tls

reqwest will pick a TLS backend by default. This is true when the
`default-tls` feature is enabled.

While it currently uses `rustls`, the feature set is designed to only
enable configuration that is shared among available backends. This allows
reqwest to change the default to `native-tls` (or another) by configuration.

<div class="warning">This feature is enabled by default, and takes
precedence if any other crate enables it. This is true even if you declare
`features = []`. You must set `default-features = false` instead.</div>

Since Cargo features are additive, other crates in your dependency tree can
cause the default backend to be enabled. If you wish to ensure your
`Client` uses a specific backend, call the appropriate builder methods
(such as [`tls_backend_rustls()`][]).

[`tls_backend_rustls()`]: crate::ClientBuilder::tls_backend_rustls()

## native-tls

This backend uses the [native-tls][] crate. That will try to use the system
TLS on Windows and Mac, and OpenSSL on Linux targets.

Enabling the feature explicitly allows for `native-tls`-specific
configuration options.

[native-tls]: https://crates.io/crates/native-tls

## rustls

This backend uses the [rustls][] crate, a TLS library written in Rust.

[rustls]: https://crates.io/crates/rustls

## rustls-no-provider

Like `rustls`, but without a built-in crypto provider. This is useful when
you want to supply your own [rustls CryptoProvider][], for example to use
[ring][] instead of the default `aws-lc-rs`.

**You must install a crypto provider before building a `Client`.** If none
is installed the client will panic at construction time. Install one via
[`CryptoProvider::install_default`][]:

```rust,ignore
rustls::crypto::ring::default_provider()
    .install_default()
    .expect("Failed to install rustls crypto provider");

let client = reqwest::Client::new();
```

[rustls CryptoProvider]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html
[ring]: https://crates.io/crates/ring
[`CryptoProvider::install_default`]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#method.install_default



