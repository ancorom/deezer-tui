**reqwest > proxy**

# Module: proxy

## Contents

**Structs**

- [`NoProxy`](#noproxy) - A configuration for filtering out requests that shouldn't be proxied
- [`Proxy`](#proxy) - Configuration of a proxy that a `Client` should pass requests to.

**Traits**

- [`IntoProxy`](#intoproxy) - Trait used for converting into a proxy scheme. This trait supports

---

## reqwest::proxy::IntoProxy

*Trait*

Trait used for converting into a proxy scheme. This trait supports
parsing from a URL-like type, whilst also supporting proxy schemes
built directly using the factory methods.

**Methods:**

- `into_proxy`



## reqwest::proxy::NoProxy

*Struct*

A configuration for filtering out requests that shouldn't be proxied

**Methods:**

- `fn from_env() -> Option<NoProxy>` - Returns a new no-proxy configuration based on environment variables (or `None` if no variables are set)
- `fn from_string(no_proxy_list: &str) -> Option<Self>` - Returns a new no-proxy configuration based on a `no_proxy` string (or `None` if no variables

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## reqwest::proxy::Proxy

*Struct*

Configuration of a proxy that a `Client` should pass requests to.

A `Proxy` has a couple pieces to it:

- a URL of how to talk to the proxy
- rules on what `Client` requests should be directed to the proxy

For instance, let's look at `Proxy::http`:

```rust
# fn run() -> Result<(), Box<dyn std::error::Error>> {
let proxy = reqwest::Proxy::http("https://secure.example")?;
# Ok(())
# }
```

This proxy will intercept all HTTP requests, and make use of the proxy
at `https://secure.example`. A request to `http://hyper.rs` will talk
to your proxy. A request to `https://hyper.rs` will not.

Multiple `Proxy` rules can be configured for a `Client`. The `Client` will
check each `Proxy` in the order it was added. This could mean that a
`Proxy` added first with eager intercept rules, such as `Proxy::all`,
would prevent a `Proxy` later in the list from ever working, so take care.

By enabling the `"socks"` feature it is possible to use a socks proxy:
```rust
# fn run() -> Result<(), Box<dyn std::error::Error>> {
let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000")?;
# Ok(())
# }
```

**Methods:**

- `fn http<U>(proxy_scheme: U) -> crate::Result<Proxy>` - Proxy all HTTP traffic to the passed URL.
- `fn https<U>(proxy_scheme: U) -> crate::Result<Proxy>` - Proxy all HTTPS traffic to the passed URL.
- `fn all<U>(proxy_scheme: U) -> crate::Result<Proxy>` - Proxy **all** traffic to the passed URL.
- `fn custom<F, U>(fun: F) -> Proxy` - Provide a custom function to determine what traffic to proxy to where.
- `fn basic_auth(self: Self, username: &str, password: &str) -> Proxy` - Set the `Proxy-Authorization` header using Basic auth.
- `fn custom_http_auth(self: Self, header_value: HeaderValue) -> Proxy` - Set the `Proxy-Authorization` header to a specified value.
- `fn headers(self: Self, headers: HeaderMap) -> Proxy` - Adds a Custom Headers to Proxy
- `fn no_proxy(self: Self, no_proxy: Option<NoProxy>) -> Proxy` - Adds a `No Proxy` exclusion list to this Proxy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



