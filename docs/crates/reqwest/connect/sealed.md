**reqwest > connect > sealed**

# Module: connect::sealed

## Contents

**Structs**

- [`Conn`](#conn) - Note: the `is_proxy` member means *is plain text HTTP proxy*.
- [`Unnameable`](#unnameable)

---

## reqwest::connect::sealed::Conn

*Struct*

Note: the `is_proxy` member means *is plain text HTTP proxy*.
This tells hyper whether the URI should be written in
* origin-form (`GET /just/a/path HTTP/1.1`), when `is_proxy == false`, or
* absolute-form (`GET http://foo.bar/and/a/path HTTP/1.1`), otherwise.



## reqwest::connect::sealed::Unnameable

*Struct*

**Tuple Struct**: `()`



