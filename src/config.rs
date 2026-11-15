//! Чтение и сохранение конфигурации в `~/.config/deezer-tui/config.toml`.

#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::api::models::AudioQuality;

/// Конфигурация приложения: arl-токен, громкость, качество потока и параметры UI/DSP.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    /// Аутентификационная кука Deezer (`arl`).
    /// Если не задана в `config.toml`, берётся из переменной окружения `DEEZER_ARL`.
    #[serde(default)]
    pub arl: Option<String>,
    /// Громкость воспроизведения в диапазоне `0.0..=1.0`.
    #[serde(default = "default_volume")]
    pub volume: f32,
    /// Частота дискретизации (Гц). `44100.0` — стандарт Deezer HiFi FLAC.
    #[serde(default = "default_sample_rate_hz")]
    pub sample_rate_hz: f32,
    /// Предпочитаемое качество потока: `"FLAC"`, `"MP3_320"`, `"MP3_128"`.
    #[serde(default = "default_preferred_quality")]
    pub preferred_quality: String,
    /// Отображать ли глобальный спектроанализатор снизу (переключается по `v`).
    #[serde(default = "default_show_spectrum")]
    pub show_spectrum: bool,
    /// Отображать ли синхронную панель текста песни на экранах Now Playing и Flow (переключается по `y`).
    #[serde(default = "default_show_lyrics")]
    pub show_lyrics: bool,
    /// Использовать ли логарифмическую шкалу децибел (`dB`) для спектра.
    #[serde(default)]
    pub db_scale: bool,
    /// Свёрнут ли левый сайдбар навигации (переключается по `s`).
    #[serde(default)]
    pub sidebar_collapsed: bool,
}

fn default_volume() -> f32 {
    0.8
}

fn default_sample_rate_hz() -> f32 {
    44_100.0
}

fn default_preferred_quality() -> String {
    "FLAC".to_string()
}

fn default_show_spectrum() -> bool {
    true
}

fn default_show_lyrics() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            arl: None,
            volume: default_volume(),
            sample_rate_hz: default_sample_rate_hz(),
            preferred_quality: default_preferred_quality(),
            show_spectrum: default_show_spectrum(),
            show_lyrics: default_show_lyrics(),
            db_scale: false,
            sidebar_collapsed: false,
        }
    }
}

impl AppConfig {
    /// Путь к `config.toml`: `~/.config/deezer-tui/config.toml`.
    /// Переменная окружения `DEEZER_TUI_CONFIG` позволяет переопределить путь.
    pub fn path() -> PathBuf {
        env::var("DEEZER_TUI_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home)
                    .join(".config/deezer-tui")
                    .join("config.toml")
            })
    }

    /// Загрузка конфигурации по стандартному пути.
    pub fn load() -> Self {
        Self::load_from(&Self::path())
    }

    /// Загрузка конфигурации из указанного файла.
    /// Если файл отсутствует или повреждён, возвращает значения по умолчанию.
    pub fn load_from(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => match toml::from_str::<AppConfig>(&contents) {
                Ok(mut config) => {
                    config.volume = normalize_volume(config.volume);
                    config
                }
                Err(e) => {
                    tracing::warn!(
                        "config: некорректный config.toml ({}): {e} — используем значения по умолчанию",
                        path.display()
                    );
                    Self::default()
                }
            },
            Err(_) => {
                tracing::info!(
                    "config: файл {} не найден — используем значения по умолчанию",
                    path.display()
                );
                Self::default()
            }
        }
    }

    /// Сохранение конфигурации по стандартному пути.
    pub fn save(&self) -> Result<()> {
        self.save_to(&Self::path())
    }

    /// Сохранение конфигурации по указанному пути (автоматически создаёт родительские директории).
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("не удалось создать каталог {}", parent.display()))?;
        }
        let mut normalized = self.clone();
        normalized.volume = normalize_volume(normalized.volume);

        let contents = toml::to_string(&normalized)
            .with_context(|| format!("ошибка сериализации конфига {}", path.display()))?;
        fs::write(path, contents)
            .with_context(|| format!("ошибка записи конфига {}", path.display()))?;
        Ok(())
    }

    /// Возвращает актуальный `arl`: сначала из `config.toml`, затем из `DEEZER_ARL`.
    pub fn effective_arl(&self) -> Option<String> {
        self.arl
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                env::var("DEEZER_ARL")
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
    }

    /// Конвертирует строковую настройку `preferred_quality` в `AudioQuality`.
    pub fn preferred_audio_quality(&self) -> AudioQuality {
        match self.preferred_quality.trim().to_uppercase().as_str() {
            "MP3_320" => AudioQuality::Mp3_320,
            "MP3_128" => AudioQuality::Mp3_128,
            "MP3_64" => AudioQuality::Mp3_64,
            _ => AudioQuality::Flac,
        }
    }
}

/// Нормализация громкости в диапазон `0.0..=1.0`.
pub fn normalize_volume(v: f32) -> f32 {
    if v.is_nan() {
        default_volume()
    } else {
        v.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_file_returns_defaults() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("nonexistent").join("config.toml");

        let config = AppConfig::load_from(&cfg_path);
        assert_eq!(config.volume, 0.8);
        assert_eq!(config.arl, None);
        assert_eq!(config.sample_rate_hz, 44_100.0);
        assert_eq!(config.preferred_quality, "FLAC");
        assert!(config.show_spectrum);
        assert!(config.show_lyrics);
        assert!(!config.db_scale);
        assert!(!config.sidebar_collapsed);
    }

    #[test]
    fn normalize_volume_clamps() {
        assert_eq!(normalize_volume(0.5), 0.5);
        assert_eq!(normalize_volume(-1.0), 0.0);
        assert_eq!(normalize_volume(2.0), 1.0);
        assert_eq!(normalize_volume(f32::NAN), 0.8);
    }

    #[test]
    fn save_creates_dir_and_loads_back() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("nested").join("config.toml");

        let config = AppConfig {
            arl: Some("test-arl-token".into()),
            volume: 0.65,
            sample_rate_hz: 44_100.0,
            preferred_quality: "MP3_320".into(),
            show_spectrum: false,
            show_lyrics: false,
            db_scale: true,
            sidebar_collapsed: true,
        };
        config.save_to(&cfg_path).expect("save_to failed");

        let loaded = AppConfig::load_from(&cfg_path);
        assert_eq!(loaded, config);
        assert_eq!(loaded.preferred_audio_quality(), AudioQuality::Mp3_320);
    }
}

