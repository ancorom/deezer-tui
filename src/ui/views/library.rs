//! Library screen: a playlist tree, a track table for the selected playlist,
//! a "CHOOSE PLAYLIST" prompt, and a `Modal` chooser when open.

use ratatui::prelude::*;

use crate::ui::theme;
use crate::ui::views::search::{MiniPlayer, SearchRow};
use crate::ui::widgets::Modal;

/// One node in the playlist tree.
#[derive(Debug, Default)]
pub struct PlaylistEntry {
    pub id: String,
    pub title: String,
    pub num_tracks: u64,
    /// Tracks of this playlist (for the table).
    pub tracks: Vec<SearchRow>,
}

/// The library screen.
#[derive(Debug)]
pub struct Library {
    pub playlists: Vec<PlaylistEntry>,
    /// Highlighted playlist in the tree.
    pub selected_playlist: usize,
    /// Highlighted track in the table.
    pub selected_track: usize,
    /// `true` while loading.
    pub loading: bool,
    /// `true` when the "CHOOSE PLAYLIST" modal is open.
    pub show_choose_modal: bool,
    /// Mini-player state (shared with the main player).
    pub player: MiniPlayer,
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

impl Library {
    /// A fresh, empty library screen (with demo playlists).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            playlists: vec![
                PlaylistEntry {
                    id: "1".into(),
                    title: String::from("My Mix"),
                    num_tracks: 2,
                    tracks: vec![
                        SearchRow {
                            artist: "Queen".into(),
                            album: "A Day in the Life".into(),
                            track: "Bohemian Rhapsody".into(),
                            time: "5:55".into(),
                            favorite: true,
                        },
                        SearchRow {
                            artist: "Metallica".into(),
                            album: "Master of Puppets".into(),
                            track: String::from("Master of Puppets"),
                            time: "8:05".into(),
                            favorite: false,
                        },
                    ],
                },
                PlaylistEntry {
                    id: "2".into(),
                    title: String::from("Favorites"),
                    num_tracks: 12,
                    tracks: vec![],
                },
                PlaylistEntry {
                    id: "3".into(),
                    title: String::from("Chill"),
                    num_tracks: 5,
                    tracks: vec![],
                },
            ],
            selected_playlist: 0,
            selected_track: 0,
            loading: false,
            show_choose_modal: false,
            player: MiniPlayer::default(),
        }
    }
}

impl Widget for &Library {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 3 || h < 3 {
            return;
        }
        // "CHOOSE PLAYLIST" modal centered when open.
        if self.show_choose_modal {
            self.draw_choose_modal(buf, area, w, h);
            return;
        }
        // Reserve 2 rows for the mini-player.
        let mini_start = area.bottom().saturating_sub(2);
        let body_end = mini_start;
        if body_end > area.top() {
            self.draw_mini_player(buf, area, mini_start, area.bottom(), w);
        }
        if body_end <= area.y + 1 {
            return;
        }
        // Title.
        {
            let y = area.y;
            if y < body_end {
                self.draw_title(buf, area, y, w);
            }
        }
        // Playlist tree.
        let tree_start = area.y + 1;
        let tree_rows = self.playlists.len().min(3) as u16;
        let tree_end = tree_start + tree_rows;
        if tree_start < tree_end {
            self.draw_playlist_tree(buf, area, tree_start, tree_end, w);
        }
        // CHOOSE PLAYLIST button row.
        {
            let choose_y = tree_end;
            if choose_y < body_end {
                self.draw_choose_button(buf, area, choose_y, w);
            }
        }
        // Track table for the selected playlist.
        {
            let table_start = tree_end + 1;
            let table_end = body_end;
            if table_start < table_end {
                self.draw_track_table(buf, area, table_start, table_end, w);
            }
        }
    }
}

impl Library {
    fn truncate(text: &str, max_width: usize) -> String {
        if max_width == 0 {
            return String::new();
        }
        text.chars().take(max_width).collect::<String>()
    }

    /// Render the "CHOOSE PLAYLIST" modal centered in `area`.
    fn draw_choose_modal(&self, buf: &mut Buffer, area: Rect, w: u16, h: u16) {
        let modal_w = (30.min(w)).max(3);
        let modal_h = (8.min(h)).max(3);
        let modal_x = area.x + (w - modal_w) / 2;
        let modal_y = area.y + (h - modal_h) / 2;
        let modal_area = Rect::new(modal_x, modal_y, modal_w, modal_h);
        let titles: Vec<String> = self.playlists.iter().map(|p| p.title.clone()).collect();
        (Modal::new("CHOOSE PLAYLIST", titles, 0)).render(modal_area, buf);
    }

    fn draw_title(&self, buf: &mut Buffer, area: Rect, y: u16, _w: u16) {
        let style = Style::default().fg(theme::color_from_hex(theme::GRAD_END)).bold();
        buf.set_string(area.x, y, "LIBRARY", style);
    }

    /// Draw the playlist tree: each entry on its own row (selected highlighted).
    fn draw_playlist_tree(
        &self,
        buf: &mut Buffer,
        area: Rect,
        row_start: u16,
        row_end: u16,
        _w: u16,
    ) {
        if row_start >= row_end {
            return;
        }
        if self.loading && row_start < area.bottom() {
            buf.set_string(
                area.x,
                row_start,
                "Loading playlists…",
                Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY)),
            );
            return;
        }
        let max_rows = row_end.saturating_sub(row_start) as usize;
        for i in 0..std::cmp::min(self.playlists.len(), max_rows) {
            let y = row_start + i as u16;
            if y >= row_end || y >= area.bottom() {
                break;
            }
            let entry = &self.playlists[i];
            let selected = i == self.selected_playlist;
            let label = format!(
                "{}  {} ({} tracks)",
                if selected { "▶" } else { " " },
                entry.title,
                entry.num_tracks,
            );
            let style = if selected {
                Style::default().fg(theme::color_from_hex(theme::GRAD_START)).bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            buf.set_string(area.x, y, &label, style);
        }
    }

    fn draw_choose_button(&self, buf: &mut Buffer, area: Rect, y: u16, _w: u16) {
        if y >= area.bottom() {
            return;
        }
        let style = Style::default().fg(theme::color_from_hex(theme::GRAD_START)).bold();
        buf.set_string(area.x, y, "CHOOSE PLAYLIST", style);
    }

    /// Draw the track table (`# / TRACK / ARTIST / ALBUM / TIME / ♥`) for the
    /// selected playlist.
    fn draw_track_table(
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
        // Fixed columns: `#` (3), TIME (5) and ♥ (1); the remaining width is
        // split TRACK 40% / ARTIST 20% / ALBUM 40%.
        let hash_w = 3u16;
        let time_w = 5u16;
        let heart_w = 1u16;
        let rest = width.saturating_sub(hash_w + time_w + heart_w);
        let track_w = rest * 2 / 5;
        let artist_w = rest / 5;
        let album_w = rest - track_w - artist_w;
        let x0 = area.x;
        let x_track = x0 + hash_w;
        let x_artist = x_track + track_w;
        let x_album = x_artist + artist_w;
        let x_time = x_album + album_w;
        let x_heart = x_time + time_w;
        // Header row.
        {
            let y = row_start;
            if y < row_end && y < area.bottom() {
                self.draw_header_cell(buf, area, x0, y, "#", hash_w as usize);
                self.draw_header_cell(buf, area, x_track, y, "TRACK", track_w as usize);
                self.draw_header_cell(buf, area, x_artist, y, "ARTIST", artist_w as usize);
                self.draw_header_cell(buf, area, x_album, y, "ALBUM", album_w as usize);
                self.draw_header_cell(buf, area, x_time, y, "TIME", time_w as usize);
                self.draw_header_cell(buf, area, x_heart, y, "♥", heart_w as usize);
            }
        }
        // Data rows (selected playlist's tracks).
        if let Some(entry) = self.playlists.get(self.selected_playlist) {
            let max_rows = row_end.saturating_sub(row_start + 1) as usize;
            for i in 0..max_rows {
                let y = row_start + 1 + i as u16;
                if y >= row_end || y >= area.bottom() || i >= entry.tracks.len() {
                    break;
                }
                let track = &entry.tracks[i];
                let sel = self.selected_track == i;
                let num = (i as u16 + 1).to_string();
                self.draw_row_cell(buf, area, Position { x: x0, y }, &num, hash_w as usize, sel);
                self.draw_row_cell(buf, area, Position { x: x_track, y }, &track.track, track_w as usize, sel);
                self.draw_row_cell(buf, area, Position { x: x_artist, y }, &track.artist, artist_w as usize, sel);
                self.draw_row_cell(buf, area, Position { x: x_album, y }, &track.album, album_w as usize, sel);
                self.draw_row_cell(buf, area, Position { x: x_time, y }, &track.time, time_w as usize, sel);
                if track.favorite && x_heart < area.right() && y < area.bottom() {
                    let style = if sel {
                        Style::default().fg(theme::color_from_hex(theme::GRAD_START)).bold()
                    } else {
                        Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
                    };
                    buf.set_string(x_heart, y, "♥", style);
                }
            }
        }
    }

    fn draw_header_cell(
        &self,
        buf: &mut Buffer,
        area: Rect,
        x: u16,
        y: u16,
        header: &str,
        max_width: usize,
    ) {
        if x >= area.right() || y >= area.bottom() {
            return;
        }
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bold();
        buf.set_string(x, y, Self::truncate(header, max_width), style);
    }

    fn draw_row_cell(
        &self,
        buf: &mut Buffer,
        area: Rect,
        pos: Position,
        text: &str,
        max_width: usize,
        selected: bool,
    ) {
        if pos.x >= area.right() || pos.y >= area.bottom() {
            return;
        }
        let style = if selected {
            Style::default()
                .fg(theme::color_from_hex(theme::GRAD_START))
                .bold()
        } else {
            Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
        };
        buf.set_string(pos.x, pos.y, Self::truncate(text, max_width), style);
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
            let y = start;
            self.draw_mini_player_compact(buf, area, y, width);
            return;
        }
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
        {
            let y = start + 1;
            if y >= end {
                return;
            }
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
            let control_style = Style::default().fg(theme::color_from_hex(theme::GRAD_END)).bold();
            if x0 + bar_w + 4 < area.right() {
                buf.set_string(x0 + bar_w + 4, y, "▶", control_style);
            }
        }
    }

    /// A single compact row for the mini-player.
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

    fn make_library() -> Library {
        let mut lib = Library::new();
        lib.player = crate::ui::views::search::MiniPlayer {
            track: "Bohemian Rhapsody".into(),
            artist: "Queen".into(),
            progress: 0.5,
            paused: false,
        };
        lib
    }

    fn render(screen: &Library, width: u16, height: u16) -> Terminal<TestBackend> {
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
        let screen = make_library();
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "LIBRARY"));
        assert!(row_has(&terminal, 1, "My Mix"));
        assert!(row_has(&terminal, 2, "Favorites"));
        assert!(row_has(&terminal, 3, "Chill"));
        // CHOOSE PLAYLIST row (row 4).
        assert!(row_has(&terminal, 4, "CHOOSE"));
        // Track table header (row 5): # / TRACK / ARTIST / ALBUM / TIME / ♥.
        assert!(row_has(&terminal, 5, "TRACK"));
        assert!(row_has(&terminal, 5, "ARTIST"));
        assert!(row_has(&terminal, 5, "ALBUM"));
        assert!(row_has(&terminal, 5, "TIME"));
        // First track row (row 6): title, duration and favorite heart.
        assert!(row_has(&terminal, 6, "Bohemian Rhapsody"));
        assert!(row_has(&terminal, 6, "5:55"));
        assert!(row_has(&terminal, 6, "♥"));
        // Mini-player (bottom two rows: 38-39).
        assert!(row_has(&terminal, 38, "Bohemian"));
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_library();
        let terminal = render(&screen, 40, 12);
        assert!(row_has(&terminal, 0, "LIBRARY"));
        assert!(row_has(&terminal, 1, "My Mix"));
        // CHOOSE PLAYLIST + table present somewhere in the body.
        let any = (1..10).any(|y| row_has(&terminal, y, "CHOOSE"));
        assert!(any);
    }

    #[test]
    fn choose_modal_does_not_panic() {
        let mut screen = make_library();
        screen.show_choose_modal = true;
        let terminal = render(&screen, 120, 40);
        // Modal title drawn (centered region: modal_y = 16, title on row 17).
        assert!(row_has(&terminal, 17, "CHOOSE"));
        // First playlist row inside the modal (row 18).
        assert!(row_has(&terminal, 18, "My Mix"));
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let screen = make_library();
        let terminal = render(&screen, 15, 8);
        assert!(row_has(&terminal, 0, "LIBRARY"));
    }
}
