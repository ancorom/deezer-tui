**toml > de > parser > devalue > DeInteger**

# Module: de::parser::devalue::DeInteger

## Contents

**Functions**

- [`as_str`](#as_str) - [`from_str_radix`][i64::from_str_radix]-compatible representation of an integer
- [`radix`](#radix) - Numeric base of [`DeInteger::as_str`]

---

## toml::de::parser::devalue::DeInteger::as_str

*Function*

[`from_str_radix`][i64::from_str_radix]-compatible representation of an integer

Requires [`DeInteger::radix`] to interpret

See [`Display`][std::fmt::Display] for a representation that includes the radix

```rust
fn as_str(self: &Self) -> &str
```



## toml::de::parser::devalue::DeInteger::radix

*Function*

Numeric base of [`DeInteger::as_str`]

```rust
fn radix(self: &Self) -> u32
```



