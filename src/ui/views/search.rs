//! Экран 02 (Search & Discover): левый сайдбар `NAVIGATION` с мини-спектром из 10 столбиков,
//! поле ввода `Search:`, 3 колонки (`ARTISTS`, `ALBUMS`, `TRACKS`) и нижний мини-плеер.

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
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        let sidebar_w = 20u16;
        if w >= sidebar_w + 28 {
            self.draw_sidebar(buf, Rect::new(area.x, area.y, sidebar_w, h));
            self.draw_content(buf, Rect::new(area.x + sidebar_w, area.y, w - sidebar_w, h));
        } else {
            self.draw_content(buf, area);
        }
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
        let mini_player_start = area.bottom().saturating_sub(2);
        let mini_player_end = area.bottom();

        self.draw_query(buf, area, query_y);

        let table_start = area.y + 1;
        let table_end = mini_player_start;
        if table_start < table_end {
            self.draw_table(buf, area, table_start, table_end, w);
        }

        if mini_player_end > mini_player_start {
            self.draw_mini_player(buf, area, mini_player_start, mini_player_end, w);
        }
    }

    fn draw_sidebar(&self, buf: &mut Buffer, area: Rect) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        let spec_h = 3u16.min(h.saturating_sub(1));
        let spec_top = area.bottom() - spec_h;

        let header_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        buf.set_string(
            area.x,
            area.y,
            Self::truncate("NAVIGATION", w as usize),
            header_style,
        );

        let items: [(&str, bool); 4] = [
            ("HOME", false),
            ("SEARCH", true),
            ("LIBRARY", false),
            ("FLOW", false),
        ];
        for (i, (label, active)) in items.iter().enumerate() {
            let y = area.y + 1 + i as u16;
            if y >= spec_top {
                break;
            }
            let text = if *active {
                format!("▶  {label}")
            } else {
                format!("   {label}")
            };
            let style = if *active {
                Style::default()
                    .fg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            buf.set_string(area.x, y, Self::truncate(&text, w as usize), style);
        }

        if spec_h > 0 {
            (&self.spectrum).render(Rect::new(area.x, spec_top, w, spec_h), buf);
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
        let label_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
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
        let text_style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let display = if self.loading {
            String::from("Loading…")
        } else if self.query.is_empty() {
            String::from("type a query…")
        } else {
            self.query.clone()
        };
        let available = (area.right() - x) as usize;
        let text = Self::truncate(&display, available);
        buf.set_string(x, y, text.as_str(), text_style);

        if !self.query.is_empty() {
            let cursor_x = x.saturating_add(text.chars().count() as u16);
            if cursor_x < area.right() {
                buf.set_string(cursor_x, y, "|", text_style);
            }
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
        for i in 0..max_rows {
            let y = row_start + 1 + i as u16;
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

    fn draw_mini_player(
        &self,
        buf: &mut Buffer,
        area: Rect,
        start: u16,
        end: u16,
        width: u16,
    ) {
        if start >= end || start < area.top() || end > area.bottom() {
            return;
        }
        let x0 = area.x;
        if end - start < 2 {
            self.draw_mini_player_compact(buf, area, start);
            return;
        }
        {
            let y = start;
            let style = Style::default()
                .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
                .bold();
            let label = format!("Now playing: {}", self.player.track);
            buf.set_string(x0, y, Self::truncate(&label, width as usize), style);
            if !self.player.artist.is_empty() {
                let offset = (label.chars().count() + 2).min(width as usize) as u16;
                let x = x0 + offset;
                if x < area.right() {
                    let dim = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
                    buf.set_string(
                        x,
                        y,
                        Self::truncate(&self.player.artist, (area.right() - x) as usize),
                        dim,
                    );
                }
            }
        }
        {
            let y = start + 1;
            let bar_w = (width * 60) / 100;
            if bar_w > 0 {
                let bar_style = Style::default().bg(theme::color_from_hex(theme::BG_TERMINAL));
                for x in 0..bar_w {
                    buf.set_string(x0 + x, y, "░", bar_style);
                }
                let col = (self.player.progress.clamp(0.0, 1.0) * (bar_w.saturating_sub(1) as f32))
                    as u16;
                let fill_style = Style::default().fg(theme::color_from_hex(theme::GRAD_START));
                for x in 0..col {
                    buf.set_string(x0 + x, y, "■", fill_style);
                }
                buf.set_string(x0 + col, y, "●", fill_style);
            }
            let control_style = Style::default()
                .fg(theme::color_from_hex(theme::GRAD_END))
                .bold();
            if x0 + bar_w + 4 < area.right() {
                buf.set_string(x0 + bar_w + 4, y, "▶", control_style);
            }
        }
    }

    fn draw_mini_player_compact(&self, buf: &mut Buffer, area: Rect, y: u16) {
        if y >= area.bottom() {
            return;
        }
        let x0 = area.x;
        let label = format!("● {}", self.player.track);
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let text = if self.player.paused {
            format!("{label}  [Paused]")
        } else {
            format!("{label}  [Playing]")
        };
        let available = (area.right() - x0) as usize;
        buf.set_string(x0, y, Self::truncate(&text, available).as_str(), style);
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
            player: MiniPlayer {
                track: "Bohemian Rhapsody".into(),
                artist: "Queen".into(),
                progress: 0.5,
                paused: false,
            },
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
        assert!(row_has(&terminal, 38, "Bohemian"));
    }

    #[test]
    fn sidebar_has_navigation_and_mini_spectrum() {
        let screen = make_search();
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "NAVIGATION"));
        assert!(row_has(&terminal, 1, "HOME"));
        assert!(row_has(&terminal, 2, "SEARCH"));
        assert!(row_has(&terminal, 3, "LIBRARY"));
        assert!(row_has(&terminal, 4, "FLOW"));
        let has_bars = (37..40).any(|y| {
            (0..20).any(|x| {
                matches!(
                    at(&terminal, x, y),
                    '█' | '▇' | '▆' | '▅' | '▄' | '▃' | '▂' | '▁' | '▪'
                )
            })
        });
        assert!(has_bars);
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_search();
        let terminal = render(&screen, 40, 12);
        assert!(!row_has(&terminal, 0, "NAVIGATION"));
        assert!(row_has(&terminal, 0, "queen"));
        assert!(row_has(&terminal, 10, "Bohemian"));
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
        // No panic; query still drawn.
        assert!(row_has(&terminal, 0, "queen"));
    }
}
