**reqwest > retry > Builder**

# Module: retry::Builder

## Contents

**Functions**

- [`scoped`](#scoped) - Create a scoped retry policy.

---

## reqwest::retry::Builder::scoped

*Function*

Create a scoped retry policy.

For a more convenient constructor, see [`for_host()`].

```rust
fn scoped<impl scope::Scope>(scope: impl Trait) -> Self
```



