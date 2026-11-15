//! Поток `DeezerStream`, реализующий `Read + Seek + Send + Sync + MediaSource`.
//!
//! Поддерживает как синхронную работу с буфером, так и прогрессивную потоковую
//! загрузку (`fetch_streaming`), при которой воспроизведение начинается сразу после
//! буферизации первых 256 КБ аудиопотока.

#![allow(dead_code)]

pub use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use symphonia::core::io::MediaSource;

use crate::api::crypto::{self, CHUNK_SIZE};

/// Минимальный размер предбуферизации перед стартом декодера (256 КБ = 128 чанков по 2048 байт).
const INITIAL_PREBUFFER_BYTES: u64 = 256 * 1024;

/// Кэширующий поток расшифрованного аудио Deezer.
pub struct DeezerStream {
    file: File,
    total_len: Arc<AtomicU64>,
    written_len: Arc<AtomicU64>,
    finished: Arc<AtomicBool>,
    abort_download: Arc<AtomicBool>,
}

impl Drop for DeezerStream {
    fn drop(&mut self) {
        self.abort_download.store(true, Ordering::Relaxed);
    }
}

impl DeezerStream {
    /// Оборачивает уже готовый временный `File`.
    pub fn from_file(file: File) -> Self {
        let len = file.metadata().map(|m| m.len()).unwrap_or(0);
        Self {
            file,
            total_len: Arc::new(AtomicU64::new(len)),
            written_len: Arc::new(AtomicU64::new(len)),
            finished: Arc::new(AtomicBool::new(true)),
            abort_download: Arc::new(AtomicBool::new(false)),
        }
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
        let len = decrypted.len() as u64;
        Ok(Self {
            file,
            total_len: Arc::new(AtomicU64::new(len)),
            written_len: Arc::new(AtomicU64::new(len)),
            finished: Arc::new(AtomicBool::new(true)),
            abort_download: Arc::new(AtomicBool::new(false)),
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
        let len = encrypted.len() as u64;
        Ok(Self {
            file,
            total_len: Arc::new(AtomicU64::new(len)),
            written_len: Arc::new(AtomicU64::new(len)),
            finished: Arc::new(AtomicBool::new(true)),
            abort_download: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Скачивает зашифрованный трек целиком (используется для фонового Prefetch следующего трека).
    pub async fn fetch(http: &reqwest::Client, url: &str, track_id: &str) -> Result<Self> {
        let key = crypto::generate_blowfish_key(track_id);
        let resp = http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("CDN returned HTTP status {}", resp.status()));
        }
        let raw = resp.bytes().await?;
        Self::from_encrypted(&raw, &key)
    }

    /// Потоковое скачивание и дешифровка `BF_CBC_STRIPE` на лету:
    /// возвращает `DeezerStream` сразу после загрузки первых 256 КБ, продолжая докачку в фоне.
    pub async fn fetch_streaming(http: &reqwest::Client, url: &str, track_id: &str) -> Result<Self> {
        let key = crypto::generate_blowfish_key(track_id);
        let mut resp = http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("CDN returned HTTP status {}", resp.status()));
        }

        let content_len = resp.content_length().unwrap_or(0);
        let reader_file = tempfile::tempfile().context("не удалось создать tempfile")?;
        let mut writer_file = reader_file
            .try_clone()
            .context("не удалось клонировать дескриптор tempfile")?;

        let total_len = Arc::new(AtomicU64::new(content_len));
        let written_len = Arc::new(AtomicU64::new(0));
        let finished = Arc::new(AtomicBool::new(false));
        let abort_download = Arc::new(AtomicBool::new(false));

        let mut pending: Vec<u8> = Vec::with_capacity(CHUNK_SIZE * 4);
        let mut chunk_idx = 0usize;
        let mut chunk_buf = [0u8; CHUNK_SIZE];
        let mut written_so_far = 0u64;

        // 1. Буферизуем первые 256 КБ перед стартом декодера
        while written_so_far < INITIAL_PREBUFFER_BYTES {
            match resp.chunk().await? {
                Some(bytes) => {
                    pending.extend_from_slice(&bytes);
                    while pending.len() >= CHUNK_SIZE {
                        chunk_buf.copy_from_slice(&pending[..CHUNK_SIZE]);
                        pending.drain(..CHUNK_SIZE);
                        if chunk_idx.is_multiple_of(3) {
                            crypto::decrypt_chunk(&mut chunk_buf, chunk_idx, &key)?;
                        }
                        writer_file.write_all(&chunk_buf)?;
                        chunk_idx += 1;
                        written_so_far += CHUNK_SIZE as u64;
                    }
                    written_len.store(written_so_far, Ordering::Release);
                }
                None => {
                    if !pending.is_empty() {
                        writer_file.write_all(&pending)?;
                        written_so_far += pending.len() as u64;
                        pending.clear();
                    }
                    writer_file.flush()?;
                    written_len.store(written_so_far, Ordering::Release);
                    total_len.store(written_so_far, Ordering::Release);
                    finished.store(true, Ordering::Release);
                    break;
                }
            }
        }
        let _ = writer_file.flush();

        // 2. Если файл ещё не докачан до конца — продолжаем докачку в фоновой задаче tokio
        if !finished.load(Ordering::Acquire) {
            let written_clone = Arc::clone(&written_len);
            let total_clone = Arc::clone(&total_len);
            let finished_clone = Arc::clone(&finished);
            let abort_clone = Arc::clone(&abort_download);

            tokio::spawn(async move {
                while !abort_clone.load(Ordering::Relaxed) {
                    match resp.chunk().await {
                        Ok(Some(bytes)) => {
                            pending.extend_from_slice(&bytes);
                            while pending.len() >= CHUNK_SIZE {
                                chunk_buf.copy_from_slice(&pending[..CHUNK_SIZE]);
                                pending.drain(..CHUNK_SIZE);
                                if chunk_idx.is_multiple_of(3)
                                    && crypto::decrypt_chunk(&mut chunk_buf, chunk_idx, &key)
                                        .is_err()
                                {
                                    break;
                                }
                                if writer_file.write_all(&chunk_buf).is_err() {
                                    break;
                                }
                                chunk_idx += 1;
                                written_so_far += CHUNK_SIZE as u64;
                            }
                            let _ = writer_file.flush();
                            written_clone.store(written_so_far, Ordering::Release);
                        }
                        Ok(None) => {
                            if !pending.is_empty() {
                                let _ = writer_file.write_all(&pending);
                                written_so_far += pending.len() as u64;
                                pending.clear();
                            }
                            let _ = writer_file.flush();
                            written_clone.store(written_so_far, Ordering::Release);
                            total_clone.store(written_so_far, Ordering::Release);
                            break;
                        }
                        Err(_) => break,
                    }
                }
                finished_clone.store(true, Ordering::Release);
            });
        }

        let mut reader_file = reader_file;
        reader_file.seek(SeekFrom::Start(0))?;

        Ok(Self {
            file: reader_file,
            total_len,
            written_len,
            finished,
            abort_download,
        })
    }

    /// Длина потока в байтах.
    pub fn len(&self) -> u64 {
        let tot = self.total_len.load(Ordering::Acquire);
        if tot > 0 {
            tot
        } else {
            self.written_len.load(Ordering::Acquire)
        }
    }

    /// Пуст ли поток.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Read for DeezerStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            let pos = self.file.stream_position().unwrap_or(0);
            let written = self.written_len.load(Ordering::Acquire);
            let done = self.finished.load(Ordering::Acquire);

            if pos < written {
                let n = self.file.read(buf)?;
                if n > 0 || done {
                    return Ok(n);
                }
            } else if done {
                return Ok(0);
            }

            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

impl Seek for DeezerStream {
    fn seek(&mut self, target: SeekFrom) -> std::io::Result<u64> {
        let desired = match target {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(delta) => {
                let cur = self.file.stream_position().unwrap_or(0) as i64;
                (cur + delta).max(0) as u64
            }
            SeekFrom::End(delta) => {
                let end = self.len() as i64;
                (end + delta).max(0) as u64
            }
        };

        // Если перематываем вперёд в область, которая ещё докачивается — ждём до 1.5 с
        let mut waited_ms = 0u32;
        while desired > self.written_len.load(Ordering::Acquire)
            && !self.finished.load(Ordering::Acquire)
            && waited_ms < 1500
        {
            std::thread::sleep(Duration::from_millis(10));
            waited_ms += 10;
        }

        let max_avail = self.written_len.load(Ordering::Acquire);
        let safe_pos = if self.finished.load(Ordering::Acquire) {
            desired
        } else {
            desired.min(max_avail)
        };
        self.file.seek(SeekFrom::Start(safe_pos))
    }
}

impl MediaSource for DeezerStream {
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        let len = self.len();
        if len > 0 {
            Some(len)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for DeezerStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeezerStream({} bytes)", self.len())
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

