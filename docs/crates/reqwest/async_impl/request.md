**reqwest > async_impl > request**

# Module: async_impl::request

## Contents

**Structs**

- [`Request`](#request) - A request which can be executed with `Client::execute()`.
- [`RequestBuilder`](#requestbuilder) - A builder to construct the properties of a `Request`.

---

## reqwest::async_impl::request::Request

*Struct*

A request which can be executed with `Client::execute()`.

**Methods:**

- `fn new(method: Method, url: Url) -> Self` - Constructs a new request.
- `fn method(self: &Self) -> &Method` - Get the method.
- `fn method_mut(self: & mut Self) -> & mut Method` - Get a mutable reference to the method.
- `fn url(self: &Self) -> &Url` - Get the url.
- `fn url_mut(self: & mut Self) -> & mut Url` - Get a mutable reference to the url.
- `fn headers(self: &Self) -> &HeaderMap` - Get the headers.
- `fn headers_mut(self: & mut Self) -> & mut HeaderMap` - Get a mutable reference to the headers.
- `fn body(self: &Self) -> Option<&Body>` - Get the body.
- `fn body_mut(self: & mut Self) -> & mut Option<Body>` - Get a mutable reference to the body.
- `fn timeout(self: &Self) -> Option<&Duration>` - Get the timeout.
- `fn timeout_mut(self: & mut Self) -> & mut Option<Duration>` - Get a mutable reference to the timeout.
- `fn version(self: &Self) -> Version` - Get the http version.
- `fn version_mut(self: & mut Self) -> & mut Version` - Get a mutable reference to the http version.
- `fn try_clone(self: &Self) -> Option<Request>` - Attempt to clone the request.

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(req: HttpRequest<T>) -> crate::Result<Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## reqwest::async_impl::request::RequestBuilder

*Struct*

A builder to construct the properties of a `Request`.

To construct a `RequestBuilder`, refer to the `Client` documentation.

**Methods:**

- `fn from_parts(client: Client, request: Request) -> RequestBuilder` - Assemble a builder starting from an existing `Client` and a `Request`.
- `fn header<K, V>(self: Self, key: K, value: V) -> RequestBuilder` - Add a `Header` to this Request.
- `fn headers(self: Self, headers: crate::header::HeaderMap) -> RequestBuilder` - Add a set of Headers to the existing ones on this Request.
- `fn basic_auth<U, P>(self: Self, username: U, password: Option<P>) -> RequestBuilder` - Enable HTTP basic authentication.
- `fn bearer_auth<T>(self: Self, token: T) -> RequestBuilder` - Enable HTTP bearer authentication.
- `fn body<T>(self: Self, body: T) -> RequestBuilder` - Set the request body.
- `fn timeout(self: Self, timeout: Duration) -> RequestBuilder` - Enables a request timeout.
- `fn version(self: Self, version: Version) -> RequestBuilder` - Set HTTP version
- `fn json<T>(self: Self, json: &T) -> RequestBuilder` - Send a JSON body.
- `fn build(self: Self) -> crate::Result<Request>` - Build a `Request`, which can be inspected, modified and executed with
- `fn build_split(self: Self) -> (Client, crate::Result<Request>)` - Build a `Request`, which can be inspected, modified and executed with
- `fn send(self: Self) -> impl Trait` - Constructs the Request and sends it to the target URL, returning a
- `fn try_clone(self: &Self) -> Option<RequestBuilder>` - Attempt to clone the RequestBuilder.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



