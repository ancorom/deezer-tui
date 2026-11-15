//! Криптография Deezer: генерация 16-байтного Blowfish-ключа из SNG_ID
//! (MD5 hex + XOR с секретом `g4el58wc0zvf9na1`) и BF_CBC_STRIPE-дешифровка
//! аудио-чанков по 2048 байт через [`cbc::Decryptor<blowfish::Blowfish>`] с
//! фиксированным IV `[0, 1, 2, 3, 4, 5, 6, 7]`.

#![allow(dead_code)]

use anyhow::{anyhow, Error};
use blowfish::cipher::block_padding::NoPadding;
use blowfish::cipher::generic_array::GenericArray;
use blowfish::cipher::{BlockDecryptMut, BlockEncryptMut, InnerIvInit, KeyInit};
use cbc::{Decryptor, Encryptor};
use md5::{Digest, Md5};

/// Размер каждого аудио-блока (2048 байт).
pub const CHUNK_SIZE: usize = 2048;

/// Фиксированный вектор инициализации (IV) для Blowfish-CBC.
pub const IV: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

/// Секретная константа Deezer для генерации ключа.
const KEY_SECRET: &[u8; 16] = b"g4el58wc0zvf9na1";

type BlowfishCbcDec = Decryptor<blowfish::Blowfish>;
type BlowfishCbcEnc = Encryptor<blowfish::Blowfish>;

/// Генерирует 16-байтный ключ Blowfish из `sng_id`.
pub fn generate_blowfish_key(sng_id: &str) -> [u8; 16] {
    let hash = Md5::digest(sng_id.as_bytes());
    let id_md5 = hex::encode(hash);
    let id_bytes = id_md5.as_bytes();
    let mut key = [0u8; 16];
    for i in 0..16 {
        key[i] = id_bytes[i] ^ id_bytes[i + 16] ^ KEY_SECRET[i];
    }
    key
}

/// Синоним `generate_blowfish_key`.
pub fn generate_key(sng_id: &str) -> [u8; 16] {
    generate_blowfish_key(sng_id)
}

/// Расшифровывает один 2048-байтовый чанк на месте, если `chunk_idx % 3 == 0` и `chunk.len() == 2048`.
pub fn decrypt_chunk(chunk: &mut [u8], chunk_idx: usize, key: &[u8; 16]) -> Result<(), Error> {
    if !chunk_idx.is_multiple_of(3) || chunk.len() != CHUNK_SIZE {
        return Ok(());
    }
    let bf = blowfish::Blowfish::new_from_slice(key)
        .map_err(|e| anyhow!("invalid blowfish key length: {e}"))?;
    let iv = GenericArray::from_slice(&IV);
    let cipher = BlowfishCbcDec::inner_iv_init(bf, iv);
    cipher
        .decrypt_padded_mut::<NoPadding>(chunk)
        .map_err(|e| anyhow!("blowfish decrypt error: {e}"))?;
    Ok(())
}

/// Шифрует один 2048-байтовый чанк на месте (обратная операция к `decrypt_chunk` для тестов).
pub fn encrypt_chunk(chunk: &mut [u8], chunk_idx: usize, key: &[u8; 16]) -> Result<(), Error> {
    if !chunk_idx.is_multiple_of(3) || chunk.len() != CHUNK_SIZE {
        return Ok(());
    }
    let bf = blowfish::Blowfish::new_from_slice(key)
        .map_err(|e| anyhow!("invalid blowfish key length: {e}"))?;
    let iv = GenericArray::from_slice(&IV);
    let cipher = BlowfishCbcEnc::inner_iv_init(bf, iv);
    cipher
        .encrypt_padded_mut::<NoPadding>(chunk, chunk.len())
        .map_err(|e| anyhow!("blowfish encrypt error: {e}"))?;
    Ok(())
}

/// Расшифровывает весь буфер потока по чанкам `CHUNK_SIZE` (каждый `chunk_idx % 3 == 0`).
pub fn decrypt_stream(data: &mut [u8], key: &[u8; 16]) -> Result<(), Error> {
    let (chunks, _rest) = data.as_chunks_mut::<CHUNK_SIZE>();
    for (block_idx, block) in chunks.iter_mut().enumerate() {
        decrypt_chunk(block, block_idx, key)?;
    }
    Ok(())
}

/// Шифрует весь буфер потока по чанкам `CHUNK_SIZE` (каждый `chunk_idx % 3 == 0`).
pub fn encrypt_stream(data: &mut [u8], key: &[u8; 16]) -> Result<(), Error> {
    let (chunks, _rest) = data.as_chunks_mut::<CHUNK_SIZE>();
    for (block_idx, block) in chunks.iter_mut().enumerate() {
        encrypt_chunk(block, block_idx, key)?;
    }
    Ok(())
}

/// Возвращает фиксированный IV.
pub fn iv_bytes() -> [u8; 8] {
    IV
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_buf(n: usize) -> Vec<u8> {
        (0..n)
            .map(|i| ((i * 0x9F) as u8).wrapping_add(0x42))
            .collect()
    }

    #[test]
    fn test_generate_key_length_is_16() {
        let key = generate_key("123456");
        assert_eq!(key.len(), 16);
    }

    #[test]
    fn test_generate_key_is_deterministic() {
        assert_eq!(generate_key("123456"), generate_blowfish_key("123456"));
        assert_eq!(generate_key("abc"), generate_key("abc"));
    }

    #[test]
    fn test_generate_key_differs_per_id() {
        assert_ne!(generate_key("123456"), generate_key("654321"));
        assert_ne!(generate_key("abc"), generate_key("xyz"));
    }

    #[test]
    fn test_plaintext_chunk_passes_through() {
        let key = generate_key("123456");
        let original = rand_buf(CHUNK_SIZE);
        let mut chunk = original.clone();
        decrypt_chunk(&mut chunk, 1, &key).unwrap();
        assert_eq!(chunk, original);
        decrypt_chunk(&mut chunk, 2, &key).unwrap();
        assert_eq!(chunk, original);
    }

    #[test]
    fn test_single_encrypted_chunk_roundtrip() {
        let key = generate_key("123456");
        let original = rand_buf(CHUNK_SIZE);
        let mut enc = original.clone();
        encrypt_chunk(&mut enc, 0, &key).unwrap();
        assert_ne!(enc, original);
        let mut dec = enc.clone();
        decrypt_chunk(&mut dec, 0, &key).unwrap();
        assert_eq!(dec, original);
    }

    #[test]
    fn test_stream_roundtrip_multi_chunk() {
        let key = generate_key("123456");
        let total = 6 * CHUNK_SIZE;
        let original = rand_buf(total);
        let mut enc = original.clone();
        encrypt_stream(&mut enc, &key).unwrap();
        let mut dec = enc.clone();
        decrypt_stream(&mut dec, &key).unwrap();
        assert_eq!(dec, original);
        assert_eq!(enc[CHUNK_SIZE..2 * CHUNK_SIZE], original[CHUNK_SIZE..2 * CHUNK_SIZE]);
        assert_ne!(enc[0..CHUNK_SIZE], original[0..CHUNK_SIZE]);
        assert_ne!(
            enc[3 * CHUNK_SIZE..4 * CHUNK_SIZE],
            original[3 * CHUNK_SIZE..4 * CHUNK_SIZE]
        );
    }

    #[test]
    fn test_stream_with_trailing_partial() {
        let key = generate_key("123456");
        let total = 2 * CHUNK_SIZE + 1024;
        let original = rand_buf(total);
        let mut enc = original.clone();
        encrypt_stream(&mut enc, &key).unwrap();
        let mut dec = enc.clone();
        decrypt_stream(&mut dec, &key).unwrap();
        assert_eq!(dec, original);
    }
}
