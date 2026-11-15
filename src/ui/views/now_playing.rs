use ratatui::prelude::*;

use crate::ui::theme;
use crate::ui::widgets::{CoverArt, Spectrum};

/// Now Playing screen (Экран 01).
///
/// The top half of the viewport holds the cover art (top-left, square), the
/// track metadata (badge / track / artist) to the right of the cover, and the
/// progress bar + transport controls on the bottom rows of that half. The
/// bottom half holds the full spectrum.
///
/// The layout is adaptive: on a short or narrow terminal the cover art shrinks
/// (it is always a square and is never larger than the content area) and the
/// text is truncated to the remaining width.
pub struct NowPlaying {
    /// "FLAC | 1411 kbps" style badge.
    pub badge: String,
    pub track: String,
    pub artist: String,
    /// 0.0 ..= 1.0 playback progress.
    pub progress: f32,
    /// `true` when playback is paused/stopped.
    pub paused: bool,
    /// Cover image to draw in the top-left (top half).
    pub cover: Option<CoverArt>,
    /// Audio spectrum rendered into the bottom half.
    pub spectrum: Spectrum,
}

impl Default for NowPlaying {
    fn default() -> Self {
        Self::default_screen()
    }
}

impl NowPlaying {
    /// A default screen, used before real audio data is available.
    pub fn default_screen() -> Self {
        Self {
            badge: "FLAC | 1411 kbps".into(),
            track: "No track loaded".into(),
            artist: "Deezer".into(),
            progress: 0.0,
            paused: true,
            cover: None,
            spectrum: Spectrum::new(0),
        }
    }

    /// Column of the `●` cursor for a bar of `width` cells.
    fn cursor_col(width: u16, progress: f32) -> u16 {
        let max = width.saturating_sub(1);
        (progress.clamp(0.0, 1.0) * (max as f32)) as u16
    }

    /// Draw the progress bar (background + filled portion + `●` cursor).
    fn draw_progress(&self, buf: &mut Buffer, area: Rect, row: u16, width: u16) {
        if row >= area.height {
            return;
        }
        let x0 = area.x;
        let y = area.y + row;
        let bar_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL));
        for x in 0..width {
            buf.set_string(x0 + x, y, "░", bar_style);
        }
        let col = Self::cursor_col(width, self.progress);
        let fill_style = Style::default().fg(theme::gradient_color(1.0));
        for x in 0..col {
            buf.set_string(x0 + x, y, "■", fill_style);
        }
        buf.set_string(x0 + col, y, "●", fill_style);
    }

    /// Draw the transport controls row (prev / play-pause / next).
    fn draw_controls(&self, buf: &mut Buffer, area: Rect, row: u16, width: u16) {
        if row >= area.height {
            return;
        }
        let y = area.y + row;
        let x0 = area.x;
        let play = if self.paused { "▶" } else { "❚❚" };
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let q1 = width / 4;
        let q2 = width / 2;
        let q3 = 3 * width / 4;
        buf.set_string(x0 + q1, y, "⏮", style);
        buf.set_string(x0 + q2, y, play, style);
        buf.set_string(x0 + q3, y, "⏭", style);
    }

    /// Draw the cover art (top-left square) and the track metadata to its right.
    fn draw_cover_and_meta(&self, buf: &mut Buffer, area: Rect, meta_h: u16, width: u16) {
        let cover_side = meta_h.min(width.saturating_sub(12)).clamp(1, 24);
        if cover_side > 0 && width > cover_side {
            let cover_rect = Rect::new(area.x, area.y, cover_side, cover_side);
            if let Some(ref cover_art) = self.cover {
                cover_art.render(cover_rect, buf);
            }
        }
        let meta_x = area.x + (cover_side + 2).min(width);
        if width > meta_x {
            let meta_w = width - meta_x;
            if meta_h >= 1 {
                self.draw_meta_line(buf, meta_x, area.y, meta_w, &self.badge, true);
            }
            if meta_h >= 2 {
                self.draw_meta_line(buf, meta_x, area.y + 1, meta_w, &self.track, false);
            }
            if meta_h >= 3 {
                self.draw_meta_line(buf, meta_x, area.y + 2, meta_w, &self.artist, false);
            }
        }
    }

    /// Draw a single metadata line (badge / track / artist), truncated to width.
    fn draw_meta_line(
        &self,
        buf: &mut Buffer,
        x0: u16,
        y: u16,
        width: u16,
        text: &str,
        is_dim: bool,
    ) {
        let style = Style::default().fg(theme::color_from_hex(if is_dim {
            theme::TEXT_SECONDARY
        } else {
            theme::TEXT_PRIMARY
        }));
        let mut truncated = String::new();
        for ch in text.chars() {
            if truncated.len() < width as usize {
                truncated.push(ch);
            }
            if truncated.len() >= width as usize {
                break;
            }
        }
        buf.set_string(x0, y, truncated.as_str(), style);
    }
}

impl Widget for &NowPlaying {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        // Split into a content half (top) and a spectrum half (bottom).
        let content_h = h / 2;
        if content_h == 0 {
            return;
        }
        let spec_h = h - content_h;

        // Spectrum: bottom half.
        if spec_h > 0 {
            let spec = Rect::new(0, content_h, w, spec_h);
            (&self.spectrum).render(spec, buf);
        }

        // Content half: cover + metadata + progress + controls.
        if content_h < 3 {
            if content_h >= 1 {
                self.draw_controls(buf, area, content_h - 1, w);
            }
            return;
        }
        let controls_row = content_h - 1;
        let progress_row = content_h - 2;
        let meta_h = content_h - 2; // rows 0..=meta_h-1
        if meta_h >= 1 {
            self.draw_cover_and_meta(buf, area, meta_h, w);
        }
        self.draw_progress(buf, area, progress_row, w);
        self.draw_controls(buf, area, controls_row, w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use ratatui::backend::TestBackend;

    fn make_screen() -> NowPlaying {
        let mut spectrum = Spectrum::new(3);
        spectrum.set_bars(&[1.0, 1.0, 1.0]);
        NowPlaying {
            badge: "FLAC | 1411 kbps".into(),
            track: "Bohemian Rhapsody".into(),
            artist: "Queen".into(),
            progress: 0.5,
            paused: false,
            cover: Some(CoverArt::from_image(RgbaImage::new(2, 2))),
            spectrum,
        }
    }

    fn render(screen: &NowPlaying, width: u16, height: u16) -> Terminal<TestBackend> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(screen, frame.area());
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
    fn renders_in_normal_size() {
        let screen = make_screen();
        let terminal = render(&screen, 120, 40);
        // Cover art area contains half-block glyphs.
        assert_eq!(at(&terminal, 0, 0), '▀');
        // Track text present in row 1 (first non-space is 'B').
        let track_row: Vec<char> = (0..120).map(|x| at(&terminal, x, 1)).collect();
        assert!(track_row.contains(&'B'));
        // Progress bar contains the `●` cursor (progress row = 18 for h=40).
        let mut found_cursor = false;
        for x in 0..120 {
            if at(&terminal, x, 18) == '●' {
                found_cursor = true;
                break;
            }
        }
        assert!(found_cursor);
        // Spectrum has content in the bottom half (rows 20..=39).
        let has_spectrum = (20..40)
            .any(|y| (0..120).any(|x| !at(&terminal, x, y).is_whitespace()));
        assert!(has_spectrum);
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_screen();
        let terminal = render(&screen, 40, 12);
        // Cover art still renders half-blocks.
        assert_eq!(at(&terminal, 0, 0), '▀');
        // Spectrum has content in the bottom half (rows 6..=11).
        let has_spectrum = (6..12)
            .any(|y| (0..40).any(|x| !at(&terminal, x, y).is_whitespace()));
        assert!(has_spectrum);
    }
}
