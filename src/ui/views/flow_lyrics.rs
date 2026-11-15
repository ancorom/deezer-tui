//! Flow & Lyrics screen (Экран 04): the Deezer Flow track list on the left,
//! synced LYRICS with an active-line highlight on the right, and a live audio
//! spectrum along the bottom.

use ratatui::prelude::*;

use crate::api::models::LyricLine;
use crate::ui::theme;
use crate::ui::widgets::{CoverArt, Spectrum};

/// One entry of the Deezer Flow list.
#[derive(Debug, Default)]
pub struct FlowRow {
    pub title: String,
    pub artist: String,
}

/// Flow & Lyrics screen (Экран 04).
pub struct FlowLyrics {
    /// Deezer Flow track list.
    pub flow: Vec<FlowRow>,
    /// Highlighted flow entry.
    pub selected_flow: usize,
    /// Synced lyric lines (ascending `time_ms`).
    pub lyrics: Vec<LyricLine>,
    /// Current playback position in milliseconds (drives the active line).
    pub position_ms: u64,
    /// `true` when playback is paused.
    pub paused: bool,
    /// Audio spectrum rendered along the bottom.
    pub spectrum: Spectrum,
    /// Cover of the current track (left panel of Экран 04).
    pub cover: Option<CoverArt>,
}

impl Default for FlowLyrics {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowLyrics {
    /// A default screen with demo data, used before real Flow/Lyrics data
    /// arrives.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut spectrum = Spectrum::new(4);
        spectrum.set_bars(&[0.6, 0.8, 0.5, 0.9]);
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
            // Placeholder cover (solid turquoise) until a real JPEG arrives.
            cover: Some(CoverArt::solid(0x4F, 0xD1, 0xC5)),
        }
    }

    /// Index of the currently active lyric line (the last line whose
    /// `time_ms` is `<= position_ms`).
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
        // Bottom spectrum (at least 4 rows when the height allows).
        let spec_h = (h / 5).max(4).min(h.saturating_sub(2));
        let body_h = h.saturating_sub(spec_h);
        if body_h > 0 {
            let spec_top = body_h;
            (&self.spectrum).render(Rect::new(0, spec_top, w, spec_h), buf);
        }

        // Title row.
        if h >= 2 {
            let style = Style::default().fg(theme::color_from_hex(theme::GRAD_END)).bold();
            buf.set_string(area.x, area.y, "FLOW", style);
        }

        // Two side-by-side panels: Flow list (left) and Lyrics (right).
        if body_h >= 3 {
            let half = w / 2;
            let left_x = area.x;
            let right_x = left_x + half;
            let panel_h = body_h - 1;
            self.draw_flow_panel(buf, area, left_x, half, panel_h, body_h);
            if right_x < area.right() {
                self.draw_lyrics_panel(buf, area, right_x, w - half, panel_h, body_h);
            }
        }
    }
}

impl FlowLyrics {
    /// Panel header (dim, secondary color).
    fn draw_header(buf: &mut Buffer, x: u16, y: u16, text: &str) {
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bold();
        buf.set_string(x, y, text, style);
    }

    /// Deezer Flow panel (Экран 04, left side): the current track's cover on
    /// top, a transport row (`⏮ ▶/❚❚ ⏭`) beneath it, then the DEEZER FLOW
    /// list — one entry per row, the selected one highlighted.
    fn draw_flow_panel(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        width: u16,
        _panel_h: u16,
        row_end: u16,
    ) {
        if x >= area.right() || row_end <= 1 {
            return;
        }
        let y_start = area.y + 1;
        let body_rows = row_end.saturating_sub(y_start);
        // Cover: a square (at most 16 cells) that leaves ≥ 3 rows for the
        // transport row, the header and at least one flow entry.
        let cover_side = width.min(body_rows.saturating_sub(3)).min(16);
        let mut y_cursor = y_start;
        if cover_side >= 2 {
            if let Some(cover) = &self.cover {
                cover.render(Rect::new(x, y_cursor, cover_side, cover_side), buf);
            }
            y_cursor += cover_side;
        }
        // Transport controls row (⏮ ▶/❚❚ ⏭), centered in the panel width.
        let y_ctrl = y_cursor;
        if y_ctrl < row_end {
            let play = if self.paused { "▶" } else { "❚❚" };
            let label = format!("⏮  {}  ⏭", play);
            let label_w = label.chars().count();
            let w = width as usize;
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
        // DEEZER FLOW header + list.
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
                Style::default().fg(theme::color_from_hex(theme::GRAD_START)).bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            let text = Self::truncate(&label, width as usize);
            buf.set_string(x, y, &text, style);
        }
    }

    /// Lyrics panel: header + synced lines; the active line is highlighted,
    /// earlier lines are dimmed, later lines stay normal.
    fn draw_lyrics_panel(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        width: u16,
        _panel_h: u16,
        row_end: u16,
    ) {
        if x >= area.right() || row_end <= 1 {
            return;
        }
        let y0 = area.y + 1;
        Self::draw_header(buf, x, y0, "LYRICS");
        let active = self.active_lyric_index();
        for (i, line) in self.lyrics.iter().enumerate() {
            let y = y0 + 1 + i as u16;
            if y >= row_end || y >= area.bottom() {
                break;
            }
            // Active line: turquoise (#4FD1C5) background with dark text,
            // padded across the whole panel width.
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
                let pad = width as usize - truncated.chars().count();
                truncated + &" ".repeat(pad)
            } else {
                truncated
            };
            buf.set_string(x, y, &text, style);
        }
    }

    /// Truncate `text` to `max_width` cells.
    fn truncate(text: &str, max_width: usize) -> String {
        if max_width == 0 {
            return String::new();
        }
        text.chars().take(max_width).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::style::Color;
    use ratatui::Terminal;

    fn make_screen() -> FlowLyrics {
        FlowLyrics::new()
    }

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
        let row: String = (0..width)
            .map(|x| at(terminal, x, y))
            .collect();
        row.contains(needle)
    }

    #[test]
    fn renders_in_normal_size() {
        let screen = make_screen();
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "FLOW"));
        assert!(row_has(&terminal, 1, "LYRICS"));
        // Cover art (top-left) renders half-blocks.
        assert_eq!(at(&terminal, 0, 1), '▀');
        // DEEZER FLOW header + flow list below the cover and transport row.
        let flow_header = (1..30).any(|y| row_has(&terminal, y, "DEEZER FLOW"));
        assert!(flow_header);
        assert!((1..30).any(|y| row_has(&terminal, y, "Bohemian Rhapsody")));
        assert!((1..30).any(|y| row_has(&terminal, y, "Under Pressure")));
        // Lyric lines (rows 2+ in the right half).
        assert!(row_has(&terminal, 2, "Is this the real life?"));
        assert!(row_has(&terminal, 3, "Is this just fantasy?"));
        // Spectrum occupies the bottom rows (non-whitespace content).
        let has_spectrum = (30..40)
            .any(|y| (0..120).any(|x| !at(&terminal, x, y).is_whitespace()));
        assert!(has_spectrum);
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_screen();
        let terminal = render(&screen, 40, 12);
        assert!(row_has(&terminal, 0, "FLOW"));
        // Cover + transport + DEEZER FLOW header fit in the left half.
        let flow_header = (1..12).any(|y| row_has(&terminal, y, "DEEZER FLOW"));
        assert!(flow_header);
        // Lyric lines still render in the right half.
        let any_lyric = (2..12).any(|y| row_has(&terminal, y, "this the real"));
        assert!(any_lyric);
    }

    #[test]
    fn active_lyric_line_has_turquoise_background() {
        let screen = make_screen();
        let terminal = render(&screen, 120, 40);
        let size = terminal.size().unwrap();
        let buf = terminal.backend().buffer();
        // The active line is "Caught in a landslide" (position_ms = 9000);
        // find its row in the right half and check the background colour.
        let mut found = false;
        for y in 0..size.height {
            let row: String = (0..size.width)
                .map(|x| at(&terminal, x, y))
                .collect();
            if row.contains("Caught in a landslide") {
                let cell = buf.cell((size.width / 2 + 1, y)).expect("cell in bounds");
                assert_eq!(cell.style().bg, Some(Color::Rgb(0x4F, 0xD1, 0xC5)));
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn active_lyric_tracks_position() {
        let mut screen = make_screen();
        screen.position_ms = 0;
        assert_eq!(screen.active_lyric_index(), 0);
        screen.position_ms = 8500;
        assert_eq!(screen.active_lyric_index(), 2);
        screen.position_ms = 999_999;
        assert_eq!(screen.active_lyric_index(), 3);
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let screen = make_screen();
        let terminal = render(&screen, 15, 8);
        assert!(row_has(&terminal, 0, "FLOW"));
    }
}
