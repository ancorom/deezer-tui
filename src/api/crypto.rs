//! Криптовеска Deezer: генерация 16-байтного Blowfish-ключа из SNG_ID
//! (MD5 hex + XOR с секретом `g4el58wc0zvf9na1`) и BF_CBC_STRIPE-дешифровка
//! аудио-чанков по 2048 байт через [`cbc::Decryptor::<blowfish::Blowfish>]` с
//! фиксированным IV [0,1,2,3,4,5,6,7].
//!
//! Алгоритм `BF_CBC_STRIPE`: только каждый третий 2048-байтовый блок
//! (`chunk_idx % 3 == 0`) зашифрован Blowfish-CBC; остальные — чистые текст.
//! Каждая зашифрованная строка перезапускает цепочку CBC с одного и того же IV,
//! поэтому блоки расшифровываются независимо (стриминг).
 
use anyhow::Error;
use blowfish::cipher::block_padding::NoPadding;
use blowfish::cipher::generic_array::GenericArray;
use blowfish::cipher::{BlockDecryptMut, BlockEncryptMut, InnerIvInit, KeyInit};
use cbc::{Decryptor, Encryptor};
use md5::{Digest, Md5};

/// Размер каждого аудио-блока (2 KB).
#[allow(dead_code)]
pub const CHUNK_SIZE: usize = 2048;

/// Фиксированный IV для Blowfish CBC (используется для каждого зашифрованного чанка).
#[allow(dead_code)]
pub const IV: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

/// Фиксированный секрет для генерации ключа.
#[allow(dead_code)]
const KEY_SECRET: [u8; 16] = *b"g4el58wc0zvf9na1";

#[allow(dead_code)]
type BlowfishCbcDec = Decryptor<blowfish::Blowfish>;
#[allow(dead_code)]
type BlowfishCbcEnc = Encryptor<blowfish::Blowfish>;

/// Генерирует 16-байтный Blowfish-ключ из SNG_ID.
///
/// Формула:
/// ```text
/// key[i] = md5hex[i] ^ md5hex[i+16] ^ KEY_SECRET[i]   (i в 0..16)
/// ```
/// где `md5hex` — 32-символьная строка hex-кода MD5 от `sng_id`.
#[allow(dead_code)]
pub fn generate_key(sng_id: &str) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(sng_id.as_bytes());
    let hex = format!("{:x}", hasher.finalize());
    let bytes = hex.as_bytes();
    debug_assert_eq!(bytes.len(), 32, "MD5 hex length must be 32");
    let mut key = [0u8; 16];
    for i in 0..16 {
        key[i] = bytes[i] ^ bytes[i + 16] ^ KEY_SECRET[i];
    }
    key
}

/// Расшифровывает один 2048-байтовый чанк in-place через Blowfish-CBC
/// с фиксированным IV [0,1,2,3,4,5,6,7].
#[allow(dead_code)]
pub fn decrypt_chunk(chunk: &mut [u8], chunk_idx: usize, key: &[u8; 16]) -> Result<(), Error> {
    if !chunk_idx.is_multiple_of(3) || chunk.len() != CHUNK_SIZE {
        return Ok(());
    }
    let bf = blowfish::Blowfish::new_from_slice(key).map_err(Error::from)?;
    let iv = GenericArray::from_slice(&IV);
    let cipher = BlowfishCbcDec::inner_iv_init(bf, iv);
    cipher
        .decrypt_padded_mut::<NoPadding>(chunk)
        .map_err(Error::from)?;
    Ok(())
}

/// Шифрует один 2048-байтовый чанк in-place — обратная операция
/// к [`decrypt_chunk`](Self::decrypt_chunk).
#[allow(dead_code)]
pub fn encrypt_chunk(chunk: &mut [u8], chunk_idx: usize, key: &[u8; 16]) -> Result<(), Error> {
    if !chunk_idx.is_multiple_of(3) || chunk.len() != CHUNK_SIZE {
        return Ok(());
    }
    let bf = blowfish::Blowfish::new_from_slice(key).map_err(Error::from)?;
    let iv = GenericArray::from_slice(&IV);
    let cipher = BlowfishCbcEnc::inner_iv_init(bf, iv);
    cipher
        .encrypt_padded_mut::<NoPadding>(chunk, chunk.len())
        .map_err(Error::from)?;
    Ok(())
}

/// Расшифровывает весь буфер аудио in-place, обходя чанки по `CHUNK_SIZE`.
#[allow(dead_code)]
pub fn decrypt_stream(data: &mut [u8], key: &[u8; 16]) -> Result<(), Error> {
    for (block_idx, block) in data.as_chunks_mut::<CHUNK_SIZE>().0.iter_mut().enumerate() {
        decrypt_chunk(block, block_idx, key)?;
    }
    Ok(())
}

/// Шифрует весь буфер аудио in-place.
#[allow(dead_code)]
pub fn encrypt_stream(data: &mut [u8], key: &[u8; 16]) -> Result<(), Error> {
    for (block_idx, block) in data.as_chunks_mut::<CHUNK_SIZE>().0.iter_mut().enumerate() {
        encrypt_chunk(block, block_idx, key)?;
    }
    Ok(())
}

/// Возвращает фиксированный IV (полезно для тестов).
#[allow(dead_code)]
pub fn iv_bytes() -> [u8; 8] {
    IV
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Буфер с предсказуемым паттерном (чтобы тесты детерминированы).
    fn rand_buf(n: usize) -> Vec<u8> {
        let mut v = vec![0u8; n];
        for (i, b) in v.iter_mut().enumerate() {
            *b = ((i * 0x9F) as u8).wrapping_add(0x42);
        }
        v
    }

    #[test]
    fn test_generate_key_length_is_16() {
        let key = generate_key("123456");
        assert_eq!(key.len(), 16, "key must be 16 bytes");
    }

    #[test]
    fn test_generate_key_is_deterministic() {
        let k1 = generate_key("123456");
        let k2 = generate_key("123456");
        assert_eq!(k1, k2);
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
        // Индексы 1 и 2 — чистые тексты, не меняются.
        assert_eq!(decrypt_chunk(&mut chunk, 1, &key).unwrap(), ());
        assert_eq!(chunk, original, "plaintext chunk unchanged at idx 1");
        chunk = original.clone();
        assert_eq!(decrypt_chunk(&mut chunk, 2, &key).unwrap(), ());
        assert_eq!(chunk, original, "plaintext chunk unchanged at idx 2");
    }

    #[test]
    fn test_single_encrypted_chunk_roundtrip() {
        let key = generate_key("123456");
        let original = rand_buf(CHUNK_SIZE);
        let mut enc = original.clone();
        assert_eq!(encrypt_chunk(&mut enc, 0, &key).unwrap(), ());
        assert_ne!(enc, original, "encrypted chunk must differ from plaintext");
        let mut dec = enc.clone();
        assert_eq!(decrypt_chunk(&mut dec, 0, &key).unwrap(), ());
        assert_eq!(dec, original, "decrypt(encrypt(P)) == P at idx 0");
    }

    #[test]
    fn test_stream_roundtrip_multi_chunk() {
        let key = generate_key("123456");
        let total = 6 * CHUNK_SIZE;
        let original = rand_buf(total);
        let mut enc = original.clone();
        assert_eq!(encrypt_stream(&mut enc, &key).unwrap(), ());
        let mut dec = enc.clone();
        assert_eq!(decrypt_stream(&mut dec, &key).unwrap(), ());
        assert_eq!(dec, original, "decrypt_stream(encrypt_stream(P)) == P");
        // Чанки 1, 2, 4, 5 (не кратные 3) не менялись при шифровании:
        assert_eq!(
            enc[(CHUNK_SIZE)..(2 * CHUNK_SIZE)],
            original[(CHUNK_SIZE)..(2 * CHUNK_SIZE)],
        );
        assert_eq!(
            enc[(2 * CHUNK_SIZE)..(2 * CHUNK_SIZE + CHUNK_SIZE)],
            original[(2 * CHUNK_SIZE)..(2 * CHUNK_SIZE + CHUNK_SIZE)],
        );
        // Чанки 0, 3 (кратные 3) изменены:
        assert_ne!(
            enc[0..CHUNK_SIZE],
            original[0..CHUNK_SIZE],
        );
        assert_ne!(
            enc[(3 * CHUNK_SIZE)..(3 * CHUNK_SIZE + CHUNK_SIZE)],
            original[(3 * CHUNK_SIZE)..(3 * CHUNK_SIZE + CHUNK_SIZE)],
        );
    }

    #[test]
    fn test_stream_with_trailing_partial() {
        let key = generate_key("123456");
        let total = 2 * CHUNK_SIZE + 1024;
        let original = rand_buf(total);
        let mut enc = original.clone();
        assert_eq!(encrypt_stream(&mut enc, &key).unwrap(), ());
        let mut dec = enc.clone();
        assert_eq!(decrypt_stream(&mut dec, &key).unwrap(), ());
        assert_eq!(dec, original, "trailing partial survives round-trip");
        // Остаток не менялся (не полный чанк).
        assert_eq!(
            dec[2 * CHUNK_SIZE..2 * CHUNK_SIZE + 1024],
            original[2 * CHUNK_SIZE..2 * CHUNK_SIZE + 1024],
        );
    }
}
