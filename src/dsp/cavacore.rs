//! Математическое ядро спектроанализатора Cavacore на чистом Rust (`realfft` 3.5).
//!
//! Спецификация (`ARCHITECTURE.md`, раздел 5):
//! - Двойное окно FFT: `N_BASS = 4096` для частот ниже 300 Гц (высокое разрешение по частоте)
//!   и `N_MID_HIGH = 2048` для средних и высоких частот (быстрый временной отклик).
//! - Оконная функция Ханна: `w[n] = 0.5 * (1.0 - cos(2 * PI * n / (N - 1)))`.
//! - Логарифмическое октавное распределение частот от 20 Гц до 20 000 Гц по `num_bars` полосам
//!   (например, 15 полос с шагом 2/3 октавы или 30 полос с шагом 1/3 октавы).
//! - Энергия полосы (v2.5): сумма магнитуд `Σ|X_k|` по бинам полосы плюс полка
//!   пикового бина `PEAK_BLEND * max|X_k|` — равная энергия на октаву и заметные
//!   ВЧ-транзиенты (RMS-усреднение по всем бинам размазывало разреженное содержимое
//!   широких ВЧ-полос, правая половина спектра оставалась плоской).
//! - По-полосный AGC (v2.5): долгосрочный уровень полосы приводится к среднему уровню
//!   спектра (`AGC_STRENGTH = 0.7`), относительный порог тишины `REL_SILENCE_GATE`
//!   гасит шумовую полку и утечку FFT. Без них один общий множитель усиления оставляет
//!   ВЧ в 30–40 дБ ниже баса, и «подпрыгивает» только самый громкий столбик.
//! - Пространственный фильтр сглаживания Monstercat (в Legacy — `1.35^|i - j|`,
//!   в относительных шкалах — `2.0^|i - j|`, как в эталонной cava она отсутствует).
//! - Параболическая гравитация падения (falloff) и интегральный фильтр
//!   (`0.62` в Legacy, `0.5` в относительных шкалах).
//! - Три шкалы высоты (`SpectrumScale`):
//!   * `Legacy` — логарифмическое окно 52 дБ от 0 dBFS (v2.5) с по-полосным AGC и
//!     авточувствительностью; честная, но сжатая в середину 0.2–0.6 манера;
//!   * `TrackPeak` (по умолчанию) — окно 30 дБ от долгосрочного пика громкости трека:
//!     тихие места не поднимаются в потолок, полоса в −30 дБ от пика уходит в ноль;
//!   * `FramePeak` — окно 30 дБ от пика текущего кадра (манера cava: самый громкий
//!     столбик всегда в потолке). В обеих относительных шкалах AGC, sens и
//!     авточувствительность выключены, чтобы нормировка оставалась единственной.
//! - Линейный режим (`db_scale = false`) — мягкая компрессия `scaled^0.45` (только Legacy).
//! - Авточувствительность (`autosens`) с симметричной мёртвой зоной [0.4, 0.85].
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

/// Нижняя и верхняя границы спектра (Гц): стандартный слышимый диапазон 20 Гц — 20 кГц.
const LOW_HZ: f32 = 20.0;
const HIGH_HZ: f32 = 20_000.0;

/// Коэффициент интегрального сглаживания.
/// 0.62 (а не 0.77): постоянная времени ~40 мс вместо ~70 мс, иначе короткие
/// ВЧ-транзиенты (шляпки, щелчки, атака синтезатора) успевают усредниться
/// до незаметного «подскока» на пару пикселей.
const INTEGRAL: f32 = 0.62;

/// Полка пикового бина в энергии полосы: короткие ВЧ-транзиенты (шляпки, щелчки,
/// атака синтеза) сосредоточены в 1–2 бинах и при суммировании по широкой полосе
/// почти не влияют на сумму, поэтому пик добавляется отдельным слагаемым.
const PEAK_BLEND: f32 = 0.35;

/// Эквизация восприятия: опорная частота и экспонента компенсации наклона
/// музыкального спектра. Реальный наклон музыки ≈ −5 дБ/октава выше ~120 Гц
/// (вдвое выше частоты — примерно в 0.56 раза меньше амплитуды), поэтому
/// компенсация ×1.28 на октаву (экспонента 0.35) вместе с логарифмической шкалой
/// даёт «прыгающие» ВЧ вместо плоской правой половины спектра.
const FREQ_TILT_REF_HZ: f32 = 120.0;
const FREQ_TILT_EXP: f32 = 0.18;
const FREQ_TILT_MIN: f32 = 0.85;
const FREQ_TILT_MAX: f32 = 2.2;

/// По-полосная авточувствительность (AGC): долгосрочный уровень каждой полосы
/// приводится к среднему уровню спектра, иначе одно общее усиление на все полосы
/// оставляет ВЧ в 30–40 дБ ниже баса и они «почти не подпрыгивают».
const AGC_REF_ATTACK: f32 = 0.06;
const AGC_REF_RELEASE: f32 = 0.008;
const AGC_MIN: f32 = 0.35;
const AGC_MAX: f32 = 4.5;
/// Сила AGC: `gain = (среднее_по_спектру / уровень_полосы)^AGC_STRENGTH`.
/// 0.7 — компромисс: полосы видны и «прыгают», но бас остаётся visibly крупнее.
const AGC_STRENGTH: f32 = 0.7;
/// Относительный порог тишины: полоса ниже 0.4 % от самой громкой полосы кадра
/// считается нулевой (обрезает шумовую полку и утечку FFT, а не только нули).
const REL_SILENCE_GATE: f32 = 0.004;

/// Коэффициенты авточувствительности (`autosens`). Прежние 0.985 вниз / 1.001 вверх
/// давали сильную асимметрию (падение ~1.5 с, восстановление ~35 с), из-за чего
/// один громкий столбик занижал весь спектр.
const AUTOSENS_DOWN: f32 = 0.99;
const AUTOSENS_UP: f32 = 1.005;
/// Мёртвая зона авточувствительности в амплитудных единицах (после AGC, до
/// логарифмической шкалы): пока пик внутри окна [LOW, HIGH] (~6 дБ), чувствительность
/// не меняется — динамика трека сохраняется, а не сжимается к одной высоте.
const AUTOSENS_TARGET_HIGH: f32 = 0.85;
const AUTOSENS_TARGET_LOW: f32 = 0.4;
/// Пределы авточувствительности: не дают тихому треку раскачать спектр в 10 раз.
const AUTOSENS_MIN: f32 = 0.25;
const AUTOSENS_MAX: f32 = 8.0;
/// Коэффициент параболической гравитации падения столбиков (Legacy / TrackPeak).
const GRAVITY: f32 = 0.025;

/// Ширина dB-окна шкалы `Absolute`: в дефолте cava высота = `amplitude_to_decibels(v)/70`,
/// то есть полоса на −70 дБ от 0 dBFS уже нуль. Широкое окно — как раз то, из-за чего
/// соседи доминанты не упёрты в потолок: −20 дБ даёт 0.71, −40 дБ даёт 0.43.
pub const ABS_DB_WINDOW: f32 = 70.0;
/// Нижняя граница настраиваемого dB-окна `Absolute`: уже этого окна пики обрезаются
/// так сильно, что картина рассыпается в отдельные столбики.
pub const ABS_WINDOW_MIN: f32 = 24.0;
/// Верхняя граница: шире дефолта cava смысла нет — всё и так лежит в потолке.
pub const ABS_WINDOW_MAX: f32 = 90.0;
/// Полоса ниже −100 дБ считается тишиной (в cava не-конечное значение даёт 0).
const ABS_SILENCE_FLOOR: f32 = 1e-5;
/// Интегральное сглаживание дефолта cava (`[smoothing] integral = 0.77`).
const ABS_INTEGRAL: f32 = 0.77;
/// Гравитация падения, подобранная под дефолт cava (`gravity = 2.5`, `fall += 0.028`
/// при 60 FPS → столбик доходит до нуля за ~22 кадра ≈ 0.37 с). Наша форма
/// `1 - gravity·fall²` с шагом `fall = 1` за кадр даёт ноль при `fall = 1/√gravity`.
const ABS_GRAVITY: f32 = 0.0021;
/// Авточувствительность дефолта cava: один общий `sens`, медленный рост каждый кадр
/// (`×(1 + 0.001·framerate_mod)`) и падение только при переполнении
/// (`×(1 − 0.02·framerate_mod)`), где `framerate_mod = 66/60 = 1.1`.
const ABS_SENS_UP: f32 = 1.0011;
const ABS_SENS_DOWN: f32 = 0.978;
/// Быстрый отклик для `Absolute`:integral 0.55 поднимает полосу до 0.5 за
/// ~1.5 кадра (у дефолта cava 0.77 — ~3.6 кадра), более крутая гравитация
/// короче возвращает столбик в долину, а `sens` растёт втрое быстрее, поэтому
/// смена трека не выглядит «запозданием» на секунды.
const SNAPPY_INTEGRAL: f32 = 0.55;
const SNAPPY_GRAVITY: f32 = 0.0040;
const SNAPPY_SENS_UP: f32 = 1.0035;

/// Скорость отклика полос в шкале `Absolute`. Дефолт повторяет cava; быстрый
/// вариант подбирать тот же вид, но столбики взлетают и падают заметно живее.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumResponse {
    /// Дефолт cava: integral 0.77, gravity 0.0021, медленный рост `sens`.
    #[default]
    Cava,
    /// Быстрее rise/fall и быстрее адаптация `sens` при смене трека.
    Snappy,
}

impl SpectrumResponse {
    /// Короткая подпись для экрана Settings.
    pub fn label(self) -> &'static str {
        match self {
            Self::Cava => "Cava smoothing",
            Self::Snappy => "Snappy",
        }
    }
}

/// Коэффициент мягкой компрессии для линейного (не-dB) режима: `val^0.45`.
/// Показатель меньше 1 поднимает тихие полосы, сохраняя «удар» баса.
const LIN_COMPRESSION: f32 = 0.45;

/// Окно пик-нормированной шкалы (дБ): полоса ниже опорного пика на всю ширину окна
/// уходит в нуль. Замер на реальных треках: внутри кадра полосы различаются примерно
/// на 20–25 дБ. 26 дБ вместо 30 (v2.7): при 30 дБ полосы в −10…−15 дБ от пика сидят в
/// 0.5–0.65 и вместе с соседями дают «плато» (доля кадров с ≥6 столбиками выше 0.7 —
/// 0.12–0.15), при 26 дБ они падают в долины (плато 0.05–0.07, полос <0.2 — 0.19–0.24).
const REL_WINDOW: f32 = 26.0;
/// Пик кадра ниже этого порога — считаем тишину и обнуляем состояние spectrum.
const REL_SILENCE_PEAK: f32 = 1e-4;
/// Полоса ниже 1 % от пика кадра (−40 дБ) гасится в пик-режимах (в Legacy — 0.4 %).
const REL_BAND_GATE: f32 = 0.01;
/// База пространственного фильтра соседних полос (Monstercat) в шкале `Legacy`.
/// Соседняя полоса получает 74 % от пика, через одну 55 %, через три 41 %.
pub const MONSTERCAT_BASE: f32 = 1.35;
/// Порог включения пространственного фильтра: база ≤ него означает «выключен»
/// (так же, как в эталонной cava, где фильтра нет вовсе).
pub const SPREAD_MIN_BASE: f32 = 1.05;
/// Верхний предел базы фильтра: сильнее 4.0 соседние полосы получают меньше 25 %/6 %
/// от пика — фильтр практически перестает что-либо менять.
pub const SPREAD_MAX: f32 = 4.0;
/// Значение «выключен» для базы фильтра.
const SPREAD_OFF: f32 = 0.0;
/// Интегральное сглаживание по шкалам: `TrackPeak` чуть быстрее Legacy, `FramePeak`
/// медленнее — опорный пик кадра меняется каждый кадр, и без более долгой постоянной
/// времени столбики «дёргаются» (замер на реальном MP3, сред |Δ| за кадр: 0.032 при 0.5,
/// 0.024 при 0.72 в `FramePeak`; 0.032 при 0.5, 0.026 при 0.70 в `TrackPeak`).
const TRACK_PEAK_INTEGRAL: f32 = 0.70;
const FRAME_PEAK_INTEGRAL: f32 = 0.72;
/// Гравитация падения: в `FramePeak` падение быстрее Legacy, чтобы одиночный
/// транзиент не висел столбом ~20 кадров и не собирался в плато.
const FRAME_PEAK_GRAVITY: f32 = 0.05;
/// Спуск опорного пика кадра после транзиента (~0.1 с удержания: подъём не ограничен,
/// поэтому самый громкий столбик по-манере cava остаётся в потолке, но нормировка
/// соседних полос не скачет вверх-вниз каждый кадр).
const FRAME_PEAK_RELEASE: f32 = 0.12;
/// EMA медленного пика трека: подъём быстрый (0.05), спуск очень медленный (0.0005),
/// чтобы опорная точка была «громкостью трека», а не «громкостью кадра».
const TRACK_PEAK_ATTACK: f32 = 0.05;
const TRACK_PEAK_RELEASE: f32 = 0.0005;

/// Шкала преобразования энергии полосы в высоту столбика.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumScale {
    /// Логарифмическая шкала от 0 dBFS в окне 52 дБ + по-полосный AGC
    /// и авточувствительность по амплитуде (режим v2.5).
    #[default]
    Legacy,
    /// Относительная шкала: высота = расстояние полосы до **медленного пика трека**
    /// в децибелах, окно 30 дБ. Громкость трека остаётся честной, контраст полный.
    TrackPeak,
    /// Относительная шкала от пика **кадра**: самый «кавовский» вид — столбики
    /// регулярно упёрты в потолок, но средняя высота выше, а тихие треки становятся
    /// такими же яркими, как громкие.
    FramePeak,
    /// Абсолютная шкала по образцу дефолта cava: полоса = **средняя** магнитуда своих
    /// бинов (сумма делится на их число), высота = расстояние до 0 dBFS в окне 70 дБ,
    /// без по-полосного AGC, без гейта от пика кадра и без нормировки по пикам.
    /// Полосы не связаны общей опорной точкой, поэтому перескок доминанты не меняет
    /// высоты остальных: сосед на −30 дБ остаётся на 0.57, а не упёрт в потолок.
    Absolute,
}

impl SpectrumScale {
    /// Каноническое имя для сохранения в `config.toml`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::TrackPeak => "track_peak",
            Self::FramePeak => "frame_peak",
            Self::Absolute => "absolute",
        }
    }

    /// Короткая подпись для экрана Settings.
    pub fn label(self) -> &'static str {
        match self {
            Self::Legacy => "0 dBFS · 32 dB",
            Self::TrackPeak => "Track peak · 26 dB",
            Self::FramePeak => "Frame peak · 26 dB",
            Self::Absolute => "Absolute · cava-style dB",
        }
    }
}

impl std::str::FromStr for SpectrumScale {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let v = value.trim().to_lowercase();
        match v.as_str() {
            "legacy" | "dbfs" | "0dbfs" => Ok(Self::Legacy),
            "track_peak" | "trackpeak" | "slow" => Ok(Self::TrackPeak),
            "frame_peak" | "framepeak" => Ok(Self::FramePeak),
            "absolute" | "cava" | "cava_default" => Ok(Self::Absolute),
            _ => Err("ожидалось legacy | track_peak | frame_peak | absolute"),
        }
    }
}

/// Динамический диапазон шкалы `Legacy` (дБ от 0 dBFS). 32 дБ вместо 52 (v2.7): при 52 дБ
/// разброс полос внутри кадра (20–30 дБ) сжимался в 0.55–0.95 — «плато, упёртое вверх»
/// (≥6 столбиков выше 0.7 в 100 % кадров, σ 0.10–0.12). 32 дБ даёт рельеф: плато 0.51–0.73,
/// полос >0.85 — 0.06–0.07 вместо 0.26, при сохранном бас-доминанте.
const DB_DYNAMIC_RANGE: f32 = 32.0;
/// Порог тишины в амплитудных единицах: ниже него столбик остаётся нулевым
/// (иначе логарифмическая шкала показала бы шум квантования как сигнал).
const DB_SILENCE_FLOOR: f32 = 1e-4;

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
/// Для 15 столбиков это шаг 2/3 октавы (20 - 32 - 50 - 80 ... 20 000 Гц).
/// Для 30 столбиков это шаг 1/3 октавы (20 - 25 - 32 ... 20 000 Гц).
pub fn bar_boundaries(num_bars: usize) -> Vec<f32> {
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

    // Предвычисленные веса Monstercat: spread_base^d
    monstercat_weights: Vec<f32>,
    /// Активная шкала высот и её параметры (базовый spread, медленный пик трека).
    scale_mode: SpectrumScale,
    spread_base: f32,
    slow_peak: f32,
    /// Опорный пик кадра с медленным спуском (эталон нормировки в `FramePeak`)
    /// и скорость его спуска.
    frame_ref: f32,
    frame_ref_release: f32,
    /// Ширина dB-окна относительной шкалы, постоянная сглаживания и гравитация —
    /// подбираются по активной шкале (`set_scale_mode`).
    rel_window: f32,
    /// Ширина окна шкалы `Legacy` (от 0 dBFS), по умолчанию `DB_DYNAMIC_RANGE`.
    db_window: f32,
    /// Ширина dB-окна шкалы `Absolute`: насколько тихие полосы ниже громких.
    /// Узкое окно даёт высокий контраст (пики up, фон низкий), широкое — плотную
    /// картину. Настраивается из конфигурации, см. `ABS_WINDOW_MIN`/`ABS_WINDOW_MAX`.
    abs_window: f32,
    response: SpectrumResponse,
    sens_up: f32,
    integral: f32,
    gravity: f32,
    raw_bars: Vec<f32>,

    // Состояние временного сглаживания и гравитации
    mem: Vec<f32>,
    prev: Vec<f32>,
    fall_step: Vec<f32>,
    sensitivity: f32,
    /// Долгосрочный уровень каждой полосы (для по-полосного AGC).
    band_ref: Vec<f32>,

    // Выходной буфер высот столбиков в диапазоне [0.0, 1.0]
    out_buf: Vec<f32>,
}

impl Cavacore {
    fn compute_band_bins(
        num_bars: usize,
        sample_rate: f32,
    ) -> (Vec<bool>, Vec<usize>, Vec<usize>, Vec<f32>) {
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

            let f_center = (f_lo * f_hi).sqrt();
            let norm_fft = 4.0 / fft_size as f32;
            // Эквализация восприятия: компенсирует наклон спектра реальной музыки
            // (~−5 дБ/октава) относительно опорной частоты FREQ_TILT_REF_HZ.
            let freq_tilt = (f_center / FREQ_TILT_REF_HZ)
                .powf(FREQ_TILT_EXP)
                .clamp(FREQ_TILT_MIN, FREQ_TILT_MAX);
            let eq = norm_fft * freq_tilt;

            use_bass_fft.push(is_bass);
            lower_bin.push(lo);
            upper_bin.push(hi);
            eq_weights.push(eq);
        }

        (use_bass_fft, lower_bin, upper_bin, eq_weights)
    }

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

        let (use_bass_fft, lower_bin, upper_bin, eq_weights) =
            Self::compute_band_bins(num_bars, sample_rate);

        let spread_base = SPREAD_OFF;
        let monstercat_weights: Vec<f32> = (0..num_bars).map(|_| 1.0f32).collect();

        Self {
            num_bars,
            sample_rate,
            db_scale: false,
            autosens: true,
            scale_mode: SpectrumScale::Legacy,
            spread_base,
            slow_peak: 0.0,
            frame_ref: 0.0,
            frame_ref_release: FRAME_PEAK_RELEASE,
            rel_window: REL_WINDOW,
            db_window: DB_DYNAMIC_RANGE,
            abs_window: ABS_DB_WINDOW,
            response: SpectrumResponse::default(),
            sens_up: ABS_SENS_UP,
            integral: INTEGRAL,
            gravity: GRAVITY,
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
            band_ref: vec![0.0; num_bars],
            out_buf: vec![0.0; num_bars],
        }
    }

    /// Динамически изменяет количество столбиков (при ресайзе окна терминала)
    /// с сохранением аудио-буферов и непрерывности сигнала (без сброса в тишину).
    pub fn set_num_bars(&mut self, num_bars: usize) {
        let num_bars = num_bars.clamp(1, 256);
        if self.num_bars == num_bars {
            return;
        }
        self.num_bars = num_bars;
        let (use_bass_fft, lower_bin, upper_bin, eq_weights) =
            Self::compute_band_bins(num_bars, self.sample_rate);
        self.use_bass_fft = use_bass_fft;
        self.lower_bin = lower_bin;
        self.upper_bin = upper_bin;
        self.eq_weights = eq_weights;

        self.rebuild_spread_weights();

        self.raw_bars.resize(num_bars, 0.0);
        self.band_ref.resize(num_bars, 0.0);
        self.mem.resize(num_bars, 0.0);
        self.prev.resize(num_bars, 0.0);
        self.fall_step.resize(num_bars, 0.0);
        self.out_buf.resize(num_bars, 0.0);
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

    /// Устанавливает шкалу высот столбиков, сбрасывая сглаживание и опорный пик.
    pub fn set_scale_mode(&mut self, mode: SpectrumScale) {
        if self.scale_mode == mode {
            return;
        }
        self.scale_mode = mode;
        self.rel_window = REL_WINDOW;
        match mode {
            SpectrumScale::Legacy => {
                self.db_window = DB_DYNAMIC_RANGE;
                self.integral = INTEGRAL;
                self.gravity = GRAVITY;
            }
            SpectrumScale::TrackPeak => {
                self.integral = TRACK_PEAK_INTEGRAL;
                self.gravity = GRAVITY;
            }
            SpectrumScale::FramePeak => {
                self.integral = FRAME_PEAK_INTEGRAL;
                self.gravity = FRAME_PEAK_GRAVITY;
                self.frame_ref_release = FRAME_PEAK_RELEASE;
            }
            SpectrumScale::Absolute => {
                // Окно `abs_window` здесь не трогаем: оно общий регулятор контраста,
                // и при переключении шкал туда-обратно пользовательское значение должно
                // сохраняться.
                self.apply_response();
            }
        }
        self.slow_peak = 0.0;
        self.frame_ref = 0.0;
        self.sensitivity = 1.0;
        self.reset();
    }

    /// Устанавливает ширину dB-окна шкалы `Absolute` (контраст): узкое окно — пики
    /// up в потолок и низкий фон, широкое — плотная картина. Значение вне границ
    /// `ABS_WINDOW_MIN`..`ABS_WINDOW_MAX` прижимается к ближайшей границе. Окно
    /// применяется только к `Absolute`; переключение шкалы его не сбрасывает.
    pub fn set_abs_window(&mut self, window: f32) {
        self.abs_window = window.clamp(ABS_WINDOW_MIN, ABS_WINDOW_MAX);
    }

    /// Builder-вариант `set_abs_window`.
    pub fn with_abs_window(mut self, window: f32) -> Self {
        self.set_abs_window(window);
        self
    }

    /// Активная ширина dB-окна шкалы `Absolute`.
    pub fn abs_window(&self) -> f32 {
        self.abs_window
    }

    /// Устанавливает скорость отклика полос шкалы `Absolute`: пресет меняет
    /// `integral`, гравитацию падения и скорость роста `sens`. Для остальных шкал
    /// значение сохраняется и применяется при возврате к `Absolute`.
    pub fn set_response(&mut self, response: SpectrumResponse) {
        self.response = response;
        if self.scale_mode == SpectrumScale::Absolute {
            self.apply_response();
        }
    }

    /// Builder-вариант `set_response`.
    pub fn with_response(mut self, response: SpectrumResponse) -> Self {
        self.set_response(response);
        self
    }

    /// Активный пресет отклика.
    pub fn response(&self) -> SpectrumResponse {
        self.response
    }

    /// Выставляет константы сглаживания и роста `sens` по выбранному пресету.
    fn apply_response(&mut self) {
        match self.response {
            SpectrumResponse::Cava => {
                self.integral = ABS_INTEGRAL;
                self.gravity = ABS_GRAVITY;
                self.sens_up = ABS_SENS_UP;
            }
            SpectrumResponse::Snappy => {
                self.integral = SNAPPY_INTEGRAL;
                self.gravity = SNAPPY_GRAVITY;
                self.sens_up = SNAPPY_SENS_UP;
            }
        }
    }

    /// Устанавливает базу пространственного фильтра соседних полос. Значение
    /// `≤ 1.05` (в том числе `0.0`) выключает фильтр — как в эталонной cava:
    /// доминирующая полоса не подтягивает соседей, и тёмные полосы остаются тёмными.
    /// Больше базы — сильнее «плато» вокруг пика (1.35 — поведение шкалы v2.5).
    pub fn set_spread_base(&mut self, base: f32) {
        let base = if base <= SPREAD_MIN_BASE {
            SPREAD_OFF
        } else {
            base.clamp(SPREAD_MIN_BASE, SPREAD_MAX)
        };
        if (self.spread_base - base).abs() < 1e-6 {
            return;
        }
        self.spread_base = base;
        self.rebuild_spread_weights();
    }

    /// Текущая база пространственного фильтра (`0.0` = выключен).
    pub fn spread_base(&self) -> f32 {
        self.spread_base
    }

    /// Builder-версия `set_spread_base`.
    pub fn with_spread_base(mut self, base: f32) -> Self {
        self.set_spread_base(base);
        self
    }

    /// Builder-версия `set_scale_mode`.
    pub fn with_scale_mode(mut self, mode: SpectrumScale) -> Self {
        self.set_scale_mode(mode);
        self
    }

    /// Возвращает активную шкалу высот.
    pub fn scale_mode(&self) -> SpectrumScale {
        self.scale_mode
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
        self.band_ref.fill(0.0);
        self.slow_peak = 0.0;
        self.frame_ref = 0.0;
    }

    /// Пересчитывает веса пространственного фильтра; выключенный фильтр даёт веса 1.0
    /// (в цикле распространения он тогда не участвует).
    fn rebuild_spread_weights(&mut self) {
        self.monstercat_weights = if self.spread_base > SPREAD_MIN_BASE {
            (0..self.num_bars)
                .map(|d| self.spread_base.powi(d as i32))
                .collect()
        } else {
            vec![1.0f32; self.num_bars]
        };
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

        // 1. Энергия октавных полос: сумма магнитуд по бинам (равная энергия на
        //    октаву) плюс полка пикового бина для коротких ВЧ-транзиентов.
        //    RMS-усреднение по всем бинам полосы размазывало разреженное содержимое
        //    широких ВЧ-полос (полоса 10–20 кГц — сотни бинов), из-за чего правая
        //    половина спектра оставалась плоской, а доминировал самый громкий столбик.
        let ratio = HIGH_HZ / LOW_HZ;
        let mut peak_lin = 0.0f32;
        let mut peak_eq = 0.0f32;
        for n in 0..self.num_bars {
            let spectrum = if self.use_bass_fft[n] {
                &self.bass_out
            } else {
                &self.mid_out
            };
            let lo = self.lower_bin[n];
            let hi = self.upper_bin[n].min(spectrum.len());
            let count = (hi - lo).max(1);

            let mag = if count <= 1 && spectrum.len() > 2 {
                let f_center = LOW_HZ * ratio.powf((n as f32 + 0.5) / self.num_bars as f32);
                let fft_size = if self.use_bass_fft[n] {
                    N_BASS
                } else {
                    N_MID_HIGH
                };
                let exact_bin = (f_center * fft_size as f32 / self.sample_rate)
                    .clamp(1.0, (spectrum.len() - 2) as f32);
                let b0 = exact_bin.floor() as usize;
                let frac = exact_bin - b0 as f32;
                spectrum[b0].norm() * (1.0 - frac) + spectrum[b0 + 1].norm() * frac
            } else {
                let mut sum = 0.0f32;
                let mut peak = 0.0f32;
                for c in &spectrum[lo..hi] {
                    let m = c.norm();
                    sum += m;
                    if m > peak {
                        peak = m;
                    }
                }
                sum + PEAK_BLEND * peak
            };

            // В `Absolute` полоса = среднее магнитуд по её бинам: дефолт cava делит
            // `eq[n]` на число бинов полосы (`upper_cut_off - lower_cut_off + 1`).
            // Без этого деления широкие ВЧ-полосы (сотни бинов) систематически выше
            // узких НЧ-полос, и вся картина дышит целиком вместе с доминантой.
            let lin = if self.scale_mode == SpectrumScale::Absolute {
                mag * self.eq_weights[n] / count as f32
            } else {
                mag * self.eq_weights[n]
            };
            if lin > peak_lin {
                peak_lin = lin;
            }

            // Долгосрочный уровень полосы: быстрый вход, медленный выход, чтобы AGC
            // не «дышал» в такт музыке и не сжимал динамику.
            let ref_i = &mut self.band_ref[n];
            let toward = if lin > *ref_i { AGC_REF_ATTACK } else { AGC_REF_RELEASE };
            *ref_i += (lin - *ref_i) * toward;

            self.raw_bars[n] = lin;        }

        // Средний долгосрочный уровень спектра — опорная точка по-полосного AGC.
        let mut ref_sum = 0.0f32;
        for r in &self.band_ref {
            ref_sum += *r;
        }
        let ref_mean = ref_sum / self.num_bars as f32;

        // 1b. Относительные режимы: тишина гасится целиком, опорная точка шкалы —
        //     медленный пик трека (EMA) или пик кадра.
        // Относительные режимы — только TrackPeak и FramePeak. `Absolute`, как и
        // `Legacy`, не нормируется по пикам, но и AGC/гейта у него нет.
        let rel = matches!(
            self.scale_mode,
            SpectrumScale::TrackPeak | SpectrumScale::FramePeak
        );
        if rel {
            if peak_lin < REL_SILENCE_PEAK {
                self.mem.fill(0.0);
                self.prev.fill(0.0);
                self.fall_step.fill(0.0);
                self.out_buf.fill(0.0);
                return &self.out_buf;
            }
            if self.scale_mode == SpectrumScale::FramePeak {
                // Подъём эталона не ограничен (пик кадра всегда эталон), спуск медленный:
                // после транзиента нормировка «держится», а не скачет вверх-вниз каждый кадр.
                if peak_lin > self.frame_ref {
                    self.frame_ref = peak_lin;
                } else {
                    self.frame_ref += (peak_lin - self.frame_ref) * self.frame_ref_release;
                }
            }
            if self.scale_mode == SpectrumScale::TrackPeak {
                let toward = if peak_lin > self.slow_peak {
                    TRACK_PEAK_ATTACK
                } else {
                    TRACK_PEAK_RELEASE
                };
                self.slow_peak += (peak_lin - self.slow_peak) * toward;
            }
        }
        let denom = match self.scale_mode {
            SpectrumScale::Legacy | SpectrumScale::Absolute => 1.0,
            SpectrumScale::TrackPeak => self.slow_peak.max(1e-7),
            SpectrumScale::FramePeak => self.frame_ref.max(1e-7),
        };

        // 2. По-полосный AGC + логарифмическая (dB) шкала по умолчанию или мягкая
        //    компрессия в линейной. Без них самый громкий столбик съедает весь
        //    диапазон высот, и остальные полосы почти не «прыгают».
        // В относительных режимах AGC и autosens не нужны: нормировка от опорного пика
        // сама убирает общую громкость трека, а равная энергия на октаву (наклон
        // `eq_weights`) уже компенсирует спектральный спад.
        let band_gate = if rel { REL_BAND_GATE } else { REL_SILENCE_GATE };
        for n in 0..self.num_bars {
            let lin = self.raw_bars[n];
            if self.scale_mode == SpectrumScale::Absolute {
                // Абсолютная dB-шкала: никакой опорной точки, никакого по-полосного AGC
                // и никакого гейта относительно пика кадра — только порог тишины и один
                // общий медленный `sens` ниже. Полосы физически независимы друг от друга.
                let val = if lin <= ABS_SILENCE_FLOOR {
                    0.0
                } else {
                    (20.0 * lin.log10() + self.abs_window) / self.abs_window
                };
                // Высоту здесь не обрезаем до 1.0: в дефолте cava значения выше единицы
                // доходят до сглаживания, и именно там переполнение (`mem > 1`) заставляет
                // общий `sens` падать. С обрезкой до единицы переполнение невозможно, sens
                // растёт бесконечно, и вся картина прилипает к потолку.
                self.raw_bars[n] = val * self.sensitivity;
                continue;
            }
            if rel {
                let val = if lin <= peak_lin * band_gate {
                    0.0
                } else {
                    (20.0 * (lin / denom).log10() + self.rel_window) / self.rel_window
                };
                self.raw_bars[n] = val.clamp(0.0, 1.0);
                continue;
            }
            let scaled = if lin <= peak_lin * band_gate {
                0.0
            } else {
                let ref_i = self.band_ref[n].max(1e-7);
                let gain = (ref_mean / ref_i)
                    .clamp(AGC_MIN, AGC_MAX)
                    .powf(AGC_STRENGTH)
                    .max(0.05);
                let eq = lin * gain;
                if eq > peak_eq {
                    peak_eq = eq;
                }
                eq * self.sensitivity
            };

            let val = if scaled <= DB_SILENCE_FLOOR {
                0.0
            } else if self.db_scale {
                ((20.0 * scaled.log10()) + self.db_window) / self.db_window
            } else {
                scaled.powf(LIN_COMPRESSION)
            };

            self.raw_bars[n] = val.clamp(0.0, 1.0);
        }

        // 2. Пространственный фильтр сглаживания Monstercat (без аллокаций):
        // bars[i] = max_j(bars[j] / spread_base^|i - j|). Выключен (база ≤ 1.05) —
        // полосы остаются самостоятельными, как в эталонной cava.
        if self.spread_base > SPREAD_MIN_BASE {
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
        } else {
            self.out_buf.copy_from_slice(&self.raw_bars);
        }

        // 3. Параболическая гравитация падения, интегральное сглаживание (0.77)
        //    и авточувствительность по сырой амплитуде (не по уже сжатой высоте).
        let mut overshoot = false;

        for i in 0..self.num_bars {
            let mut v = self.out_buf[i];
            if v < self.prev[i] {
                let factor = (1.0 - self.gravity * self.fall_step[i].powi(2)).max(0.0);
                v = (self.prev[i] * factor).max(v);
                self.fall_step[i] += 1.0;
            } else {
                self.fall_step[i] = 0.0;
            }

            self.prev[i] = v;

            let integral = self.integral;
            let smoothed = self.mem[i] * integral + v * (1.0 - integral);
            self.mem[i] = if smoothed < 1e-6 { 0.0 } else { smoothed };

            if self.mem[i] > 1.0 {
                overshoot = true;
                self.out_buf[i] = 1.0;
            } else {
                self.out_buf[i] = self.mem[i];
            }
        }

        if self.scale_mode == SpectrumScale::Absolute {
            // Как в дефолте cava: один общий `sens` растёт медленно каждый кадр и падает
            // только когда высоты упёрлись в потолок. Он не реагирует на перескок
            // доминирующей полосы, поэтому соседние столбики не подпрыгивают вместе с ней.
            if self.autosens {
                if overshoot {
                    self.sensitivity *= ABS_SENS_DOWN;
                } else if peak_lin > ABS_SILENCE_FLOOR {
                    self.sensitivity *= self.sens_up;
                }
                self.sensitivity = self.sensitivity.clamp(AUTOSENS_MIN, AUTOSENS_MAX);
            }
        } else if overshoot && self.autosens {
            // Жёсткий антиклиппинг: если высоты всё же упёрлись в потолок, сбрасываем
            // чувствительность быстрее, иначе спектр «прилипает» к максимуму.
            self.sensitivity =
                (self.sensitivity * (AUTOSENS_DOWN * 0.9)).clamp(AUTOSENS_MIN, AUTOSENS_MAX);
        }

        if self.autosens && !rel && self.scale_mode != SpectrumScale::Absolute && peak_eq > 1e-5 {
            let scaled_peak = peak_eq * self.sensitivity;
            // Мёртвая зона: пока пик в пределах [TARGET_LOW, TARGET_HIGH], чувствительность
            // не меняется — динамика трека сохраняется, а не «сжимается» до одной высоты.
            if scaled_peak > AUTOSENS_TARGET_HIGH {
                self.sensitivity *= AUTOSENS_DOWN;
            } else if scaled_peak < AUTOSENS_TARGET_LOW {
                self.sensitivity *= AUTOSENS_UP;
            }
            self.sensitivity = self.sensitivity.clamp(AUTOSENS_MIN, AUTOSENS_MAX);
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

    /// Кормит сигнал до тех пор, пока высоты не перестанут меняться.
    fn steady_state(cava: &mut Cavacore, signal: &[f32]) -> Vec<f32> {
        let mut prev = cava.process(signal).to_vec();
        for _ in 0..60 {
            let next = cava.process(signal).to_vec();
            if next
                .iter()
                .zip(prev.iter())
                .all(|(a, b)| (a - b).abs() < 1e-3)
            {
                return next;
            }
            prev = next;
        }
        prev
    }

    #[test]
    fn relative_modes_zero_on_silence() {
        for mode in [SpectrumScale::TrackPeak, SpectrumScale::FramePeak] {
            let mut cava = Cavacore::new(44_100.0, 15).with_scale_mode(mode);
            let zeros = vec![0.0f32; N_BASS];
            let bars = cava.process(&zeros);
            assert!(
                bars.iter().all(|v| *v == 0.0),
                "{mode:?}: тишина должна обнулять все столбики"
            );
        }
    }

    #[test]
    fn switching_scale_mode_resets_smoothing() {
        let signal = sine_wave(100.0, 44_100.0, N_BASS);
        let mut cava = Cavacore::new(44_100.0, 15).with_db_scale(true);
        let _ = steady_state(&mut cava, &signal);

        cava.set_scale_mode(SpectrumScale::TrackPeak);
        assert_eq!(cava.scale_mode(), SpectrumScale::TrackPeak);
        let after_switch = cava.process(&signal).to_vec();

        let mut fresh = Cavacore::new(44_100.0, 15)
            .with_db_scale(true)
            .with_scale_mode(SpectrumScale::TrackPeak);
        let fresh_first = fresh.process(&signal).to_vec();

        for (a, b) in after_switch.iter().zip(fresh_first.iter()) {
            assert!(
                (a - b).abs() < 1e-6,
                "переключение шкалы должно сбрасывать сглаживание: {a} vs {b}"
            );
        }
    }

    #[test]
    fn snappy_response_rises_faster_than_cava_default() {
        let tone: Vec<f32> = sine_wave(120.0, 44_100.0, 735)
            .iter()
            .map(|s| s * 0.25)
            .collect();
        let build = |response: SpectrumResponse| {
            Cavacore::new(44_100.0, 32)
                .with_scale_mode(SpectrumScale::Absolute)
                .with_abs_window(40.0)
                .with_response(response)
        };
        let mut cava = build(SpectrumResponse::Cava);
        let mut snappy = build(SpectrumResponse::Snappy);

        // Первые кадры после сброса: `sens` ещё равен 1 у обоих пресетов, поэтому
        // разница здесь чисто от скорости сглаживания (`integral`).
        let frames = 16;
        let rise = |core: &mut Cavacore| -> Vec<f32> {
            core.reset();
            (0..frames)
                .map(|_| {
                    let bars = core.process(&tone);
                    bars.iter().copied().fold(0.0f32, f32::max)
                })
                .collect()
        };
        let slow_rise = rise(&mut cava);
        let fast_rise = rise(&mut snappy);
        println!(
            "cava   : {}",
            slow_rise.iter().map(|v| format!("{v:.2}")).collect::<Vec<_>>().join(" ")
        );
        println!(
            "snappy : {}",
            fast_rise.iter().map(|v| format!("{v:.2}")).collect::<Vec<_>>().join(" ")
        );

        for (idx, (fast, slow)) in fast_rise.iter().zip(slow_rise.iter()).enumerate() {
            assert!(
                fast >= slow,
                "кадр {idx}: быстрый пресет не может быть тише дефолта ({fast:.2} < {slow:.2})"
            );
        }
        assert!(
            fast_rise[1] > slow_rise[1],
            "на втором кадре быстрый пресет уже должен быть впереди: {:.2} против {:.2}",
            fast_rise[1],
            slow_rise[1]
        );

        let threshold = slow_rise[frames - 1].min(fast_rise[frames - 1]) * 0.8;
        let reached = |bars: &[f32]| bars.iter().position(|&v| v >= threshold).unwrap_or(frames);
        let fast = reached(&fast_rise);
        let slow = reached(&slow_rise);
        println!(
            "порог {threshold:.2}: cava {slow} кадров ({:.0} мс), snappy {fast} кадров ({:.0} мс)",
            (slow + 1) as f64 * 16.67,
            (fast + 1) as f64 * 16.67
        );
        assert!(
            fast < slow,
            "быстрый пресет должен раньше достичь порога: {fast} против {slow} кадров"
        );
    }

    #[test]
    fn track_peak_keeps_track_loudness_as_reference() {
        let sr = 44_100.0;
        let loud = sine_wave(100.0, sr, N_BASS);
        let quiet: Vec<f32> = (0..N_BASS)
            .map(|i| 0.01 * (2.0 * PI * 100.0 * i as f32 / sr).sin())
            .collect();

        let mut peak = Cavacore::new(sr, 15)
            .with_db_scale(true)
            .with_scale_mode(SpectrumScale::TrackPeak);
        for _ in 0..120 {
            let _ = peak.process(&loud);
        }
        for _ in 0..30 {
            let _ = peak.process(&quiet);
        }
        let bars = peak.process(&quiet);
        let max_bar = bars.iter().copied().fold(0.0f32, f32::max);
        assert!(
            max_bar < 0.2,
            "тихий_pass после громкого не должен подняться до полной высоты: {max_bar}"
        );

        // FramePeak нормируется от собственного кадра и поднимает тихий фрагмент в потолок.
        let mut frame = Cavacore::new(sr, 15)
            .with_db_scale(true)
            .with_scale_mode(SpectrumScale::FramePeak);
        for _ in 0..120 {
            let _ = frame.process(&loud);
        }
        for _ in 0..30 {
            let _ = frame.process(&quiet);
        }
        let bars = frame.process(&quiet);
        let max_bar = bars.iter().copied().fold(0.0f32, f32::max);
        assert!(
            max_bar > 0.5,
            "FramePeak нормируется от кадра (с удержанием ~0.1 с) и поднимает тихий \
             фрагмент, но не в самый потолок: {max_bar}"
        );
    }

    #[test]
    fn track_peak_drops_quiet_bands_into_valleys_legacy_keeps_midrange() {
        let sr = 44_100.0;
        // Громкий бас 60 Гц + тихая ВЧ-полоса 5 кГц (−26 дБ): окно 26 дБ в `TrackPeak`
        // показывает и потолок, и провалы, а окно 32 дБ от 0 dBFS в `Legacy` держит
        // громкие СЧ-полосы в середине шкалы и проваливает только действительно тихие.
        let signal: Vec<f32> = (0..N_BASS)
            .map(|i| {
                (2.0 * PI * 60.0 * i as f32 / sr).sin()
                    + 0.05 * (2.0 * PI * 5_000.0 * i as f32 / sr).sin()
            })
            .collect();

        let mut legacy = Cavacore::new(sr, 15).with_db_scale(true);
        let mut peak = Cavacore::new(sr, 15)
            .with_db_scale(true)
            .with_scale_mode(SpectrumScale::TrackPeak);

        let l = steady_state(&mut legacy, &signal);
        let p = steady_state(&mut peak, &signal);
        let range = |v: &[f32]| {
            let hi = v.iter().copied().fold(0.0f32, f32::max);
            let lo = v.iter().copied().fold(1.0f32, f32::min);
            hi - lo
        };

        assert!(
            range(&p) > range(&l) - 0.05,
            "обе шкалы используют вертикальный диапазон почти целиком: {} vs {}",
            range(&p),
            range(&l)
        );
        assert!(
            p.iter().any(|v| *v > 0.85),
            "громкая полоса в TrackPeak должна упираться в потолок"
        );
        let dark = |v: &[f32]| v.iter().filter(|x| **x < 0.2).count();
        assert!(
            dark(&p) >= 4,
            "в TrackPeak тихие полосы должны уходить в провал: {}",
            dark(&p)
        );
        let l_mid = l.iter().filter(|x| **x > 0.4 && **x < 0.7).count();
        assert!(
            l_mid >= 2,
            "в Legacy окно 32 дБ держит громкие СЧ-полосы в середине шкалы: {l:?}"
        );
    }

    #[test]
    fn spread_filter_is_off_by_default_and_only_lifts_neighbors_when_enabled() {
        let sr = 44_100.0;
        let tone = sine_wave(60.0, sr, N_BASS);

        let mut off = Cavacore::new(sr, 15)
            .with_db_scale(true)
            .with_scale_mode(SpectrumScale::Legacy);
        assert_eq!(off.spread_base(), 0.0, "по умолчанию соседний фильтр выключен");
        let b_off = steady_state(&mut off, &tone);

        let mut on = Cavacore::new(sr, 15)
            .with_db_scale(true)
            .with_scale_mode(SpectrumScale::Legacy)
            .with_spread_base(MONSTERCAT_BASE);
        let b_on = steady_state(&mut on, &tone);

        let dark = |v: &[f32]| v.iter().filter(|x| **x < 0.2).count();
        let rest = |v: &[f32]| {
            let hi = v.iter().copied().fold(0.0f32, f32::max);
            v.iter().filter(|x| **x < hi).copied().fold(0.0f32, |acc, x| acc + x)
        };
        assert!(
            dark(&b_off) >= 10,
            "выключенный фильтр держит соседние полосы тёмными: {b_off:?}"
        );
        assert!(
            dark(&b_on) <= 8 && rest(&b_on) > rest(&b_off) + 1.5,
            "включённая база 1.35 подтягивает соседей к доминирующей полосе: {b_on:?}"
        );
    }

    #[test]
    fn spread_base_clamps_to_off_and_max() {
        assert_eq!(Cavacore::new(44_100.0, 15).with_spread_base(1.0).spread_base(), 0.0);
        assert_eq!(Cavacore::new(44_100.0, 15).with_spread_base(0.0).spread_base(), 0.0);
        assert_eq!(
            Cavacore::new(44_100.0, 15).with_spread_base(99.0).spread_base(),
            SPREAD_MAX
        );
        assert_eq!(Cavacore::new(44_100.0, 15).with_spread_base(2.5).spread_base(), 2.5);
    }

    #[test]
    fn frame_peak_hold_damps_quiet_passage_after_transient() {
        let sr = 44_100.0;
        let frame = (sr / 60.0) as usize;
        let loud = sine_wave(100.0, sr, frame);
        let quiet: Vec<f32> = sine_wave(100.0, sr, frame)
            .iter()
            .map(|s| s * 0.05)
            .collect();

        let max_after_quiet = |release: f32| -> f32 {
            let mut cava = Cavacore::new(sr, 15)
                .with_db_scale(true)
                .with_scale_mode(SpectrumScale::FramePeak);
            cava.frame_ref_release = release;
            for _ in 0..120 {
                let _ = cava.process(&loud);
            }
            for _ in 0..6 {
                let _ = cava.process(&quiet);
            }
            cava.process(&quiet).iter().copied().fold(0.0f32, f32::max)
        };

        let instant = max_after_quiet(1.0);
        let held = max_after_quiet(FRAME_PEAK_RELEASE);
        assert!(
            instant > 0.9,
            "без удержания тихий фрагмент сразу нормируется в потолок: {instant}"
        );
        assert!(
            held < instant - 0.25,
            "удержание опорного пика кадра гасит скачок высоты после транзиента: \
             {held} vs {instant}"
        );
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
            low_max > 0.15,
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
            high_max > 0.15,
            "ВЧ-пик слишком мал для видимости в TUI: {high_max}"
        );
        assert!(
            high_max > low_max * 2.0,
            "верхние полосы ({high_max}) должны значительно доминировать над НЧ ({low_max})"
        );
    }

    #[test]
    fn dynamic_resize_maintains_audio_state() {
        let mut cava = Cavacore::new(44_100.0, 30);
        let signal = sine_wave(100.0, 44_100.0, N_BASS);
        for _ in 0..4 {
            let _ = cava.process(&signal);
        }
        cava.set_num_bars(15);
        assert_eq!(cava.num_bars(), 15);
        let bars = cava.process(&[]);
        assert_eq!(bars.len(), 15);
        let low_max = bars[..7].iter().copied().fold(0.0f32, f32::max);
        assert!(low_max > 0.05, "сигнал должен сохраняться после ресайза");
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
            if step < 2 {
                assert!(
                    current_max <= peak * 1.08,
                    "первые кадры тишины не должны резко отклоняться от пика: peak={peak}, current={current_max}"
                );
            } else {
                assert!(
                    current_max <= prev_max + 1e-6,
                    "высота столбиков должна монотонно убывать: шаг {step}, {current_max} > {prev_max}"
                );
            }
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
        assert!(max_bar > 0.05 && max_bar <= 1.0);
    }

    /// Шумовой генератор для имитации широкополосных ВЧ-транзиентов.
    fn noise(seed: &mut u32) -> f32 {
        *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((*seed >> 8) as f32 / 8_388_608.0) - 1.0
    }

    /// Микс «электроника»: громкий бас 60 Гц, средний пэд 1 кГц, слабые шляпки 9 кГц.
    fn music_mix(sr: f32, len: usize, seed: &mut u32, hats_on: bool) -> Vec<f32> {
        (0..len)
            .map(|i| {
                let t = i as f32 / sr;
                let bass = 0.80 * (2.0 * PI * 60.0 * t).sin();
                let pad = 0.12 * (2.0 * PI * 1_000.0 * t).sin();
                let hat = if hats_on {
                    0.05 * noise(seed) * (2.0 * PI * 9_000.0 * t).sin()
                } else {
                    0.0
                };
                bass + pad + hat
            })
            .collect()
    }

    #[test]
    fn loud_bass_does_not_flatten_high_bands() {
        let sr = 44_100.0;
        let mut cava = Cavacore::new(sr, 15);
        let mut seed = 4242u32;

        for _ in 0..24 {
            let s = music_mix(sr, 735, &mut seed, true);
            let _ = cava.process(&s);
        }
        let s = music_mix(sr, 735, &mut seed, true);
        let with_hats = cava.process(&s).to_vec();

        for _ in 0..6 {
            let s = music_mix(sr, 735, &mut seed, false);
            let _ = cava.process(&s);
        }
        let s = music_mix(sr, 735, &mut seed, false);
        let without_hats = cava.process(&s).to_vec();

        let hi_band = |bars: &[f32]| bars[11..15].iter().copied().fold(0.0f32, f32::max);
        let hi_on = hi_band(&with_hats);
        let hi_off = hi_band(&without_hats);

        assert!(with_hats[2] > 0.4, "бас должен оставаться заметным: {}", with_hats[2]);
        assert!(
            hi_on > 0.2,
            "ВЧ-полосы не должны быть плоскими из-за доминирующего баса: on={hi_on}, off={hi_off}"
        );
        assert!(
            hi_on - hi_off > 0.05,
            "шляпки должны заметно «подпрыгивать»: on={hi_on}, off={hi_off}"
        );
    }

    /// Регрессия на «вспыхивающие от утечки ВЧ»: чистый бас не должен засвечивать
    /// полосу выше ~3 кГц (относительный порог тишины + узкое окно dB).
    #[test]
    fn pure_bass_keeps_high_bands_dark() {
        let sr = 44_100.0;
        let mut cava = Cavacore::new(sr, 15);
        let mut high = [0.0f32; 4];
        for _ in 0..30 {
            let s = sine_wave(60.0, sr, 735);
            let bars = cava.process(&s).to_vec();
            for (dst, &v) in high.iter_mut().zip(&bars[11..15]) {
                *dst = dst.max(v);
            }
        }
        let max_high = high.iter().copied().fold(0.0f32, f32::max);
        assert!(
            max_high < 0.15,
            "полосы 3–20 кГц должны быть тёмными при чистом басе: {max_high}"
        );
    }

    /// Регрессия на «доминирует самый громкий элемент»: не-басовый микс (лид 1.5 кГц)
    /// не должен гасить средины и ВЧ — все трети спектра обязаны быть заметными.
    #[test]
    fn sustained_lead_does_not_flatten_other_bands() {
        let sr = 44_100.0;
        let mut cava = Cavacore::new(sr, 15);
        let mut seed = 777u32;
        let mut peaks = [0.0f32; 15];

        for _ in 0..30 {
            let s = (0..735)
                .map(|i| {
                    let t = i as f32 / sr;
                    let lead = 0.32 * (2.0 * PI * 1_500.0 * t).sin();
                    let piano = 0.10 * (2.0 * PI * 440.0 * t).sin();
                    let hat = 0.03 * noise(&mut seed) * (2.0 * PI * 8_000.0 * t).sin();
                    lead + piano + hat
                })
                .collect::<Vec<f32>>();
            let bars = cava.process(&s).to_vec();
            for (dst, &v) in peaks.iter_mut().zip(&bars) {
                *dst = dst.max(v);
            }
        }

        let avg = |from: usize, to: usize| -> f32 {
            peaks[from..to].iter().sum::<f32>() / (to - from) as f32
        };
        assert!(avg(0, 5) > 0.15, "низы должны быть видны: {}", avg(0, 5));
        assert!(avg(5, 10) > 0.2, "средины должны быть видны: {}", avg(5, 10));
        assert!(avg(10, 15) > 0.2, "верхи должны быть видны: {}", avg(10, 15));
    }

    /// Регрессия на асимметрию авточувствительности: тихий участок обязан поднять
    /// чувствительность, а громкий — вернуть её назад за секунды, а не за десятки секунд.
    #[test]
    fn autosens_recovers_after_loud_passage() {
        let sr = 44_100.0;
        let mut cava = Cavacore::new(sr, 15);

        let quiet = sine_wave(60.0, sr, 735)
            .into_iter()
            .map(|s| s * 0.05)
            .collect::<Vec<f32>>();
        for _ in 0..90 {
            let _ = cava.process(&quiet);
        }
        let raised = cava.sensitivity;
        assert!(
            raised > 1.2,
            "тихий участок должен поднять авточувствительность: {raised}"
        );

        for _ in 0..120 {
            let s = sine_wave(60.0, sr, 735);
            let _ = cava.process(&s);
        }
        assert!(
            cava.sensitivity < raised * 0.7,
            "громкий участок должен вернуть чувствительность вниз: {raised} -> {}",
            cava.sensitivity
        );
    }
}
