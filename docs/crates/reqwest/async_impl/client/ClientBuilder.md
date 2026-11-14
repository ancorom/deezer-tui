**reqwest > async_impl > client > ClientBuilder**

# Module: async_impl::client::ClientBuilder

## Contents

**Functions**

- [`cookie_provider`](#cookie_provider) - Set the persistent cookie store for the client.
- [`cookie_store`](#cookie_store) - Enable a persistent cookie store for the client.
- [`pool_idle_timeout`](#pool_idle_timeout) - Set an optional timeout for idle sockets being kept-alive.
- [`pool_max_idle_per_host`](#pool_max_idle_per_host) - Sets the maximum idle connection per host allowed in the pool.
- [`tls_backend_preconfigured`](#tls_backend_preconfigured) - Use a preconfigured TLS backend.
- [`tls_backend_rustls`](#tls_backend_rustls) - Force using the Rustls TLS backend.
- [`tls_certs_merge`](#tls_certs_merge) - Add custom certificate roots.
- [`tls_certs_only`](#tls_certs_only) - Use only the provided certificate roots.
- [`tls_crls_only`](#tls_crls_only) - Add multiple certificate revocation lists.
- [`tls_danger_accept_invalid_certs`](#tls_danger_accept_invalid_certs) - Controls the use of certificate validation.
- [`tls_danger_accept_invalid_hostnames`](#tls_danger_accept_invalid_hostnames) - Controls the use of hostname verification.
- [`tls_version_max`](#tls_version_max) - Set the maximum allowed TLS version for connections.
- [`tls_version_min`](#tls_version_min) - Set the minimum required TLS version for connections.

---

## reqwest::async_impl::client::ClientBuilder::cookie_provider

*Function*

Set the persistent cookie store for the client.

Cookies received in responses will be passed to this store, and
additional requests will query this store for cookies.

By default, no cookie store is used. It is **not** necessary to also call
[cookie_store(true)](crate::ClientBuilder::cookie_store) if [cookie_provider(my_cookie_store)](crate::ClientBuilder::cookie_provider) is used; calling
[cookie_store(true)](crate::ClientBuilder::cookie_store) _after_ [cookie_provider(my_cookie_store)](crate::ClientBuilder::cookie_provider) will result
in the provided `my_cookie_store` being **overridden** with a default implementation.

# Optional

This requires the optional `cookies` feature to be enabled.

```rust
fn cookie_provider<C>(self: Self, cookie_store: Arc<C>) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::cookie_store

*Function*

Enable a persistent cookie store for the client.

Cookies received in responses will be preserved and included in
additional requests.

By default, no cookie store is used. Enabling the cookie store
with `cookie_store(true)` will set the store to a default implementation.
It is **not** necessary to call [cookie_store(true)](crate::ClientBuilder::cookie_store) if [cookie_provider(my_cookie_store)](crate::ClientBuilder::cookie_provider)
is used; calling [cookie_store(true)](crate::ClientBuilder::cookie_store) _after_ [cookie_provider(my_cookie_store)](crate::ClientBuilder::cookie_provider) will result
in the provided `my_cookie_store` being **overridden** with a default implementation.

# Optional

This requires the optional `cookies` feature to be enabled.

```rust
fn cookie_store(self: Self, enable: bool) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::pool_idle_timeout

*Function*

Set an optional timeout for idle sockets being kept-alive.

Pass `None` to disable timeout.

Default is 90 seconds.

```rust
fn pool_idle_timeout<D>(self: Self, val: D) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::pool_max_idle_per_host

*Function*

Sets the maximum idle connection per host allowed in the pool.

Default is `usize::MAX` (no limit).

```rust
fn pool_max_idle_per_host(self: Self, max: usize) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_backend_preconfigured

*Function*

Use a preconfigured TLS backend.

If the passed `Any` argument is not a TLS backend that reqwest
understands, the `ClientBuilder` will error when calling `build`.

# Advanced

<div class="warning">

There is no semver stability on the internals of this method. Use at
your own risk.

</div>

This is an advanced option, and can be somewhat brittle. Usage requires
keeping the preconfigured TLS argument version in sync with reqwest,
since version mismatches will result in an "unknown" TLS backend.

If possible, it's preferable to use the methods on `ClientBuilder`
to configure reqwest's TLS.

# Optional

This requires one of the optional features `native-tls` or
`rustls(-...)` to be enabled.

```rust
fn tls_backend_preconfigured<impl Any>(self: Self, tls: impl Trait) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_backend_rustls

*Function*

Force using the Rustls TLS backend.

Since multiple TLS backends can be optionally enabled, this option will
force the `rustls` backend to be used for this `Client`.

# Optional

This requires the optional `rustls(-...)` feature to be enabled.

```rust
fn tls_backend_rustls(self: Self) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_certs_merge

*Function*

Add custom certificate roots.

This can be used to connect to a server that has a self-signed
certificate for example.

This optional attempts to merge with any native or built-in roots.

# Errors

If the selected TLS backend or verifier does not support merging
certificates, the builder will return an error.

# Optional

This requires the optional `default-tls`, `native-tls`, or `rustls(-...)`
feature to be enabled.

```rust
fn tls_certs_merge<impl IntoIterator<Item = Certificate>>(self: Self, certs: impl Trait) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_certs_only

*Function*

Use only the provided certificate roots.

This can be used to connect to a server that has a self-signed
certificate for example.

This option disables any native or built-in roots, and **only** uses
the roots provided to this method.

# Optional

This requires the optional `default-tls`, `native-tls`, or `rustls(-...)`
feature to be enabled.

```rust
fn tls_certs_only<impl IntoIterator<Item = Certificate>>(self: Self, certs: impl Trait) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_crls_only

*Function*

Add multiple certificate revocation lists.

# Errors

This only works if also using only provided root certificates. This
cannot work with the native verifier.

If CRLs are added but `tls_certs_only()` is not called, the builder
will return an error.

# Optional

This requires the `rustls(-...)` Cargo feature enabled.

```rust
fn tls_crls_only<impl IntoIterator<Item = CertificateRevocationList>>(self: Self, crls: impl Trait) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_danger_accept_invalid_certs

*Function*

Controls the use of certificate validation.

Defaults to `false`.

# Warning

You should think very carefully before using this method. If
invalid certificates are trusted, *any* certificate for *any* site
will be trusted for use. This includes expired certificates. This
introduces significant vulnerabilities, and should only be used
as a last resort.

# Optional

This requires the optional `default-tls`, `native-tls`, or `rustls(-...)`
feature to be enabled.

```rust
fn tls_danger_accept_invalid_certs(self: Self, accept_invalid_certs: bool) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_danger_accept_invalid_hostnames

*Function*

Controls the use of hostname verification.

Defaults to `false`.

# Warning

You should think very carefully before you use this method. If
hostname verification is not used, any valid certificate for any
site will be trusted for use from any other. This introduces a
significant vulnerability to man-in-the-middle attacks.

# Errors

Depending on the TLS backend and verifier, this might not work with
native certificates, only those added with [`ClientBuilder::tls_certs_only()`].

# Optional

This requires the optional `default-tls`, `native-tls`, or `rustls(-...)`
feature to be enabled.

```rust
fn tls_danger_accept_invalid_hostnames(self: Self, accept_invalid_hostname: bool) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_version_max

*Function*

Set the maximum allowed TLS version for connections.

By default, there's no maximum.

On Apple platforms, a value of `tls::Version::TLS_1_3` may cause requests
to fall back to TLS 1.2 if allowed by `tls_version_min`, or fail (with error
-9830) with the `native-tls` backend due to lack of TLS 1.3 support.

# Errors

Cannot set a maximum outside the protocol versions supported by
`rustls` with the `rustls` backend.

# Optional

This requires the optional `default-tls`, `native-tls`, or `rustls(-...)`
feature to be enabled.

```rust
fn tls_version_max(self: Self, version: tls::Version) -> ClientBuilder
```



## reqwest::async_impl::client::ClientBuilder::tls_version_min

*Function*

Set the minimum required TLS version for connections.

By default, the TLS backend's own default is used.

On Apple platforms, a value of `tls::Version::TLS_1_3` may cause requests
to fail (with error -9830) with the `native-tls` backend due to lack of
TLS 1.3 support.

# Optional

This requires the optional `default-tls`, `native-tls`, or `rustls(-...)`
feature to be enabled.

```rust
fn tls_version_min(self: Self, version: tls::Version) -> ClientBuilder
```



