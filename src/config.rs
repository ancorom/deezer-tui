//! Чтение и сохранение конфигурации в `~/.config/deezer-tui/config.toml`.

#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Конфигурация приложения: arl-токен, громкость и базовые аудио-настройки.
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
}

fn default_volume() -> f32 {
    0.8
}

fn default_sample_rate_hz() -> f32 {
    44_100.0
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            arl: None,
            volume: default_volume(),
            sample_rate_hz: default_sample_rate_hz(),
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
        };
        config.save_to(&cfg_path).expect("save_to failed");

        let loaded = AppConfig::load_from(&cfg_path);
        assert_eq!(loaded, config);
    }
}
