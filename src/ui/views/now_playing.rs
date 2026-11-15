//! Экран 01 (Now Playing): обложка альбома, бейдж `FLAC | 1411 kbps`, название трека,
//! артист, прогресс-бар с ползунком `●`, кнопки `[PAUSE] ⏮ ▶ ⏭ [FLOW]` и полноширинный спектр.

use ratatui::prelude::*;

use crate::ui::theme;
use crate::ui::widgets::{CoverArt, Spectrum};

pub struct NowPlaying {
    pub badge: String,
    pub track: String,
    pub artist: String,
    pub progress: f32,
    pub paused: bool,
    pub cover: Option<CoverArt>,
    pub spectrum: Spectrum,
}

impl Default for NowPlaying {
    fn default() -> Self {
        Self::default_screen()
    }
}

impl NowPlaying {
    pub fn default_screen() -> Self {
        let mut spectrum = Spectrum::new(32);
        spectrum.set_bars(&[
            0.2, 0.35, 0.5, 0.7, 0.85, 0.6, 0.45, 0.75, 0.9, 0.65, 0.4, 0.55, 0.8, 0.5, 0.3, 0.45,
            0.6, 0.75, 0.5, 0.35, 0.65, 0.85, 0.7, 0.5, 0.4, 0.6, 0.55, 0.35, 0.25, 0.4, 0.3, 0.2,
        ]);
        Self {
            badge: "FLAC | 1411 kbps".into(),
            track: "No track loaded".into(),
            artist: "Deezer".into(),
            progress: 0.0,
            paused: true,
            cover: Some(CoverArt::solid(0x4F, 0xD1, 0xC5)),
            spectrum,
        }
    }

    fn cursor_col(width: u16, progress: f32) -> u16 {
        let max = width.saturating_sub(1);
        (progress.clamp(0.0, 1.0) * (max as f32)) as u16
    }

    fn draw_progress(&self, buf: &mut Buffer, area: Rect, row: u16, width: u16) {
        if row >= area.height || width == 0 {
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

    fn draw_controls(&self, buf: &mut Buffer, area: Rect, row: u16, width: u16) {
        if row >= area.height || width == 0 {
            return;
        }
        let y = area.y + row;
        let x0 = area.x;
        let play = if self.paused { "▶" } else { "❚❚" };
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let accent = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_START))
            .bold();

        if width >= 32 {
            let line = format!("[PAUSE]   ⏮   {play}   ⏭   [FLOW]");
            let line_chars = line.chars().count() as u16;
            let start_x = x0 + width.saturating_sub(line_chars) / 2;
            buf.set_string(start_x, y, &line, accent);
        } else {
            let q1 = width / 4;
            let q2 = width / 2;
            let q3 = 3 * width / 4;
            buf.set_string(x0 + q1, y, "⏮", style);
            buf.set_string(x0 + q2, y, play, style);
            buf.set_string(x0 + q3, y, "⏭", style);
        }
    }

    fn draw_cover_and_meta(&self, buf: &mut Buffer, area: Rect, meta_h: u16, width: u16) {
        let cover_side = meta_h.min(width.saturating_sub(12)).clamp(1, 24);
        if cover_side > 0 && width > cover_side {
            let cover_rect = Rect::new(area.x, area.y, cover_side, cover_side);
            if let Some(ref cover_art) = self.cover {
                cover_art.render(cover_rect, buf);
            }
        }
        let rel_x = (cover_side + 2).min(width);
        let meta_x = area.x + rel_x;
        let meta_w = width.saturating_sub(rel_x);
        if meta_w > 0 {
            if meta_h >= 1 {
                self.draw_meta_line(buf, meta_x, area.y, meta_w, &self.badge, true);
            }
            if meta_h >= 2 {
                self.draw_meta_line(buf, meta_x, area.y + 1, meta_w, &self.track, false);
            }
            if meta_h >= 3 {
                self.draw_meta_line(buf, meta_x, area.y + 2, meta_w, &self.artist, true);
            }
        }
    }

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
        let truncated: String = text.chars().take(width as usize).collect();
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
        let content_h = h / 2;
        if content_h == 0 {
            return;
        }
        let spec_h = h - content_h;

        if spec_h > 0 {
            let spec = Rect::new(area.x, area.y + content_h, w, spec_h);
            (&self.spectrum).render(spec, buf);
        }

        if content_h < 3 {
            self.draw_controls(buf, area, content_h - 1, w);
            return;
        }
        let controls_row = content_h - 1;
        let progress_row = content_h - 2;
        let meta_h = content_h - 2;
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
        assert_eq!(at(&terminal, 0, 0), '▀');
        let track_row: Vec<char> = (0..120).map(|x| at(&terminal, x, 1)).collect();
        assert!(track_row.contains(&'B'));
        let found_cursor = (0..120).any(|x| at(&terminal, x, 18) == '●');
        assert!(found_cursor);
        let has_spectrum =
            (20..40).any(|y| (0..120).any(|x| !at(&terminal, x, y).is_whitespace()));
        assert!(has_spectrum);
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_screen();
        let terminal = render(&screen, 40, 12);
        assert_eq!(at(&terminal, 0, 0), '▀');
        let has_spectrum =
            (6..12).any(|y| (0..40).any(|x| !at(&terminal, x, y).is_whitespace()));
        assert!(has_spectrum);
    }
}
