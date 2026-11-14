**reqwest > async_impl > client**

# Module: async_impl::client

## Contents

**Structs**

- [`Client`](#client) - An asynchronous `Client` to make Requests with.
- [`ClientBuilder`](#clientbuilder) - A `ClientBuilder` can be used to create a `Client` with custom configuration.
- [`Pending`](#pending)

---

## reqwest::async_impl::client::Client

*Struct*

An asynchronous `Client` to make Requests with.

The Client has various configuration values to tweak, but the defaults
are set to what is usually the most commonly desired value. To configure a
`Client`, use `Client::builder()`.

The `Client` holds a connection pool internally to improve performance
by reusing connections and avoiding setup overhead, so it is advised that
you create one and **reuse** it.

You do **not** have to wrap the `Client` in an [`Rc`] or [`Arc`] to **reuse** it,
because it already uses an [`Arc`] internally.

# Connection Pooling

The connection pool can be configured using [`ClientBuilder`] methods
with the `pool_` prefix, such as [`ClientBuilder::pool_idle_timeout`]
and [`ClientBuilder::pool_max_idle_per_host`].

[`Rc`]: std::rc::Rc

**Methods:**

- `fn new() -> Client` - Constructs a new `Client`.
- `fn builder() -> ClientBuilder` - Creates a `ClientBuilder` to configure a `Client`.
- `fn get<U>(self: &Self, url: U) -> RequestBuilder` - Convenience method to make a `GET` request to a URL.
- `fn post<U>(self: &Self, url: U) -> RequestBuilder` - Convenience method to make a `POST` request to a URL.
- `fn put<U>(self: &Self, url: U) -> RequestBuilder` - Convenience method to make a `PUT` request to a URL.
- `fn patch<U>(self: &Self, url: U) -> RequestBuilder` - Convenience method to make a `PATCH` request to a URL.
- `fn delete<U>(self: &Self, url: U) -> RequestBuilder` - Convenience method to make a `DELETE` request to a URL.
- `fn head<U>(self: &Self, url: U) -> RequestBuilder` - Convenience method to make a `HEAD` request to a URL.
- `fn request<U>(self: &Self, method: Method, url: U) -> RequestBuilder` - Start building a `Request` with the `Method` and `Url`.
- `fn execute(self: &Self, request: Request) -> impl Trait` - Executes a `Request`.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Service**
  - `fn poll_ready(self: & mut Self, _cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn call(self: & mut Self, req: Request) -> <Self as >::Future`



## reqwest::async_impl::client::ClientBuilder

*Struct*

A `ClientBuilder` can be used to create a `Client` with custom configuration.

**Methods:**

- `fn build(self: Self) -> crate::Result<Client>` - Returns a `Client` that uses this `ClientBuilder` configuration.
- `fn user_agent<V>(self: Self, value: V) -> ClientBuilder` - Sets the `User-Agent` header to be used by this client.
- `fn default_headers(self: Self, headers: HeaderMap) -> ClientBuilder` - Sets the default headers for every request.
- `fn cookie_store(self: Self, enable: bool) -> ClientBuilder` - Enable a persistent cookie store for the client.
- `fn cookie_provider<C>(self: Self, cookie_store: Arc<C>) -> ClientBuilder` - Set the persistent cookie store for the client.
- `fn no_gzip(self: Self) -> ClientBuilder` - Disable auto response body gzip decompression.
- `fn no_brotli(self: Self) -> ClientBuilder` - Disable auto response body brotli decompression.
- `fn no_zstd(self: Self) -> ClientBuilder` - Disable auto response body zstd decompression.
- `fn no_deflate(self: Self) -> ClientBuilder` - Disable auto response body deflate decompression.
- `fn redirect(self: Self, policy: redirect::Policy) -> ClientBuilder` - Set a `RedirectPolicy` for this client.
- `fn referer(self: Self, enable: bool) -> ClientBuilder` - Enable or disable automatic setting of the `Referer` header.
- `fn retry(self: Self, policy: crate::retry::Builder) -> ClientBuilder` - Set a request retry policy.
- `fn proxy(self: Self, proxy: Proxy) -> ClientBuilder` - Add a `Proxy` to the list of proxies the `Client` will use.
- `fn no_proxy(self: Self) -> ClientBuilder` - Clear all `Proxies`, so `Client` will use no proxy anymore.
- `fn timeout(self: Self, timeout: Duration) -> ClientBuilder` - Enables a total request timeout.
- `fn read_timeout(self: Self, timeout: Duration) -> ClientBuilder` - Enables a read timeout.
- `fn connect_timeout(self: Self, timeout: Duration) -> ClientBuilder` - Set a timeout for only the connect phase of a `Client`.
- `fn connection_verbose(self: Self, verbose: bool) -> ClientBuilder` - Set whether connections should emit verbose logs.
- `fn pool_idle_timeout<D>(self: Self, val: D) -> ClientBuilder` - Set an optional timeout for idle sockets being kept-alive.
- `fn pool_max_idle_per_host(self: Self, max: usize) -> ClientBuilder` - Sets the maximum idle connection per host allowed in the pool.
- `fn http1_title_case_headers(self: Self) -> ClientBuilder` - Send headers as title case instead of lowercase.
- `fn http1_allow_obsolete_multiline_headers_in_responses(self: Self, value: bool) -> ClientBuilder` - Set whether HTTP/1 connections will accept obsolete line folding for
- `fn http1_ignore_invalid_headers_in_responses(self: Self, value: bool) -> ClientBuilder` - Sets whether invalid header lines should be silently ignored in HTTP/1 responses.
- `fn http1_allow_spaces_after_header_name_in_responses(self: Self, value: bool) -> ClientBuilder` - Set whether HTTP/1 connections will accept spaces between header
- `fn http1_max_headers(self: Self, max: usize) -> ClientBuilder` - Set the maximum number of headers accepted in an HTTP/1 response.
- `fn http1_only(self: Self) -> ClientBuilder` - Only use HTTP/1.
- `fn http09_responses(self: Self) -> ClientBuilder` - Allow HTTP/0.9 responses
- `fn http2_prior_knowledge(self: Self) -> ClientBuilder` - Only use HTTP/2.
- `fn http2_initial_stream_window_size<impl Into<Option<u32>>>(self: Self, sz: impl Trait) -> ClientBuilder` - Sets the `SETTINGS_INITIAL_WINDOW_SIZE` option for HTTP2 stream-level flow control.
- `fn http2_initial_connection_window_size<impl Into<Option<u32>>>(self: Self, sz: impl Trait) -> ClientBuilder` - Sets the max connection-level flow control for HTTP2
- `fn http2_adaptive_window(self: Self, enabled: bool) -> ClientBuilder` - Sets whether to use an adaptive flow control.
- `fn http2_max_frame_size<impl Into<Option<u32>>>(self: Self, sz: impl Trait) -> ClientBuilder` - Sets the maximum frame size to use for HTTP2.
- `fn http2_max_header_list_size(self: Self, max_header_size_bytes: u32) -> ClientBuilder` - Sets the maximum size of received header frames for HTTP2.
- `fn http2_keep_alive_interval<impl Into<Option<Duration>>>(self: Self, interval: impl Trait) -> ClientBuilder` - Sets an interval for HTTP2 Ping frames should be sent to keep a connection alive.
- `fn http2_keep_alive_timeout(self: Self, timeout: Duration) -> ClientBuilder` - Sets a timeout for receiving an acknowledgement of the keep-alive ping.
- `fn http2_keep_alive_while_idle(self: Self, enabled: bool) -> ClientBuilder` - Sets whether HTTP2 keep-alive should apply while the connection is idle.
- `fn tcp_nodelay(self: Self, enabled: bool) -> ClientBuilder` - Set whether sockets have `TCP_NODELAY` enabled.
- `fn local_address<T>(self: Self, addr: T) -> ClientBuilder` - Bind to a local IP Address.
- `fn interface(self: Self, interface: &str) -> ClientBuilder` - Bind connections only on the specified network interface.
- `fn tcp_keepalive<D>(self: Self, val: D) -> ClientBuilder` - Set that all sockets have `SO_KEEPALIVE` set with the supplied duration.
- `fn tcp_keepalive_interval<D>(self: Self, val: D) -> ClientBuilder` - Set that all sockets have `SO_KEEPALIVE` set with the supplied interval.
- `fn tcp_keepalive_retries<C>(self: Self, retries: C) -> ClientBuilder` - Set that all sockets have `SO_KEEPALIVE` set with the supplied retry count.
- `fn tcp_user_timeout<D>(self: Self, val: D) -> ClientBuilder` - Set that all sockets have `TCP_USER_TIMEOUT` set with the supplied duration.
- `fn unix_socket<impl UnixSocketProvider>(self: Self, path: impl Trait) -> ClientBuilder` - Set that all connections will use this Unix socket.
- `fn tls_certs_merge<impl IntoIterator<Item = Certificate>>(self: Self, certs: impl Trait) -> ClientBuilder` - Add custom certificate roots.
- `fn tls_certs_only<impl IntoIterator<Item = Certificate>>(self: Self, certs: impl Trait) -> ClientBuilder` - Use only the provided certificate roots.
- `fn add_root_certificate(self: Self, cert: Certificate) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_certs_merge()`] or
- `fn tls_crls_only<impl IntoIterator<Item = CertificateRevocationList>>(self: Self, crls: impl Trait) -> ClientBuilder` - Add multiple certificate revocation lists.
- `fn add_crl(self: Self, crl: CertificateRevocationList) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_crls_only()`] instead.
- `fn add_crls<impl IntoIterator<Item = CertificateRevocationList>>(self: Self, crls: impl Trait) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_crls_only()`] instead.
- `fn identity(self: Self, identity: Identity) -> ClientBuilder` - Sets the identity to be used for client certificate authentication.
- `fn tls_danger_accept_invalid_hostnames(self: Self, accept_invalid_hostname: bool) -> ClientBuilder` - Controls the use of hostname verification.
- `fn danger_accept_invalid_hostnames(self: Self, accept_invalid_hostname: bool) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_danger_accept_invalid_hostnames()`] instead.
- `fn tls_danger_accept_invalid_certs(self: Self, accept_invalid_certs: bool) -> ClientBuilder` - Controls the use of certificate validation.
- `fn danger_accept_invalid_certs(self: Self, accept_invalid_certs: bool) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_danger_accept_invalid_certs()`] instead.
- `fn tls_sni(self: Self, tls_sni: bool) -> ClientBuilder` - Controls the use of TLS server name indication.
- `fn tls_sslkeylogfile(self: Self, on: bool) -> ClientBuilder` - Controls if the SSLKEYLOGFILE environment variable is respected.
- `fn tls_version_min(self: Self, version: tls::Version) -> ClientBuilder` - Set the minimum required TLS version for connections.
- `fn min_tls_version(self: Self, version: tls::Version) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_version_min()`] instead.
- `fn tls_version_max(self: Self, version: tls::Version) -> ClientBuilder` - Set the maximum allowed TLS version for connections.
- `fn max_tls_version(self: Self, version: tls::Version) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_version_max()`] instead.
- `fn tls_backend_rustls(self: Self) -> ClientBuilder` - Force using the Rustls TLS backend.
- `fn use_rustls_tls(self: Self) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_backend_rustls()`] instead.
- `fn tls_backend_preconfigured<impl Any>(self: Self, tls: impl Trait) -> ClientBuilder` - Use a preconfigured TLS backend.
- `fn use_preconfigured_tls<impl Any>(self: Self, tls: impl Trait) -> ClientBuilder` - Deprecated: use [`ClientBuilder::tls_backend_preconfigured()`] instead.
- `fn tls_info(self: Self, tls_info: bool) -> ClientBuilder` - Add TLS information as `TlsInfo` extension to responses.
- `fn https_only(self: Self, enabled: bool) -> ClientBuilder` - Restrict the Client to be used with HTTPS only requests.
- `fn no_hickory_dns(self: Self) -> ClientBuilder` - Disables the hickory-dns async resolver.
- `fn resolve(self: Self, domain: &str, addr: SocketAddr) -> ClientBuilder` - Override DNS resolution for specific domains to a particular IP address.
- `fn resolve_to_addrs(self: Self, domain: &str, addrs: &[SocketAddr]) -> ClientBuilder` - Override DNS resolution for specific domains to particular IP addresses.
- `fn dns_resolver<R>(self: Self, resolver: R) -> ClientBuilder` - Override the DNS resolver implementation.
- `fn connector_layer<L>(self: Self, layer: L) -> ClientBuilder` - Adds a new Tower [`Layer`](https://docs.rs/tower/latest/tower/trait.Layer.html) to the
- `fn new() -> Self` - Constructs a new `ClientBuilder`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



## reqwest::async_impl::client::Pending

*Struct*



