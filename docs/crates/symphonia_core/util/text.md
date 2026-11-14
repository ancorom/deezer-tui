**symphonia_core > util > text**

# Module: util::text

## Contents

**Modules**

- [`filter`](#filter) - Character iterator predicates for categories of characters.

**Functions**

- [`decode_ascii_lossy`](#decode_ascii_lossy) - Create an iterator over ASCII encoded characters in `buf`.
- [`decode_cp1252_lossy`](#decode_cp1252_lossy) - Create an iterator over Windows-1252 encoded characters in `buf`.
- [`decode_iso8859_1_lossy`](#decode_iso8859_1_lossy) - Create an iterator over ISO-8859-1 (IANA defined character set) encoded characters in `buf`.
- [`decode_isoiec8859_1_lossy`](#decode_isoiec8859_1_lossy) - Create an iterator over ISO/IEC 8859-1 encoded characters in `buf`.
- [`decode_utf16be_lossy`](#decode_utf16be_lossy) - Create an iterator over UTF-16 big-endian (BE) encoded characters in `buf`.
- [`decode_utf16le_lossy`](#decode_utf16le_lossy) - Create an iterator over UTF-16 little-endian (LE) encoded characters in `buf`.

---

## symphonia_core::util::text::decode_ascii_lossy

*Function*

Create an iterator over ASCII encoded characters in `buf`.

Invalid characters are substituted with the Unicode `U+FFFD REPLACEMENT CHARACTER` (�).

```rust
fn decode_ascii_lossy(buf: &[u8]) -> impl Trait
```



## symphonia_core::util::text::decode_cp1252_lossy

*Function*

Create an iterator over Windows-1252 encoded characters in `buf`.

Invalid characters are substituted with the Unicode `U+FFFD REPLACEMENT CHARACTER` (�).

Windows-1252 is a strict superset of ISO/IEC 8859-1. In addition to the characters defined
in ISO/IEC 8859-1, it contains all C0 control characters from ISO-8859-1, but uses the space
reserved for the C1 control characters for additional printable characters.

Note: Some character codes residing in the C1 control character space are officially
undefined in Windows-1252, but are mapped to C1 control characters when using Windows
multi-byte conversion APIs. These character codes are considered invalid.

```rust
fn decode_cp1252_lossy(buf: &[u8]) -> impl Trait
```



## symphonia_core::util::text::decode_iso8859_1_lossy

*Function*

Create an iterator over ISO-8859-1 (IANA defined character set) encoded characters in `buf`.

Note: ISO-8859-1 as defined by the IANA, is a superset of the similarly named ISO/IEC 8859-1
character set that encodes C0 and C1 control characters. See [`decode_isoiec8859_1_lossy`].

```rust
fn decode_iso8859_1_lossy(buf: &[u8]) -> impl Trait
```



## symphonia_core::util::text::decode_isoiec8859_1_lossy

*Function*

Create an iterator over ISO/IEC 8859-1 encoded characters in `buf`.

Invalid characters are substituted with the Unicode `U+FFFD REPLACEMENT CHARACTER` (�).

Note: ISO/IEC 8859-1 does not encode any of the C0 or C1 control characters. This is in
contrast to the IANA's similarly named ISO-8859-1 encoding which does. See
[`decode_iso8859_1_lossy`].

```rust
fn decode_isoiec8859_1_lossy(buf: &[u8]) -> impl Trait
```



## symphonia_core::util::text::decode_utf16be_lossy

*Function*

Create an iterator over UTF-16 big-endian (BE) encoded characters in `buf`.

If a byte-order marker (BOM) is present, it will be respected. Otherwise, big-endian is
assumed.

Invalid characters are substituted with the Unicode `U+FFFD REPLACEMENT CHARACTER` (�).

```rust
fn decode_utf16be_lossy(buf: &[u8]) -> impl Trait
```



## symphonia_core::util::text::decode_utf16le_lossy

*Function*

Create an iterator over UTF-16 little-endian (LE) encoded characters in `buf`.

If a byte-order marker (BOM) is present, it will be respected. Otherwise, little-endian is
assumed.

Invalid characters are substituted with the Unicode `U+FFFD REPLACEMENT CHARACTER` (�).

```rust
fn decode_utf16le_lossy(buf: &[u8]) -> impl Trait
```



## Module: filter

Character iterator predicates for categories of characters.



