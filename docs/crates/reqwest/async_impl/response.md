**reqwest > async_impl > response**

# Module: async_impl::response

## Contents

**Structs**

- [`Response`](#response) - A Response to a submitted `Request`.

---

## reqwest::async_impl::response::Response

*Struct*

A Response to a submitted `Request`.

**Methods:**

- `fn status(self: &Self) -> StatusCode` - Get the `StatusCode` of this `Response`.
- `fn version(self: &Self) -> Version` - Get the HTTP `Version` of this `Response`.
- `fn headers(self: &Self) -> &HeaderMap` - Get the `Headers` of this `Response`.
- `fn headers_mut(self: & mut Self) -> & mut HeaderMap` - Get a mutable reference to the `Headers` of this `Response`.
- `fn content_length(self: &Self) -> Option<u64>` - Get the content length of the response, if it is known.
- `fn cookies<'a>(self: &'a Self) -> impl Trait` - Retrieve the cookies contained in the response.
- `fn url(self: &Self) -> &Url` - Get the final `Url` of this `Response`.
- `fn remote_addr(self: &Self) -> Option<SocketAddr>` - Get the remote address used to get this `Response`.
- `fn extensions(self: &Self) -> &http::Extensions` - Returns a reference to the associated extensions.
- `fn extensions_mut(self: & mut Self) -> & mut http::Extensions` - Returns a mutable reference to the associated extensions.
- `fn text(self: Self) -> crate::Result<String>` - Get the full response text.
- `fn text_with_charset(self: Self, default_encoding: &str) -> crate::Result<String>` - Get the full response text given a specific encoding.
- `fn json<T>(self: Self) -> crate::Result<T>` - Try to deserialize the response body as JSON.
- `fn bytes(self: Self) -> crate::Result<Bytes>` - Get the full response body as `Bytes`.
- `fn chunk(self: & mut Self) -> crate::Result<Option<Bytes>>` - Stream a chunk of the response body.
- `fn bytes_stream(self: Self) -> impl Trait` - Convert the response into a `Stream` of `Bytes` from the body.
- `fn error_for_status(self: Self) -> crate::Result<Self>` - Turn a response into an error if the server returned an error.
- `fn error_for_status_ref(self: &Self) -> crate::Result<&Self>` - Turn a reference to a response into an error if the server returned an error.
- `fn upgrade(self: Self) -> crate::Result<Upgraded>` - Consumes the response and returns a future for a possible HTTP upgrade.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(r: http::Response<T>) -> Response`



