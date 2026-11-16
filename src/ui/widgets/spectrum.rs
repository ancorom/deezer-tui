//! Виджет субсимвольного спектра (`[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']`)
//! с интервалом в 1 пробел, точками удержания пиков (`'▪'`) и автоматическим
//! масштабированием числа полос при изменении ширины окна терминала.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::ui::theme;

const GLYPHS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const PEAK_DOT: char = '▪';
const PEAK_DECAY_PER_FRAME: f32 = 0.015;

const fn char_str(c: char) -> &'static str {
    match c {
        ' ' => " ",
        '▁' => "▁",
        '▂' => "▂",
        '▃' => "▃",
        '▄' => "▄",
        '▅' => "▅",
        '▆' => "▆",
        '▇' => "▇",
        '█' => "█",
        '▪' => "▪",
        _ => " ",
    }
}

/// Состояние столбиков спектра и удерживаемых пиков в диапазоне `[0.0, 1.0]`.
pub struct Spectrum {
    bars: Vec<f32>,
    peak_holds: Vec<f32>,
}

impl Default for Spectrum {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Spectrum {
    pub fn new(num_bars: usize) -> Self {
        Self {
            bars: vec![0.0; num_bars],
            peak_holds: vec![0.0; num_bars],
        }
    }

    /// Обновляет высоты столбиков, плавно удерживая исторические пики.
    pub fn set_bars(&mut self, values: &[f32]) {
        self.bars = values.iter().map(|v| v.clamp(0.0, 1.0)).collect();
        if self.peak_holds.len() != self.bars.len() {
            self.peak_holds = self.bars.clone();
        } else {
            for (ph, &bar) in self.peak_holds.iter_mut().zip(self.bars.iter()) {
                *ph = (*ph - PEAK_DECAY_PER_FRAME).max(bar).clamp(0.0, 1.0);
            }
        }
    }

    /// Явно задаёт массив удерживаемых пиков (используется в unit-тестах).
    pub fn set_peak_holds(&mut self, values: &[f32]) {
        self.peak_holds = values.iter().map(|v| v.clamp(0.0, 1.0)).collect();
    }

    /// Обновляет каждый пик до `max(peak_holds[i], bars[i])`.
    pub fn update_peak_holds(&mut self) {
        for (ph, &bar) in self.peak_holds.iter_mut().zip(self.bars.iter()) {
            *ph = ph.max(bar);
        }
    }

    pub fn bars(&self) -> &[f32] {
        &self.bars
    }

    pub fn peak_holds(&self) -> &[f32] {
        &self.peak_holds
    }

    pub fn num_bars(&self) -> usize {
        self.bars.len()
    }

    fn bar_style(value: f32) -> Style {
        Style::default()
            .fg(theme::gradient_color(value))
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
    }

    #[inline]
    fn sample_slice(slice: &[f32], idx: usize, count: usize) -> f32 {
        if slice.is_empty() {
            return 0.0;
        }
        if count >= slice.len() || count <= 1 {
            return slice.get(idx).copied().unwrap_or(0.0);
        }
        let start = idx * slice.len() / count;
        let end = ((idx + 1) * slice.len() / count).max(start + 1).min(slice.len());
        let mut peak = 0.0f32;
        for &v in &slice[start..end] {
            if v > peak {
                peak = v;
            }
        }
        peak
    }
}

impl Widget for &Spectrum {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 || self.bars.is_empty() {
            return;
        }
        let total_rows = h as u32;
        let total_subpixels = total_rows * 8;
        let fit_bars = (w as usize).div_ceil(2);
        let n_bars = self.bars.len().min(fit_bars);

        for b in 0..n_bars {
            let col = (b * 2) as u16;
            if col >= w {
                break;
            }
            let x = area.x + col;
            let v = self.bars[b].clamp(0.0, 1.0);
            let style = Spectrum::bar_style(v);
            let total_fill = (v * (total_subpixels as f32)) as u32;
            let ph = self.peak_holds.get(b).copied().unwrap_or(0.0).clamp(0.0, 1.0);
            let ph_row = (ph * (total_rows as f32)).min(total_rows as f32 - 1.0) as u32;

            for r in 0..total_rows {
                let row_start = r * 8;
                let row_end = row_start + 8;
                let glyph_idx: u8 = if total_fill <= row_start {
                    0
                } else if total_fill >= row_end {
                    8
                } else {
                    (total_fill - row_start) as u8
                };
                let ch = if ph > 0.0 && r == ph_row {
                    PEAK_DOT
                } else {
                    GLYPHS[glyph_idx as usize]
                };
                let y = area.y + (h - 1 - r as u16);
                buf.set_string(x, y, char_str(ch), style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::prelude::*;

    fn render(bars: &[f32], peaks: &[f32], width: u16, height: u16) -> Terminal<TestBackend> {
        let mut spectrum = Spectrum::new(bars.len());
        spectrum.set_bars(bars);
        spectrum.set_peak_holds(peaks);
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(&spectrum, frame.area());
            })
            .unwrap();
        terminal
    }

    fn at(terminal: &Terminal<TestBackend>, x: u16, y: u16) -> char {
        terminal
            .backend()
            .buffer()
            .cell((x, y))
            .expect("buffer cell out of bounds")
            .symbol()
            .chars()
            .next()
            .unwrap_or(' ')
    }

    #[test]
    fn full_bar_is_all_gk() {
        let terminal = render(&[1.0], &[0.0], 4, 12);
        for y in 0..12 {
            assert_eq!(at(&terminal, 0, y), '█');
        }
    }

    #[test]
    fn zero_bar_is_all_space() {
        let terminal = render(&[0.0], &[0.0], 4, 12);
        for y in 0..12 {
            assert_eq!(at(&terminal, 0, y), ' ');
        }
    }

    #[test]
    fn partial_bar_frontier_glyph() {
        let terminal = render(&[0.0625], &[0.0], 4, 12);
        assert_eq!(at(&terminal, 0, 11), '▆');
        assert_eq!(at(&terminal, 0, 10), ' ');
    }

    #[test]
    fn bars_are_spaced_by_one() {
        let terminal = render(&[1.0, 1.0], &[0.0, 0.0], 8, 4);
        for y in 0..4 {
            assert_eq!(at(&terminal, 0, y), '█');
            assert_eq!(at(&terminal, 2, y), '█');
            assert_eq!(at(&terminal, 1, y), ' ');
        }
    }

    #[test]
    fn peak_hold_dot_at_top_of_max() {
        let terminal = render(&[0.5], &[0.75], 4, 12);
        assert_eq!(at(&terminal, 0, 2), PEAK_DOT);
    }

    #[test]
    fn peak_hold_dot_clamped_to_top_row() {
        let terminal = render(&[0.25], &[1.0], 4, 12);
        assert_eq!(at(&terminal, 0, 0), PEAK_DOT);
    }

    #[test]
    fn peak_hold_does_not_overwrite_when_equal_to_bar() {
        let terminal = render(&[0.75], &[0.75], 4, 12);
        assert_eq!(at(&terminal, 0, 2), PEAK_DOT);
        assert_eq!(at(&terminal, 0, 3), '█');
    }

    #[test]
    fn fits_too_many_bars_truncates() {
        let terminal = render(&[1.0; 12], &[0.0; 12], 6, 4);
        for y in 0..4 {
            assert_eq!(at(&terminal, 0, y), '█');
            assert_eq!(at(&terminal, 1, y), ' ');
            assert_eq!(at(&terminal, 2, y), '█');
            assert_eq!(at(&terminal, 3, y), ' ');
            assert_eq!(at(&terminal, 4, y), '█');
            assert_eq!(at(&terminal, 5, y), ' ');
        }
    }
}
