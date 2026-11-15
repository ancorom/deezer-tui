//! Экран 03 (Library & Overlays): двухпанельный фокус (`LibraryFocus::Playlists` и
//! `LibraryFocus::Tracks`), сортировка треков (`Default/Added`, `Title`, `Artist`, `Album`),
//! дерево плейлистов и модальное окно `CHOOSE PLAYLIST`.

#![allow(dead_code)]

use ratatui::prelude::*;

use crate::ui::theme;
use crate::ui::views::search::{MiniPlayer, SearchRow};
use crate::ui::widgets::Modal;

/// Активная панель фокуса на экране Library (переключается по `Tab` или `←`/`→`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LibraryFocus {
    Playlists,
    #[default]
    Tracks,
}

/// Поле сортировки треков в плейлисте (переключается по клавише `o`, направление — по `r`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LibrarySortBy {
    #[default]
    Default,
    Title,
    Artist,
    Album,
}

impl LibrarySortBy {
    pub fn next(self) -> Self {
        match self {
            Self::Default => Self::Title,
            Self::Title => Self::Artist,
            Self::Artist => Self::Album,
            Self::Album => Self::Default,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Default => "Added",
            Self::Title => "Title",
            Self::Artist => "Artist",
            Self::Album => "Album",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistEntry {
    pub id: String,
    pub title: String,
    pub num_tracks: u64,
    pub tracks: Vec<SearchRow>,
}

#[derive(Debug)]
pub struct Library {
    pub playlists: Vec<PlaylistEntry>,
    pub selected_playlist: usize,
    pub selected_track: usize,
    pub modal_selected: usize,
    pub focus: LibraryFocus,
    pub sort_by: LibrarySortBy,
    pub sort_asc: bool,
    pub loading: bool,
    pub show_choose_modal: bool,
    pub player: MiniPlayer,
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

impl Library {
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
            modal_selected: 0,
            focus: LibraryFocus::Tracks,
            sort_by: LibrarySortBy::Default,
            sort_asc: true,
            loading: false,
            show_choose_modal: false,
            player: MiniPlayer::default(),
        }
    }

    /// Переключает фокус между списком плейлистов и таблицей треков.
    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            LibraryFocus::Playlists => LibraryFocus::Tracks,
            LibraryFocus::Tracks => LibraryFocus::Playlists,
        };
    }

    /// Переключает критерий сортировки по кругу (`Added` -> `Title` -> `Artist` -> `Album`).
    pub fn cycle_sort_by(&mut self) {
        self.sort_by = self.sort_by.next();
        self.selected_track = 0;
    }

    /// Инвертирует направление сортировки (`Asc` <-> `Desc`).
    pub fn toggle_sort_direction(&mut self) {
        self.sort_asc = !self.sort_asc;
        self.selected_track = 0;
    }
}

impl Widget for &Library {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 3 || h < 3 {
            return;
        }

        let body_end = area.bottom();
        if body_end > area.y + 1 {
            let y = area.y;
            if y < body_end {
                self.draw_title(buf, area, y);
            }

            let tree_start = area.y + 1;
            let max_tree = if h >= 22 { 5 } else { 3 };
            let tree_rows = self.playlists.len().clamp(1, max_tree) as u16;
            let tree_end = tree_start + tree_rows;
            if tree_start < tree_end {
                self.draw_playlist_tree(buf, area, tree_start, tree_end);
            }

            let choose_y = tree_end;
            if choose_y < body_end {
                self.draw_choose_button(buf, area, choose_y);
            }

            let table_start = tree_end + 1;
            let table_end = body_end;
            if table_start < table_end {
                self.draw_track_table(buf, area, table_start, table_end, w);
            }
        }

        if self.show_choose_modal {
            self.draw_choose_modal(buf, area, w, h);
        }
    }
}

impl Library {
    fn truncate(text: &str, max_width: usize) -> String {
        text.chars().take(max_width).collect()
    }

    fn draw_choose_modal(&self, buf: &mut Buffer, area: Rect, w: u16, h: u16) {
        let modal_w = 34.min(w).max(3);
        let modal_h = 8.min(h).max(3);
        let modal_x = area.x + (w - modal_w) / 2;
        let modal_y = area.y + (h - modal_h) / 2;
        let modal_area = Rect::new(modal_x, modal_y, modal_w, modal_h);
        let titles: Vec<String> = self.playlists.iter().map(|p| p.title.clone()).collect();
        let selected = self.modal_selected.min(titles.len().saturating_sub(1));
        (&Modal::new("CHOOSE PLAYLIST", titles, selected)).render(modal_area, buf);
    }

    fn draw_title(&self, buf: &mut Buffer, area: Rect, y: u16) {
        let style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        let focus_str = match self.focus {
            LibraryFocus::Playlists => "Playlists",
            LibraryFocus::Tracks => "Tracks",
        };
        let dir_arrow = if self.sort_asc { "▲" } else { "▼" };
        let full = format!(
            "LIBRARY • [Focus: {focus_str} | Tab/←/→: Switch] • [Sort [o/r]: {} {dir_arrow}]",
            self.sort_by.label()
        );
        let title = if area.width >= 48 {
            full.as_str()
        } else {
            "LIBRARY"
        };
        buf.set_string(area.x, y, Self::truncate(title, area.width as usize), style);
    }

    fn draw_playlist_tree(&self, buf: &mut Buffer, area: Rect, row_start: u16, row_end: u16) {
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
        if max_rows == 0 || self.playlists.is_empty() {
            return;
        }
        let scroll_offset = if self.selected_playlist >= max_rows {
            self.selected_playlist + 1 - max_rows
        } else {
            0
        };

        for row_idx in 0..max_rows {
            let i = scroll_offset + row_idx;
            let y = row_start + row_idx as u16;
            if y >= row_end || y >= area.bottom() || i >= self.playlists.len() {
                break;
            }
            let entry = &self.playlists[i];
            let selected = i == self.selected_playlist;
            let marker = if selected {
                if self.focus == LibraryFocus::Playlists {
                    "▶"
                } else {
                    "•"
                }
            } else {
                " "
            };
            let label = format!(
                "{}  {} ({} tracks)",
                marker,
                entry.title,
                entry.tracks.len().max(entry.num_tracks as usize),
            );
            let style = if selected && self.focus == LibraryFocus::Playlists {
                Style::default()
                    .fg(theme::color_from_hex(theme::GRAD_START))
                    .bold()
            } else if selected {
                Style::default()
                    .fg(theme::color_from_hex(theme::GRAD_END))
                    .bold()
            } else {
                Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            };
            buf.set_string(
                area.x,
                y,
                Self::truncate(&label, area.width as usize),
                style,
            );
        }
    }

    fn draw_choose_button(&self, buf: &mut Buffer, area: Rect, y: u16) {
        if y >= area.bottom() {
            return;
        }
        let style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_START))
            .bold();
        buf.set_string(
            area.x,
            y,
            Self::truncate(
                "CHOOSE PLAYLIST [a: add to playlist | o: sort column | r: reverse | Enter: play]",
                area.width as usize,
            ),
            style,
        );
    }

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
        let hash_w = 5u16;
        let time_w = 6u16;
        let heart_w = 2u16;
        let rest = width.saturating_sub(hash_w + time_w + heart_w);
        let track_w = rest * 2 / 5;
        let artist_w = rest / 4;
        let album_w = rest.saturating_sub(track_w + artist_w);
        let x0 = area.x;
        let x_track = x0 + hash_w;
        let x_artist = x_track + track_w;
        let x_album = x_artist + artist_w;
        let x_time = x_album + album_w;
        let x_heart = x_time + time_w;

        let arrow = if self.sort_asc { "▲" } else { "▼" };
        let h_hash = if self.sort_by == LibrarySortBy::Default {
            format!("#{arrow}")
        } else {
            String::from("#")
        };
        let h_track = if self.sort_by == LibrarySortBy::Title {
            format!("TRACK {arrow}")
        } else {
            String::from("TRACK")
        };
        let h_artist = if self.sort_by == LibrarySortBy::Artist {
            format!("ARTIST {arrow}")
        } else {
            String::from("ARTIST")
        };
        let h_album = if self.sort_by == LibrarySortBy::Album {
            format!("ALBUM {arrow}")
        } else {
            String::from("ALBUM")
        };

        {
            let y = row_start;
            if y < row_end && y < area.bottom() {
                self.draw_header_cell(buf, area, x0, y, &h_hash, hash_w.saturating_sub(1) as usize);
                self.draw_header_cell(
                    buf,
                    area,
                    x_track,
                    y,
                    &h_track,
                    track_w.saturating_sub(1) as usize,
                );
                self.draw_header_cell(
                    buf,
                    area,
                    x_artist,
                    y,
                    &h_artist,
                    artist_w.saturating_sub(1) as usize,
                );
                self.draw_header_cell(
                    buf,
                    area,
                    x_album,
                    y,
                    &h_album,
                    album_w.saturating_sub(1) as usize,
                );
                self.draw_header_cell(
                    buf,
                    area,
                    x_time,
                    y,
                    "TIME",
                    time_w.saturating_sub(1) as usize,
                );
                self.draw_header_cell(buf, area, x_heart, y, "♥", heart_w as usize);
            }
        }

        if let Some(entry) = self.playlists.get(self.selected_playlist) {
            let max_rows = row_end.saturating_sub(row_start + 1) as usize;
            if max_rows == 0 || entry.tracks.is_empty() {
                return;
            }
            let scroll_offset = if self.selected_track >= max_rows {
                self.selected_track + 1 - max_rows
            } else {
                0
            };

            for row_idx in 0..max_rows {
                let i = scroll_offset + row_idx;
                let y = row_start + 1 + row_idx as u16;
                if y >= row_end || y >= area.bottom() || i >= entry.tracks.len() {
                    break;
                }
                let track = &entry.tracks[i];
                let sel = self.selected_track == i;
                let num = format!("{:<4}", i + 1);
                self.draw_row_cell(
                    buf,
                    area,
                    Position { x: x0, y },
                    &num,
                    hash_w.saturating_sub(1) as usize,
                    sel,
                );
                self.draw_row_cell(
                    buf,
                    area,
                    Position { x: x_track, y },
                    &track.track,
                    track_w.saturating_sub(1) as usize,
                    sel,
                );
                self.draw_row_cell(
                    buf,
                    area,
                    Position { x: x_artist, y },
                    &track.artist,
                    artist_w.saturating_sub(1) as usize,
                    sel,
                );
                self.draw_row_cell(
                    buf,
                    area,
                    Position { x: x_album, y },
                    &track.album,
                    album_w.saturating_sub(1) as usize,
                    sel,
                );
                self.draw_row_cell(
                    buf,
                    area,
                    Position { x: x_time, y },
                    &track.time,
                    time_w.saturating_sub(1) as usize,
                    sel,
                );
                if track.favorite && x_heart < area.right() && y < area.bottom() {
                    let style = if sel {
                        Style::default()
                            .fg(theme::color_from_hex(theme::GRAD_START))
                            .bold()
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
        let style = if selected && self.focus == LibraryFocus::Tracks {
            Style::default()
                .fg(theme::color_from_hex(theme::GRAD_START))
                .bold()
        } else if selected {
            Style::default()
                .fg(theme::color_from_hex(theme::GRAD_END))
                .bold()
        } else {
            Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
        };
        buf.set_string(pos.x, pos.y, Self::truncate(text, max_width), style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn make_library() -> Library {
        Library::new()
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
        let row: String = (0..width).map(|x| at(terminal, x, y)).collect();
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
        assert!(row_has(&terminal, 4, "CHOOSE"));
        assert!(row_has(&terminal, 5, "TRACK"));
        assert!(row_has(&terminal, 6, "Bohemian Rhapsody"));
    }

    #[test]
    fn toggle_focus_and_sort_work() {
        let mut screen = make_library();
        assert_eq!(screen.focus, LibraryFocus::Tracks);
        screen.toggle_focus();
        assert_eq!(screen.focus, LibraryFocus::Playlists);
        screen.cycle_sort_by();
        assert_eq!(screen.sort_by, LibrarySortBy::Title);
        screen.toggle_sort_direction();
        assert!(!screen.sort_asc);

        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "Focus: Playlists"));
        assert!(row_has(&terminal, 0, "Title ▼"));
    }

    #[test]
    fn choose_modal_renders_over_background_screen() {
        let mut screen = make_library();
        screen.show_choose_modal = true;
        let terminal = render(&screen, 120, 40);
        assert!(row_has(&terminal, 0, "LIBRARY"));
        assert!(row_has(&terminal, 17, "CHOOSE"));
        assert!(row_has(&terminal, 18, "My Mix"));
    }

    #[test]
    fn renders_in_narrow_size() {
        let screen = make_library();
        let terminal = render(&screen, 40, 12);
        assert!(row_has(&terminal, 0, "LIBRARY"));
        assert!(row_has(&terminal, 1, "My Mix"));
        let any = (1..10).any(|y| row_has(&terminal, y, "CHOOSE"));
        assert!(any);
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let screen = make_library();
        let terminal = render(&screen, 15, 8);
        assert!(row_has(&terminal, 0, "LIBRARY"));
    }
}

