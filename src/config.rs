//! Чтение и сохранение конфигурации в `~/.config/deezer-tui/config.toml`.

#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::api::models::AudioQuality;
use crate::dsp::cavacore::SpectrumScale;

/// Минимальная и максимальная высота панели спектра в строках терминала.
pub const SPECTRUM_ROWS_MIN: u16 = 2;
pub const SPECTRUM_ROWS_MAX: u16 = 12;

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
    /// По умолчанию `true`: линейная шкала даёт слишком большой динамический
    /// диапазон и «самый громкий столбик» подавляет остальные полосы.
    #[serde(default = "default_db_scale")]
    pub db_scale: bool,
    /// Шкала высот столбиков спектра: `"legacy"` (от 0 dBFS, окно 32 дБ),
    /// `"track_peak"` (от медленного пика трека, окно 26 дБ) или `"frame_peak"`
    /// (от пика кадра, окно 30 дБ). Переопредляется `DEEZER_TUI_SPECTRUM_SCALE`.
    #[serde(default = "default_spectrum_scale")]
    pub spectrum_scale: String,
    /// Высота панели спектра в строках терминала: `None` — автоматически (4–6 строк
    /// в зависимости от высоты окна), `Some(n)` — фиксированная (2–12).
    /// Переопределяется `DEEZER_TUI_SPECTRUM_ROWS` (`0` — авто).
    #[serde(default)]
    pub spectrum_rows: Option<u16>,
    /// База пространственного фильтра соседних полос (Monstercat): `0.0` — выключен
    /// (как в эталонной cava, по умолчанию), больше базы — сильнее «плато» вокруг
    /// доминирующей полосы (`1.35` — поведение шкалы v2.5). Значения ≤ 1.05 считаются
    /// выключенными. Переопределяется `DEEZER_TUI_SPECTRUM_SPREAD`.
    #[serde(default)]
    pub spectrum_spread: f32,
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

fn default_db_scale() -> bool {
    true
}

fn default_spectrum_scale() -> String {
    SpectrumScale::TrackPeak.as_str().to_string()
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
            db_scale: default_db_scale(),
            spectrum_scale: default_spectrum_scale(),
            spectrum_rows: None,
            spectrum_spread: 0.0,
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

    /// Активная шкала высот спектра: `DEEZER_TUI_SPECTRUM_SCALE` важнее `config.toml`,
    /// а некорректное значение приводит к `TrackPeak`.
    pub fn effective_spectrum_scale(&self) -> SpectrumScale {
        scale_from_setting(
            &self.spectrum_scale,
            env::var("DEEZER_TUI_SPECTRUM_SCALE").ok().as_deref(),
        )
    }

    /// Активная высота панели спектра: `DEEZER_TUI_SPECTRUM_ROWS` важнее `config.toml`,
    /// `0` или некорректное значение означает автоматическую высоту.
    pub fn effective_spectrum_rows(&self) -> Option<u16> {
        rows_from_setting(
            self.spectrum_rows,
            env::var("DEEZER_TUI_SPECTRUM_ROWS").ok().as_deref(),
        )
    }

    /// Активная база фильтра соседних полос: `DEEZER_TUI_SPECTRUM_SPREAD` важнее
    /// `config.toml`; `0`, «off» и некорректное значение означают «выключен».
    pub fn effective_spectrum_spread(&self) -> f32 {
        spread_from_setting(
            self.spectrum_spread,
            env::var("DEEZER_TUI_SPECTRUM_SPREAD").ok().as_deref(),
        )
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
/// Разрешение значения шкалы с учётом переопределения переменной окружения.
fn scale_from_setting(setting: &str, env_value: Option<&str>) -> SpectrumScale {
    if let Some(v) = env_value.filter(|v| !v.trim().is_empty())
        && let Ok(mode) = v.trim().parse::<SpectrumScale>()
    {
        return mode;
    }
    setting
        .trim()
        .parse::<SpectrumScale>()
        .unwrap_or(SpectrumScale::TrackPeak)
}

/// Разрешение высоты панели спектра: `0` и некорректные значения = авто.
/// Чистое разрешение базы фильтра соседних полос (без чтения окружения).
fn spread_from_setting(setting: f32, env_value: Option<&str>) -> f32 {
    let raw = match env_value.filter(|v| !v.trim().is_empty()) {
        Some("off") | Some("OFF") | Some("0") => 0.0,
        Some(v) => match v.trim().parse::<f32>() {
            Ok(v) => v,
            Err(_) => setting,
        },
        None => setting,
    };
    if raw <= crate::dsp::cavacore::SPREAD_MIN_BASE {
        0.0
    } else {
        raw.clamp(crate::dsp::cavacore::SPREAD_MIN_BASE, crate::dsp::cavacore::SPREAD_MAX)
    }
}

fn rows_from_setting(setting: Option<u16>, env_value: Option<&str>) -> Option<u16> {
    let raw = env_value
        .filter(|v| !v.trim().is_empty())
        .and_then(|v| v.trim().parse::<u16>().ok())
        .or(setting);
    match raw {
        Some(0) | None => None,
        Some(n) => Some(n.clamp(SPECTRUM_ROWS_MIN, SPECTRUM_ROWS_MAX)),
    }
}

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
    fn scale_and_rows_resolution() {
        use crate::dsp::cavacore::SpectrumScale;
        assert_eq!(scale_from_setting("legacy", None), SpectrumScale::Legacy);
        assert_eq!(scale_from_setting("bogus", None), SpectrumScale::TrackPeak);
        assert_eq!(
            scale_from_setting("legacy", Some("frame_peak")),
            SpectrumScale::FramePeak
        );
        assert_eq!(rows_from_setting(None, None), None);
        assert_eq!(rows_from_setting(Some(8), None), Some(8));
        assert_eq!(rows_from_setting(Some(99), None), Some(SPECTRUM_ROWS_MAX));
        assert_eq!(rows_from_setting(Some(8), Some("0")), None);
        assert_eq!(rows_from_setting(Some(8), Some("3")), Some(3));
        assert_eq!(rows_from_setting(Some(1), None), Some(SPECTRUM_ROWS_MIN));
        assert_eq!(spread_from_setting(0.0, None), 0.0);
        assert_eq!(spread_from_setting(1.35, None), 1.35);
        assert_eq!(
            spread_from_setting(1.35, Some("2.0")),
            2.0,
            "переменная окружения важнее config.toml"
        );
        assert_eq!(
            spread_from_setting(2.0, Some("off")),
            0.0,
            "«off» выключает фильтр"
        );
        assert_eq!(
            spread_from_setting(2.0, Some("bogus")),
            2.0,
            "некорректное значение окружения сохраняет настройку"
        );
        assert_eq!(
            spread_from_setting(99.0, None),
            crate::dsp::cavacore::SPREAD_MAX,
            "база ограничивается сверху"
        );
        assert_eq!(
            spread_from_setting(1.0, None),
            0.0,
            "база ниже порога включения считается выключенной"
        );
    }

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
        assert!(config.db_scale);
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
            spectrum_scale: "frame_peak".into(),
            spectrum_rows: Some(8),
            spectrum_spread: 2.0,
            sidebar_collapsed: true,
        };
        config.save_to(&cfg_path).expect("save_to failed");

        let loaded = AppConfig::load_from(&cfg_path);
        assert_eq!(loaded, config);
        assert_eq!(loaded.preferred_audio_quality(), AudioQuality::Mp3_320);
    }
}

