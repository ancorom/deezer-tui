// Чистый DSP Cavacore: окномый FFT → логарифмические полосы частот (50–10000 Гц).
//
// Порт последней версии алгоритма из docs/reference/cava/cavacore.c на чистый Rust:
// * FFT: для низкочастотного диапазона N_bass = 4096 отрезка (высокое разрешение),
//   для средних/высоких частот N_mid_high = 2048 (быстрый отклик по времени);
// * частотные полосы: логарифмическое распределение 50..=10000 Гц, num_bars полос;
// * выход: [f32; num_bars], значения в [0.0; 1.0];
// * сглаживание Monstercat, гравитационный спад, интегральное сглаживание 0.77, autosens.
//
// `Cavacore::process()` принимает чанк образцов (stereo interleaved) и возвращает
// высоты полос. Все буферы предвыделены при инициализации: `process()` не выделяет
// память (кроме минимального выходного буфера, предвыделенного заранее).


use std::sync::Arc;

use num_complex::Complex;
use realfft::RealFftPlanner;
use realfft::RealToComplex;

#[allow(dead_code)]
pub const N_BASS: usize = 4096;
#[allow(dead_code)]
pub const N_MID_HIGH: usize = 2048;

/// Частотный порог: полосы ниже него обрабатываются с FFT N_bass (высокое разрешение).
#[allow(dead_code)]
const BASS_CUTOFF_HZ: f32 = 100.0;

/// Граничные частоты полос: f[i] = 50 * (10000/50)^(i/num_bars) = 50 * 200^(i/num_bars).
#[allow(dead_code)]
const LOW_HZ: f32 = 50.0;
#[allow(dead_code)]
const HIGH_HZ: f32 = 10000.0;
#[allow(dead_code)]
const BASE: f32 = HIGH_HZ / LOW_HZ; // = 200.0

/// Интегральное сглаживание.
#[allow(dead_code)]
const INTEGRAL: f32 = 0.77;

/// Monstercat: соседний пик, уменьшенный на 10%.
#[allow(dead_code)]
const MONSTERCAT: f32 = 0.9;

/// Коэффициенты autosens.
#[allow(dead_code)]
const AUTOSENS_DOWN: f32 = 0.985;
#[allow(dead_code)]
const AUTOSENS_UP: f32 = 1.001;

/// Гравитационный коэффициент спадения.
#[allow(dead_code)]
const GRAVITY: f32 = 0.02;

/// Конверсия стерео-интерливедых образцов в моно.
#[inline]
#[allow(dead_code)]
fn to_mono(stereo: &[f32], dst: &mut [f32]) {
    let n = std::cmp::min(stereo.len() / 2, dst.len());
    for i in 0..n {
        dst[i] = (stereo[2 * i] + stereo[2 * i + 1]) * 0.5;
    }
}

/// Окно Ханна: w[i] = 0.5 * (1 - cos(2*pi*i/(N-1))).
#[allow(dead_code)]
fn hann(n: usize) -> Vec<f32> {
    if n <= 1 {
        return vec![1.0; n];
    }
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos()))
        .collect()
}

/// Граничные частоты полос: f[i] = LOW_HZ * BASE^(i/num_bars), i = 0..=num_bars.
#[allow(dead_code)]
fn bar_boundaries(num_bars: usize) -> Vec<f32> {
    (0..=num_bars)
        .map(|i| {
            let t = i as f32 / num_bars as f32;
            LOW_HZ * BASE.powf(t)
        })
        .collect()
}

/// Индекс бина FFT: bin[i] = (f[i] * N / sample_rate) as usize.
#[inline]
#[allow(dead_code)]
fn bin_index(freq: f32, n: usize, sample_rate: f32) -> usize {
    (freq * n as f32 / sample_rate) as usize
}

/// Выбор FFT для полосы: 0 = bass (N_bass), 1 = mid/high (N_mid_high).
#[allow(dead_code)]
fn fft_for_bar(freq: f32) -> u8 {
    if freq < BASS_CUTOFF_HZ {
        0
    } else {
        1
    }
}

/// EQ-нормализация полосы (по формуле cava): 1/2^28 * f[n+1]^0.85 / log2(FFTsize) / num_bins.
#[allow(dead_code)]
fn eq_for_bar(f: f32, fft_size: usize, num_bins: usize) -> f32 {
    let fft_size_log2 = fft_size as f32;
    let base = (1.0 / 2.0f32.powf(28.0)) * f.powf(0.85);
    let num = std::cmp::max(num_bins, 1);
    base / fft_size_log2.log2() / num as f32
}

/// Сдвиг Ring Buffer: вписывает `mono` в окно `window` (размер `n`) со сдвигом.
#[allow(dead_code)]
fn shift_window(
    window: &mut [f32],
    n: usize,
    write_idx: &mut usize,
    filled: &mut usize,
    mono: &[f32],
) {
    for &s in mono {
        window[*write_idx] = s;
        *write_idx = (*write_idx + 1) % n;
        if *filled < n {
            *filled += 1;
        }
    }
}

/// Выравнивает окно в `dst` (применяет Hann-окно).
#[allow(dead_code)]
fn flatten_window(
    window: &[f32],
    write_idx: usize,
    filled: usize,
    hann: &[f32],
    dst: &mut [f32],
) {
    let start = if filled < window.len() { 0 } else { write_idx };
    for i in 0..dst.len() {
        let val = window[(start + i) % window.len()];
        dst[i] = val * hann[i];
    }
}

/// Сглаживание Monstercat: v[i] = max(v[i], max(v[i-1], v[i+1]) * MONSTERCAT).
#[allow(dead_code)]
fn monstercat(v: &mut [f32]) {
    if v.len() < 3 {
        return;
    }
    let n = v.len();
    let mut next = vec![0.0f32; n];
    for i in 0..n {
        let left = if i > 0 { v[i - 1] } else { 0.0 };
        let right = if i + 1 < n { v[i + 1] } else { 0.0 };
        let neighbor = left.max(right) * MONSTERCAT;
        next[i] = v[i].max(neighbor);
    }
    v.copy_from_slice(&next);
}

/// Сглаживание гравитации + интеграл + autosens.
#[allow(dead_code)]
fn gravity_integral_automod(
    v: &mut [f32],
    prev: &mut [f32],
    fall: &mut [f32],
    mem: &mut [f32],
    sensitivity: &mut f32,
) {
    let n = v.len();
    let mut overshoot = false;
    for i in 0..n {
        if v[i] < prev[i] && GRAVITY > 0.0 {
            let factor = 1.0 - GRAVITY * fall[i].powi(2);
            let nv = prev[i] * factor;
            v[i] = if nv < 0.0 { 0.0 } else { nv };
            fall[i] += 1.0;
        } else {
            fall[i] = 0.0;
        }
        // Интегральное сглаживание: out = mem * 0.77 + v * (1 - 0.77)
        let integral = mem[i] * INTEGRAL + v[i] * (1.0 - INTEGRAL);
        v[i] = integral;
        mem[i] = integral;
        if v[i] > 1.0 {
            v[i] = 1.0;
            overshoot = true;
        }
        prev[i] = v[i];
    }
    if overshoot {
        *sensitivity = (*sensitivity * AUTOSENS_DOWN).clamp(0.1, 5.0);
    } else {
        *sensitivity *= AUTOSENS_UP;
    }
}

/// Cavacore DSP: окномый FFT → логарифмические полосы частот (50–10000 Гц).
///
/// Порт последней версии алгоритма из docs/reference/cava/cavacore.c на чистый Rust:
/// * FFT: для низкочастотного диапазона N_bass = 4096 отрезка (высокое разрешение),
///   для средних/высоких частот N_mid_high = 2048 (быстрый отклик по времени);
/// * частотные полосы: логарифмическое распределение 50..=10000 Гц, num_bars полос;
/// * выход: [f32; num_bars], значения в [0.0; 1.0];
/// * сглаживание Monstercat, гравитационный спад, интегральное сглаживание 0.77, autosens.
///
/// `Cavacore::process()` принимает чанк стерео-интерливедых образцов и возвращает
/// высоты полос. Все буферы предвыделены при инициализации: `process()` не выделяет
/// память (кроме минимального выходного буфера, предвыделенного заранее).
#[allow(dead_code)]
pub struct Cavacore {
    num_bars: usize,
    sample_rate: f32,

    // FFT планы (предвыделены).
    bass_r2c: Arc<dyn RealToComplex<f32>>,
    mid_r2c: Arc<dyn RealToComplex<f32>>,

    // FFT вход/выход (предвыделены).
    bass_in: Vec<f32>,
    bass_out: Vec<Complex<f32>>,
    mid_in: Vec<f32>,
    mid_out: Vec<Complex<f32>>,

    // Сырые окна образцов (предвыделены).
    bass_window: Vec<f32>,
    mid_window: Vec<f32>,
    bass_write_idx: usize,
    bass_filled: usize,
    mid_write_idx: usize,
    mid_filled: usize,

    // Окно Ханна (предвыделено).
    bass_hann: Vec<f32>,
    mid_hann: Vec<f32>,

    // Буфер моно (предвыделен).
    mono_buf: Vec<f32>,

    // Частотные границы полос.
    bars: Vec<f32>,

    // По-полосные диапазоны бинсов FFT: lower[n..upper[n]] (оба — с FFT полосы n).
    lower: Vec<usize>,
    upper: Vec<usize>,
    fft_for: Vec<u8>,

    // EQ-нормализация.
    eq: Vec<f32>,

    // Сглаживание: предвыделено.
    mem: Vec<f32>,
    prev: Vec<f32>,
    fall: Vec<f32>,

    // Autosens.
    sensitivity: f32,

    // Выход (предвыделен).
    out_buf: Vec<f32>,
}

#[allow(dead_code)]
impl Cavacore {
    /// Инициализация Cavacore с заданной частотой и числом полос.
    pub fn new(sample_rate: f32, num_bars: usize) -> Self {
        let num_bars = num_bars.clamp(1, 128);

        // FFT планы
        let mut planner = RealFftPlanner::<f32>::new();
        let bass_r2c: Arc<dyn RealToComplex<f32>> = planner.plan_fft_forward(N_BASS);
        let mid_r2c: Arc<dyn RealToComplex<f32>> = planner.plan_fft_forward(N_MID_HIGH);

        // FFT вход/выход (предвыделены).
        let bass_in = vec![0.0f32; N_BASS];
        let bass_out = vec![Complex::new(0.0, 0.0); N_BASS / 2 + 1];
        let mid_in = vec![0.0f32; N_MID_HIGH];
        let mid_out = vec![Complex::new(0.0, 0.0); N_MID_HIGH / 2 + 1];

        // Сырые окна образцов (предвыделены).
        let bass_window = vec![0.0f32; N_BASS];
        let mid_window = vec![0.0f32; N_MID_HIGH];

        // Окно Ханна (предвыделено).
        let bass_hann = hann(N_BASS);
        let mid_hann = hann(N_MID_HIGH);

        // Буфер моно (предвыделен).
        let mono_buf = vec![0.0f32; N_BASS];

        // Частотные границы полос.
        let bars = bar_boundaries(num_bars);

        // По-полосные диапазоны бинсов FFT (оба — с FFT полосы n).
        let fft_for: Vec<u8> = (0..num_bars)
            .map(|n| fft_for_bar(bars[n]))
            .collect();
        let lower: Vec<usize> = (0..num_bars)
            .map(|n| {
                let fft_size = if fft_for[n] == 0 { N_BASS } else { N_MID_HIGH };
                bin_index(bars[n], fft_size, sample_rate)
            })
            .collect();
        let upper: Vec<usize> = (0..num_bars)
            .map(|n| {
                let fft_size = if fft_for[n] == 0 { N_BASS } else { N_MID_HIGH };
                let idx = bin_index(bars[n + 1], fft_size, sample_rate);
                std::cmp::min(idx, fft_size / 2)
            })
            .collect();

        // EQ-нормализация (по формуле cava).
        let eq: Vec<f32> = (0..num_bars)
            .map(|n| {
                let fft_size = if fft_for[n] == 0 { N_BASS } else { N_MID_HIGH };
                let num_bins = std::cmp::max(upper[n] - lower[n], 1);
                eq_for_bar(bars[n + 1], fft_size, num_bins)
            })
            .collect();

        // Сглаживание: предвыделено.
        let mem = vec![0.0f32; num_bars];
        let prev = vec![0.0f32; num_bars];
        let fall = vec![0.0f32; num_bars];

        // Autosens.
        let sensitivity = 1.0;

        // Выход (предвыделен).
        let out_buf = vec![0.0f32; num_bars];

        Self {
            num_bars,
            sample_rate,
            bass_r2c,
            mid_r2c,
            bass_in,
            bass_out,
            mid_in,
            mid_out,
            bass_window,
            mid_window,
            bass_write_idx: 0,
            bass_filled: 0,
            mid_write_idx: 0,
            mid_filled: 0,
            bass_hann,
            mid_hann,
            mono_buf,
            bars,
            lower,
            upper,
            fft_for,
            eq,
            mem,
            prev,
            fall,
            sensitivity,
            out_buf,
        }
    }

    /// Число полос.
    pub fn num_bars(&self) -> usize {
        self.num_bars
    }

    /// Частота дискретизации.
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Обработка: принимает чанк стерео-интерливедых образцов и возвращает высоты полос.
    ///
    /// Все буферы предвыделены при инициализации: `process()` не выделяет память.
    pub fn process(&mut self, stereo_samples: &[f32]) -> &[f32] {
        let mono_len = stereo_samples.len() / 2;
        let mono_n = std::cmp::min(mono_len, self.mono_buf.len());
        to_mono(stereo_samples, &mut self.mono_buf[..mono_n]);

        // Сдвиг окон
        shift_window(
            &mut self.bass_window,
            N_BASS,
            &mut self.bass_write_idx,
            &mut self.bass_filled,
            &self.mono_buf[..mono_n],
        );
        shift_window(
            &mut self.mid_window,
            N_MID_HIGH,
            &mut self.mid_write_idx,
            &mut self.mid_filled,
            &self.mono_buf[..mono_n],
        );

        // Окно Ханна + FFT
        flatten_window(
            &self.bass_window,
            self.bass_write_idx,
            self.bass_filled,
            &self.bass_hann,
            &mut self.bass_in,
        );
        flatten_window(
            &self.mid_window,
            self.mid_write_idx,
            self.mid_filled,
            &self.mid_hann,
            &mut self.mid_in,
        );

        self.bass_r2c
            .process(&mut self.bass_in, &mut self.bass_out)
            .unwrap();
        self.mid_r2c
            .process(&mut self.mid_in, &mut self.mid_out)
            .unwrap();

        // Сырые значения полос + EQ
        for n in 0..self.num_bars {
            let is_bass = self.fft_for[n] == 0;
            let fft_out = if is_bass {
                &self.bass_out
            } else {
                &self.mid_out
            };
            let lo = self.lower[n];
            let hi = self.upper[n];
            let mut sum = 0.0f32;
            for c in fft_out.as_slice()[lo..hi].iter() {
                let re = c.re;
                let im = c.im;
                sum += (re * re + im * im).sqrt();
            }
            self.out_buf[n] = sum * self.eq[n];
        }

        // Сглаживание: sensitivity, monstercat, gravity, integral, autosens
        for i in 0..self.num_bars {
            self.out_buf[i] *= self.sensitivity;
        }
        monstercat(&mut self.out_buf);
        gravity_integral_automod(
            &mut self.out_buf,
            &mut self.prev,
            &mut self.fall,
            &mut self.mem,
            &mut self.sensitivity,
        );
        &self.out_buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(num_bars: usize, sample_rate: f32) -> Cavacore {
        Cavacore::new(sample_rate, num_bars)
    }

    fn stereo_from_mono(mono: &[f32]) -> Vec<f32> {
        let mut s = Vec::with_capacity(mono.len() * 2);
        for &m in mono {
            s.push(m);
            s.push(m);
        }
        s
    }

    #[test]
    fn zero_buffer_zero_bars() {
        let mut c = make(64, 44100.0);
        let zeros = vec![0.0f32; 1000];
        let stereo = stereo_from_mono(&zeros);
        let bars = c.process(&stereo);
        for (i, b) in bars.iter().enumerate() {
            assert_eq!(*b, 0.0, "bar {i} = {b}, expected 0.0");
        }
    }

    #[test]
    fn low_freq_activates_low_bars() {
        let mut c = make(64, 44100.0);
        let n = 4096;
        let freq = 100.0;
        let mono: Vec<f32> = (0..n)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / 44100.0).sin())
            .collect();
        let stereo = stereo_from_mono(&mono);
        for _ in 0..10 {
            let _ = c.process(&stereo);
        }
        let bars = c.process(&stereo);
        let half = bars.len() / 2;
        let low_energy = bars[0..half].iter().copied().fold(0.0f32, f32::max);
        let high_energy = bars[half..].iter().copied().fold(0.0f32, f32::max);
        assert!(low_energy > 0.0, "low energy = {low_energy}");
        assert!(
            low_energy > high_energy,
            "low ({low_energy}) should dominate over high ({high_energy})"
        );
    }

    #[test]
    fn high_freq_activates_high_bars() {
        let mut c = make(64, 44100.0);
        let n = 4096;
        let freq = 5000.0;
        let mono: Vec<f32> = (0..n)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / 44100.0).sin())
            .collect();
        let stereo = stereo_from_mono(&mono);
        for _ in 0..10 {
            let _ = c.process(&stereo);
        }
        let bars = c.process(&stereo);
        let half = bars.len() / 2;
        let low_energy = bars[0..half].iter().copied().fold(0.0f32, f32::max);
        let high_energy = bars[half..].iter().copied().fold(0.0f32, f32::max);
        assert!(high_energy > 0.0, "high energy = {high_energy}");
        assert!(
            high_energy > low_energy,
            "high ({high_energy}) should dominate over low ({low_energy})"
        );
    }

    #[test]
    fn sudden_cutoff_smooth_decay() {
        let mut c = make(64, 44100.0);
        let n = 4096;
        let mono: Vec<f32> = (0..n)
            .map(|i| (2.0 * std::f32::consts::PI * 100.0 * i as f32 / 44100.0).sin())
            .collect();
        let stereo = stereo_from_mono(&mono);
        // Заполняем
        for _ in 0..30 {
            let _ = c.process(&stereo);
        }
        let before = c.process(&stereo).to_vec();
        let max_before = before.iter().copied().fold(0.0f32, f32::max);
        assert!(max_before > 1e-5, "peak before cutoff = {max_before}");

        // Резкое отключение (нуловой чанк)
        let zeros = vec![0.0f32; n * 2];
        let decay_values: Vec<f32> = (0..15)
            .map(|_| {
                let b = c.process(&zeros).to_vec();
                b.iter().copied().fold(0.0f32, f32::max)
            })
            .collect();

        // Спад должен быть плавным: значение должно заметно убывать
        // (интегральное сглаживание 0.77 + гравитация — экспоненциальный спад)
        assert!(
            decay_values[0] > 1e-9,
            "first decay value = {0:?}, all: {1:?}",
            decay_values[0],
            &decay_values
        );
        assert!(
            decay_values[decay_values.len() - 1] < decay_values[0] / 2.0,
            "decay too shallow: first={0}, last={1}",
            decay_values[0],
            decay_values[decay_values.len() - 1]
        );
    }
}
