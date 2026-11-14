**cipher > stream_core**

# Module: stream_core

## Contents

**Traits**

- [`Counter`](#counter) - Counter type usable with [`StreamCipherCore`].
- [`StreamBackend`](#streambackend) - Trait implemented by stream cipher backends.
- [`StreamCipherCore`](#streamciphercore) - Block-level synchronous stream ciphers.
- [`StreamCipherSeekCore`](#streamcipherseekcore) - Block-level seeking trait for stream ciphers.
- [`StreamClosure`](#streamclosure) - Trait for [`StreamBackend`] users.

---

## cipher::stream_core::Counter

*Trait*

Counter type usable with [`StreamCipherCore`].

This trait is implemented for `i32`, `u32`, `u64`, `u128`, and `usize`.
It's not intended to be implemented in third-party crates, but doing so
is not forbidden.



## cipher::stream_core::StreamBackend

*Trait*

Trait implemented by stream cipher backends.

**Methods:**

- `gen_ks_block`: Generate keystream block.
- `gen_par_ks_blocks`: Generate keystream blocks in parallel.
- `gen_tail_blocks`: Generate keystream blocks. Length of the buffer MUST be smaller



## cipher::stream_core::StreamCipherCore

*Trait*

Block-level synchronous stream ciphers.

**Methods:**

- `remaining_blocks`: Return number of remaining blocks before cipher wraps around.
- `process_with_backend`: Process data using backend provided to the rank-2 closure.
- `write_keystream_block`: Write keystream block.
- `write_keystream_blocks`: Write keystream blocks.
- `apply_keystream_block_inout`: Apply keystream block.
- `apply_keystream_blocks`: Apply keystream blocks.
- `apply_keystream_blocks_inout`: Apply keystream blocks.
- `try_apply_keystream_partial`: Try to apply keystream to data not divided into blocks.
- `apply_keystream_partial`: Try to apply keystream to data not divided into blocks.



## cipher::stream_core::StreamCipherSeekCore

*Trait*

Block-level seeking trait for stream ciphers.

**Methods:**

- `Counter`: Counter type used inside stream cipher.
- `get_block_pos`: Get current block position.
- `set_block_pos`: Set block position.



## cipher::stream_core::StreamClosure

*Trait*

Trait for [`StreamBackend`] users.

This trait is used to define rank-2 closures.

**Methods:**

- `call`: Execute closure with the provided stream cipher backend.



