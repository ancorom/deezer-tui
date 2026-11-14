**serde_json > map > Map**

# Module: map::Map

## Contents

**Functions**

- [`entry`](#entry) - Gets the given key's corresponding entry in the map for in-place
- [`values_mut`](#values_mut) - Gets an iterator over mutable values of the map.

---

## serde_json::map::Map::entry

*Function*

Gets the given key's corresponding entry in the map for in-place
manipulation.

```rust
fn entry<S>(self: & mut Self, key: S) -> Entry
```



## serde_json::map::Map::values_mut

*Function*

Gets an iterator over mutable values of the map.

```rust
fn values_mut(self: & mut Self) -> ValuesMut
```



