**reqwest > async_impl > body**

# Module: async_impl::body

## Contents

**Structs**

- [`Body`](#body) - An asynchronous request body.

---

## reqwest::async_impl::body::Body

*Struct*

An asynchronous request body.

**Methods:**

- `fn as_bytes(self: &Self) -> Option<&[u8]>` - Returns a reference to the internal data of the `Body`.
- `fn wrap_stream<S>(stream: S) -> Body` - Wrap a futures `Stream` in a box inside `Body`.
- `fn wrap<B>(inner: B) -> Body` - Wrap a [`HttpBody`] in a box inside `Body`.

**Trait Implementations:**

- **Body**
  - `fn poll_frame(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<Result<hyper::body::Frame<<Self as >::Data>, <Self as >::Error>>>`
  - `fn size_hint(self: &Self) -> http_body::SizeHint`
  - `fn is_end_stream(self: &Self) -> bool`
- **From**
  - `fn from(s: &'static str) -> Body`
- **From**
  - `fn from(s: &'static [u8]) -> Body`
- **From**
  - `fn from(r: Response) -> Body`
- **Default**
  - `fn default() -> Body`
- **From**
  - `fn from(file: File) -> Body`
- **From**
  - `fn from(bytes: Bytes) -> Body`
- **From**
  - `fn from(s: String) -> Body`
- **From**
  - `fn from(vec: Vec<u8>) -> Body`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



