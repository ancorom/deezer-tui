**cipher > block**

# Module: block

## Contents

**Traits**

- [`BlockBackend`](#blockbackend) - Trait implemented by block cipher encryption and decryption backends.
- [`BlockCipher`](#blockcipher) - Marker trait for block ciphers.
- [`BlockClosure`](#blockclosure) - Trait for [`BlockBackend`] users.
- [`BlockDecrypt`](#blockdecrypt) - Decrypt-only functionality for block ciphers.
- [`BlockDecryptMut`](#blockdecryptmut) - Decrypt-only functionality for block ciphers and modes with mutable access to `self`.
- [`BlockEncrypt`](#blockencrypt) - Encrypt-only functionality for block ciphers.
- [`BlockEncryptMut`](#blockencryptmut) - Encrypt-only functionality for block ciphers and modes with mutable access to `self`.

---

## cipher::block::BlockBackend

*Trait*

Trait implemented by block cipher encryption and decryption backends.

**Methods:**

- `proc_block`: Process single inout block.
- `proc_par_blocks`: Process inout blocks in parallel.
- `proc_tail_blocks`: Process buffer of inout blocks. Length of the buffer MUST be smaller
- `proc_block_inplace`: Process single block in-place.
- `proc_par_blocks_inplace`: Process blocks in parallel in-place.
- `proc_tail_blocks_inplace`: Process buffer of blocks in-place. Length of the buffer MUST be smaller



## cipher::block::BlockCipher

*Trait*

Marker trait for block ciphers.



## cipher::block::BlockClosure

*Trait*

Trait for [`BlockBackend`] users.

This trait is used to define rank-2 closures.

**Methods:**

- `call`: Execute closure with the provided block cipher backend.



## cipher::block::BlockDecrypt

*Trait*

Decrypt-only functionality for block ciphers.

**Methods:**

- `decrypt_with_backend`: Decrypt data using backend provided to the rank-2 closure.
- `decrypt_block_inout`: Decrypt single `inout` block.
- `decrypt_blocks_inout`: Decrypt `inout` blocks.
- `decrypt_block`: Decrypt single block in-place.
- `decrypt_block_b2b`: Decrypt `in_block` and write result to `out_block`.
- `decrypt_blocks`: Decrypt blocks in-place.
- `decrypt_blocks_b2b`: Decrypt blocks buffer-to-buffer.
- `decrypt_padded_inout`: Decrypt input and unpad it. Returns resulting ciphertext slice.
- `decrypt_padded`: Decrypt input and unpad it in-place. Returns resulting ciphertext slice.
- `decrypt_padded_b2b`: Decrypt input and unpad it buffer-to-buffer. Returns resulting



## cipher::block::BlockDecryptMut

*Trait*

Decrypt-only functionality for block ciphers and modes with mutable access to `self`.

The main use case for this trait is blocks modes, but it also can be used
for hardware cryptographic engines which require `&mut self` access to an
underlying hardware peripheral.

**Methods:**

- `decrypt_with_backend_mut`: Decrypt data using backend provided to the rank-2 closure.
- `decrypt_block_inout_mut`: Decrypt single `inout` block.
- `decrypt_blocks_inout_mut`: Decrypt `inout` blocks.
- `decrypt_block_mut`: Decrypt single block in-place.
- `decrypt_block_b2b_mut`: Decrypt `in_block` and write result to `out_block`.
- `decrypt_blocks_mut`: Decrypt blocks in-place.
- `decrypt_blocks_b2b_mut`: Decrypt blocks buffer-to-buffer.
- `decrypt_padded_inout_mut`: Decrypt input and unpad it. Returns resulting ciphertext slice.
- `decrypt_padded_mut`: Decrypt input and unpad it in-place. Returns resulting ciphertext slice.
- `decrypt_padded_b2b_mut`: Decrypt input and unpad it buffer-to-buffer. Returns resulting



## cipher::block::BlockEncrypt

*Trait*

Encrypt-only functionality for block ciphers.

**Methods:**

- `encrypt_with_backend`: Encrypt data using backend provided to the rank-2 closure.
- `encrypt_block_inout`: Encrypt single `inout` block.
- `encrypt_blocks_inout`: Encrypt `inout` blocks.
- `encrypt_block`: Encrypt single block in-place.
- `encrypt_block_b2b`: Encrypt `in_block` and write result to `out_block`.
- `encrypt_blocks`: Encrypt blocks in-place.
- `encrypt_blocks_b2b`: Encrypt blocks buffer-to-buffer.
- `encrypt_padded_inout`: Pad input and encrypt. Returns resulting ciphertext slice.
- `encrypt_padded`: Pad input and encrypt in-place. Returns resulting ciphertext slice.
- `encrypt_padded_b2b`: Pad input and encrypt buffer-to-buffer. Returns resulting ciphertext slice.



## cipher::block::BlockEncryptMut

*Trait*

Encrypt-only functionality for block ciphers and modes with mutable access to `self`.

The main use case for this trait is blocks modes, but it also can be used
for hardware cryptographic engines which require `&mut self` access to an
underlying hardware peripheral.

**Methods:**

- `encrypt_with_backend_mut`: Encrypt data using backend provided to the rank-2 closure.
- `encrypt_block_inout_mut`: Encrypt single `inout` block.
- `encrypt_blocks_inout_mut`: Encrypt `inout` blocks.
- `encrypt_block_mut`: Encrypt single block in-place.
- `encrypt_block_b2b_mut`: Encrypt `in_block` and write result to `out_block`.
- `encrypt_blocks_mut`: Encrypt blocks in-place.
- `encrypt_blocks_b2b_mut`: Encrypt blocks buffer-to-buffer.
- `encrypt_padded_inout_mut`: Pad input and encrypt. Returns resulting ciphertext slice.
- `encrypt_padded_mut`: Pad input and encrypt in-place. Returns resulting ciphertext slice.
- `encrypt_padded_b2b_mut`: Pad input and encrypt buffer-to-buffer. Returns resulting ciphertext slice.



