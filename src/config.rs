// Конфигурация ~/.config/deezer-tui/config.toml

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

/// Конфигурация приложения: arl-токен, громкость и настройки.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    /// Аутентификационный кука Deezer (arl).
    /// Если не задан в config.toml, берётся из переменной окружения DEEZER_ARL.
    #[serde(default)]
    pub arl: Option<String>,
    /// Громкость воспроизведения, 0.0..=1.0.
    #[serde(default = "default_volume")]
    pub volume: f32,
    /// Частота воспроизведения (Гц). 44100 — стандарт Deezer FLAC.
    #[serde(default = "default_sample_rate_hz")]
    pub sample_rate_hz: f32,
}

#[allow(dead_code)]
fn default_volume() -> f32 {
    0.8
}

#[allow(dead_code)]
fn default_sample_rate_hz() -> f32 {
    44100.0
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

#[allow(dead_code)]
impl AppConfig {
    /// Путь к config.toml: ~/.config/deezer-tui/config.toml
    /// Переменная окружения DEEZER_TUI_CONFIG позволяет переопределить путь.
    pub fn path() -> PathBuf {
        env::var("DEEZER_TUI_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = env::var("HOME").unwrap_or_default();
                PathBuf::from(home)
                    .join(".config/deezer-tui")
                    .join("config.toml")
            })
    }

    /// Загрузка конфигурации: если файл отсутствует или некорректен — значения по умолчанию.
    pub fn load() -> Self {
        let path = Self::path();
        match fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str::<AppConfig>(&contents) {
                Ok(config) => config,
                Err(e) => {
                    tracing::warn!(
                        "config: некорректный config.toml ({}): {e} — значения по умолчанию",
                        path.display()
                    );
                    Self::default()
                }
            },
            Err(_) => {
                tracing::info!(
                    "config: config.toml не найден ({}): значения по умолчанию",
                    path.display()
                );
                Self::default()
            }
        }
    }

    /// Сохранение конфигурации (создаёт каталог ~/.config/deezer-tui при необходимости).
    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string(self)
            .with_context(|| format!("serialize config for {}", path.display()))?;
        fs::write(&path, contents)
            .with_context(|| format!("write config {}", path.display()))?;
        Ok(())
    }

    /// Фактический arl: сначала из config.toml, затем из DEEZER_ARL.
    pub fn effective_arl(&self) -> Option<String> {
        self.arl
            .clone()
            .or_else(|| env::var("DEEZER_ARL").ok())
    }
}

/// Громкость в диапазоне 0.0..=1.0.
#[allow(dead_code)]
pub fn normalize_volume(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_file_returns_defaults() {
        let path = "/tmp/deezer-tui-test-nonexistent-dir/config.toml";
        unsafe { std::env::set_var("DEEZER_TUI_CONFIG", path) };
        let config = AppConfig::load();
        unsafe { std::env::remove_var("DEEZER_TUI_CONFIG") };
        assert_eq!(config.volume, 0.8);
        assert_eq!(config.arl, None);
        assert_eq!(config.sample_rate_hz, 44100.0);
    }

    #[test]
    fn normalize_volume_clamps() {
        assert_eq!(normalize_volume(0.5), 0.5);
        assert_eq!(normalize_volume(-1.0), 0.0);
        assert_eq!(normalize_volume(2.0), 1.0);
    }

    #[test]
    fn save_creates_dir_and_loads_back() {
        let path = "/tmp/deezer-tui-config-save-test/config.toml";
        unsafe { std::env::set_var("DEEZER_TUI_CONFIG", path) };
        let config = AppConfig {
            arl: Some("test-arl".into()),
            volume: 0.5,
            sample_rate_hz: 44100.0,
        };
        config.save().expect("save config");
        let loaded = AppConfig::load();
        unsafe { std::env::remove_var("DEEZER_TUI_CONFIG") };
        assert_eq!(loaded.arl, Some("test-arl".into()));
        assert_eq!(loaded.volume, 0.5);
    }
}
