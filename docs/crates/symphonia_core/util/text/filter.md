**symphonia_core > util > text > filter**

# Module: util::text::filter

## Contents

**Functions**

- [`ascii_text`](#ascii_text) - Predicate for matching ASCII text characters.
- [`c0_control`](#c0_control) - Predicate for matching C0 control characters.
- [`c1_control`](#c1_control) - Predicate for matching C1 control characters.
- [`control`](#control) - Predicate for matching C0 or C1 control characters.
- [`not_ascii_text`](#not_ascii_text) - Predicate for matching non-text ASCII characters.
- [`not_c0_control`](#not_c0_control) - Predicate for matching characters that are not C0 control characters.
- [`not_c1_control`](#not_c1_control) - Predicate for matching characters that are not C1 control characters.
- [`not_control`](#not_control) - Predicate for matching characters that are not C0 or C1 control characters.
- [`not_null`](#not_null) - Predicate for matching non-null terminator characters.
- [`null`](#null) - Predicate for matching null-terminator characters.

---

## symphonia_core::util::text::filter::ascii_text

*Function*

Predicate for matching ASCII text characters.

ASCII text characters are all ASCII graphical characters and SPACE.

```rust
fn ascii_text(c: &char) -> bool
```



## symphonia_core::util::text::filter::c0_control

*Function*

Predicate for matching C0 control characters.

```rust
fn c0_control(c: &char) -> bool
```



## symphonia_core::util::text::filter::c1_control

*Function*

Predicate for matching C1 control characters.

```rust
fn c1_control(c: &char) -> bool
```



## symphonia_core::util::text::filter::control

*Function*

Predicate for matching C0 or C1 control characters.

Returns `true` if the character is a C0 or C1 control character (Unicode CC category),
or `false` otherwise.

```rust
fn control(c: &char) -> bool
```



## symphonia_core::util::text::filter::not_ascii_text

*Function*

Predicate for matching non-text ASCII characters.

ASCII text characters are all ASCII graphical characters and SPACE.

```rust
fn not_ascii_text(c: &char) -> bool
```



## symphonia_core::util::text::filter::not_c0_control

*Function*

Predicate for matching characters that are not C0 control characters.

```rust
fn not_c0_control(c: &char) -> bool
```



## symphonia_core::util::text::filter::not_c1_control

*Function*

Predicate for matching characters that are not C1 control characters.

```rust
fn not_c1_control(c: &char) -> bool
```



## symphonia_core::util::text::filter::not_control

*Function*

Predicate for matching characters that are not C0 or C1 control characters.

Returns `true` if the character is not a C0 or C1 control character (Unicode CC
category), or `false` otherwise.

```rust
fn not_control(c: &char) -> bool
```



## symphonia_core::util::text::filter::not_null

*Function*

Predicate for matching non-null terminator characters.

```rust
fn not_null(c: &char) -> bool
```



## symphonia_core::util::text::filter::null

*Function*

Predicate for matching null-terminator characters.

```rust
fn null(c: &char) -> bool
```



