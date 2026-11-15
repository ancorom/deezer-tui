# Аудит и перезапись Фаз 1 и 2 (`deezer-tui`)

Ниже приведён список критических ошибок, найденных в коде локальной модели, и готовые исправленные версии всех файлов Фаз 1 и 2, полностью соответствующие `AGENT.md`, `ARCHITECTURE.md` и строгим проверкам `cargo clippy -- -D warnings`.

---

## 1. Что локальная модель сделала неправильно (Аудит)

### `src/main.rs`
1. **Опечатка в пути к логам:** вместо скрытой папки `.config/deezer-tui` модель написала `config/deezer-tui` (без точки), из-за чего в домашней директории создавалась лишняя папка `~/config/`.
2. **Отсутствие создания директории и ANSI-кодов:** перед инициализацией логгера лучше убедиться, что директория существует (`fs::create_dir_all`), и отключить ANSI-цвета (`with_ansi(false)`), чтобы в `deezer-tui.log` не было управляющих escape-последовательностей.

### `src/config.rs`
1. **Гонка данных (Data Race) в unit-тестах:** модель использовала `unsafe { std::env::set_var("DEEZER_TUI_CONFIG", ...) }` одновременно в двух тестах (`load_missing_file_returns_defaults` и `save_creates_dir_and_loads_back`). Так как `cargo test` запускает тесты параллельно в разных потоках одного процесса, один поток удаляет переменную окружения в тот момент, когда второй поток читает или пишет конфиг.
   * **Решение:** добавить методы `load_from(&Path)` и `save_to(&Path)` и тестировать их через изолированные временные папки `tempfile::tempdir()` без `unsafe` и мутации глобального окружения.

### `src/dsp/cavacore.rs`
1. **Неверный масштаб EQ (`1.0 / 2^28`), убивающий визуализацию:** модель скопировала целочисленный делитель `1.0 / 2.0f32.powf(28.0)` из Си-кода, рассчитанного на ненормализованные 16-битные сэмплы, хотя в нашем конвейере сэмплы уже нормализованы в `[-1.0, 1.0]`. Из-за этого на полной громкости высота столбиков составляла `~0.00001` вместо `[0.0, 1.0]` (в тесте модели даже пришлось писать `assert!(max_before > 1e-5)`). На экране TUI такой спектр всегда был бы пустым (0 пикселей).
2. **Лишнее сведение стерео -> моно (искажение частот в 2 раза):** по `ARCHITECTURE.md` (п. 3.2 и 3.4) `VisRingBuffer` уже содержит моно-сэмплы `(L + R) * 0.5`. Но `Cavacore::process()` у локальной модели принимал сэмплы как стерео и усреднял соседние отсчёты ещё раз, уменьшая эффективную частоту дискретизации вдвое.
3. **Аллокация памяти в `monstercat()` на каждом кадре и неверная формула:** вопреки собственному комментарию «`process()` не выделяет память», функция `monstercat()` на каждом вызове делала `vec![0.0f32; n]` в куче и считала только соседние столбики `* 0.9` вместо формулы из `ARCHITECTURE.md`:
   $$\text{bars}[i] = \max_j \left(\text{bars}[i], \frac{\text{bars}[j]}{1.5^{|i - j|}}\right)$$
4. **Неверный порог басового окна (`100.0` Гц вместо `300.0` Гц):** по `ARCHITECTURE.md` (п. 5.1) окно `N_bass = 4096` должно использоваться для частот ниже `300.0` Гц, а не `100.0` Гц.
5. **Пустые диапазоны бинов (`lo == hi`) на низких частотах:** при большом числе полос (`64`–`128`) соседние границы басовых столбиков попадали в один и тот же индекс бина FFT, из-за чего срез `fft_out[lo..hi]` становился пустым (`0..0`) и столбик всегда выдавал `0.0`.
6. **Отсутствие опционального dB-скейлинга:** пункт из `TODO.md` про опциональное логарифмическое (dB) масштабирование был пропущен, а `sensitivity` в `autosens` бесконечно росла даже в полной тишине.

### `src/audio/decoder.rs` и `src/audio/ring_buffers.rs`
1. **Потеря `Consumer` при создании `Decoder::new()`:** метод `Decoder::new()` принимал `AudioRing` и `VisRing` по значению и делал `audio.into_producer().0`, **уничтожая `Consumer<f32>`** внутри конструктора! В результате ни CPAL, ни UI-поток не могли получить доступ к чтению буферов.
2. **Риск дедлока на `VisRingBuffer`:** `AudioRingBuffer` вмещает `32 768` стерео-фреймов, а `VisRingBuffer` — только `16 384` моно-отсчёта. В коде модели `write_interleaved_stereo_to_rings` вызывал бесконечный цикл `push_all` для обоих буферов: если при подаче чанка в `20 000` фреймов UI не успел вычитать `VisRingBuffer`, поток декодера аудио намертво зависал. Запись в буфер визуализации не должна блокировать декодирование звука.
3. **Ошибки `cargo clippy -- -D warnings`:** в тестах `src/audio/decoder.rs` остались неиспользуемые мутабельные переменные (`let mut vis = ...`, `let mut stereo = ...`).

### `src/audio/output.rs`
1. **Отсутствие вызова `stream.play()` и фильтрации каналов/формата:** при поиске конфигурации `44 100 Гц` модель не проверяла, что конфиг является 2-канальным (`channels == 2`) и формата `F32`, а после создания потока забыла вызвать `stream.play()`.

---

## 2. Полная перезапись файлов Фаз 1 и 2

Ты можешь скопировать и вставить весь блок ниже прямо в терминал в папке `/home/deepseekharness/deezertuiclient` — он перезапишет `src/main.rs`, `src/config.rs`, `src/dsp/cavacore.rs`, `src/audio/ring_buffers.rs`, `src/audio/decoder.rs`, `src/audio/output.rs` и обновит `TODO.md`:

```bash
# ==============================================================================
# 1. src/main.rs
# ==============================================================================
cat << 'EOF' > src/main.rs
mod api;
mod audio;
mod config;
mod dsp;
mod ui;

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Инициализация файлового логгера tracing: логи пишутся в ~/.config/deezer-tui/deezer-tui.log
/// (ежедневная ротация файлов).
///
/// В TUI-режиме никогда не вызывайте println!/eprintln! — только макросы tracing.
fn init_file_logger() -> Result<()> {
    let log_dir = env::var("DEEZER_TUI_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(PathBuf::from)
                .map(|p| p.join(".config/deezer-tui"))
                .unwrap_or_else(|_| PathBuf::from(".config/deezer-tui"))
        });

    fs::create_dir_all(&log_dir)
        .with_context(|| format!("не удалось создать директорию логов {}", log_dir.display()))?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, "deezer-tui.log");
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_writer(file_appender)
        .try_init();

    tracing::info!(
        "deezer-tui: файловый логгер инициализирован в {}",
        log_dir.display()
    );
    Ok(())
}

fn main() -> Result<()> {
    init_file_logger()?;
    let cfg = config::AppConfig::load();
    tracing::info!(
        volume = cfg.volume,
        sample_rate_hz = cfg.sample_rate_hz,
        has_arl = cfg.effective_arl().is_some(),
        "конфигурация загружена"
    );
    Ok(())
}
EOF

# ==============================================================================
# 2. src/config.rs
# ==============================================================================
cat << 'EOF' > src/config.rs
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
EOF

# ==============================================================================
# 3. src/dsp/cavacore.rs
# ==============================================================================
cat << 'EOF' > src/dsp/cavacore.rs
//! Математическое ядро спектроанализатора Cavacore на чистом Rust (`realfft` 3.5).
//!
//! Спецификация (`ARCHITECTURE.md`, раздел 5):
//! - Двойное окно FFT: `N_BASS = 4096` для частот ниже 300 Гц (высокое разрешение по частоте)
//!   и `N_MID_HIGH = 2048` для средних и высоких частот (быстрый временной отклик).
//! - Оконная функция Ханна: `w[n] = 0.5 * (1.0 - cos(2 * PI * n / (N - 1)))`.
//! - Логарифмическое распределение частот от 50 Гц до 10 000 Гц по `num_bars` полосам.
//! - Пространственный фильтр сглаживания Monstercat (`1.5^|i - j|`).
//! - Параболическая гравитация падения (falloff) и интегральный фильтр (`0.77`).
//! - Опциональное логарифмическое (dB) масштабирование и авточувствительность (`autosens`).
//! - Полное отсутствие аллокаций памяти внутри `process()` и `process_stereo()`.

#![allow(dead_code)]

use std::f32::consts::PI;
use std::sync::Arc;

use num_complex::Complex;
use realfft::{RealFftPlanner, RealToComplex};

/// Размер окна FFT для басовых частот (< 300 Гц).
pub const N_BASS: usize = 4096;
/// Размер окна FFT для средних и высоких частот (>= 300 Гц).
pub const N_MID_HIGH: usize = 2048;

/// Граница переключения между окнами `N_BASS` и `N_MID_HIGH` (Гц).
const BASS_CUTOFF_HZ: f32 = 300.0;

/// Нижняя и верхняя границы спектра (Гц).
const LOW_HZ: f32 = 50.0;
const HIGH_HZ: f32 = 10_000.0;

/// Коэффициент интегрального сглаживания.
const INTEGRAL: f32 = 0.77;

/// База пространственного затухания фильтра Monstercat.
const MONSTERCAT_BASE: f32 = 1.5;

/// Коэффициенты авточувствительности (`autosens`).
const AUTOSENS_DOWN: f32 = 0.985;
const AUTOSENS_UP: f32 = 1.001;

/// Коэффициент параболической гравитации падения столбиков.
const GRAVITY: f32 = 0.025;

/// Динамический диапазон для опционального dB-скейлинга (в децибелах).
const DB_DYNAMIC_RANGE: f32 = 60.0;

/// Вычисление коэффициентов окна Ханна длины `n`.
fn hann_window(n: usize) -> Vec<f32> {
    if n <= 1 {
        return vec![1.0; n];
    }
    let denom = (n - 1) as f32;
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / denom).cos()))
        .collect()
}

/// Логарифмическая разметка граничных частот столбиков:
/// `f[i] = f_min * (f_max / f_min)^(i / num_bars)`.
fn bar_boundaries(num_bars: usize) -> Vec<f32> {
    let ratio = HIGH_HZ / LOW_HZ;
    (0..=num_bars)
        .map(|i| LOW_HZ * ratio.powf(i as f32 / num_bars as f32))
        .collect()
}

/// Перевод частоты (Гц) в индекс бина FFT.
#[inline]
fn freq_to_bin(freq: f32, fft_size: usize, sample_rate: f32) -> usize {
    ((freq * fft_size as f32 / sample_rate).round() as usize).clamp(1, fft_size / 2)
}

/// Запись новых моно-сэмплов в кольцевое окно.
#[inline]
fn push_to_ring(window: &mut [f32], write_idx: &mut usize, samples: &[f32]) {
    let n = window.len();
    for &s in samples {
        window[*write_idx] = s;
        *write_idx = (*write_idx + 1) % n;
    }
}

/// Разворачивание кольцевого буфера в хронологическом порядке с умножением на окно Ханна.
#[inline]
fn apply_hann_from_ring(window: &[f32], write_idx: usize, hann: &[f32], dst: &mut [f32]) {
    let n = window.len();
    for i in 0..n {
        let sample = window[(write_idx + i) % n];
        dst[i] = sample * hann[i];
    }
}

/// Спектроанализатор Cavacore.
pub struct Cavacore {
    num_bars: usize,
    sample_rate: f32,
    db_scale: bool,
    autosens: bool,

    // Планировщики и рабочие буферы realfft (предвыделены)
    bass_r2c: Arc<dyn RealToComplex<f32>>,
    mid_r2c: Arc<dyn RealToComplex<f32>>,
    bass_in: Vec<f32>,
    bass_out: Vec<Complex<f32>>,
    bass_scratch: Vec<Complex<f32>>,
    mid_in: Vec<f32>,
    mid_out: Vec<Complex<f32>>,
    mid_scratch: Vec<Complex<f32>>,

    // Кольцевые буферы входного сигнала
    bass_window: Vec<f32>,
    mid_window: Vec<f32>,
    bass_write_idx: usize,
    mid_write_idx: usize,

    // Предвычисленные окна Ханна
    bass_hann: Vec<f32>,
    mid_hann: Vec<f32>,

    // Предвыделенный буфер для сведения стерео в моно (при вызове process_stereo)
    mono_scratch: Vec<f32>,

    // Разметка частот и бинов по столбикам
    use_bass_fft: Vec<bool>,
    lower_bin: Vec<usize>,
    upper_bin: Vec<usize>,
    eq_weights: Vec<f32>,

    // Предвычисленные веса Monstercat: 1.5^d
    monstercat_weights: Vec<f32>,
    raw_bars: Vec<f32>,

    // Состояние временного сглаживания и гравитации
    mem: Vec<f32>,
    prev: Vec<f32>,
    fall_step: Vec<f32>,
    sensitivity: f32,

    // Выходной буфер высот столбиков в диапазоне [0.0, 1.0]
    out_buf: Vec<f32>,
}

impl Cavacore {
    /// Создаёт новый экземпляр `Cavacore` для заданной частоты дискретизации и числа столбиков.
    pub fn new(sample_rate: f32, num_bars: usize) -> Self {
        let num_bars = num_bars.clamp(1, 256);
        let sample_rate = sample_rate.max(8_000.0);

        let mut planner = RealFftPlanner::<f32>::new();
        let bass_r2c = planner.plan_fft_forward(N_BASS);
        let mid_r2c = planner.plan_fft_forward(N_MID_HIGH);

        let bass_in = bass_r2c.make_input_vec();
        let bass_out = bass_r2c.make_output_vec();
        let bass_scratch = bass_r2c.make_scratch_vec();

        let mid_in = mid_r2c.make_input_vec();
        let mid_out = mid_r2c.make_output_vec();
        let mid_scratch = mid_r2c.make_scratch_vec();

        let bass_hann = hann_window(N_BASS);
        let mid_hann = hann_window(N_MID_HIGH);

        let boundaries = bar_boundaries(num_bars);
        let mut use_bass_fft = Vec::with_capacity(num_bars);
        let mut lower_bin = Vec::with_capacity(num_bars);
        let mut upper_bin = Vec::with_capacity(num_bars);
        let mut eq_weights = Vec::with_capacity(num_bars);

        for n in 0..num_bars {
            let f_lo = boundaries[n];
            let f_hi = boundaries[n + 1];
            let is_bass = f_hi <= BASS_CUTOFF_HZ || f_lo < BASS_CUTOFF_HZ;
            let fft_size = if is_bass { N_BASS } else { N_MID_HIGH };
            let max_bin = fft_size / 2;

            let lo = freq_to_bin(f_lo, fft_size, sample_rate).min(max_bin.saturating_sub(1));
            let hi = freq_to_bin(f_hi, fft_size, sample_rate)
                .max(lo + 1)
                .min(max_bin + 1);

            let bin_count = (hi - lo).max(1) as f32;
            // Нормализация амплитуды окна Ханна (4.0 / N) + частотная компенсация ВЧ
            let norm_fft = 4.0 / fft_size as f32;
            let freq_tilt = (f_hi / 120.0).powf(0.32);
            let eq = (norm_fft * freq_tilt) / bin_count;

            use_bass_fft.push(is_bass);
            lower_bin.push(lo);
            upper_bin.push(hi);
            eq_weights.push(eq);
        }

        let monstercat_weights: Vec<f32> = (0..num_bars)
            .map(|d| MONSTERCAT_BASE.powi(d as i32))
            .collect();

        Self {
            num_bars,
            sample_rate,
            db_scale: false,
            autosens: true,
            bass_r2c,
            mid_r2c,
            bass_in,
            bass_out,
            bass_scratch,
            mid_in,
            mid_out,
            mid_scratch,
            bass_window: vec![0.0; N_BASS],
            mid_window: vec![0.0; N_MID_HIGH],
            bass_write_idx: 0,
            mid_write_idx: 0,
            bass_hann,
            mid_hann,
            mono_scratch: vec![0.0; N_BASS],
            use_bass_fft,
            lower_bin,
            upper_bin,
            eq_weights,
            monstercat_weights,
            raw_bars: vec![0.0; num_bars],
            mem: vec![0.0; num_bars],
            prev: vec![0.0; num_bars],
            fall_step: vec![0.0; num_bars],
            sensitivity: 1.0,
            out_buf: vec![0.0; num_bars],
        }
    }

    /// Включает или выключает опциональное логарифмическое (dB) масштабирование спектра.
    pub fn with_db_scale(mut self, enabled: bool) -> Self {
        self.db_scale = enabled;
        self
    }

    /// Переключает режим dB-скейлинга на лету.
    pub fn set_db_scale(&mut self, enabled: bool) {
        self.db_scale = enabled;
    }

    /// Возвращает количество столбиков спектра.
    pub fn num_bars(&self) -> usize {
        self.num_bars
    }

    /// Возвращает частоту дискретизации (Гц).
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Сбрасывает внутренние окна и сглаживание в ноль.
    pub fn reset(&mut self) {
        self.bass_window.fill(0.0);
        self.mid_window.fill(0.0);
        self.bass_write_idx = 0;
        self.mid_write_idx = 0;
        self.mem.fill(0.0);
        self.prev.fill(0.0);
        self.fall_step.fill(0.0);
        self.out_buf.fill(0.0);
        self.sensitivity = 1.0;
    }

    /// Основной метод обработки моно-сэмплов (например, напрямую из `VisRingBuffer`).
    /// Не выделяет память в куче и возвращает срез высот столбиков в `[0.0, 1.0]`.
    pub fn process(&mut self, mono_samples: &[f32]) -> &[f32] {
        push_to_ring(&mut self.bass_window, &mut self.bass_write_idx, mono_samples);
        push_to_ring(&mut self.mid_window, &mut self.mid_write_idx, mono_samples);
        self.compute_spectrum()
    }

    /// Обработка интерливированных стерео-сэмплов (`[L0, R0, L1, R1, ...]`).
    /// Сводит стерео в моно `(L + R) * 0.5` без аллокаций и обновляет спектр.
    pub fn process_stereo(&mut self, stereo_samples: &[f32]) -> &[f32] {
        let total_frames = stereo_samples.len() / 2;
        let chunk_cap = self.mono_scratch.len();
        let mut offset = 0;

        if total_frames == 0 {
            return self.compute_spectrum();
        }

        while offset < total_frames {
            let count = (total_frames - offset).min(chunk_cap);
            for i in 0..count {
                let base = (offset + i) * 2;
                self.mono_scratch[i] = (stereo_samples[base] + stereo_samples[base + 1]) * 0.5;
            }
            push_to_ring(
                &mut self.bass_window,
                &mut self.bass_write_idx,
                &self.mono_scratch[..count],
            );
            push_to_ring(
                &mut self.mid_window,
                &mut self.mid_write_idx,
                &self.mono_scratch[..count],
            );
            offset += count;
        }

        self.compute_spectrum()
    }

    fn compute_spectrum(&mut self) -> &[f32] {
        apply_hann_from_ring(
            &self.bass_window,
            self.bass_write_idx,
            &self.bass_hann,
            &mut self.bass_in,
        );
        apply_hann_from_ring(
            &self.mid_window,
            self.mid_write_idx,
            &self.mid_hann,
            &mut self.mid_in,
        );

        let _ = self.bass_r2c.process_with_scratch(
            &mut self.bass_in,
            &mut self.bass_out,
            &mut self.bass_scratch,
        );
        let _ = self.mid_r2c.process_with_scratch(
            &mut self.mid_in,
            &mut self.mid_out,
            &mut self.mid_scratch,
        );

        // 1. Сбор магнитуд по логарифмическим полосам
        for n in 0..self.num_bars {
            let spectrum = if self.use_bass_fft[n] {
                &self.bass_out
            } else {
                &self.mid_out
            };
            let lo = self.lower_bin[n];
            let hi = self.upper_bin[n].min(spectrum.len());

            let mut sum = 0.0f32;
            for c in &spectrum[lo..hi] {
                sum += c.norm();
            }

            let mut val = sum * self.eq_weights[n];
            if self.db_scale {
                if val > 1e-6 {
                    let db = 20.0 * val.log10();
                    val = ((db + DB_DYNAMIC_RANGE) / DB_DYNAMIC_RANGE).clamp(0.0, 1.0);
                } else {
                    val = 0.0;
                }
            }

            self.raw_bars[n] = val * self.sensitivity;
        }

        // 2. Пространственный фильтр сглаживания Monstercat (без аллокаций):
        // bars[i] = max_j(bars[j] / 1.5^|i - j|)
        for i in 0..self.num_bars {
            let mut peak = self.raw_bars[i];
            for j in 0..self.num_bars {
                let dist = i.abs_diff(j);
                let candidate = self.raw_bars[j] / self.monstercat_weights[dist];
                if candidate > peak {
                    peak = candidate;
                }
            }
            self.out_buf[i] = peak;
        }

        // 3. Параболическая гравитация падения, интегральное сглаживание (0.77) и autosens
        let mut overshoot = false;
        let mut has_signal = false;

        for i in 0..self.num_bars {
            let mut v = self.out_buf[i];
            if v > 1e-5 {
                has_signal = true;
            }

            if v < self.prev[i] {
                let factor = (1.0 - GRAVITY * self.fall_step[i].powi(2)).max(0.0);
                v = (self.prev[i] * factor).max(v);
                self.fall_step[i] += 1.0;
            } else {
                self.fall_step[i] = 0.0;
            }

            self.prev[i] = v;

            let smoothed = self.mem[i] * INTEGRAL + v * (1.0 - INTEGRAL);
            self.mem[i] = if smoothed < 1e-6 { 0.0 } else { smoothed };

            if self.mem[i] > 1.0 {
                overshoot = true;
                self.out_buf[i] = 1.0;
            } else {
                self.out_buf[i] = self.mem[i];
            }
        }

        if self.autosens && has_signal {
            if overshoot {
                self.sensitivity = (self.sensitivity * AUTOSENS_DOWN).clamp(0.05, 10.0);
            } else {
                self.sensitivity = (self.sensitivity * AUTOSENS_UP).clamp(0.05, 10.0);
            }
        }

        &self.out_buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine_wave(freq_hz: f32, sample_rate: f32, len: usize) -> Vec<f32> {
        (0..len)
            .map(|i| (2.0 * PI * freq_hz * i as f32 / sample_rate).sin())
            .collect()
    }

    #[test]
    fn silence_produces_exact_zeros() {
        let mut cava = Cavacore::new(44_100.0, 64);
        let zeros = vec![0.0f32; N_BASS];
        let bars = cava.process(&zeros);
        assert_eq!(bars.len(), 64);
        for (idx, &val) in bars.iter().enumerate() {
            assert_eq!(val, 0.0, "столбик {idx} должен быть равен 0.0 в тишине");
        }
    }

    #[test]
    fn sine_100hz_activates_bass_bars() {
        let mut cava = Cavacore::new(44_100.0, 64);
        let signal = sine_wave(100.0, 44_100.0, N_BASS);

        for _ in 0..8 {
            let _ = cava.process(&signal);
        }
        let bars = cava.process(&signal);
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        assert!(
            low_max > 0.2,
            "басовый пик слишком мал для видимости в TUI: {low_max}"
        );
        assert!(
            low_max > high_max * 2.0,
            "басовые полосы ({low_max}) должны значительно доминировать над ВЧ ({high_max})"
        );
    }

    #[test]
    fn sine_5000hz_activates_high_bars() {
        let mut cava = Cavacore::new(44_100.0, 64);
        let signal = sine_wave(5_000.0, 44_100.0, N_BASS);

        for _ in 0..8 {
            let _ = cava.process(&signal);
        }
        let bars = cava.process(&signal);
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        assert!(
            high_max > 0.2,
            "ВЧ-пик слишком мал для видимости в TUI: {high_max}"
        );
        assert!(
            high_max > low_max * 2.0,
            "верхние полосы ({high_max}) должны значительно доминировать над НЧ ({low_max})"
        );
    }

    #[test]
    fn smooth_decay_after_signal_stops() {
        let mut cava = Cavacore::new(44_100.0, 32);
        let signal = sine_wave(100.0, 44_100.0, N_BASS);

        for _ in 0..10 {
            let _ = cava.process(&signal);
        }
        let peak = cava.process(&signal).iter().copied().fold(0.0f32, f32::max);
        assert!(peak > 0.2, "ожидался заметный пик перед отключением сигнала: {peak}");

        let silence = vec![0.0f32; N_BASS];
        let mut prev_max = peak;
        for step in 0..12 {
            let current_max = cava
                .process(&silence)
                .iter()
                .copied()
                .fold(0.0f32, f32::max);
            if step == 0 {
                assert!(
                    current_max > 0.05 && current_max < peak,
                    "спад на первом кадре должен быть плавным: peak={peak}, current={current_max}"
                );
            }
            assert!(
                current_max <= prev_max + 1e-6,
                "высота столбиков должна монотонно убывать: шаг {step}, {current_max} > {prev_max}"
            );
            prev_max = current_max;
        }
        assert!(
            prev_max < peak * 0.25,
            "после 12 кадров тишины сигнал должен значительно затухнуть: {prev_max}"
        );
    }

    #[test]
    fn optional_db_scaling_works() {
        let mut cava = Cavacore::new(44_100.0, 32).with_db_scale(true);
        let signal = sine_wave(440.0, 44_100.0, N_BASS);
        let bars = cava.process(&signal);
        let max_bar = bars.iter().copied().fold(0.0f32, f32::max);
        assert!(max_bar > 0.1 && max_bar <= 1.0);
    }
}
EOF

# ==============================================================================
# 4. src/audio/ring_buffers.rs
# ==============================================================================
cat << 'EOF' > src/audio/ring_buffers.rs
//! Lock-free кольцевые SPSC-буферы на базе `rtrb` 0.4.
//!
//! - `AudioRingBuffer`: 65 536 сэмплов (32 768 стерео-фреймов `f32`).
//! - `VisRingBuffer`: 16 384 моно-сэмпла `(L + R) * 0.5`.

#![allow(dead_code)]

use rtrb::{Consumer, Producer, RingBuffer};

/// Ёмкость стерео-буфера воспроизведения (в сэмплах `f32`).
pub const AUDIO_RING_BUFFER_CAPACITY: usize = 65_536;

/// Ёмкость моно-буфера визуализации (в сэмплах `f32`).
pub const VIS_RING_BUFFER_CAPACITY: usize = 16_384;

/// Стерео-буфер для передачи аудио из потока декодера в callback `cpal`.
pub struct AudioRing {
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

impl Default for AudioRing {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioRing {
    /// Создаёт новый кольцевой буфер на 65 536 сэмплов.
    pub fn new() -> Self {
        let (producer, consumer) = RingBuffer::<f32>::new(AUDIO_RING_BUFFER_CAPACITY);
        Self { producer, consumer }
    }

    /// Мутабельная ссылка на `Producer<f32>`.
    pub fn producer(&mut self) -> &mut Producer<f32> {
        &mut self.producer
    }

    /// Мутабельная ссылка на `Consumer<f32>`.
    pub fn consumer(&mut self) -> &mut Consumer<f32> {
        &mut self.consumer
    }

    /// Разделяет буфер на независимые `(Producer<f32>, Consumer<f32>)` для передачи в разные потоки.
    pub fn split(self) -> (Producer<f32>, Consumer<f32>) {
        (self.producer, self.consumer)
    }

    /// Синоним `split()` для обратной совместимости с тестами.
    pub fn into_producer(self) -> (Producer<f32>, Consumer<f32>) {
        self.split()
    }
}

/// Моно-буфер для передачи сэмплов `(L + R) * 0.5` из декодера в поток UI/Cavacore.
pub struct VisRing {
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

impl Default for VisRing {
    fn default() -> Self {
        Self::new()
    }
}

impl VisRing {
    /// Создаёт новый кольцевой буфер на 16 384 моно-сэмпла.
    pub fn new() -> Self {
        let (producer, consumer) = RingBuffer::<f32>::new(VIS_RING_BUFFER_CAPACITY);
        Self { producer, consumer }
    }

    /// Мутабельная ссылка на `Producer<f32>`.
    pub fn producer(&mut self) -> &mut Producer<f32> {
        &mut self.producer
    }

    /// Мутабельная ссылка на `Consumer<f32>`.
    pub fn consumer(&mut self) -> &mut Consumer<f32> {
        &mut self.consumer
    }

    /// Разделяет буфер на независимые `(Producer<f32>, Consumer<f32>)` для передачи в разные потоки.
    pub fn split(self) -> (Producer<f32>, Consumer<f32>) {
        (self.producer, self.consumer)
    }

    /// Синоним `split()` для обратной совместимости с тестами.
    pub fn into_producer(self) -> (Producer<f32>, Consumer<f32>) {
        self.split()
    }
}

/// Создаёт пару буферов `(AudioRing, VisRing)`.
pub fn create_rings() -> (AudioRing, VisRing) {
    (AudioRing::new(), VisRing::new())
}
EOF

# ==============================================================================
# 5. src/audio/decoder.rs
# ==============================================================================
cat << 'EOF' > src/audio/decoder.rs
//! Декодер аудиопотока на базе `symphonia` 0.6 с разветвлением в два lock-free буфера `rtrb`.
//!
//! Записывает интерливированные стерео-сэмплы в `AudioRingBuffer` (65 536 сэмплов)
//! и сведённые моно-сэмплы `(L + R) * 0.5` в `VisRingBuffer` (16 384 сэмпла).

#![allow(dead_code)]

use std::io::Read;

use anyhow::{Context, Result};
use rtrb::Producer;
use symphonia::core::codecs::audio::{AudioDecoder, AudioDecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, FormatReader, TrackType};
use symphonia::core::io::{MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;

/// Записывает все сэмплы `src` в `dst`, уступая квант времени потоку-потребителю при заполнении.
fn push_all_blocking(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    let mut remaining = src;
    let mut written = 0;
    while !remaining.is_empty() {
        if dst.is_full() {
            std::thread::yield_now();
            continue;
        }
        let (pushed, rest) = dst.push_partial_slice(remaining);
        written += pushed.len();
        remaining = rest;
    }
    written
}

/// Неблокирующая запись в `VisRingBuffer`: записывает столько моно-сэмплов, сколько помещается
/// прямо сейчас, чтобы визуализация никогда не блокировала декодирование звука.
fn push_vis_nonblocking(dst: &mut Producer<f32>, src: &[f32]) -> usize {
    if dst.is_full() || src.is_empty() {
        return 0;
    }
    let (pushed, _) = dst.push_partial_slice(src);
    pushed.len()
}

/// Записывает чанк интерливированного стерео `[L0, R0, L1, R1, ...]` в оба кольцевых буфера:
/// - стерео целиком в `audio`;
/// - сведённое моно `(L + R) * 0.5` в `vis`.
pub fn write_interleaved_stereo_to_rings(
    stereo: &[f32],
    audio: &mut Producer<f32>,
    vis: &mut Producer<f32>,
) -> (usize, usize) {
    let frames = stereo.len() / 2;
    let audio_written = push_all_blocking(audio, stereo);

    let mut mono = Vec::with_capacity(frames);
    for i in 0..frames {
        mono.push((stereo[2 * i] + stereo[2 * i + 1]) * 0.5);
    }
    let vis_written = push_vis_nonblocking(vis, &mono);

    (audio_written, vis_written)
}

/// Декодер `symphonia` 0.6, подключённый к двум `rtrb::Producer<f32>`.
pub struct Decoder {
    format: Box<dyn FormatReader>,
    track_id: u32,
    decoder: Box<dyn AudioDecoder>,
    audio_producer: Producer<f32>,
    vis_producer: Producer<f32>,
    interleaved_buf: Vec<f32>,
}

impl Decoder {
    /// Инициализирует декодер для произвольного потока чтения `reader` и пары `Producer<f32>`.
    /// Вызывающая сторона сохраняет оба `Consumer<f32>` (для потока `cpal` и потока UI/DSP).
    pub fn new(
        reader: Box<dyn Read + Send + Sync>,
        audio_producer: Producer<f32>,
        vis_producer: Producer<f32>,
    ) -> Result<Self> {
        let mss = MediaSourceStream::new(
            Box::new(ReadOnlySource::new(reader)),
            Default::default(),
        );
        let format = symphonia::default::get_probe()
            .probe(
                &Hint::new(),
                mss,
                FormatOptions::default(),
                MetadataOptions::default(),
            )
            .context("не удалось распознать аудиоформат потока")?;

        let track = format
            .default_track(TrackType::Audio)
            .context("в медиапотоке не найдено аудиодорожек")?;
        let track_id = track.id;

        let audio_params = track
            .codec_params
            .as_ref()
            .context("отсутствуют параметры кодека")?
            .audio()
            .context("дорожка не является аудиокодеком")?;

        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(audio_params, &AudioDecoderOptions::default())
            .context("неподдерживаемый аудиокодек")?;

        Ok(Self {
            format,
            track_id,
            decoder,
            audio_producer,
            vis_producer,
            interleaved_buf: Vec::with_capacity(8192),
        })
    }

    /// Декодирует следующий пакет и записывает сэмплы в оба кольцевых буфера.
    /// Возвращает `Ok(Some(frames))` или `Ok(None)` при достижении конца потока (EOF).
    pub fn process_next(&mut self) -> Result<Option<usize>, SymphoniaError> {
        loop {
            let Some(pkt) = self.format.next_packet()? else {
                return Ok(None);
            };
            if pkt.track_id != self.track_id {
                continue;
            }

            let decoded = self.decoder.decode(&pkt)?;
            let channels = decoded.channels().count();
            let total_samples = decoded.samples_interleaved();

            self.interleaved_buf.resize(total_samples, 0.0);
            decoded.copy_to_slice_interleaved(&mut self.interleaved_buf);

            if channels == 1 {
                // Если поток монофонический — дублируем каждый сэмпл в L и R
                let frames = total_samples;
                let mut stereo = Vec::with_capacity(frames * 2);
                for &s in &self.interleaved_buf[..frames] {
                    stereo.push(s);
                    stereo.push(s);
                }
                write_interleaved_stereo_to_rings(
                    &stereo,
                    &mut self.audio_producer,
                    &mut self.vis_producer,
                );
                return Ok(Some(frames));
            } else {
                let frames = total_samples / 2;
                write_interleaved_stereo_to_rings(
                    &self.interleaved_buf,
                    &mut self.audio_producer,
                    &mut self.vis_producer,
                );
                return Ok(Some(frames));
            }
        }
    }

    /// Декодирует поток до конца (EOF), возвращая общее число декодированных стерео-фреймов.
    pub fn run(&mut self) -> Result<usize, SymphoniaError> {
        let mut total_frames = 0;
        while let Some(frames) = self.process_next()? {
            total_frames += frames;
        }
        Ok(total_frames)
    }

    /// Извлекает оба `Producer<f32>` обратно из декодера.
    pub fn into_producers(self) -> (Producer<f32>, Producer<f32>) {
        (self.audio_producer, self.vis_producer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::ring_buffers::{
        create_rings, AudioRing, VisRing, VIS_RING_BUFFER_CAPACITY,
    };
    use crate::dsp::cavacore::{Cavacore, N_BASS};
    use rtrb::{Consumer, PopError};

    fn drain(consumer: &mut Consumer<f32>) -> Vec<f32> {
        let mut out = Vec::new();
        while let Ok(s) = consumer.pop() {
            out.push(s);
        }
        assert!(matches!(consumer.pop(), Err(PopError::Empty)));
        out
    }

    fn stereo_sine(freq_hz: f32, frames: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let s = (2.0 * std::f32::consts::PI * freq_hz * i as f32 / 44_100.0).sin();
            out.push(s);
            out.push(s);
        }
        out
    }

    #[test]
    fn generated_buffer_fills_both_rings_without_blocking() {
        let stereo = stereo_sine(440.0, 1_000);
        let (audio, vis) = create_rings();
        let (mut ap, mut ac) = audio.split();
        let (mut vp, mut vc) = vis.split();

        let (aw, vw) = write_interleaved_stereo_to_rings(&stereo, &mut ap, &mut vp);
        assert_eq!(aw, stereo.len());
        assert_eq!(vw, stereo.len() / 2);

        let audio_drained = drain(&mut ac);
        let vis_drained = drain(&mut vc);

        assert_eq!(audio_drained, stereo);
        assert_eq!(vis_drained.len(), 1_000);
        for i in 0..1_000 {
            let expected = (stereo[2 * i] + stereo[2 * i + 1]) * 0.5;
            assert_eq!(vis_drained[i], expected);
        }
    }

    #[test]
    fn vis_ring_to_cavacore_100hz_activates_low_bars() {
        let stereo = stereo_sine(100.0, N_BASS * 2);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        // Читаем именно из VisRingBuffer (моно), как это делает UI-поток в ARCHITECTURE.md!
        let mono_vis = drain(vis.consumer());
        assert_eq!(mono_vis.len(), N_BASS * 2);

        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&mono_vis);
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        assert!(low_max > 0.2, "низкие полосы должны активироваться на 100 Гц: {low_max}");
        assert!(
            low_max > high_max * 2.0,
            "НЧ ({low_max}) должны превышать ВЧ ({high_max}) для сигнала 100 Гц"
        );
    }

    #[test]
    fn vis_ring_to_cavacore_5000hz_activates_high_bars() {
        let stereo = stereo_sine(5_000.0, N_BASS * 2);
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        let mono_vis = drain(vis.consumer());
        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&mono_vis);
        let half = bars.len() / 2;

        let low_max = bars[..half].iter().copied().fold(0.0f32, f32::max);
        let high_max = bars[half..].iter().copied().fold(0.0f32, f32::max);

        assert!(high_max > 0.2, "верхние полосы должны активироваться на 5000 Гц: {high_max}");
        assert!(
            high_max > low_max * 2.0,
            "ВЧ ({high_max}) должны превышать НЧ ({low_max}) для сигнала 5000 Гц"
        );
    }

    #[test]
    fn vis_ring_overflow_never_deadlocks_audio_ring() {
        // Подаём больше фреймов (20 000), чем ёмкость VisRingBuffer (16 384),
        // но меньше, чем ёмкость AudioRingBuffer (32 768 фреймов).
        // Запись не должна блокироваться!
        let frames = 20_000;
        let stereo = vec![0.0f32; frames * 2];
        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();

        let (aw, vw) = write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());
        assert_eq!(aw, frames * 2);
        assert_eq!(vw, VIS_RING_BUFFER_CAPACITY);

        let drained_v = drain(vis.consumer());
        let mut cava = Cavacore::new(44_100.0, 32);
        let bars = cava.process(&drained_v);
        assert!(bars.iter().all(|&b| b == 0.0));
    }
}
EOF

# ==============================================================================
# 6. src/audio/output.rs
# ==============================================================================
cat << 'EOF' > src/audio/output.rs
//! Реалтаймовый аудиовывод на базе `cpal` 0.18.
//!
//! Особенности (`AGENT.md`, п. 2.1–2.2):
//! - В `cpal` 0.18 `default_output_config()` по умолчанию выбирает 48 000 Гц, поэтому мы
//!   явно запрашиваем стерео-конфигурацию `f32` с частотой `cpal::SAMPLE_RATE_CD` (44 100 Гц).
//! - Внутри callback-функции нет ни одной блокировки или аллокации памяти: сэмплы читаются
//!   из `rtrb::Consumer<f32>`, умножаются на атомарный коэффициент громкости (`AtomicU32`)
//!   и при нехватке данных (underrun) заполняются нулями (`0.0f32`).

#![allow(dead_code)]

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{OutputCallbackInfo, SampleFormat};
use rtrb::{Consumer, PopError};

/// Стандартная частота дискретизации Deezer FLAC (44 100 Гц).
pub const SAMPLE_RATE_HZ: u32 = 44_100;

/// Битовое представление громкости `1.0f32` для `AtomicU32`.
pub fn default_volume_bits() -> u32 {
    f32::to_bits(1.0)
}

/// Битовое представление тишины `0.0f32` для `AtomicU32`.
pub fn muted_volume_bits() -> u32 {
    f32::to_bits(0.0)
}

/// Заполняет выходной буфер звуковой карты из `rtrb::Consumer<f32>` без блокировок и аллокаций.
/// При опустошении кольцевого буфера оставшиеся сэмплы заполняются тишиной (`0.0`).
#[inline]
pub fn fill_buffer_from_consumer(
    buffer: &mut [f32],
    consumer: &mut Consumer<f32>,
    volume: &AtomicU32,
) -> usize {
    let vol = f32::from_bits(volume.load(Ordering::Relaxed)).clamp(0.0, 1.0);
    let mut filled = 0;
    for slot in buffer.iter_mut() {
        match consumer.pop() {
            Ok(sample) => {
                *slot = sample * vol;
                filled += 1;
            }
            Err(PopError::Empty) => {
                *slot = 0.0;
            }
        }
    }
    filled
}

/// Управляет потоком вывода `cpal::Stream` и атомарным регулятором громкости.
pub struct AudioOutput {
    stream: cpal::Stream,
    volume: Arc<AtomicU32>,
    sample_rate: u32,
}

impl AudioOutput {
    /// Открывает аудиопоток (предпочтительно 44 100 Гц, 2 канала, `f32`), перемещает
    /// `consumer` внутрь real-time callback и запускает воспроизведение.
    pub fn new(consumer: Consumer<f32>, initial_volume: f32) -> Result<Self> {
        let clamped_vol = initial_volume.clamp(0.0, 1.0);
        let volume = Arc::new(AtomicU32::new(f32::to_bits(clamped_vol)));

        let host = cpal::Host::default();
        let device = host
            .default_output_device()
            .context("аудиоустройство вывода по умолчанию не найдено")?;

        // Ищем стерео F32 конфигурацию с частотой 44 100 Гц (SAMPLE_RATE_CD)
        let config: cpal::StreamConfig = device
            .supported_output_configs()
            .ok()
            .and_then(|configs| {
                configs
                    .filter(|r| r.channels() == 2 && r.sample_format() == SampleFormat::F32)
                    .find_map(|r| r.try_with_sample_rate(cpal::SAMPLE_RATE_CD))
            })
            .map(Into::into)
            .or_else(|| device.default_output_config().ok().map(Into::into))
            .context("не удалось подобрать конфигурацию аудиопотока")?;

        let actual_sample_rate = u32::from(config.sample_rate);
        tracing::info!(
            sample_rate = actual_sample_rate,
            channels = config.channels,
            "аудиопоток CPAL сконфигурирован"
        );

        let volume_for_cb = Arc::clone(&volume);
        let mut consumer_for_cb = consumer;

        let error_callback = move |err: cpal::Error| {
            tracing::warn!(target: "audio_output", "ошибка потока CPAL: {err:?}");
        };

        let stream = device
            .build_output_stream(
                config,
                move |buffer: &mut [f32], _info: &OutputCallbackInfo| {
                    fill_buffer_from_consumer(buffer, &mut consumer_for_cb, &volume_for_cb);
                },
                error_callback,
                None,
            )
            .context("не удалось создать выходной поток CPAL")?;

        stream
            .play()
            .context("не удалось запустить воспроизведение потока CPAL")?;

        Ok(Self {
            stream,
            volume,
            sample_rate: actual_sample_rate,
        })
    }

    /// Возвращает ссылку на атомарный регулятор громкости.
    pub fn volume_atomic(&self) -> &Arc<AtomicU32> {
        &self.volume
    }

    /// Устанавливает громкость в диапазоне `0.0..=1.0` атомарно без блокировок.
    pub fn set_volume(&self, vol: f32) {
        let clamped = vol.clamp(0.0, 1.0);
        self.volume.store(f32::to_bits(clamped), Ordering::Relaxed);
    }

    /// Возвращает текущее значение громкости `0.0..=1.0`.
    pub fn get_volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Relaxed))
    }

    /// Возвращает фактическую частоту дискретизации открытого потока (Гц).
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::decoder::write_interleaved_stereo_to_rings;
    use crate::audio::ring_buffers::{AudioRing, VisRing};
    use rtrb::RingBuffer;

    fn consumer_with(samples: &[f32]) -> Consumer<f32> {
        let (mut producer, consumer) = RingBuffer::<f32>::new(samples.len().max(1));
        for &s in samples {
            producer.push(s).unwrap();
        }
        consumer
    }

    #[test]
    fn fill_buffer_applies_volume_and_silence_on_empty() {
        let mut consumer = consumer_with(&[0.5, -0.5, 0.25, 1.0, 0.0, -1.0]);
        let mut buffer = [0.0f32; 6];
        let volume = AtomicU32::new(default_volume_bits());

        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 6);
        assert!((buffer[0] - 0.5).abs() < 1e-6);
        assert!((buffer[1] + 0.5).abs() < 1e-6);
        assert!((buffer[2] - 0.25).abs() < 1e-6);
        assert!((buffer[3] - 1.0).abs() < 1e-6);
        assert!((buffer[4] - 0.0).abs() < 1e-6);
        assert!((buffer[5] + 1.0).abs() < 1e-6);
    }

    #[test]
    fn fill_buffer_halves_with_half_gain() {
        let mut consumer = consumer_with(&[1.0, -1.0, 0.4]);
        let mut buffer = [0.0f32; 3];
        let volume = AtomicU32::new(f32::to_bits(0.5));

        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 3);
        assert!((buffer[0] - 0.5).abs() < 1e-6);
        assert!((buffer[1] + 0.5).abs() < 1e-6);
        assert!((buffer[2] - 0.2).abs() < 1e-6);
    }

    #[test]
    fn fill_buffer_underrun_fills_silence_without_blocking() {
        let mut consumer = consumer_with(&[0.3, 0.7]);
        let mut buffer = [1.0f32; 6];
        let volume = AtomicU32::new(default_volume_bits());

        let filled = fill_buffer_from_consumer(&mut buffer, &mut consumer, &volume);
        assert_eq!(filled, 2);
        assert!((buffer[0] - 0.3).abs() < 1e-6);
        assert!((buffer[1] - 0.7).abs() < 1e-6);
        assert_eq!(&buffer[2..], &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn end_to_end_rtrb_to_output_buffer() {
        let frames = 1_000usize;
        let mut stereo = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let s = (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 44_100.0).sin();
            stereo.push(s);
            stereo.push(s);
        }

        let mut audio = AudioRing::new();
        let mut vis = VisRing::new();
        write_interleaved_stereo_to_rings(&stereo, audio.producer(), vis.producer());

        let mut output_buf = vec![0.0f32; stereo.len()];
        let volume = AtomicU32::new(default_volume_bits());
        let filled = fill_buffer_from_consumer(&mut output_buf, audio.consumer(), &volume);

        assert_eq!(filled, stereo.len());
        assert_eq!(output_buf, stereo);
    }
}
EOF

# ==============================================================================
# 7. Отмечаем выполненные пункты Фаз 1 и 2 в TODO.md
# ==============================================================================
cat << 'EOF' > TODO.md
# TODO.md — Пошаговый чеклист задач для AI-агента (Редакция: Ноябрь 2026)

Выполняй задачи строго сверху вниз по одной за итерацию. После прохождения cargo test и cargo clippy -- -D warnings меняй [ ] на [x].

## Фаза 1: Инициализация и математическое ядро Cavacore
- [x] Создать Cargo.toml с актуальными зависимостями 2026 года из ARCHITECTURE.md и базовую структуру модулей в src/.
- [x] Реализовать src/config.rs (чтение и сохранение arl, громкости и настроек в ~/.config/deezer-tui/config.toml через toml 1.2) и настроить файловый логгер tracing в src/main.rs.
- [x] Реализовать src/dsp/cavacore.rs: структура Cavacore, инициализация окон Ханна (4096 для басов и 2048 для СЧ/ВЧ), планировщик realfft 3.5 и логарифмическое распределение частот (50–10000 Гц) по столбикам.
- [x] Добавить в src/dsp/cavacore.rs фильтр сглаживания Monstercat, параболическую гравитацию падения (falloff), интегральное сглаживание (0.77), опциональный dB-скейлинг и авточувствительность (autosens).
- [x] Написать unit-тесты для Cavacore по образцу docs/reference/cava/cavacore_test.c (тишина, синусоида 100 Гц, синусоида 5000 Гц, плавный спад после пика).

## Фаза 2: Локальный аудиодвижок (Symphonia 0.6 + CPAL 0.18 + rtrb 0.4)
- [x] Реализовать src/audio/decoder.rs: декодирование аудиопотока через symphonia 0.6 (FLAC/MP3), конвертация буферов в f32 и запись в два lock-free буфера rtrb::Producer<f32> (AudioRingBuffer для стерео и VisRingBuffer для моно).
- [x] Реализовать src/audio/output.rs: инициализация cpal 0.18 с явным выбором частоты SampleRate(44100) (вместо нового дефолта 48 кГц), чтение из rtrb::Consumer<f32> без блокировок и аллокаций, атомарная регулировка громкости.
- [x] Написать интеграционный тест передачи декодированных сэмплов через rtrb в Cavacore.

## Фаза 3: Графическое ядро (Ratatui 0.30) и Экран 01 (Now Playing)
- [ ] Реализовать src/ui/theme.rs: константы цветов (#13151F, #1E2233, #3E445E, #ECEFF4, #94A3B8) и функцию интерполяции градиента от #4FD1C5 до #7F5AF0.
- [ ] Реализовать src/ui/widgets/spectrum.rs: отрисовка столбиков символами [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'] с интервалом в 1 пробел и точками удержания пиков сверху.
- [ ] Реализовать src/ui/widgets/cover_art.rs: чтение через image::ImageReader, масштабирование DynamicImage и рендеринг через полублоки ▀ (fg — верхний пиксель, bg — нижний пиксель).
- [ ] Сверстать глобальный верхний статус-бар (DEEZER-TUI, вкладки [1]..[4], CONNECTED, VOL, часы) и нижний бар хоткеев в src/ui/app.rs с инициализацией терминала через ratatui::init() и ratatui::restore().
- [ ] Сверстать src/ui/views/now_playing.rs (Экран 01: обложка, бейдж FLAC | 1411 kbps, название трека, артист, прогресс-бар ●, кнопки управления и спектр во всю нижнюю половину).
- [ ] Написать рендер-тест Экрана 01 на ratatui::backend::TestBackend.

## Фаза 4: Криптография Blowfish и API Deezer (reqwest 0.13)
- [ ] Реализовать src/api/crypto.rs: генерация 16-байтного ключа из SNG_ID (MD5 hex + XOR с g4el58wc0zvf9na1) и функция расшифровки чанка 2048 байт через cbc::Decryptor<blowfish::Blowfish> с IV [0,1,2,3,4,5,6,7].
- [ ] Написать unit-тесты для src/api/crypto.rs (проверка длины ключа, детерминированности и обратимости шифрования/дешифрования).
- [ ] Реализовать src/audio/stream.rs: структура DeezerStream, реализующая Read + Seek + Send + Sync + MediaSource, расшифровывающая каждый чанк chunk_idx % 3 == 0 размером ровно 2048 байт с сохранением в tempfile для мгновенного Seek.
- [ ] Написать unit-тест для DeezerStream с произвольными операциями SeekFrom::Start и невыровненными чтениями read().
- [ ] Реализовать src/api/models.rs и src/api/client.rs на базе reqwest 0.13: авторизация по куке arl в gw-light.php (deezer.getUserData), получение URL потока в media.deezer.com/v1/get_url, методы поиска, плейлистов, лайков, Flow и Lyrics.

## Фаза 5: Экраны 02, 03, 04 и финализация
- [ ] Сверстать src/ui/views/search.rs (Экран 02: сайдбар NAVIGATION с мини-спектром, строка ввода поиска и 3 колонки ARTISTS, ALBUMS, TRACKS + нижний мини-плеер).
- [ ] Сверстать src/ui/views/library.rs и src/ui/widgets/modal.rs (Экран 03: дерево плейлистов слева, таблица треков справа и модальное окно CHOOSE PLAYLIST по клавише добавления в плейлист).
- [ ] Сверстать src/ui/views/flow_lyrics.rs (Экран 04: блок Deezer Flow, синхронная прокрутка LYRICS по таймкоду декодера с бирюзовой подсветкой активной строки и полный спектр снизу).
- [ ] Подключить D-Bus MPRIS через souvlaki 0.9 (use_zbus) и префетч следующего трека за 20 секунд до окончания текущего для Gapless-воспроизведения.
- [ ] Прогнать финальный cargo test и cargo clippy -- -D warnings.
EOF
```

---

## 3. Проверка и коммит

После вставки команд выше выполни в терминале проверку по `AGENT.md`:

```bash
cargo check --all-targets
cargo test -- --nocapture
cargo clippy -- -D warnings
```

И если все тесты зелёные, зафиксируй чистые Фазы 1 и 2 в Git:

```bash
git add .
git commit -m "feat(phase1-2): rewrite cavacore dsp and rtrb/cpal audio engine"
git push
```