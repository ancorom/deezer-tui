//! Search screen (Экран 02): a left NAVIGATION sidebar with a 10-bar
//! mini-spectrum at the bottom, a query field, a three-column results table
//! (ARTISTS / ALBUMS / TRACKS) and a compact mini-player at the bottom.

use ratatui::prelude::*;

use crate::ui::theme;
use crate::ui::widgets::Spectrum;

/// One row of the search results table (three columns).
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SearchRow {
    pub artist: String,
    pub album: String,
    pub track: String,
    /// Playback duration in `m:ss` form (shown by the library table).
    pub time: String,
    /// `true` when the track is a user favorite (♥ in the library table).
    pub favorite: bool,
}

/// Compact player drawn at the bottom of the search screen.
#[allow(dead_code)]
#[derive(Debug)]
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
    /// A default player (before real audio data is available).
    #[allow(dead_code)]
    pub fn default_screen() -> Self {
        Self::default()
    }
}

/// The search screen (Навигация + поиск + колонки ARTISTS/ALBUMS/TRACKS + мини-плеер).
///
/// Layout (Экран 02): a left NAVIGATION sidebar whose bottom holds a 10-bar
/// mini-spectrum, and to its right the query field, the results table and the
/// mini-player.
#[allow(dead_code)]
pub struct Search {
    /// Search query text.
    pub query: String,
    /// Currently selected result row index.
    pub selected: usize,
    /// `true` while a search is in flight.
    pub loading: bool,
    /// Populated search result rows.
    pub rows: Vec<SearchRow>,
    /// Mini-player state (shared with the main player).
    pub player: MiniPlayer,
    /// 10-bar mini-spectrum rendered at the bottom of the sidebar.
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
    /// A fresh, empty search screen.
    #[allow(dead_code)]
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
        // Left NAVIGATION sidebar (Экран 02) with the mini-spectrum at its
        // bottom; shown whenever there is room for it plus usable content.
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
    /// The query field, results table and mini-player (right of the sidebar).
    fn draw_content(&self, buf: &mut Buffer, area: Rect) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        // Reserve one row for the query and two rows for the mini-player.
        let query_y = area.y;
        let mini_player_start = area.bottom().saturating_sub(2);
        let mini_player_end = area.bottom(); // exclusive

        // 1. Query row.
        self.draw_query(buf, area, query_y, w);

        // 2. Results table (between the query row and the mini-player).
        let table_start = area.y + 1;
        let table_end = mini_player_start;
        if table_start < table_end {
            self.draw_table(buf, area, table_start, table_end, w);
        }

        // 3. Mini-player (bottom two rows).
        if mini_player_end > mini_player_start {
            self.draw_mini_player(buf, area, mini_player_start, mini_player_end, w);
        }
    }

    /// Left NAVIGATION sidebar: section header, navigation items (the active
    /// SEARCH item highlighted) and the 10-bar mini-spectrum at the bottom.
    fn draw_sidebar(&self, buf: &mut Buffer, area: Rect) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        // Bottom rows of the sidebar belong to the mini-spectrum.
        let spec_h = 3u16.min(h - 1);
        let spec_top = area.bottom() - spec_h;

        let header_style = Style::default().fg(theme::color_from_hex(theme::GRAD_END)).bold();
        buf.set_string(
            area.x,
            area.y,
            Self::truncate("NAVIGATION", w as usize),
            header_style,
        );

        // (label, is_active) — this screen is SEARCH.
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
                format!("▶  {}", label)
            } else {
                format!("   {}", label)
            };
            let style = if *active {
                Style::default().fg(theme::color_from_hex(theme::GRAD_START)).bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            buf.set_string(area.x, y, Self::truncate(&text, w as usize), style);
        }

        // 10-bar mini-spectrum at the bottom of the sidebar.
        if spec_h > 0 {
            (&self.spectrum).render(Rect::new(area.x, spec_top, w, spec_h), buf);
        }
    }

    /// Truncate a cell's text to `max_width` cells (one `char` per cell).
    fn truncate(text: &str, max_width: usize) -> String {
        if max_width == 0 {
            return String::new();
        }
        text.chars().take(max_width).collect::<String>()
    }

    /// Draw the search query on `row` of `area`.
    fn draw_query(&self, buf: &mut Buffer, area: Rect, row: u16, _width: u16) {
        if row >= area.height {
            return;
        }
        let y = area.y + row;
        let x0 = area.x;
        let label_style =
            Style::default().fg(theme::color_from_hex(theme::GRAD_END)).bold();
        buf.set_string(x0, y, "Search:", label_style);
        // Text starts after the label plus one separator space.
        let x = x0 + 10;
        if x >= area.right() {
            return;
        }
        let text_style =
            Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
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
        // Cursor when typing.
        if self.query.is_empty() {
            return;
        }
        let cursor_x = x0 + (10 + text.len()).min(area.width as usize) as u16;
        if cursor_x < area.right() {
            buf.set_string(cursor_x, y, "|", text_style);
        }
    }

    /// Draw the results table (header + data) in `[row_start, row_end)`.
    fn draw_table(
        &self,
        buf: &mut Buffer,
        area: Rect,
        row_start: u16,
        row_end: u16,
        width: u16,
    ) {
        if row_start >= row_end {
            return;
        }
        // Column widths (roughly equal thirds).
        let col1_w = width / 3;
        let col2_w = width / 3;
        let col1_x = area.x;
        let col2_x = col1_x + col1_w;
        let col3_x = col2_x + col2_w;

        // Header row.
        {
            let y = row_start;
            if y < row_end && y < area.bottom() {
                self.draw_header_cell(buf, area, col1_x, y, "ARTISTS");
                self.draw_header_cell(buf, area, col2_x, y, "ALBUMS");
                self.draw_header_cell(buf, area, col3_x, y, "TRACKS");
            }
        }

        // Data rows.
        if self.loading || self.rows.is_empty() {
            self.draw_status(buf, area, col1_x, row_start + 1, width);
            return;
        }
        let max_rows = row_end.saturating_sub(row_start + 1) as usize;
        if max_rows > 0 {
            for i in 0..max_rows {
                let y = row_start + 1 + i as u16;
                if y >= row_end || y >= area.bottom() || i >= self.rows.len() {
                    break;
                }
                let row = &self.rows[i];
                self.draw_row_cell(buf, area, col1_x, y, &row.artist, self.selected == i);
                self.draw_row_cell(buf, area, col2_x, y, &row.album, self.selected == i);
                self.draw_row_cell(buf, area, col3_x, y, &row.track, self.selected == i);
            }
        }
    }

    /// Draw a single header cell.
    fn draw_header_cell(&self, buf: &mut Buffer, area: Rect, x: u16, y: u16, header: &str) {
        if x >= area.right() || y >= area.bottom() {
            return;
        }
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bold();
        buf.set_string(x, y, header, style);
    }

    /// Draw a single data cell with optional selection highlight.
    fn draw_row_cell(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        y: u16,
        text: &str,
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
        buf.set_string(x, y, text, style);
    }

    /// Draw a centered status / "no results" message.
    fn draw_status(&self, buf: &mut Buffer, area: Rect, x: u16, y: u16, _width: u16) {
        if y >= area.bottom() {
            return;
        }
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let msg = if self.loading {
            String::from("Loading…")
        } else {
            String::from("No results")
        };
        // Center horizontally within the column area.
        let area_width = (area.right() - x) as usize;
        let offset = (area_width - msg.len()) / 2;
        let px = x + area_width as u16 - (offset as u16 + 2);
        let start = area.x + px;
        if start >= area.right() || start < area.x {
            return;
        }
        buf.set_string(start, y, msg.as_str(), style);
    }

    /// Draw the two-row mini-player in `[start, end)`.
    fn draw_mini_player(
        &self,
        buf: &mut Buffer,
        area: Rect,
        start: u16,
        end: u16,
        width: u16,
    ) {
        if start >= end {
            return;
        }
        if start < area.top() || end > area.bottom() {
            return;
        }
        let x0 = area.x;
        if end - start < 2 {
            // Too short: one compact row.
            let y = start;
            self.draw_mini_player_compact(buf, area, y, width);
            return;
        }
        // Row 0: track + artist.
        {
            let y = start;
            if y >= end {
                return;
            }
            let style = Style::default()
                .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
                .bold();
            let label = format!("Now playing: {}", self.player.track);
            buf.set_string(x0, y, &label, style);
            if !self.player.artist.is_empty() {
                let x = x0 + (label.len() + 2).min(area.width as usize) as u16;
                let dim = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
                buf.set_string(x, y, &self.player.artist, dim);
            }
        }
        // Row 1: progress bar + play/pause.
        {
            let y = start + 1;
            if y >= end {
                return;
            }
            // Progress bar fills ~ 60% of the width.
            let bar_w = (width * 60) / 100;
            if bar_w > 0 {
                let bar_style =
                    Style::default().bg(theme::color_from_hex(theme::BG_TERMINAL));
                for x in 0..bar_w {
                    buf.set_string(x0 + x, y, "░", bar_style);
                }
                let col = ((self.player.progress.clamp(0.0, 1.0)) * (bar_w as f32)) as u16;
                let fill_style = Style::default().fg(theme::color_from_hex(theme::GRAD_START));
                for x in 0..col {
                    buf.set_string(x0 + x, y, "■", fill_style);
                }
                buf.set_string(x0 + col, y, "●", fill_style);
            }
            // Play/pause control.
            let control_style = Style::default().fg(theme::color_from_hex(theme::GRAD_END)).bold();
            if x0 + bar_w + 4 < area.right() {
                buf.set_string(x0 + bar_w + 4, y, "▶", control_style);
            }
        }
    }

    /// A single compact row for the mini-player (used on very short areas).
    fn draw_mini_player_compact(
        &self,
        buf: &mut Buffer,
        area: Rect,
        y: u16,
        _width: u16,
    ) {
        if y >= area.bottom() {
            return;
        }
        let x0 = area.x;
        let label = format!("● {}", self.player.track);
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let text = if self.player.paused {
            format!("{}  [Paused]", label)
        } else {
            format!("{}  [Playing]", label)
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
                SearchRow {
                    artist: "Metallica".into(),
                    album: "Master of Puppets".into(),
                    track: "Master of Puppets".into(),
                    time: "8:05".into(),
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
        let row: String = (0..width)
            .map(|x| at(&terminal, x, y))
            .collect();
        row.contains(needle)
    }

    #[test]
    fn renders_in_normal_size() {
        let screen = make_search();
        let terminal = render(&screen, 120, 40);
        // Query row.
        assert!(row_has(&terminal, 0, "queen"));
        // Table headers.
        assert!(row_has(&terminal, 1, "ARTISTS"));
        assert!(row_has(&terminal, 1, "ALBUMS"));
        assert!(row_has(&terminal, 1, "TRACKS"));
        // First data row.
        assert!(row_has(&terminal, 2, "Queen"));
        // Mini-player (bottom two rows: rows 38-39).
        assert!(row_has(&terminal, 38, "Bohemian"));
    }

    #[test]
    fn sidebar_has_navigation_and_mini_spectrum() {
        let screen = make_search();
        let terminal = render(&screen, 120, 40);
        // Sidebar header and navigation items (left of the content).
        assert!(row_has(&terminal, 0, "NAVIGATION"));
        assert!(row_has(&terminal, 1, "HOME"));
        assert!(row_has(&terminal, 2, "SEARCH"));
        assert!(row_has(&terminal, 3, "LIBRARY"));
        assert!(row_has(&terminal, 4, "FLOW"));
        // 10-bar mini-spectrum at the bottom of the sidebar: its filled
        // glyphs occupy the bottom-left rows (columns 0..=18).
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
        // No sidebar at this width; query row still present.
        assert!(!row_has(&terminal, 0, "NAVIGATION"));
        assert!(row_has(&terminal, 0, "queen"));
        // Mini-player present.
        assert!(row_has(&terminal, 10, "Bohemian"));
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
