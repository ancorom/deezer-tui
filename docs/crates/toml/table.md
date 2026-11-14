**toml > table**

# Module: table

## Contents

**Type Aliases**

- [`Table`](#table) - Type representing a TOML table, payload of the `Value::Table` variant.

---

## toml::table::Table

*Type Alias*: `crate::map::Map<alloc::string::String, crate::Value>`

Type representing a TOML table, payload of the `Value::Table` variant.

By default it entries are stored in
[lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)
of the keys. Enable the `preserve_order` feature to store entries in the order they appear in
the source file.



