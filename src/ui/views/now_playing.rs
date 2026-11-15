//! Экран 01 (Now Playing): крупная квадратная обложка (2:1), метаданные трека
//! и переключаемая панель синхронного текста песни `LYRICS [Y]` с переносом строк.
//! Прогресс-бар, кнопки управления и спектр рендерятся глобально внизу окна (`App::render`).

#![allow(dead_code)]

use ratatui::prelude::*;

use crate::api::models::LyricLine;
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
    pub lyrics: Vec<LyricLine>,
    pub position_ms: u64,
    pub show_lyrics: bool,
}

impl Default for NowPlaying {
    fn default() -> Self {
        Self::default_screen()
    }
}

impl NowPlaying {
    pub fn default_screen() -> Self {
        let mut spectrum = Spectrum::new(64);
        let demo: Vec<f32> = (0..64)
            .map(|i| {
                let x = i as f32 / 64.0;
                ((x * std::f32::consts::PI * 3.0).sin().abs() * 0.75 + 0.1).clamp(0.0, 1.0)
            })
            .collect();
        spectrum.set_bars(&demo);
        Self {
            badge: "FLAC | 1411 kbps".into(),
            track: "Press [Enter] on any track in [2]Search or [3]Library".into(),
            artist: "Deezer TUI (Guest Preview & HiFi ARL ready)".into(),
            progress: 0.0,
            paused: true,
            cover: Some(CoverArt::solid(0x4F, 0xD1, 0xC5)),
            spectrum,
            lyrics: Vec::new(),
            position_ms: 0,
            show_lyrics: true,
        }
    }

    pub fn active_lyric_index(&self) -> usize {
        let mut idx = 0;
        for (i, line) in self.lyrics.iter().enumerate() {
            if line.time_ms <= self.position_ms {
                idx = i;
            } else {
                break;
            }
        }
        idx
    }

    fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
        if max_width == 0 {
            return Vec::new();
        }
        let mut lines = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let word_len = word.chars().count();
            let cur_len = current.chars().count();
            if cur_len == 0 {
                if word_len <= max_width {
                    current.push_str(word);
                } else {
                    let chars: Vec<char> = word.chars().collect();
                    for ch_slice in chars.chunks(max_width) {
                        lines.push(ch_slice.iter().collect());
                    }
                }
            } else if cur_len + 1 + word_len <= max_width {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(std::mem::take(&mut current));
                if word_len <= max_width {
                    current.push_str(word);
                } else {
                    let chars: Vec<char> = word.chars().collect();
                    for ch_slice in chars.chunks(max_width) {
                        lines.push(ch_slice.iter().collect());
                    }
                }
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        if lines.is_empty() {
            lines.push(String::new());
        }
        lines
    }

    fn draw_left_panel(&self, buf: &mut Buffer, area: Rect) {
        if area.width < 4 || area.height < 2 {
            return;
        }
        let header_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        let title_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            .bold();
        let artist_style = Style::default().fg(theme::color_from_hex(theme::GRAD_START));
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));

        buf.set_string(
            area.x,
            area.y,
            "NOW PLAYING".chars().take(area.width as usize).collect::<String>(),
            header_style,
        );

        let body_y = area.y + 1;
        let body_h = area.height.saturating_sub(1);
        if body_h == 0 {
            return;
        }

        // Вычисляем пропорциональный размер квадратной обложки (2x1 символа на пиксель)
        let max_cover_h = body_h.saturating_sub(6).clamp(4, 18);
        let max_cover_w = (area.width.saturating_sub(2) / 2).clamp(2, 18);
        let cover_h = max_cover_h.min(max_cover_w);
        let cover_w = (cover_h * 2).min(area.width);

        let mut cursor_y = body_y;
        if cover_h >= 2 && area.height >= 6 {
            if let Some(ref cover_art) = self.cover {
                let cover_rect = Rect::new(area.x, cursor_y, cover_w, cover_h);
                cover_art.render(cover_rect, buf);
            }
            cursor_y += cover_h + 1;
        } else if let Some(ref cover_art) = self.cover {
            let ch = body_h.clamp(1, 4);
            let cw = (ch * 2).min(area.width).max(1);
            cover_art.render(Rect::new(area.x, area.y, cw, ch), buf);
            cursor_y = area.y + ch;
        }

        let max_w = area.width.saturating_sub(1) as usize;
        if max_w == 0 {
            return;
        }

        // Название трека (с переносом на 2 строки, если длинное)
        for line in Self::wrap_text(&self.track, max_w).into_iter().take(2) {
            if cursor_y >= area.bottom() {
                break;
            }
            buf.set_string(area.x, cursor_y, &line, title_style);
            cursor_y += 1;
        }

        // Исполнитель и альбом
        for line in Self::wrap_text(&self.artist, max_w).into_iter().take(2) {
            if cursor_y >= area.bottom() {
                break;
            }
            buf.set_string(area.x, cursor_y, &line, artist_style);
            cursor_y += 1;
        }

        if cursor_y < area.bottom() {
            let badge_line: String = self.badge.chars().take(max_w).collect();
            buf.set_string(area.x, cursor_y, &badge_line, dim_style);
            cursor_y += 2;
        }

        if cursor_y < area.bottom() {
            let lyr_state = if self.show_lyrics { "ON" } else { "OFF" };
            let hints = format!("[y] Lyrics: {lyr_state}   [i] Track Info   [v] Spectrum");
            let truncated: String = hints.chars().take(max_w).collect();
            buf.set_string(area.x, cursor_y, &truncated, dim_style);
        }
    }

    fn draw_lyrics_panel(&self, buf: &mut Buffer, area: Rect) {
        if area.width < 6 || area.height < 3 {
            return;
        }
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let header_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();

        // Рисуем вертикальный разделитель слева от панели лирики
        for ry in area.top()..area.bottom() {
            buf.set_string(area.x, ry, "│", dim_style);
        }

        let text_x = area.x + 2;
        let text_w = area.width.saturating_sub(2) as usize;
        if text_w == 0 {
            return;
        }

        let hdr: String = "LYRICS [Y to Hide]".chars().take(text_w).collect();
        buf.set_string(text_x, area.y, &hdr, header_style);

        let list_start = area.y + 1;
        let visible_rows = area.bottom().saturating_sub(list_start) as usize;
        if visible_rows == 0 {
            return;
        }

        if self.lyrics.is_empty() {
            let msg = "No synchronized lyrics for this track (press [y] to hide panel)";
            for (idx, line) in Self::wrap_text(msg, text_w).into_iter().enumerate() {
                let y = list_start + idx as u16;
                if y >= area.bottom() {
                    break;
                }
                buf.set_string(text_x, y, &line, dim_style);
            }
            return;
        }

        let active = self.active_lyric_index();
        let scroll_offset = if active >= visible_rows / 2 {
            active.saturating_sub(visible_rows / 3)
        } else {
            0
        };

        let mut y = list_start;
        for (i, lyric) in self.lyrics.iter().enumerate().skip(scroll_offset) {
            if y >= area.bottom() {
                break;
            }
            let style = if i == active {
                Style::default()
                    .fg(theme::color_from_hex(theme::BG_TERMINAL))
                    .bg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else if i < active {
                dim_style
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };

            for wrapped in Self::wrap_text(&lyric.line, text_w) {
                if y >= area.bottom() {
                    break;
                }
                let text = if i == active {
                    let pad = text_w.saturating_sub(wrapped.chars().count());
                    format!("{wrapped}{}", " ".repeat(pad))
                } else {
                    wrapped
                };
                buf.set_string(text_x, y, &text, style);
                y += 1;
            }
        }
    }
}

impl Widget for &NowPlaying {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        if self.show_lyrics && area.width >= 56 {
            let left_w = (area.width * 45 / 100).clamp(26, area.width.saturating_sub(20));
            let left_area = Rect::new(area.x, area.y, left_w, area.height);
            let right_area = Rect::new(
                area.x + left_w,
                area.y,
                area.width - left_w,
                area.height,
            );
            self.draw_left_panel(buf, left_area);
            self.draw_lyrics_panel(buf, right_area);
        } else {
            self.draw_left_panel(buf, area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use ratatui::backend::TestBackend;

    fn make_screen() -> NowPlaying {
        let mut np = NowPlaying::default_screen();
        np.badge = "FLAC | 1411 kbps".into();
        np.track = "Bohemian Rhapsody".into();
        np.artist = "Queen".into();
        np.paused = false;
        np.cover = Some(CoverArt::from_image(RgbaImage::new(2, 2)));
        np.lyrics = vec![
            LyricLine {
                time_ms: 0,
                line: "Is this the real life?".into(),
            },
            LyricLine {
                time_ms: 4000,
                line: "Is this just fantasy?".into(),
            },
        ];
        np
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

    fn screen_has(terminal: &Terminal<TestBackend>, needle: &str) -> bool {
        let size = terminal.size().unwrap();
        for y in 0..size.height {
            let row: String = (0..size.width).map(|x| at(terminal, x, y)).collect();
            if row.contains(needle) {
                return true;
            }
        }
        false
    }

    #[test]
    fn renders_cover_metadata_and_lyrics_in_normal_size() {
        let screen = make_screen();
        let terminal = render(&screen, 120, 32);
        assert_eq!(at(&terminal, 0, 1), '▀');
        assert!(screen_has(&terminal, "Bohemian Rhapsody"));
        assert!(screen_has(&terminal, "Queen"));
        assert!(screen_has(&terminal, "FLAC | 1411 kbps"));
        assert!(screen_has(&terminal, "LYRICS [Y to Hide]"));
        assert!(screen_has(&terminal, "Is this the real life?"));
    }

    #[test]
    fn hides_lyrics_panel_when_toggled_off() {
        let mut screen = make_screen();
        screen.show_lyrics = false;
        let terminal = render(&screen, 120, 32);
        assert!(screen_has(&terminal, "Bohemian Rhapsody"));
        assert!(!screen_has(&terminal, "LYRICS [Y to Hide]"));
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_screen();
        let terminal = render(&screen, 40, 12);
        assert!(screen_has(&terminal, "NOW PLAYING"));
    }
}

