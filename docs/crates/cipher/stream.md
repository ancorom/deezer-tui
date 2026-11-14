**cipher > stream**

# Module: stream

## Contents

**Traits**

- [`AsyncStreamCipher`](#asyncstreamcipher) - Marker trait for block-level asynchronous stream ciphers
- [`SeekNum`](#seeknum) - Trait implemented for numeric types which can be used with the
- [`StreamCipher`](#streamcipher) - Synchronous stream cipher core trait.
- [`StreamCipherSeek`](#streamcipherseek) - Trait for seekable stream ciphers.

---

## cipher::stream::AsyncStreamCipher

*Trait*

Marker trait for block-level asynchronous stream ciphers

**Methods:**

- `encrypt_inout`: Encrypt data using `InOutBuf`.
- `decrypt_inout`: Decrypt data using `InOutBuf`.
- `encrypt`: Encrypt data in place.
- `decrypt`: Decrypt data in place.
- `encrypt_b2b`: Encrypt data from buffer to buffer.
- `decrypt_b2b`: Decrypt data from buffer to buffer.



## cipher::stream::SeekNum

*Trait*

Trait implemented for numeric types which can be used with the
[`StreamCipherSeek`] trait.

This trait is implemented for `i32`, `u32`, `u64`, `u128`, and `usize`.
It is not intended to be implemented in third-party crates.

**Methods:**

- `from_block_byte`: Try to get position for block number `block`, byte position inside
- `into_block_byte`: Try to get block number and bytes position for given block size `bs`.



## cipher::stream::StreamCipher

*Trait*

Synchronous stream cipher core trait.

**Methods:**

- `try_apply_keystream_inout`: Apply keystream to `inout` data.
- `try_apply_keystream`: Apply keystream to data behind `buf`.
- `apply_keystream_inout`: Apply keystream to `inout` data.
- `apply_keystream`: Apply keystream to data in-place.
- `apply_keystream_b2b`: Apply keystream to data buffer-to-buffer.



## cipher::stream::StreamCipherSeek

*Trait*

Trait for seekable stream ciphers.

Methods of this trait are generic over the [`SeekNum`] trait, which is
implemented for primitive numeric types, i.e.: `i32`, `u32`, `u64`,
`u128`, and `usize`.

**Methods:**

- `try_current_pos`: Try to get current keystream position
- `try_seek`: Try to seek to the given position
- `current_pos`: Get current keystream position
- `seek`: Seek to the given position



