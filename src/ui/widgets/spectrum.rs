//! Spectrum widget: renders a normalized set of audio bars as a vertical-bar
//! equalizer.
//!
//! Each bar is drawn as a column of characters using the glyph scale
//! `[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']`, separated by a single
//! space. The spectrum's height (`area.height` character rows) maps to
//! `8 * area.height` subpixels of value, so a value `v in [0, 1]` fills
//! `v * 8 * rows` subpixels from the bottom. The frontier cell of every bar
//! uses the partial glyph that matches the exact subpixel fill. Peak-hold
//! dots ('▪') are drawn at each bar's historical maximum.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::ui::theme;

/// The glyph scale: index 0 is empty, 1..8 are the filled block elements.
const GLYPHS: [char; 9] = [
    ' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█',
];

/// Peak-hold dot glyph.
const PEAK_DOT: char = '▪';

/// Return a `&'static str` for a single glyph character (avoids per-call
/// allocation in the per-frame render path).
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

/// A normalized set of spectrum bars (each in `[0, 1]`).
pub struct Spectrum {
    /// Current bar values (normalized `[0, 1]`).
    bars: Vec<f32>,
    /// Historical maximum per bar (normalized `[0, 1]`).
    peak_holds: Vec<f32>,
}

impl Default for Spectrum {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Spectrum {
    /// Create an empty spectrum with `num_bars` zero bars.
    pub fn new(num_bars: usize) -> Self {
        Self {
            bars: vec![0.0; num_bars],
            peak_holds: vec![0.0; num_bars],
        }
    }

    /// Set the current normalized bars (resizes, clamping values to `[0, 1]`).
    pub fn set_bars(&mut self, values: &[f32]) {
        self.bars = values.iter().map(|v| v.clamp(0.0, 1.0)).collect();
        // Reset peak holds to the current bars so they track the new range.
        self.peak_holds = self.bars.clone();
    }

    /// Set the historical maximums per bar (resizes, clamping to `[0, 1]`).
    pub fn set_peak_holds(&mut self, values: &[f32]) {
        self.peak_holds = values.iter().map(|v| v.clamp(0.0, 1.0)).collect();
    }

    /// Update each peak hold to `max(peak_holds[i], bars[i])`.
    pub fn update_peak_holds(&mut self) {
        for i in 0..self.bars.len() {
            if i < self.peak_holds.len() {
                self.peak_holds[i] = self.peak_holds[i].max(self.bars[i]);
            }
        }
    }

    /// Current bar values.
    #[allow(dead_code)]
    pub fn bars(&self) -> &[f32] {
        &self.bars
    }

    /// Historical maximums per bar.
    #[allow(dead_code)]
    pub fn peak_holds(&self) -> &[f32] {
        &self.peak_holds
    }

    /// Number of bars.
    #[allow(dead_code)]
    pub fn num_bars(&self) -> usize {
        self.bars.len()
    }

    /// Color for a bar given its normalized value `v` (gradient by height).
    fn bar_style(value: f32) -> Style {
        Style::default()
            .fg(theme::gradient_color(value))
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
    }
}

impl Widget for &Spectrum {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        let total_rows = h as u32;
        let total_subpixels = total_rows * 8;

        for (b, bar) in self.bars.iter().enumerate() {
            let col = (b * 2) as u16;
            if col >= w {
                break;
            }
            let x = area.x + col;
            let v = bar.clamp(0.0, 1.0);
            let style = Spectrum::bar_style(v);
            // Total subpixel fill for this bar across the whole spectrum height.
            let total_fill = (v * (total_subpixels as f32)) as u32;
            // Peak-hold row for this bar (0 = bottom), clamped to a valid row.
            let ph = self.peak_holds.get(b).copied().unwrap_or(0.0).clamp(0.0, 1.0);
            let ph_row = (ph * (total_rows as f32)).min(total_rows as f32 - 1.0) as u32;
            for r in 0..h as u32 {
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

    /// Render a spectrum to a `TestBackend` and return the resulting `Terminal`.
    fn render(bars: &[f32], peaks: &[f32], width: u16, height: u16) -> Terminal<TestBackend> {
        let mut spectrum = Spectrum::new(bars.len());
        spectrum.set_bars(bars);
        spectrum.set_peak_holds(peaks);
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| {
            frame.render_widget(&spectrum, frame.area());
        }).unwrap();
        terminal
    }

    /// Symbol at (x, y) of a rendered terminal's buffer.
    fn at(terminal: &Terminal<TestBackend>, x: u16, y: u16) -> char {
        let buf = terminal.backend().buffer();
        let cell = buf
            .cell((x, y))
            .expect("buffer cell out of bounds");
        cell
            .symbol()
            .chars()
            .next()
            .unwrap_or(' ')
    }

    #[test]
    fn full_bar_is_all_gk() {
        let terminal = render(&[1.0], &[0.0], 4, 12);
        // Bar 0 at column 0, all 12 rows must be '█' (no peak holds).
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
        // 0.0625 in a 12-row area -> 0.0625 * 96 = 6 subpixels -> '▆' (idx 6).
        let terminal = render(&[0.0625], &[0.0], 4, 12);
        assert_eq!(at(&terminal, 0, 11), '▆'); // bottom row (y=11)
        assert_eq!(at(&terminal, 0, 10), ' '); // row above is empty
    }

    #[test]
    fn bars_are_spaced_by_one() {
        let terminal = render(&[1.0, 1.0], &[0.0, 0.0], 8, 4);
        // bar0 at col 0 full, bar1 at col 2 full, col 1 is the gap.
        for y in 0..4 {
            assert_eq!(at(&terminal, 0, y), '█');
            assert_eq!(at(&terminal, 2, y), '█');
            assert_eq!(at(&terminal, 1, y), ' ');
        }
    }

    #[test]
    fn peak_hold_dot_at_top_of_max() {
        // current bar 0.5, peak hold 0.75 in a 12-row area -> peak at row 9 (from bottom)
        // -> y = 12 - 1 - 9 = 2.
        let terminal = render(&[0.5], &[0.75], 4, 12);
        assert_eq!(at(&terminal, 0, 2), PEAK_DOT);
    }

    #[test]
    fn peak_hold_dot_clamped_to_top_row() {
        // current bar 0.25, peak hold 1.0 in a 12-row area -> ph_row = min(12.0, 11.0) = 11
        // -> y = 12 - 1 - 11 = 0 (top row).
        let terminal = render(&[0.25], &[1.0], 4, 12);
        assert_eq!(at(&terminal, 0, 0), PEAK_DOT);
    }

    #[test]
    fn peak_hold_does_not_overwrite_when_equal_to_bar() {
        // current bar 0.75, peak hold 0.75 -> the peak row is r = 0.75 * 12 = 9 (from bottom),
        // -> y = 12 - 1 - 9 = 2.
        let terminal = render(&[0.75], &[0.75], 4, 12);
        assert_eq!(at(&terminal, 0, 2), PEAK_DOT);
        assert_eq!(at(&terminal, 0, 3), '█'); // just below the peak is a full bar
    }

    #[test]
    fn fits_too_many_bars_truncates() {
        let terminal = render(&[1.0; 12], &[0.0; 12], 6, 4);
        // width 6 -> cols 0,2,4 fit (3 bars); bar 3 at col 6 == width -> truncated.
        // Check every in-bounds column (0..=5) is either a full bar or a gap.
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
