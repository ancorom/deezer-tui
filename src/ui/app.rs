//! The Deezer-TUI application: owns the per-screen state and drives the 60 FPS
//! event loop with a global top/bottom status bar around the active screen.
//!
//! Uses [`ratatui::init`] and [`ratatui::restore`] (per the project requirement).

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::prelude::*;

use crate::api::client::DeezerClient;
use crate::ui::theme;
use crate::ui::views::flow_lyrics::FlowLyrics;
use crate::ui::views::library::Library;
use crate::ui::views::now_playing::NowPlaying;
use crate::ui::views::search::Search;
use ratatui::init::DefaultTerminal;

/// The Deezer-TUI application state.
pub struct App {
    /// Now Playing data (Screen 01).
    pub now_playing: NowPlaying,
    /// Search screen (Screen 02).
    pub search: Search,
    /// Library screen (Screen 03).
    pub library: Library,
    /// Flow & Lyrics screen (Screen 04).
    pub flow_lyrics: FlowLyrics,
    /// Active screen tab (1 = Now Playing, 2 = Search, 3 = Library, 4 = Flow).
    pub active_tab: u8,
    /// Flipped by `q` to stop the loop.
    pub running: bool,
    /// Volume level (`0.0`–`1.0`).
    pub volume: f32,
    /// Deezer API client (shared with background fetch tasks).
    pub client: Arc<DeezerClient>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            now_playing: NowPlaying::default(),
            search: Search::default(),
            library: Library::default(),
            flow_lyrics: FlowLyrics::default(),
            active_tab: 1,
            running: true,
            volume: 0.5,
            client: Arc::new(DeezerClient::new().expect("build reqwest client")),
        }
    }

    /// Draw the top status bar on row 0 of `area`.
    fn draw_top_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.top();
        let title_style = Style::default().fg(theme::color_from_hex(theme::GRAD_END));
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let primary_style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        // "DEEZER-TUI"
        buf.set_string(area.left(), y, "DEEZER-TUI", title_style);
        // Tabs [1]..[4].
        let tabs = ["Now Playing", "Search", "Library", "Flow"];
        let mut x = (area.left() as usize) + 12;
        for (i, name) in tabs.iter().enumerate() {
            let style = if (i + 1) as u8 == self.active_tab {
                primary_style
            } else {
                dim_style
            };
            let label = format!("[{i}]{name}");
            buf.set_string(x as u16, y, label.as_str(), style);
            x += label.len() + 2;
        }
        // Right-aligned cluster: CONNECTED + VOL (avoid underflow with saturating).
        let right = area.right() as usize;
        let c1 = right.saturating_sub(20).max(area.left() as usize);
        let c2 = c1 + 12;
        let status = if self.client.is_authenticated() {
            " CONNECTED"
        } else {
            " GUEST"
        };
        buf.set_string(c1 as u16, y, status, dim_style);
        let vol_label = format!(" VOL {}%", (self.volume * 100.0).round() as i32);
        buf.set_string(c2 as u16, y, vol_label.as_str(), dim_style);
    }

    /// Draw the bottom hotkey bar on the last row of `area`.
    fn draw_bottom_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.bottom() - 1;
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let hotkeys = "q quit | 1-4 screen | space play/pause | <- -> seek | + - vol | esc back";
        buf.set_string(area.left(), y, hotkeys, style);
    }

    /// Render the entire UI: top bar + active screen (middle) + bottom bar.
    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        if area.is_empty() {
            return;
        }
        self.draw_top_bar(buf, area);
        self.draw_bottom_bar(buf, area);
        if area.height >= 3 {
            let screen_area = Rect::new(
                area.left(),
                area.top() + 1,
                area.width,
                area.height - 2,
            );
            if !screen_area.is_empty() {
                match self.active_tab {
                    1 => (&self.now_playing).render(screen_area, buf),
                    2 => (&self.search).render(screen_area, buf),
                    3 => (&self.library).render(screen_area, buf),
                    4 => (&self.flow_lyrics).render(screen_area, buf),
                    _ => {}
                }
            }
        }
    }

    /// The 60 FPS event loop: `ratatui::init`, render each frame, poll input,
    /// and finally [`ratatui::restore`] before returning.
    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        self.loop_(&mut terminal)?;
        ratatui::restore();
        Ok(())
    }

    /// Render and pump the loop for as long as `self.running`.
    fn loop_(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let interval = Duration::from_millis(16);
        let mut last = Instant::now();
        while self.running {
            let _ = terminal.draw(|frame| {
                let area = frame.area();
                self.render(frame.buffer_mut(), area);
            });
            self.handle_events(terminal)?;
            let now = Instant::now();
            let elapsed = now.duration_since(last);
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
            last = now;
        }
        Ok(())
    }

    /// Non-blocking input polling (so the 60 FPS loop keeps the frame rate).
    fn handle_events(&mut self, _terminal: &mut DefaultTerminal) -> Result<()> {
        use crossterm::event::{self, Event, KeyEventKind, KeyCode};
        if let Ok(true) = event::poll(Duration::from_nanos(0)) {
            match event::read() {
                Ok(Event::Key(k)) => {
                    if k.kind == KeyEventKind::Press {
                        match k.code {
                            KeyCode::Char('q') => self.running = false,
                            KeyCode::Char('1') => self.active_tab = 1,
                            KeyCode::Char('2') => self.active_tab = 2,
                            KeyCode::Char('3') => self.active_tab = 3,
                            KeyCode::Char('4') => self.active_tab = 4,
                            KeyCode::Char(' ') => self.toggle_play_pause(),
                            KeyCode::Char('+') => {
                                self.volume = (self.volume + 0.05).min(1.0)
                            }
                            KeyCode::Char('-') => {
                                self.volume = (self.volume - 0.05).max(0.0)
                            }
                            KeyCode::Up => self.navigate_up(),
                            KeyCode::Down => self.navigate_down(),
                            KeyCode::Left => self.navigate_left(),
                            KeyCode::Right => self.navigate_right(),
                            KeyCode::Enter => self.press_enter(),
                            KeyCode::Esc => self.press_escape(),
                            _ => {}
                        }
                    }
                }
                Ok(Event::Resize(_, _)) => {}
                Ok(_) => {}
                Err(_) => {}
            }
        }

        Ok(())
    }

    /// Toggle play/pause on the active screen's player (Now Playing, Search,
    /// Library mini-players).
    fn toggle_play_pause(&mut self) {
        match self.active_tab {
            1 => self.now_playing.paused = !self.now_playing.paused,
            2 => self.search.player.paused = !self.search.player.paused,
            3 => self.library.player.paused = !self.library.player.paused,
            _ => {}
        }
    }

    /// Seek the active screen's player by a fixed amount.
    fn seek(&mut self, forward: bool) {
        let delta = if forward { 0.1 } else { -0.1 };
        match self.active_tab {
            1 => {
                self.now_playing.progress = (self.now_playing.progress + delta).clamp(0.0, 1.0)
            }
            2 => self.search.player.progress = (self.search.player.progress + delta).clamp(0.0, 1.0),
            3 => {
                self.library.player.progress =
                    (self.library.player.progress + delta).clamp(0.0, 1.0)
            }
            4 => {
                let delta_ms = if forward { 1000 } else { -1000 };
                self.flow_lyrics.position_ms =
                    (self.flow_lyrics.position_ms as i64 + delta_ms).max(0) as u64;
            }
            _ => {}
        }
    }

    /// Move the selection up on the active screen.
    fn navigate_up(&mut self) {
        match self.active_tab {
            2 => {
                if self.search.selected > 0 {
                    self.search.selected -= 1;
                }
            }
            3 => {
                if self.library.selected_track > 0 {
                    self.library.selected_track -= 1;
                } else if self.library.selected_playlist > 0 {
                    self.library.selected_playlist -= 1;
                    self.library.selected_track = 0;
                }
            }
            4 if self.flow_lyrics.selected_flow > 0 => self.flow_lyrics.selected_flow -= 1,
            _ => {}
        }
    }

    /// Move the selection down on the active screen.
    fn navigate_down(&mut self) {
        match self.active_tab {
            2 => {
                if !self.search.rows.is_empty()
                    && self.search.selected + 1 < self.search.rows.len()
                {
                    self.search.selected += 1;
                }
            }
            3 => {
                if let Some(entry) = self.library.playlists.get(self.library.selected_playlist) {
                    if self.library.selected_track < entry.tracks.len() {
                        self.library.selected_track += 1;
                    } else if self.library.selected_playlist + 1 < self.library.playlists.len() {
                        self.library.selected_playlist += 1;
                        self.library.selected_track = 0;
                    }
                }
            }
            4 if !self.flow_lyrics.flow.is_empty()
                && self.flow_lyrics.selected_flow + 1 < self.flow_lyrics.flow.len() =>
            {
                self.flow_lyrics.selected_flow += 1;
            },
            _ => {}
        }
    }

    /// Move selection left: library track selection on the Library screen,
    /// otherwise seek backward.
    fn navigate_left(&mut self) {
        if self.active_tab == 3 {
            if self.library.selected_track > 0 {
                self.library.selected_track -= 1;
            }
        } else {
            self.seek(false);
        }
    }

    /// Move selection right: library track selection on the Library screen,
    /// otherwise seek forward.
    fn navigate_right(&mut self) {
        if self.active_tab == 3 {
            if let Some(entry) = self.library.playlists.get(self.library.selected_playlist)
                && self.library.selected_track < entry.tracks.len()
            {
                self.library.selected_track += 1;
            }
        } else {
            self.seek(true);
        }
    }

    /// Enter: play the selected search row, or toggle the Library playlist
    /// chooser modal.
    fn press_enter(&mut self) {
        match self.active_tab {
            2 => {
                if let Some(row) = self.search.rows.get(self.search.selected) {
                    self.now_playing.track = row.track.clone();
                    self.now_playing.artist = row.artist.clone();
                    self.now_playing.paused = false;
                    self.search.player.track = row.track.clone();
                    self.search.player.artist = row.artist.clone();
                    self.search.player.paused = false;
                }
            }
            3 => {
                self.library.show_choose_modal = !self.library.show_choose_modal;
            }
            _ => {}
        }
    }

    /// Esc: close the Library playlist chooser modal if open, otherwise go
    /// back to the Now Playing screen.
    fn press_escape(&mut self) {
        if self.library.show_choose_modal {
            self.library.show_choose_modal = false;
        } else {
            self.active_tab = 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    fn render_ui(app: &App, width: u16, height: u16) -> Terminal<TestBackend> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                let area = frame.area();
                app.render(frame.buffer_mut(), area);
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
    fn renders_top_and_bottom_bars() {
        let app = App::new();
        let terminal = render_ui(&app, 80, 20);
        // Top bar has the title and the tabs.
        assert!(row_has(&terminal, 0, "DEEZER-TUI"));
        assert!(row_has(&terminal, 0, "Now Playing"));
        // Bottom bar has the hotkeys.
        assert!(row_has(&terminal, 19, "q quit"));
        assert!(row_has(&terminal, 19, "play/pause"));
    }

    fn screen_has(terminal: &Terminal<TestBackend>, needle: &str) -> bool {
        let size = terminal.size().unwrap();
        (0..size.height).any(|y| row_has(terminal, y, needle))
    }

    #[test]
    fn renders_each_tab_screen() {
        let expectations: [(u8, &str); 3] = [(2, "ARTISTS"), (3, "LIBRARY"), (4, "FLOW")];
        for (tab, needle) in expectations {
            let mut app = App::new();
            app.active_tab = tab;
            let terminal = render_ui(&app, 120, 40);
            assert!(
                row_has(&terminal, 0, "DEEZER-TUI"),
                "top bar missing on tab {tab}"
            );
            assert!(
                screen_has(&terminal, needle),
                "screen content for tab {tab} missing: {needle}"
            );
        }
    }

    #[test]
    fn renders_on_narrow_terminal() {
        let app = App::new();
        let terminal = render_ui(&app, 30, 8);
        // No panic; title still drawn.
        assert!(row_has(&terminal, 0, "DEEZER"));
        // Bottom bar drawn on the last row.
        assert!(row_has(&terminal, 7, "q quit"));
    }
}
