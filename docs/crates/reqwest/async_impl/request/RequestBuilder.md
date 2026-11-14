**reqwest > async_impl > request > RequestBuilder**

# Module: async_impl::request::RequestBuilder

## Contents

**Functions**

- [`build`](#build) - Build a `Request`, which can be inspected, modified and executed with

---

## reqwest::async_impl::request::RequestBuilder::build

*Function*

Build a `Request`, which can be inspected, modified and executed with
`Client::execute()`.

```rust
fn build(self: Self) -> crate::Result<Request>
```



