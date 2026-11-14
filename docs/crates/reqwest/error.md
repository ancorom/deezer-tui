**reqwest > error**

# Module: error

## Contents

**Structs**

- [`Error`](#error) - The Errors that may occur when processing a `Request`.

**Type Aliases**

- [`Result`](#result) - A `Result` alias where the `Err` case is `reqwest::Error`.

---

## reqwest::error::Error

*Struct*

The Errors that may occur when processing a `Request`.

Note: Errors may include the full URL used to make the `Request`. If the URL
contains sensitive information (e.g. an API key as a query parameter), be
sure to remove it ([`without_url`](Error::without_url))

**Methods:**

- `fn url(self: &Self) -> Option<&Url>` - Returns a possible URL related to this error.
- `fn url_mut(self: & mut Self) -> Option<& mut Url>` - Returns a mutable reference to the URL related to this error
- `fn with_url(self: Self, url: Url) -> Self` - Add a url related to this error (overwriting any existing)
- `fn without_url(self: Self) -> Self` - Strip the related url from this error (if, for example, it contains
- `fn is_builder(self: &Self) -> bool` - Returns true if the error is from a type Builder.
- `fn is_redirect(self: &Self) -> bool` - Returns true if the error is from a `RedirectPolicy`.
- `fn is_status(self: &Self) -> bool` - Returns true if the error is from `Response::error_for_status`.
- `fn is_timeout(self: &Self) -> bool` - Returns true if the error is related to a timeout.
- `fn is_request(self: &Self) -> bool` - Returns true if the error is related to the request
- `fn is_connect(self: &Self) -> bool` - Returns true if the error is related to connect
- `fn is_dns(self: &Self) -> bool` - Returns true if the error is related to DNS resolution.
- `fn is_body(self: &Self) -> bool` - Returns true if the error is related to the request or response body
- `fn is_decode(self: &Self) -> bool` - Returns true if the error is related to decoding the response's body
- `fn status(self: &Self) -> Option<StatusCode>` - Returns the status code, if the error was generated from a response.
- `fn is_upgrade(self: &Self) -> bool` - Returns true if the error is related to a protocol upgrade request

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn StdError>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## reqwest::error::Result

*Type Alias*: `std::result::Result<T, Error>`

A `Result` alias where the `Err` case is `reqwest::Error`.



