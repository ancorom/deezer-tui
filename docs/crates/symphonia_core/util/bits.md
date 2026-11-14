**symphonia_core > util > bits**

# Module: util::bits

## Contents

**Functions**

- [`contains_ones_byte_u16`](#contains_ones_byte_u16) - Returns true if the unsigned 16-bit integer contains one or more bytes which have all bits
- [`contains_ones_byte_u32`](#contains_ones_byte_u32) - Returns true if the unsigned 32-bit integer contains one or more bytes which have all bits
- [`contains_ones_byte_u64`](#contains_ones_byte_u64) - Returns true if the unsigned 64-bit integer contains one or more bytes which have all bits
- [`mask_at`](#mask_at) - Masks the bit at the specified bit index.
- [`mask_lower`](#mask_lower) - Masks all bits with an index less than idx.
- [`mask_lower_eq`](#mask_lower_eq) - Masks all bits with an index less than or equal to idx.
- [`mask_range`](#mask_range) - Masks out all bits in positions less than upper, but greater than or equal to lower
- [`mask_upper`](#mask_upper) - Masks all bits with an index greater than idx.
- [`mask_upper_eq`](#mask_upper_eq) - Masks all bits with an index greater than or equal to idx.
- [`sign_extend_leq16_to_i16`](#sign_extend_leq16_to_i16) - Sign extends an arbitrary, 16-bit or less, signed two's complement integer stored within an
- [`sign_extend_leq32_to_i32`](#sign_extend_leq32_to_i32) - Sign extends an arbitrary, 32-bit or less, signed two's complement integer stored within an
- [`sign_extend_leq64_to_i64`](#sign_extend_leq64_to_i64) - Sign extends an arbitrary, 64-bit or less, signed two's complement integer stored within an
- [`sign_extend_leq8_to_i8`](#sign_extend_leq8_to_i8) - Sign extends an arbitrary, 8-bit or less, signed two's complement integer stored within an

---

## symphonia_core::util::bits::contains_ones_byte_u16

*Function*

Returns true if the unsigned 16-bit integer contains one or more bytes which have all bits
set.

```rust
fn contains_ones_byte_u16(value: u16) -> bool
```



## symphonia_core::util::bits::contains_ones_byte_u32

*Function*

Returns true if the unsigned 32-bit integer contains one or more bytes which have all bits
set.

```rust
fn contains_ones_byte_u32(value: u32) -> bool
```



## symphonia_core::util::bits::contains_ones_byte_u64

*Function*

Returns true if the unsigned 64-bit integer contains one or more bytes which have all bits
set.

```rust
fn contains_ones_byte_u64(value: u64) -> bool
```



## symphonia_core::util::bits::mask_at

*Function*

Masks the bit at the specified bit index.

```rust
fn mask_at(idx: u32) -> u8
```



## symphonia_core::util::bits::mask_lower

*Function*

Masks all bits with an index less than idx.

```rust
fn mask_lower(idx: u32) -> u8
```



## symphonia_core::util::bits::mask_lower_eq

*Function*

Masks all bits with an index less than or equal to idx.

```rust
fn mask_lower_eq(idx: u32) -> u8
```



## symphonia_core::util::bits::mask_range

*Function*

Masks out all bits in positions less than upper, but greater than or equal to lower
(upper < bit <= lower)

```rust
fn mask_range(upper: u32, lower: u32) -> u8
```



## symphonia_core::util::bits::mask_upper

*Function*

Masks all bits with an index greater than idx.

```rust
fn mask_upper(idx: u32) -> u8
```



## symphonia_core::util::bits::mask_upper_eq

*Function*

Masks all bits with an index greater than or equal to idx.

```rust
fn mask_upper_eq(idx: u32) -> u8
```



## symphonia_core::util::bits::sign_extend_leq16_to_i16

*Function*

Sign extends an arbitrary, 16-bit or less, signed two's complement integer stored within an
u16 to a full width i16.

```rust
fn sign_extend_leq16_to_i16(value: u16, width: u32) -> i16
```



## symphonia_core::util::bits::sign_extend_leq32_to_i32

*Function*

Sign extends an arbitrary, 32-bit or less, signed two's complement integer stored within an
u32 to a full width i32.

```rust
fn sign_extend_leq32_to_i32(value: u32, width: u32) -> i32
```



## symphonia_core::util::bits::sign_extend_leq64_to_i64

*Function*

Sign extends an arbitrary, 64-bit or less, signed two's complement integer stored within an
u64 to a full width i64.

```rust
fn sign_extend_leq64_to_i64(value: u64, width: u32) -> i64
```



## symphonia_core::util::bits::sign_extend_leq8_to_i8

*Function*

Sign extends an arbitrary, 8-bit or less, signed two's complement integer stored within an
u8 to a full width i8.

```rust
fn sign_extend_leq8_to_i8(value: u8, width: u32) -> i8
```



