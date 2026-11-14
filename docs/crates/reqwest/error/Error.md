**reqwest > error > Error**

# Module: error::Error

## Contents

**Functions**

- [`without_url`](#without_url) - Strip the related url from this error (if, for example, it contains

---

## reqwest::error::Error::without_url

*Function*

Strip the related url from this error (if, for example, it contains
sensitive information)

```rust
fn without_url(self: Self) -> Self
```



