//! Экран 04 (Flow & Live Lyrics): квадратная обложка 2:1, управление Flow,
//! синхронная прокрутка `LYRICS` и полноширинный спектр снизу.

use ratatui::prelude::*;

use crate::api::models::LyricLine;
use crate::ui::theme;
use crate::ui::widgets::{CoverArt, Spectrum};

#[derive(Debug, Clone, Default)]
pub struct FlowRow {
    pub title: String,
    pub artist: String,
}

pub struct FlowLyrics {
    pub flow: Vec<FlowRow>,
    pub selected_flow: usize,
    pub lyrics: Vec<LyricLine>,
    pub position_ms: u64,
    pub paused: bool,
    pub spectrum: Spectrum,
    pub cover: Option<CoverArt>,
}

impl Default for FlowLyrics {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowLyrics {
    pub fn new() -> Self {
        let mut spectrum = Spectrum::new(64);
        let demo: Vec<f32> = (0..64)
            .map(|i| {
                let x = i as f32 / 64.0;
                ((x * std::f32::consts::PI * 4.0).sin().abs() * 0.7 + 0.15).clamp(0.0, 1.0)
            })
            .collect();
        spectrum.set_bars(&demo);
        Self {
            flow: vec![
                FlowRow {
                    title: String::from("Bohemian Rhapsody"),
                    artist: String::from("Queen"),
                },
                FlowRow {
                    title: String::from("Under Pressure"),
                    artist: String::from("Queen"),
                },
                FlowRow {
                    title: String::from("Somebody to Love"),
                    artist: String::from("Queen"),
                },
            ],
            selected_flow: 0,
            lyrics: vec![
                LyricLine {
                    time_ms: 0,
                    line: String::from("Is this the real life?"),
                },
                LyricLine {
                    time_ms: 4000,
                    line: String::from("Is this just fantasy?"),
                },
                LyricLine {
                    time_ms: 8000,
                    line: String::from("Caught in a landslide"),
                },
                LyricLine {
                    time_ms: 12000,
                    line: String::from("No exit from the desert of the real"),
                },
            ],
            position_ms: 9000,
            paused: false,
            spectrum,
            cover: Some(CoverArt::solid(0x4F, 0xD1, 0xC5)),
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
}

impl Widget for &FlowLyrics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 3 || h < 3 {
            return;
        }
        let spec_h = (h / 4).max(4).min(h.saturating_sub(2));
        let body_h = h.saturating_sub(spec_h);
        let body_end_y = area.y + body_h;

        if body_h > 0 && spec_h > 0 {
            (&self.spectrum).render(Rect::new(area.x, body_end_y, w, spec_h), buf);
        }

        if h >= 2 {
            let style = Style::default()
                .fg(theme::color_from_hex(theme::GRAD_END))
                .bold();
            buf.set_string(area.x, area.y, "FLOW", style);
        }

        if body_h >= 3 {
            let half = w / 2;
            let left_x = area.x;
            let right_x = left_x + half;
            self.draw_flow_panel(buf, area, left_x, half, body_end_y);
            if right_x < area.right() {
                self.draw_lyrics_panel(buf, area, right_x, w - half, body_end_y);
            }
        }
    }
}

impl FlowLyrics {
    fn draw_header(buf: &mut Buffer, x: u16, y: u16, text: &str) {
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bold();
        buf.set_string(x, y, text, style);
    }

    fn draw_flow_panel(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        width: u16,
        row_end: u16,
    ) {
        if x >= area.right() || row_end <= area.y + 1 {
            return;
        }
        let y_start = area.y + 1;
        let body_rows = row_end.saturating_sub(y_start);
        let cover_h = body_rows.saturating_sub(4).min(width / 2).min(12);
        let cover_w = (cover_h * 2).min(width);
        let mut y_cursor = y_start;

        if cover_h >= 2 {
            if let Some(cover) = &self.cover {
                cover.render(Rect::new(x, y_cursor, cover_w, cover_h), buf);
            }
            y_cursor += cover_h;
        }

        let y_ctrl = y_cursor;
        if y_ctrl < row_end {
            let play = if self.paused { "▶" } else { "❚❚" };
            let label = format!("⏮  {play}  ⏭");
            let label_w = label.chars().count();
            let w = cover_w.max(width.min(24)) as usize;
            let cx = if label_w < w {
                x + ((w - label_w) / 2) as u16
            } else {
                x
            };
            let style = Style::default()
                .fg(theme::color_from_hex(theme::GRAD_END))
                .bold();
            buf.set_string(cx, y_ctrl, Self::truncate(&label, width as usize), style);
        }

        let y_head = y_ctrl + 1;
        if y_head < row_end {
            Self::draw_header(buf, x, y_head, "DEEZER FLOW");
        }
        for (i, row) in self.flow.iter().enumerate() {
            let y = y_head + 1 + i as u16;
            if y >= row_end || y >= area.bottom() {
                break;
            }
            let selected = self.selected_flow == i;
            let label = format!(
                "{} {} — {}",
                if selected { "▶" } else { " " },
                row.title,
                row.artist,
            );
            let style = if selected {
                Style::default()
                    .fg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            let text = Self::truncate(&label, width as usize);
            buf.set_string(x, y, &text, style);
        }
    }

    fn draw_lyrics_panel(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        width: u16,
        row_end: u16,
    ) {
        if x >= area.right() || row_end <= area.y + 1 {
            return;
        }
        let y0 = area.y + 1;
        Self::draw_header(buf, x, y0, "LYRICS");

        let list_start = y0 + 1;
        let visible_rows = row_end.saturating_sub(list_start) as usize;
        if visible_rows == 0 || self.lyrics.is_empty() {
            return;
        }

        let active = self.active_lyric_index();
        let scroll_offset = if active >= visible_rows {
            active - visible_rows / 2
        } else {
            0
        };

        for (row_idx, (i, line)) in self
            .lyrics
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_rows)
            .enumerate()
        {
            let y = list_start + row_idx as u16;
            if y >= row_end || y >= area.bottom() {
                break;
            }
            let style = if i == active {
                Style::default()
                    .fg(theme::color_from_hex(theme::BG_TERMINAL))
                    .bg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else if i < active {
                Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            let truncated = Self::truncate(&line.line, width as usize);
            let text = if i == active {
                let pad = (width as usize).saturating_sub(truncated.chars().count());
                truncated + &" ".repeat(pad)
            } else {
                truncated
            };
            buf.set_string(x, y, &text, style);
        }
    }

    fn truncate(text: &str, max_width: usize) -> String {
        text.chars().take(max_width).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::style::Color;
    use ratatui::Terminal;

    fn render(screen: &FlowLyrics, width: u16, height: u16) -> Terminal<TestBackend> {
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

    fn row_has(terminal: &Terminal<TestBackend>, y: u16, needle: &str) -> bool {
        let width = terminal.size().unwrap().width;
        let row: String = (0..width).map(|x| at(terminal, x, y)).collect();
        row.contains(needle)
    }

    #[test]
    fn renders_in_normal_size() {
        let screen = FlowLyrics::new();
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "FLOW"));
        assert!(row_has(&terminal, 1, "LYRICS"));
        assert_eq!(at(&terminal, 0, 1), '▀');
        assert!((1..30).any(|y| row_has(&terminal, y, "DEEZER FLOW")));
        assert!((1..30).any(|y| row_has(&terminal, y, "Bohemian Rhapsody")));
        assert!(row_has(&terminal, 2, "Is this the real life?"));
        let has_spectrum =
            (30..40).any(|y| (0..120).any(|x| !at(&terminal, x, y).is_whitespace()));
        assert!(has_spectrum);
    }

    #[test]
    fn active_lyric_line_has_turquoise_background() {
        let screen = FlowLyrics::new();
        let terminal = render(&screen, 120, 40);
        let size = terminal.size().unwrap();
        let buf = terminal.backend().buffer();
        let mut found = false;
        for y in 0..size.height {
            if row_has(&terminal, y, "Caught in a landslide") {
                let cell = buf.cell((size.width / 2 + 1, y)).expect("cell in bounds");
                assert_eq!(cell.style().bg, Some(Color::Rgb(0x4F, 0xD1, 0xC5)));
                found = true;
                break;
            }
        }
        assert!(found);
    }
    #[test]
    fn renders_in_narrow_size() {
        let screen = FlowLyrics::new();
        let terminal = render(&screen, 40, 12);
        assert!(row_has(&terminal, 0, "FLOW"));
        assert!((1..12).any(|y| row_has(&terminal, y, "DEEZER FLOW")));
        assert!((2..12).any(|y| row_has(&terminal, y, "this the real")));
    }

    #[test]
    fn active_lyric_tracks_position() {
        let mut screen = FlowLyrics::new();
        screen.position_ms = 0;
        assert_eq!(screen.active_lyric_index(), 0);
        screen.position_ms = 8500;
        assert_eq!(screen.active_lyric_index(), 2);
        screen.position_ms = 999_999;
        assert_eq!(screen.active_lyric_index(), 3);
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let screen = FlowLyrics::new();
        let terminal = render(&screen, 15, 8);
        assert!(row_has(&terminal, 0, "FLOW"));
    }
}
