**reqwest > proxy > Proxy**

# Module: proxy::Proxy

## Contents

**Functions**

- [`no_proxy`](#no_proxy) - Adds a `No Proxy` exclusion list to this Proxy

---

## reqwest::proxy::Proxy::no_proxy

*Function*

Adds a `No Proxy` exclusion list to this Proxy

# Example

```
# extern crate reqwest;
# fn run() -> Result<(), Box<dyn std::error::Error>> {
let proxy = reqwest::Proxy::https("http://localhost:1234")?
    .no_proxy(reqwest::NoProxy::from_string("direct.tld, sub.direct2.tld"));
# Ok(())
# }
# fn main() {}
```

```rust
fn no_proxy(self: Self, no_proxy: Option<NoProxy>) -> Proxy
```



