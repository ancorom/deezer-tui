//! Экран 04 (Flow & Live Lyrics): бесконечный поток рекомендаций `DEEZER FLOW` слева
//! и отключаемая панель синхронного текста песни `LYRICS [Y]` справа с защитой от наложения текста.

#![allow(dead_code)]

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
    pub show_lyrics: bool,
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
            show_lyrics: true,
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
}

impl Widget for &FlowLyrics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 3 || h < 2 {
            return;
        }

        let title_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        let lyr_state = if self.show_lyrics { "ON" } else { "OFF" };
        let header = format!("FLOW • Infinite Recommendations  [y: Lyrics {lyr_state}]");
        buf.set_string(area.x, area.y, FlowLyrics::truncate(&header, w as usize), title_style);

        if h < 3 {
            return;
        }

        let row_end = area.bottom();
        if self.show_lyrics && w >= 52 {
            let left_w = (w / 2).clamp(24, w.saturating_sub(20));
            let right_x = area.x + left_w;
            let right_w = w - left_w;
            self.draw_flow_panel(buf, area, area.x, left_w.saturating_sub(1), row_end);
            self.draw_lyrics_panel(buf, area, right_x, right_w, row_end);
        } else {
            self.draw_flow_panel(buf, area, area.x, w, row_end);
        }
    }
}

impl FlowLyrics {
    fn draw_header(buf: &mut Buffer, x: u16, y: u16, text: &str, max_w: usize) {
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bold();
        buf.set_string(x, y, Self::truncate(text, max_w), style);
    }

    fn draw_flow_panel(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        width: u16,
        row_end: u16,
    ) {
        if x >= area.right() || width == 0 || row_end <= area.y + 1 {
            return;
        }
        let y_start = area.y + 1;
        let body_rows = row_end.saturating_sub(y_start);
        let cover_h = body_rows.saturating_sub(6).min(width / 2).min(10);
        let cover_w = (cover_h * 2).min(width);
        let mut y_cursor = y_start;

        if cover_h >= 2 {
            if let Some(cover) = &self.cover {
                cover.render(Rect::new(x, y_cursor, cover_w, cover_h), buf);
            }
            y_cursor += cover_h;
        }

        if y_cursor < row_end {
            let queue_hdr = format!("DEEZER FLOW ({} tracks queued)", self.flow.len());
            Self::draw_header(buf, x, y_cursor, &queue_hdr, width as usize);
            y_cursor += 1;
        }

        let avail_rows = row_end.saturating_sub(y_cursor) as usize;
        if avail_rows == 0 || self.flow.is_empty() {
            return;
        }

        let scroll_offset = if self.selected_flow >= avail_rows / 2 {
            self.selected_flow.saturating_sub(avail_rows / 3)
        } else {
            0
        };

        let max_w = width as usize;
        for (i, row) in self.flow.iter().enumerate().skip(scroll_offset) {
            if y_cursor >= row_end || y_cursor >= area.bottom() {
                break;
            }
            let selected = self.selected_flow == i;
            let prefix = if selected { "▶ " } else { "  " };
            let full_label = format!("{}{} — {}", prefix, row.title, row.artist);
            let style = if selected {
                Style::default()
                    .fg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };

            // Переносим длинные названия до 2 строк строго внутри ширины левой колонки
            let wrapped = Self::wrap_text(&full_label, max_w);
            for (line_idx, wline) in wrapped.into_iter().take(2).enumerate() {
                if y_cursor >= row_end || y_cursor >= area.bottom() {
                    break;
                }
                let display_line = if line_idx > 0 {
                    Self::truncate(&format!("    {wline}"), max_w)
                } else {
                    Self::truncate(&wline, max_w)
                };
                buf.set_string(x, y_cursor, &display_line, style);
                y_cursor += 1;
            }
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
        if x >= area.right() || width < 4 || row_end <= area.y + 1 {
            return;
        }
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));

        // Вертикальная граница между списком Flow и текстом лирики
        for ry in (area.y + 1)..row_end {
            buf.set_string(x, ry, "│", dim_style);
        }

        let text_x = x + 2;
        let text_w = width.saturating_sub(2) as usize;
        if text_w == 0 {
            return;
        }

        let y0 = area.y + 1;
        Self::draw_header(buf, text_x, y0, "LYRICS [Y to Hide]", text_w);

        let list_start = y0 + 1;
        let visible_rows = row_end.saturating_sub(list_start) as usize;
        if visible_rows == 0 {
            return;
        }
        if self.lyrics.is_empty() {
            let empty_msg = "No synchronized lyrics available (Instrumental / Unsynced)";
            for (idx, wline) in Self::wrap_text(empty_msg, text_w).into_iter().enumerate() {
                let y = list_start + idx as u16;
                if y >= row_end || y >= area.bottom() {
                    break;
                }
                buf.set_string(text_x, y, &wline, dim_style);
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
        for (i, line) in self.lyrics.iter().enumerate().skip(scroll_offset) {
            if y >= row_end || y >= area.bottom() {
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

            for wrapped in Self::wrap_text(&line.line, text_w) {
                if y >= row_end || y >= area.bottom() {
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
    }

    #[test]
    fn long_track_title_never_overwrites_lyrics_column() {
        let mut screen = FlowLyrics::new();
        screen.flow[0].title =
            "Piano Sonata No. 14 in C-Sharp Minor Op. 27 No. 2 Moonlight I. Adagio sostenuto"
                .into();
        let terminal = render(&screen, 80, 24);
        let sep_x = 40;
        for y in 2..20 {
            assert_eq!(
                at(&terminal, sep_x, y),
                '│',
                "разделитель колонок не должен затираться длинным названием трека"
            );
        }
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
                let cell = buf.cell((size.width / 2 + 3, y)).expect("cell in bounds");
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

