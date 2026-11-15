//! Стриминг аудио Deezer: `DeezerStream` — это `Read + Seek + Send + Sync`
//! поток, который можно передать в `ReadOnlySource` (обёртка symphonia в
//! [`MediaSource`]). Расшифровывает каждый 2048-байтовый чанк, где
//! `chunk_idx % 3 == 0`, через [`crypto`](crate::api::crypto) и пишет
//! расшифрованный звук в анонимный `TempFile`, чтобы `Seek` был мгновенным
//! (O(1)) — файл unlink'нут и освободится из подсистемы файлов, как только
//! закроется файловый дескриптор.
//!
//! # Инкапсуляция
//!
//! `DeezerStream` хранит только `std::fs::File`, поэтому он легко может быть
//! `Send + Sync` (внутри только один `File`).
pub use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use anyhow::{anyhow, Error};
use crate::api::crypto;

/// Стриминг-обёртка над расшифрованным аудио Deezer.
#[allow(dead_code)]
pub struct DeezerStream {
    /// Файл с расшифрованным аудио (анонимный, unlink'нутый).
    file: File,
    /// Итого (расшифрованные) байты.
    total_len: u64,
}

impl DeezerStream {
    /// Обёртка вокруг уже созданного `TempFile` с расшифрованным звуком.
    #[allow(dead_code)]
    pub fn from_file(file: File) -> Self {
        let total_len = file
            .metadata()
            .map(|m| m.len())
            .unwrap_or(0);
        Self { file, total_len }
    }

    /// Создаёт `DeezerStream` из уже расшифрованных байтов: пишет их в
    /// анонимный `TempFile` и возвращает обёртку.
    ///
    /// Файл unlink'нут, поэтому `seek` мгновенный: не требуется перемотка по
    /// памяти, только системный вызов `lseek`.
    #[allow(dead_code)]
    pub fn from_decrypted(decrypted: Vec<u8>) -> Self {
        let mut file = tempfile::tempfile().expect("create tempfile");
        file
            .write_all(&decrypted)
            .expect("write to tempfile");
        file.flush().expect("flush tempfile");
        // Вернуть точку чтения в начало — после `write_all` позиция в конце.
        file
            .seek(SeekFrom::Start(0))
            .expect("seek to start after writing");
        let total_len = decrypted.len() as u64;
        Self { file, total_len }
    }

    /// Скачивает, расшифровывает и возвращает `DeezerStream`.
    #[allow(dead_code)]
    pub async fn fetch(
        http: &reqwest::Client,
        url: &str,
        track_id: &str,
    ) -> Result<Self, Error> {
        let key = crypto::generate_key(track_id);
        let resp = http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!(
                "CDN returned status {}",
                resp.status(),
            ));
        }
        let raw = resp.bytes().await?;
        let encrypted: Vec<u8> = {
            let mut encrypted = vec![0u8; raw.len()];
            encrypted.copy_from_slice(&raw);
            encrypted
        };
        let mut decrypted = encrypted;
        crypto::decrypt_stream(&mut decrypted, &key)?;
        Ok(Self::from_decrypted(decrypted))
    }

    /// Длина расшифрованного звука.
    #[allow(dead_code)]
    pub fn len(&self) -> u64 {
        self.total_len
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

impl std::fmt::Debug for DeezerStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeezerStream({})", self.total_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, SeekFrom};

    fn sample(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i * 0x9F + 0x37) as u8).collect()
    }

    #[test]
    fn test_seek_start_and_unaligned_read() {
        let data = sample(10000);
        let mut stream = DeezerStream::from_decrypted(data.clone());
        // SeekFrom::Start(0) — перемещаем к началу.
        let pos = stream
            .seek(SeekFrom::Start(0))
            .unwrap();
        assert_eq!(pos, 0);
        // Не-выровненный read: читаем 777 байт (не кратное 2048).
        let mut buf = vec![0u8; 777];
        let n = stream.read(&mut buf).unwrap();
        assert_eq!(n, 777);
        assert_eq!(&buf, &data[0..777]);
    }

    #[test]
    fn test_seek_middle_and_read_cont() {
        let data = sample(30000);
        let mut stream = DeezerStream::from_decrypted(data.clone());
        let offset = 15371u64;
        stream
            .seek(SeekFrom::Start(offset))
            .unwrap();
        let mut buf = vec![0u8; 500];
        let n = stream.read(&mut buf).unwrap();
        assert_eq!(n, 500);
        assert_eq!(&buf, &data[offset as usize..offset as usize + 500]);
    }

    #[test]
    fn test_read_exact_end() {
        let data = sample(4000);
        let mut stream = DeezerStream::from_decrypted(data.clone());
        let mut buf = vec![0u8; 4000];
        stream.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, &data);
    }

    #[test]
    fn test_from_decrypted_len() {
        let data = sample(12345);
        let stream = DeezerStream::from_decrypted(data);
        assert_eq!(stream.len(), 12345);
    }

    #[test]
    fn test_decrypted_roundtrip() {
        // Имитация: шифруем, затем расшифровываем через crypto и проверяем,
        // что read() из потока вернёт исходные данные.
        let key = crypto::generate_key("123456");
        let original = sample(3 * crypto::CHUNK_SIZE);
        let mut enc = original.clone();
        crypto::encrypt_stream(&mut enc, &key).unwrap();
        let decrypted = {
            let mut d = enc.clone();
            crypto::decrypt_stream(&mut d, &key).unwrap();
            d
        };
        assert_eq!(decrypted, original);
        let mut stream = DeezerStream::from_decrypted(decrypted);
        let mut buf = vec![0u8; 123];
        stream
            .seek(SeekFrom::Start(0))
            .unwrap();
        assert_eq!(stream.read(&mut buf).unwrap(), 123);
        assert_eq!(&buf, &original[0..123]);
    }
}
