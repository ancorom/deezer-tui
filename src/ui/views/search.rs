//! Экран 02 (Search & Discover): модальное поле ввода `Search:` (`input_focused`)
//! и 3 колонки результатов (`ARTISTS`, `ALBUMS`, `TRACKS`).
//! Сайдбар `NAVIGATION`, нижний плеер и спектр в v2.0 рендерятся глобально в `App::render`.

#![allow(dead_code)]

use ratatui::prelude::*;

use crate::ui::theme;
use crate::ui::widgets::Spectrum;

#[derive(Debug, Clone, Default)]
pub struct SearchRow {
    pub artist: String,
    pub album: String,
    pub track: String,
    pub time: String,
    pub favorite: bool,
}

#[derive(Debug, Clone)]
pub struct MiniPlayer {
    pub track: String,
    pub artist: String,
    pub progress: f32,
    pub paused: bool,
}

impl Default for MiniPlayer {
    fn default() -> Self {
        Self {
            track: "No track loaded".into(),
            artist: "Deezer".into(),
            progress: 0.0,
            paused: true,
        }
    }
}

impl MiniPlayer {
    pub fn default_screen() -> Self {
        Self::default()
    }
}

pub struct Search {
    pub query: String,
    pub input_focused: bool,
    pub selected: usize,
    pub loading: bool,
    pub rows: Vec<SearchRow>,
    pub player: MiniPlayer,
    pub spectrum: Spectrum,
}

impl std::fmt::Debug for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Search")
            .field("query", &self.query)
            .field("input_focused", &self.input_focused)
            .field("selected", &self.selected)
            .field("loading", &self.loading)
            .field("rows", &self.rows)
            .field("player", &self.player)
            .field("spectrum_bars", &self.spectrum.num_bars())
            .finish()
    }
}

impl Default for Search {
    fn default() -> Self {
        Self::new()
    }
}

impl Search {
    pub fn new() -> Self {
        let mut spectrum = Spectrum::new(10);
        spectrum.set_bars(&[0.2, 0.45, 0.8, 1.0, 0.9, 0.6, 0.4, 0.65, 0.85, 0.3]);
        Self {
            query: String::new(),
            input_focused: false,
            selected: 0,
            loading: false,
            rows: Vec::new(),
            player: MiniPlayer::default(),
            spectrum,
        }
    }
}

impl Widget for &Search {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.draw_content(buf, area);
    }
}

impl Search {
    fn draw_content(&self, buf: &mut Buffer, area: Rect) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        let query_y = area.y;
        self.draw_query(buf, area, query_y);

        let table_start = area.y + 1;
        let table_end = area.bottom();
        if table_start < table_end {
            self.draw_table(buf, area, table_start, table_end, w);
        }
    }

    fn truncate(text: &str, max_width: usize) -> String {
        text.chars().take(max_width).collect()
    }

    fn draw_query(&self, buf: &mut Buffer, area: Rect, y: u16) {
        if y >= area.bottom() {
            return;
        }
        let x0 = area.x;
        let label_style = if self.input_focused {
            Style::default()
                .fg(theme::color_from_hex(theme::GRAD_START))
                .bold()
        } else {
            Style::default()
                .fg(theme::color_from_hex(theme::GRAD_END))
                .bold()
        };
        buf.set_string(
            x0,
            y,
            Self::truncate("Search:", area.width as usize),
            label_style,
        );

        let x = x0.saturating_add(8);
        if x >= area.right() {
            return;
        }
        let text_style = if self.input_focused {
            Style::default()
                .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
                .bold()
        } else {
            Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
        };
        let display = if self.loading {
            String::from("Loading…")
        } else if self.query.is_empty() {
            if self.input_focused {
                String::from("type a query…")
            } else {
                String::from("press [/] or [f] to type…")
            }
        } else {
            self.query.clone()
        };
        let available = (area.right() - x) as usize;
        let text = Self::truncate(&display, available);
        buf.set_string(x, y, text.as_str(), text_style);

        if self.input_focused || !self.query.is_empty() {
            let cursor_x = x.saturating_add(text.chars().count() as u16);
            if cursor_x < area.right() {
                let cursor_sym = if self.input_focused { "█" } else { "|" };
                buf.set_string(cursor_x, y, cursor_sym, label_style);
            }
        }

        let mode_hint = if self.input_focused {
            "[EDITING: Enter/Esc]"
        } else {
            "[/ or f: Edit]"
        };
        let hint_len = mode_hint.chars().count() as u16;
        if area.width > hint_len + 28 {
            let hx = area.right().saturating_sub(hint_len);
            let hint_style = if self.input_focused {
                Style::default()
                    .fg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            };
            buf.set_string(hx, y, mode_hint, hint_style);
        }
    }

    fn draw_table(
        &self,
        buf: &mut Buffer,
        area: Rect,
        row_start: u16,
        row_end: u16,
        width: u16,
    ) {
        if row_start >= row_end || width == 0 {
            return;
        }
        let col1_w = width / 3;
        let col2_w = width / 3;
        let col3_w = width.saturating_sub(col1_w + col2_w);
        let col1_x = area.x;
        let col2_x = col1_x + col1_w;
        let col3_x = col2_x + col2_w;

        let y = row_start;
        if y < row_end && y < area.bottom() {
            self.draw_header_cell(buf, area, col1_x, y, "ARTISTS", col1_w as usize);
            self.draw_header_cell(buf, area, col2_x, y, "ALBUMS", col2_w as usize);
            self.draw_header_cell(buf, area, col3_x, y, "TRACKS", col3_w as usize);
        }

        if self.loading || self.rows.is_empty() {
            self.draw_status(buf, area, row_start + 1);
            return;
        }

        let max_rows = row_end.saturating_sub(row_start + 1) as usize;
        if max_rows == 0 {
            return;
        }
        let scroll_offset = if self.selected >= max_rows {
            self.selected + 1 - max_rows
        } else {
            0
        };

        for row_idx in 0..max_rows {
            let i = scroll_offset + row_idx;
            let y = row_start + 1 + row_idx as u16;
            if y >= row_end || y >= area.bottom() || i >= self.rows.len() {
                break;
            }
            let row = &self.rows[i];
            let sel = self.selected == i;
            self.draw_row_cell(buf, area, col1_x, y, &row.artist, col1_w as usize, sel);
            self.draw_row_cell(buf, area, col2_x, y, &row.album, col2_w as usize, sel);
            self.draw_row_cell(buf, area, col3_x, y, &row.track, col3_w as usize, sel);
        }
    }

    fn draw_header_cell(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        y: u16,
        header: &str,
        max_w: usize,
    ) {
        if x >= area.right() || y >= area.bottom() {
            return;
        }
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bold();
        buf.set_string(x, y, Self::truncate(header, max_w), style);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_row_cell(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        y: u16,
        text: &str,
        max_w: usize,
        selected: bool,
    ) {
        if x >= area.right() || y >= area.bottom() {
            return;
        }
        let style = if selected {
            Style::default()
                .fg(theme::color_from_hex(theme::GRAD_START))
                .bold()
        } else {
            Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
        };
        buf.set_string(x, y, Self::truncate(text, max_w), style);
    }

    fn draw_status(&self, buf: &mut Buffer, area: Rect, y: u16) {
        if y >= area.bottom() || area.width == 0 {
            return;
        }
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let msg = if self.loading {
            "Loading…"
        } else {
            "No results"
        };
        let msg_w = msg.chars().count() as u16;
        let start_x = area.x + area.width.saturating_sub(msg_w) / 2;
        buf.set_string(start_x, y, Self::truncate(msg, area.width as usize), style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn make_search() -> Search {
        let mut spectrum = Spectrum::new(10);
        spectrum.set_bars(&[0.25, 0.5, 0.9, 1.0, 0.8, 0.55, 0.35, 0.7, 0.95, 0.4]);
        Search {
            query: "queen".into(),
            input_focused: false,
            selected: 1,
            loading: false,
            rows: vec![
                SearchRow {
                    artist: "Queen".into(),
                    album: "A Day in the Life".into(),
                    track: "Bohemian Rhapsody".into(),
                    time: "5:55".into(),
                    favorite: true,
                },
                SearchRow {
                    artist: "Queen".into(),
                    album: "A Day in the Life".into(),
                    track: "Another One Bites the Dust".into(),
                    time: "3:29".into(),
                    favorite: false,
                },
            ],
            player: MiniPlayer::default(),
            spectrum,
        }
    }

    fn render(screen: &Search, width: u16, height: u16) -> Terminal<TestBackend> {
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
        let screen = make_search();
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "queen"));
        assert!(row_has(&terminal, 1, "ARTISTS"));
        assert!(row_has(&terminal, 1, "ALBUMS"));
        assert!(row_has(&terminal, 1, "TRACKS"));
        assert!(row_has(&terminal, 2, "Queen"));
        assert!(row_has(&terminal, 2, "Bohemian Rhapsody"));
    }

    #[test]
    fn shows_editing_badge_when_input_focused() {
        let mut screen = make_search();
        screen.input_focused = true;
        let terminal = render(&screen, 120, 20);
        assert!(row_has(&terminal, 0, "[EDITING: Enter/Esc]"));
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_search();
        let terminal = render(&screen, 40, 12);
        assert!(row_has(&terminal, 0, "queen"));
        assert!(row_has(&terminal, 2, "Queen"));
    }

    #[test]
    fn empty_state_on_tiny_area_does_not_panic() {
        let mut screen = Search::new();
        screen.query = "q".into();
        let terminal = render(&screen, 6, 4);
        assert!(row_has(&terminal, 0, "Search"));
    }

    #[test]
    fn loading_state_does_not_panic() {
        let mut screen = make_search();
        screen.loading = true;
        screen.query.clear();
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "Loading"));
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let screen = make_search();
        let terminal = render(&screen, 15, 8);
        assert!(row_has(&terminal, 0, "queen"));
    }
}
