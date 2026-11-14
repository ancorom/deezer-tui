**serde_json > value > Value**

# Module: value::Value

## Contents

**Functions**

- [`as_str`](#as_str) - If the `Value` is a String, returns the associated str. Returns None
- [`get`](#get) - Index into a JSON array or map. A string index can be used to access a
- [`get_mut`](#get_mut) - Mutably index into a JSON array or map. A string index can be used to
- [`sort_all_objects`](#sort_all_objects) - Reorders the entries of all `Value::Object` nested within this JSON

---

## serde_json::value::Value::as_str

*Function*

If the `Value` is a String, returns the associated str. Returns None
otherwise.

```
# use serde_json::json;
#
let v = json!({ "a": "some string", "b": false });

assert_eq!(v["a"].as_str(), Some("some string"));

// The boolean `false` is not a string.
assert_eq!(v["b"].as_str(), None);

// JSON values are printed in JSON representation, so strings are in quotes.
//
//    The value is: "some string"
println!("The value is: {}", v["a"]);

// Rust strings are printed without quotes.
//
//    The value is: some string
println!("The value is: {}", v["a"].as_str().unwrap());
```

```rust
fn as_str(self: &Self) -> Option<&str>
```



## serde_json::value::Value::get

*Function*

Index into a JSON array or map. A string index can be used to access a
value in a map, and a usize index can be used to access an element of an
array.

Returns `None` if the type of `self` does not match the type of the
index, for example if the index is a string and `self` is an array or a
number. Also returns `None` if the given key does not exist in the map
or the given index is not within the bounds of the array.

```
# use serde_json::json;
#
let object = json!({ "A": 65, "B": 66, "C": 67 });
assert_eq!(*object.get("A").unwrap(), json!(65));

let array = json!([ "A", "B", "C" ]);
assert_eq!(*array.get(2).unwrap(), json!("C"));

assert_eq!(array.get("A"), None);
```

Square brackets can also be used to index into a value in a more concise
way. This returns `Value::Null` in cases where `get` would have returned
`None`.

```
# use serde_json::json;
#
let object = json!({
    "A": ["a", "á", "à"],
    "B": ["b", "b́"],
    "C": ["c", "ć", "ć̣", "ḉ"],
});
assert_eq!(object["B"][0], json!("b"));

assert_eq!(object["D"], json!(null));
assert_eq!(object[0]["x"]["y"]["z"], json!(null));
```

```rust
fn get<I>(self: &Self, index: I) -> Option<&Value>
```



## serde_json::value::Value::get_mut

*Function*

Mutably index into a JSON array or map. A string index can be used to
access a value in a map, and a usize index can be used to access an
element of an array.

Returns `None` if the type of `self` does not match the type of the
index, for example if the index is a string and `self` is an array or a
number. Also returns `None` if the given key does not exist in the map
or the given index is not within the bounds of the array.

```
# use serde_json::json;
#
let mut object = json!({ "A": 65, "B": 66, "C": 67 });
*object.get_mut("A").unwrap() = json!(69);

let mut array = json!([ "A", "B", "C" ]);
*array.get_mut(2).unwrap() = json!("D");
```

```rust
fn get_mut<I>(self: & mut Self, index: I) -> Option<& mut Value>
```



## serde_json::value::Value::sort_all_objects

*Function*

Reorders the entries of all `Value::Object` nested within this JSON
value according to `str`'s usual ordering.

If serde_json's "preserve_order" feature is not enabled, this method
does no work because all JSON maps are always kept in a sorted state.

If serde_json's "preserve_order" feature is enabled, this method
destroys the original source order or insertion order of the JSON
objects in favor of an alphanumerical order that matches how a BTreeMap
with the same contents would be ordered.

```rust
fn sort_all_objects(self: & mut Self)
```



