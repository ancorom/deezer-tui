**toml > de > parser > detable**

# Module: de::parser::detable

## Contents

**Type Aliases**

- [`DeTable`](#detable) - Type representing a TOML table, payload of the `Value::Table` variant.

---

## toml::de::parser::detable::DeTable

*Type Alias*: `crate::map::Map<serde_spanned::Spanned<crate::de::DeString<'i>>, serde_spanned::Spanned<crate::de::DeValue<'i>>>`

Type representing a TOML table, payload of the `Value::Table` variant.

By default it entries are stored in
[lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)
of the keys. Enable the `preserve_order` feature to store entries in the order they appear in
the source file.



