**toml > map**

# Module: map

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An owning iterator over a `toml::Map`'s entries.
- [`Iter`](#iter) - An iterator over a `toml::Map`'s entries.
- [`IterMut`](#itermut) - A mutable iterator over a `toml::Map`'s entries.
- [`Keys`](#keys) - An iterator over a `toml::Map`'s keys.
- [`Map`](#map) - Represents a TOML key/value type.
- [`OccupiedEntry`](#occupiedentry) - An occupied Entry. It is part of the [`Entry`] enum.
- [`VacantEntry`](#vacantentry) - A vacant Entry. It is part of the [`Entry`] enum.
- [`Values`](#values) - An iterator over a `toml::Map`'s values.

**Enums**

- [`Entry`](#entry) - A view into a single entry in a map, which may either be vacant or occupied.

---

## toml::map::Entry

*Enum*

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the [`entry`] method on [`Map`].

[`entry`]: struct.Map.html#method.entry
[`Map`]: struct.Map.html

**Generic Parameters:**
- 'a
- K
- V

**Variants:**
- `Vacant(VacantEntry<'a, K, V>)` - A vacant Entry.
- `Occupied(OccupiedEntry<'a, K, V>)` - An occupied Entry.

**Methods:**

- `fn key(self: &Self) -> &K` - Returns a reference to this entry's key.
- `fn or_insert(self: Self, default: V) -> &'a  mut V` - Ensures a value is in the entry by inserting the default if empty, and
- `fn or_insert_with<F>(self: Self, default: F) -> &'a  mut V` - Ensures a value is in the entry by inserting the result of the default



## toml::map::IntoIter

*Struct*

An owning iterator over a `toml::Map`'s entries.

**Generic Parameters:**
- K
- V

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## toml::map::Iter

*Struct*

An iterator over a `toml::Map`'s entries.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## toml::map::IterMut

*Struct*

A mutable iterator over a `toml::Map`'s entries.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## toml::map::Keys

*Struct*

An iterator over a `toml::Map`'s keys.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## toml::map::Map

*Struct*

Represents a TOML key/value type.

**Generic Parameters:**
- K
- V

**Methods:**

- `fn parse(input: &'i str) -> Result<Spanned<Self>, crate::de::Error>` - Parse a TOML document
- `fn parse_recoverable(input: &'i str) -> (Spanned<Self>, Vec<crate::de::Error>)` - Parse a TOML document, with best effort recovery on error
- `fn make_owned(self: & mut Self)` - Ensure no data is borrowed
- `fn new() -> Self` - Makes a new empty Map.
- `fn with_capacity(capacity: usize) -> Self` - Makes a new empty Map with the given initial capacity.
- `fn clear(self: & mut Self)` - Clears the map, removing all values.
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>` - Returns a reference to the value corresponding to the key.
- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns true if the map contains a value for the specified key.
- `fn get_mut<Q>(self: & mut Self, key: &Q) -> Option<& mut V>` - Returns a mutable reference to the value corresponding to the key.
- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>` - Returns the key-value pair matching the given key.
- `fn insert(self: & mut Self, k: K, v: V) -> Option<V>` - Inserts a key-value pair into the map.
- `fn remove<Q>(self: & mut Self, key: &Q) -> Option<V>` - Removes a key from the map, returning the value at the key if the key
- `fn remove_entry<Q>(self: & mut Self, key: &Q) -> Option<(K, V)>` - Removes a key from the map, returning the stored key and value if the key was previously in the map.
- `fn retain<F>(self: & mut Self, keep: F)` - Retains only the elements specified by the `keep` predicate.
- `fn entry<S>(self: & mut Self, key: S) -> Entry<K, V>` - Gets the given key's corresponding entry in the map for in-place
- `fn len(self: &Self) -> usize` - Returns the number of elements in the map.
- `fn is_empty(self: &Self) -> bool` - Returns true if the map contains no elements.
- `fn iter(self: &Self) -> Iter<K, V>` - Gets an iterator over the entries of the map.
- `fn iter_mut(self: & mut Self) -> IterMut<K, V>` - Gets a mutable iterator over the entries of the map.
- `fn keys(self: &Self) -> Keys<K, V>` - Gets an iterator over the keys of the map.
- `fn values(self: &Self) -> Values<K, V>` - Gets an iterator over the values of the map.
- `fn try_from<T>(value: T) -> Result<Self, crate::ser::Error>` - Convert a `T` into `toml::Table`.
- `fn try_into<'de, T>(self: Self) -> Result<T, crate::de::Error>` - Interpret a `toml::Table` as an instance of type `T`.

**Trait Implementations:**

- **Index**
  - `fn index(self: &Self, index: &Q) -> &V`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: &Q) -> & mut V`
- **Default**
  - `fn default() -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## toml::map::OccupiedEntry

*Struct*

An occupied Entry. It is part of the [`Entry`] enum.

[`Entry`]: enum.Entry.html

**Generic Parameters:**
- 'a
- K
- V

**Methods:**

- `fn key(self: &Self) -> &K` - Gets a reference to the key in the entry.
- `fn get(self: &Self) -> &V` - Gets a reference to the value in the entry.
- `fn get_mut(self: & mut Self) -> & mut V` - Gets a mutable reference to the value in the entry.
- `fn into_mut(self: Self) -> &'a  mut V` - Converts the entry into a mutable reference to its value.
- `fn insert(self: & mut Self, value: V) -> V` - Sets the value of the entry with the `OccupiedEntry`'s key, and returns
- `fn remove(self: Self) -> V` - Takes the value of the entry out of the map, and returns it.



## toml::map::VacantEntry

*Struct*

A vacant Entry. It is part of the [`Entry`] enum.

[`Entry`]: enum.Entry.html

**Generic Parameters:**
- 'a
- K
- V

**Methods:**

- `fn key(self: &Self) -> &K` - Gets a reference to the key that would be used when inserting a value
- `fn insert(self: Self, value: V) -> &'a  mut V` - Sets the value of the entry with the `VacantEntry`'s key, and returns a



## toml::map::Values

*Struct*

An iterator over a `toml::Map`'s values.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



