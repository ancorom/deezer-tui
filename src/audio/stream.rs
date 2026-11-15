//! Поток `DeezerStream`, реализующий `Read + Seek + Send + Sync + MediaSource`.
//!
//! Расшифровывает каждый чанк `chunk_idx % 3 == 0` размером ровно 2048 байт
//! через Blowfish-CBC и сохраняет данные во временный файл `tempfile`,
//! обеспечивая мгновенный `Seek` вперёд и назад для декодера `symphonia`.

#![allow(dead_code)]

pub use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use anyhow::{anyhow, Context, Result};
use symphonia::core::io::MediaSource;

use crate::api::crypto::{self, CHUNK_SIZE};

/// Кэширующий поток расшифрованного аудио Deezer.
pub struct DeezerStream {
    file: File,
    total_len: u64,
}

impl DeezerStream {
    /// Оборачивает уже готовый временный `File`.
    pub fn from_file(file: File) -> Self {
        let total_len = file.metadata().map(|m| m.len()).unwrap_or(0);
        Self { file, total_len }
    }

    /// Создаёт `DeezerStream` из уже расшифрованного буфера байт.
    pub fn from_decrypted(decrypted: Vec<u8>) -> Self {
        Self::try_from_decrypted(&decrypted).expect("failed to write decrypted buffer to tempfile")
    }

    /// Безопасно записывает расшифрованные байты в `tempfile` и возвращает `DeezerStream`.
    pub fn try_from_decrypted(decrypted: &[u8]) -> Result<Self> {
        let mut file = tempfile::tempfile().context("не удалось создать tempfile")?;
        file.write_all(decrypted)
            .context("ошибка записи аудиопотока в tempfile")?;
        file.flush().context("ошибка сброса буфера tempfile")?;
        file.seek(SeekFrom::Start(0))
            .context("ошибка перемотки tempfile в начало")?;
        Ok(Self {
            file,
            total_len: decrypted.len() as u64,
        })
    }

    /// Принимает зашифрованный `BF_CBC_STRIPE` буфер, расшифровывает каждый блок
    /// `chunk_idx % 3 == 0` размером ровно 2048 байт и сохраняет результат в `tempfile`.
    pub fn from_encrypted(encrypted: &[u8], key: &[u8; 16]) -> Result<Self> {
        let mut file = tempfile::tempfile().context("не удалось создать tempfile")?;
        let mut chunk_buf = [0u8; CHUNK_SIZE];

        for (chunk_idx, slice) in encrypted.chunks(CHUNK_SIZE).enumerate() {
            if slice.len() == CHUNK_SIZE && chunk_idx.is_multiple_of(3) {
                chunk_buf.copy_from_slice(slice);
                crypto::decrypt_chunk(&mut chunk_buf, chunk_idx, key)?;
                file.write_all(&chunk_buf)?;
            } else {
                file.write_all(slice)?;
            }
        }

        file.flush()?;
        file.seek(SeekFrom::Start(0))?;
        Ok(Self {
            file,
            total_len: encrypted.len() as u64,
        })
    }

    /// Скачивает зашифрованный трек с CDN Deezer, расшифровывает чанки на лету и возвращает `DeezerStream`.
    pub async fn fetch(http: &reqwest::Client, url: &str, track_id: &str) -> Result<Self> {
        let key = crypto::generate_blowfish_key(track_id);
        let resp = http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("CDN returned HTTP status {}", resp.status()));
        }
        let raw = resp.bytes().await?;
        Self::from_encrypted(&raw, &key)
    }

    /// Длина потока в байтах.
    pub fn len(&self) -> u64 {
        self.total_len
    }

    /// Пуст ли поток.
    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }
}

impl Read for DeezerStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for DeezerStream {
    fn seek(&mut self, target: SeekFrom) -> std::io::Result<u64> {
        self.file.seek(target)
    }
}

impl MediaSource for DeezerStream {
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        Some(self.total_len)
    }
}

impl std::fmt::Debug for DeezerStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeezerStream({} bytes)", self.total_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek, SeekFrom};

    fn sample(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i * 0x9F + 0x37) as u8).collect()
    }

    #[test]
    fn test_seek_start_and_unaligned_read() {
        let data = sample(10_000);
        let mut stream = DeezerStream::from_decrypted(data.clone());
        assert!(stream.is_seekable());
        assert_eq!(stream.byte_len(), Some(10_000));

        let pos = stream.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(pos, 0);

        let mut buf = vec![0u8; 777];
        let n = stream.read(&mut buf).unwrap();
        assert_eq!(n, 777);
        assert_eq!(&buf, &data[0..777]);
    }

    #[test]
    fn test_seek_middle_and_read_cont() {
        let data = sample(30_000);
        let mut stream = DeezerStream::from_decrypted(data.clone());
        let offset = 15_371u64;
        stream.seek(SeekFrom::Start(offset)).unwrap();

        let mut buf = vec![0u8; 500];
        let n = stream.read(&mut buf).unwrap();
        assert_eq!(n, 500);
        assert_eq!(&buf, &data[offset as usize..offset as usize + 500]);
    }

    #[test]
    fn test_8192_bytes_encrypted_stripes_with_arbitrary_seeks_and_unaligned_reads() {
        // Критерий приёмки Этапа 4 из PLAN.md:
        // Буфер 8192 байт (4 чанка по 2048 байт, чанки 0 и 3 зашифрованы Blowfish-CBC).
        let key = crypto::generate_blowfish_key("3135556");
        let original = sample(8192);
        let mut encrypted = original.clone();
        crypto::encrypt_stream(&mut encrypted, &key).unwrap();

        assert_ne!(&encrypted[0..2048], &original[0..2048]);
        assert_eq!(&encrypted[2048..4096], &original[2048..4096]);
        assert_eq!(&encrypted[4096..6144], &original[4096..6144]);
        assert_ne!(&encrypted[6144..8192], &original[6144..8192]);

        let mut stream = DeezerStream::from_encrypted(&encrypted, &key).unwrap();
        assert_eq!(stream.len(), 8192);

        for &(offset, len) in &[
            (0usize, 333usize),
            (1900, 400),
            (6000, 500),
            (7500, 692),
            (123, 4567),
        ] {
            stream.seek(SeekFrom::Start(offset as u64)).unwrap();
            let mut buf = vec![0u8; len];
            stream.read_exact(&mut buf).unwrap();
            assert_eq!(
                &buf[..],
                &original[offset..offset + len],
                "несовпадение данных при чтении со смещения {offset} длины {len}"
            );
        }
    }
}
