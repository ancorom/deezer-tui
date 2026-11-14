**reqwest > cookie**

# Module: cookie

## Contents

**Structs**

- [`Cookie`](#cookie) - A single HTTP cookie.
- [`Jar`](#jar) - A good default `CookieStore` implementation.

**Traits**

- [`CookieStore`](#cookiestore) - Actions for a persistent cookie store providing session support.

---

## reqwest::cookie::Cookie

*Struct*

A single HTTP cookie.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn name(self: &Self) -> &str` - The name of the cookie.
- `fn value(self: &Self) -> &str` - The value of the cookie.
- `fn http_only(self: &Self) -> bool` - Returns true if the 'HttpOnly' directive is enabled.
- `fn secure(self: &Self) -> bool` - Returns true if the 'Secure' directive is enabled.
- `fn same_site_lax(self: &Self) -> bool` - Returns true if  'SameSite' directive is 'Lax'.
- `fn same_site_strict(self: &Self) -> bool` - Returns true if  'SameSite' directive is 'Strict'.
- `fn path(self: &Self) -> Option<&str>` - Returns the path directive of the cookie, if set.
- `fn domain(self: &Self) -> Option<&str>` - Returns the domain directive of the cookie, if set.
- `fn max_age(self: &Self) -> Option<std::time::Duration>` - Get the Max-Age information.
- `fn expires(self: &Self) -> Option<SystemTime>` - The cookie expiration time.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## reqwest::cookie::CookieStore

*Trait*

Actions for a persistent cookie store providing session support.

**Methods:**

- `set_cookies`: Store a set of Set-Cookie header values received from `url`
- `cookies`: Get any Cookie values in the store for `url`



## reqwest::cookie::Jar

*Struct*

A good default `CookieStore` implementation.

This is the implementation used when simply calling `cookie_store(true)`.
This type is exposed to allow creating one and filling it with some
existing cookies more easily, before creating a `Client`.

For more advanced scenarios, such as needing to serialize the store or
manipulate it between requests, you may refer to the
[reqwest_cookie_store crate](https://crates.io/crates/reqwest_cookie_store).

**Tuple Struct**: `()`

**Methods:**

- `fn add_cookie_str(self: &Self, cookie: &str, url: &url::Url)` - Add a cookie to this jar.

**Trait Implementations:**

- **CookieStore**
  - `fn set_cookies(self: &Self, cookie_headers: & mut dyn Iterator, url: &url::Url)`
  - `fn cookies(self: &Self, url: &url::Url) -> Option<HeaderValue>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



