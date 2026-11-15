//! Поток `DeezerStream`, реализующий `Read + Seek + Send + Sync + MediaSource`.
//!
//! Использует разделяемый буфер в памяти `Arc<RwLock<Vec<u8>>>` с независимым курсором
//! чтения `pos: usize`, что полностью исключает гонки файловых дескрипторов при
//! одновременной фоновой докачке (`fetch_streaming`) и чтении декодером `Symphonia`.

#![allow(dead_code)]

use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::{anyhow, Result};
use symphonia::core::io::MediaSource;

use crate::api::crypto::{self, CHUNK_SIZE};

/// Минимальный размер предбуферизации перед стартом декодера (256 КБ = 128 чанков по 2048 байт).
const INITIAL_PREBUFFER_BYTES: usize = 256 * 1024;

/// Потоковый буфер расшифрованного аудио Deezer с независимым курсором чтения.
pub struct DeezerStream {
    data: Arc<RwLock<Vec<u8>>>,
    pos: usize,
    total_len: Arc<AtomicU64>,
    written_len: Arc<AtomicU64>,
    finished: Arc<AtomicBool>,
    abort_download: Arc<AtomicBool>,
}

impl Clone for DeezerStream {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            pos: 0,
            total_len: Arc::clone(&self.total_len),
            written_len: Arc::clone(&self.written_len),
            finished: Arc::clone(&self.finished),
            abort_download: Arc::clone(&self.abort_download),
        }
    }
}

impl Drop for DeezerStream {
    fn drop(&mut self) {
        self.abort_download.store(true, Ordering::Relaxed);
    }
}

impl DeezerStream {
    /// Создаёт `DeezerStream` из уже расшифрованного вектора байт.
    pub fn from_decrypted(decrypted: Vec<u8>) -> Self {
        let len = decrypted.len() as u64;
        Self {
            data: Arc::new(RwLock::new(decrypted)),
            pos: 0,
            total_len: Arc::new(AtomicU64::new(len)),
            written_len: Arc::new(AtomicU64::new(len)),
            finished: Arc::new(AtomicBool::new(true)),
            abort_download: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Создаёт `DeezerStream` из среза расшифрованных байт.
    pub fn try_from_decrypted(decrypted: &[u8]) -> Result<Self> {
        Ok(Self::from_decrypted(decrypted.to_vec()))
    }

    /// Принимает зашифрованный `BF_CBC_STRIPE` буфер, расшифровывает каждый блок
    /// `chunk_idx % 3 == 0` размером ровно 2048 байт и сохраняет результат в памяти.
    pub fn from_encrypted(encrypted: &[u8], key: &[u8; 16]) -> Result<Self> {
        let mut out = Vec::with_capacity(encrypted.len());
        let mut chunk_buf = [0u8; CHUNK_SIZE];

        for (chunk_idx, slice) in encrypted.chunks(CHUNK_SIZE).enumerate() {
            if slice.len() == CHUNK_SIZE && chunk_idx.is_multiple_of(3) {
                chunk_buf.copy_from_slice(slice);
                crypto::decrypt_chunk(&mut chunk_buf, chunk_idx, key)?;
                out.extend_from_slice(&chunk_buf);
            } else {
                out.extend_from_slice(slice);
            }
        }

        Ok(Self::from_decrypted(out))
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
        let cap = (content_len as usize).clamp(INITIAL_PREBUFFER_BYTES, 64 * 1024 * 1024);
        let data = Arc::new(RwLock::new(Vec::with_capacity(cap)));
        let total_len = Arc::new(AtomicU64::new(content_len));
        let written_len = Arc::new(AtomicU64::new(0));
        let finished = Arc::new(AtomicBool::new(false));
        let abort_download = Arc::new(AtomicBool::new(false));

        let mut pending: Vec<u8> = Vec::with_capacity(CHUNK_SIZE * 8);
        let mut chunk_idx = 0usize;
        let mut chunk_buf = [0u8; CHUNK_SIZE];
        let mut written_so_far = 0usize;

        // 1. Буферизуем первые 256 КБ перед стартом декодера
        while written_so_far < INITIAL_PREBUFFER_BYTES {
            match resp.chunk().await? {
                Some(bytes) => {
                    pending.extend_from_slice(&bytes);
                    if pending.len() >= CHUNK_SIZE {
                        let mut batch = Vec::with_capacity(pending.len());
                        while pending.len() >= CHUNK_SIZE {
                            chunk_buf.copy_from_slice(&pending[..CHUNK_SIZE]);
                            pending.drain(..CHUNK_SIZE);
                            if chunk_idx.is_multiple_of(3) {
                                crypto::decrypt_chunk(&mut chunk_buf, chunk_idx, &key)?;
                            }
                            batch.extend_from_slice(&chunk_buf);
                            chunk_idx += 1;
                            written_so_far += CHUNK_SIZE;
                        }
                        if let Ok(mut guard) = data.write() {
                            guard.extend_from_slice(&batch);
                        }
                        written_len.store(written_so_far as u64, Ordering::Release);
                    }
                }
                None => {
                    if !pending.is_empty() {
                        if let Ok(mut guard) = data.write() {
                            guard.extend_from_slice(&pending);
                        }
                        written_so_far += pending.len();
                        pending.clear();
                    }
                    written_len.store(written_so_far as u64, Ordering::Release);
                    total_len.store(written_so_far as u64, Ordering::Release);
                    finished.store(true, Ordering::Release);
                    break;
                }
            }
        }

        // 2. Если файл ещё не докачан до конца — продолжаем докачку в фоновой задаче tokio
        if !finished.load(Ordering::Acquire) {
            let data_clone = Arc::clone(&data);
            let written_clone = Arc::clone(&written_len);
            let total_clone = Arc::clone(&total_len);
            let finished_clone = Arc::clone(&finished);
            let abort_clone = Arc::clone(&abort_download);

            tokio::spawn(async move {
                while !abort_clone.load(Ordering::Relaxed) {
                    match resp.chunk().await {
                        Ok(Some(bytes)) => {
                            pending.extend_from_slice(&bytes);
                            if pending.len() >= CHUNK_SIZE {
                                let mut batch = Vec::with_capacity(pending.len());
                                while pending.len() >= CHUNK_SIZE {
                                    chunk_buf.copy_from_slice(&pending[..CHUNK_SIZE]);
                                    pending.drain(..CHUNK_SIZE);
                                    if chunk_idx.is_multiple_of(3)
                                        && crypto::decrypt_chunk(&mut chunk_buf, chunk_idx, &key)
                                            .is_err()
                                    {
                                        break;
                                    }
                                    batch.extend_from_slice(&chunk_buf);
                                    chunk_idx += 1;
                                    written_so_far += CHUNK_SIZE;
                                }
                                if let Ok(mut guard) = data_clone.write() {
                                    guard.extend_from_slice(&batch);
                                }
                                written_clone.store(written_so_far as u64, Ordering::Release);
                            }
                        }
                        Ok(None) => {
                            if !pending.is_empty() {
                                if let Ok(mut guard) = data_clone.write() {
                                    guard.extend_from_slice(&pending);
                                }
                                written_so_far += pending.len();
                                pending.clear();
                            }
                            written_clone.store(written_so_far as u64, Ordering::Release);
                            total_clone.store(written_so_far as u64, Ordering::Release);
                            break;
                        }
                        Err(_) => break,
                    }
                }
                finished_clone.store(true, Ordering::Release);
            });
        }

        Ok(Self {
            data,
            pos: 0,
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
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if self.abort_download.load(Ordering::Relaxed) {
                return Ok(0);
            }
            let written = self.written_len.load(Ordering::Acquire) as usize;
            let done = self.finished.load(Ordering::Acquire);

            if self.pos < written {
                if let Ok(guard) = self.data.read() {
                    let avail = guard.len().saturating_sub(self.pos);
                    if avail > 0 {
                        let to_copy = avail.min(buf.len());
                        buf[..to_copy].copy_from_slice(&guard[self.pos..self.pos + to_copy]);
                        self.pos += to_copy;
                        return Ok(to_copy);
                    }
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
        let desired: i64 = match target {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::Current(delta) => (self.pos as i64) + delta,
            SeekFrom::End(delta) => (self.len() as i64) + delta,
        };
        let desired_pos = desired.max(0) as usize;

        // Если целевая позиция ещё докачивается — ждём до 1.5 с (прерываясь при отмене)
        let mut waited_ms = 0u32;
        while desired_pos > self.written_len.load(Ordering::Acquire) as usize
            && !self.finished.load(Ordering::Acquire)
            && !self.abort_download.load(Ordering::Relaxed)
            && waited_ms < 1500
        {
            std::thread::sleep(Duration::from_millis(10));
            waited_ms += 10;
        }

        let max_avail = self.written_len.load(Ordering::Acquire) as usize;
        self.pos = if self.finished.load(Ordering::Acquire) {
            desired_pos.min(max_avail)
        } else {
            desired_pos
        };
        Ok(self.pos as u64)
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
