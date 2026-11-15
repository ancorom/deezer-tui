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
            if step < 2 {
                // Параболическая гравитация стартует с нулевого наклона (fall_step = 0),
                // а интегральный фильтр (0.77) продолжает догонять уровень сигнала:
                // столбики не должны резко прыгнуть, но могут слегка подняться
                // на первых кадрах тишины.
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
        assert!(max_bar > 0.1 && max_bar <= 1.0);
    }
}
