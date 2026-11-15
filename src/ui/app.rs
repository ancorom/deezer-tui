//! Глобальное состояние приложения `deezer-tui`, роутинг экранов `[1]..[4]`,
//! верхний и нижний статус-бары и главный 60 FPS цикл (`ratatui::init` / `ratatui::restore`).

#![allow(dead_code)]

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use ratatui::init::DefaultTerminal;
use ratatui::prelude::*;
use rtrb::Consumer;

use crate::api::client::DeezerClient;
use crate::audio::prefetch::{GaplessPolicy, PrefetchSlot};
use crate::config::AppConfig;
use crate::dsp::cavacore::Cavacore;
use crate::mpris::Mpris;
use crate::ui::theme;
use crate::ui::views::flow_lyrics::FlowLyrics;
use crate::ui::views::library::Library;
use crate::ui::views::now_playing::NowPlaying;
use crate::ui::views::search::Search;

pub struct App {
    pub now_playing: NowPlaying,
    pub search: Search,
    pub library: Library,
    pub flow_lyrics: FlowLyrics,
    pub active_tab: u8,
    pub running: bool,
    pub volume: f32,
    pub client: Arc<DeezerClient>,
    pub cavacore: Cavacore,
    pub vis_consumer: Option<Consumer<f32>>,
    pub prefetch_slot: Arc<PrefetchSlot>,
    pub gapless_policy: GaplessPolicy,
    pub mpris: Option<Mpris>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self::with_config(AppConfig::default())
    }

    pub fn with_config(cfg: AppConfig) -> Self {
        let mpris = Mpris::new().ok();
        Self {
            now_playing: NowPlaying::default(),
            search: Search::default(),
            library: Library::default(),
            flow_lyrics: FlowLyrics::default(),
            active_tab: 1,
            running: true,
            volume: cfg.volume.clamp(0.0, 1.0),
            client: Arc::new(DeezerClient::new().expect("build reqwest client")),
            cavacore: Cavacore::new(cfg.sample_rate_hz, 32),
            vis_consumer: None,
            prefetch_slot: Arc::new(PrefetchSlot::new()),
            gapless_policy: GaplessPolicy::default(),
            mpris,
        }
    }

    /// Подключает `Consumer<f32>` из `VisRingBuffer` для живого обновления спектра 60 FPS.
    pub fn attach_vis_consumer(&mut self, consumer: Consumer<f32>) {
        self.vis_consumer = Some(consumer);
    }

    /// Вычитывает накопившиеся сэмплы из `VisRingBuffer` и обновляет спектр на всех экранах.
    pub fn step_dsp(&mut self) {
        let Some(ref mut consumer) = self.vis_consumer else {
            return;
        };
        let mut samples = Vec::new();
        while let Ok(s) = consumer.pop() {
            samples.push(s);
            if samples.len() >= 4096 {
                break;
            }
        }
        if !samples.is_empty() {
            let bars = self.cavacore.process(&samples);
            self.now_playing.spectrum.set_bars(bars);
            self.flow_lyrics.spectrum.set_bars(bars);
            let mini_len = bars.len().min(10);
            self.search.spectrum.set_bars(&bars[..mini_len]);
        }
    }

    fn current_clock_hhmm() -> String {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let hours = (secs / 3600) % 24;
        let mins = (secs / 60) % 60;
        format!("{hours:02}:{mins:02}")
    }

    /// Отрисовывает глобальный верхний статус-бар: `DEEZER-TUI`, вкладки `[1]..[4]`, `CONNECTED`, `VOL`, часы.
    fn draw_top_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.top();
        let title_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let primary_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            .bold();

        buf.set_string(area.left(), y, "DEEZER-TUI", title_style);

        let tabs = ["Now Playing", "Search", "Library", "Flow"];
        let mut x = (area.left() as usize) + 12;
        for (i, name) in tabs.iter().enumerate() {
            let tab_num = (i + 1) as u8;
            let style = if tab_num == self.active_tab {
                primary_style
            } else {
                dim_style
            };
            let label = format!("[{tab_num}]{name}");
            if x + label.len() < area.right() as usize {
                buf.set_string(x as u16, y, label.as_str(), style);
            }
            x += label.len() + 2;
        }

        let status = if self.client.is_authenticated() {
            "CONNECTED"
        } else {
            "GUEST"
        };
        let vol_pct = (self.volume * 100.0).round() as i32;
        let clock = Self::current_clock_hhmm();
        let right_cluster = format!("{status} | VOL {vol_pct}% | {clock}");
        let cluster_len = right_cluster.chars().count();
        if (area.width as usize) > cluster_len + 12 {
            let rx = area.right().saturating_sub(cluster_len as u16);
            buf.set_string(rx, y, &right_cluster, dim_style);
        }
    }

    /// Отрисовывает глобальный нижний хоткей-бар.
    fn draw_bottom_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.bottom() - 1;
        let style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let hotkeys = "q quit | 1-4 screen | space play/pause | <- -> seek | + - vol | esc back";
        let truncated: String = hotkeys.chars().take(area.width as usize).collect();
        buf.set_string(area.left(), y, &truncated, style);
    }

    /// Рендерит верхний бар, активный экран `[1]..[4]` и нижний бар хоткеев.
    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        if area.is_empty() {
            return;
        }
        self.draw_top_bar(buf, area);
        self.draw_bottom_bar(buf, area);
        if area.height >= 3 {
            let screen_area = Rect::new(area.left(), area.top() + 1, area.width, area.height - 2);
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

    /// Запускает 60 FPS TUI-цикл с использованием `ratatui::init()` и `ratatui::restore()`.
    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        let res = self.loop_(&mut terminal);
        ratatui::restore();
        res
    }

    fn loop_(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let interval = Duration::from_millis(16);
        let mut last = Instant::now();
        while self.running {
            self.step_dsp();
            let _ = terminal.draw(|frame| {
                let area = frame.area();
                self.render(frame.buffer_mut(), area);
            });
            self.handle_events()?;
            let now = Instant::now();
            let elapsed = now.duration_since(last);
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
            last = Instant::now();
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use crossterm::event::{self, Event, KeyCode, KeyEventKind};
        if let Ok(true) = event::poll(Duration::from_nanos(0))
            && let Ok(Event::Key(k)) = event::read()
            && k.kind == KeyEventKind::Press
        {
            match k.code {
                KeyCode::Char('q') => self.running = false,
                KeyCode::Char('1') => self.active_tab = 1,
                KeyCode::Char('2') => self.active_tab = 2,
                KeyCode::Char('3') => self.active_tab = 3,
                KeyCode::Char('4') => self.active_tab = 4,
                KeyCode::Char(' ') => self.toggle_play_pause(),
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    self.volume = (self.volume + 0.05).min(1.0);
                    if let Some(ref mpris) = self.mpris {
                        let _ = mpris.set_volume(f64::from(self.volume));
                    }
                }
                KeyCode::Char('-') => {
                    self.volume = (self.volume - 0.05).max(0.0);
                    if let Some(ref mpris) = self.mpris {
                        let _ = mpris.set_volume(f64::from(self.volume));
                    }
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
        Ok(())
    }

    fn toggle_play_pause(&mut self) {
        match self.active_tab {
            1 => self.now_playing.paused = !self.now_playing.paused,
            2 => self.search.player.paused = !self.search.player.paused,
            3 => self.library.player.paused = !self.library.player.paused,
            4 => self.flow_lyrics.paused = !self.flow_lyrics.paused,
            _ => {}
        }
    }

    fn seek(&mut self, forward: bool) {
        let delta = if forward { 0.1 } else { -0.1 };
        match self.active_tab {
            1 => self.now_playing.progress = (self.now_playing.progress + delta).clamp(0.0, 1.0),
            2 => {
                self.search.player.progress =
                    (self.search.player.progress + delta).clamp(0.0, 1.0)
            }
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
                    if self.library.selected_track + 1 < entry.tracks.len() {
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
            }
            _ => {}
        }
    }

    fn navigate_left(&mut self) {
        if self.active_tab == 3 {
            if self.library.selected_track > 0 {
                self.library.selected_track -= 1;
            }
        } else {
            self.seek(false);
        }
    }

    fn navigate_right(&mut self) {
        if self.active_tab == 3 {
            if let Some(entry) = self.library.playlists.get(self.library.selected_playlist)
                && self.library.selected_track + 1 < entry.tracks.len()
            {
                self.library.selected_track += 1;
            }
        } else {
            self.seek(true);
        }
    }

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
        let row: String = (0..width).map(|x| at(terminal, x, y)).collect();
        row.contains(needle)
    }

    fn screen_has(terminal: &Terminal<TestBackend>, needle: &str) -> bool {
        let size = terminal.size().unwrap();
        (0..size.height).any(|y| row_has(terminal, y, needle))
    }

    #[test]
    fn renders_top_and_bottom_bars_with_1_to_4_tabs() {
        let app = App::new();
        let terminal = render_ui(&app, 100, 20);
        assert!(row_has(&terminal, 0, "DEEZER-TUI"));
        assert!(row_has(&terminal, 0, "[1]Now Playing"));
        assert!(row_has(&terminal, 0, "[2]Search"));
        assert!(row_has(&terminal, 0, "[3]Library"));
        assert!(row_has(&terminal, 0, "[4]Flow"));
        assert!(row_has(&terminal, 19, "q quit"));
        assert!(row_has(&terminal, 19, "play/pause"));
    }

    #[test]
    fn renders_each_tab_screen() {
        let expectations: [(u8, &str); 4] = [
            (1, "FLAC | 1411 kbps"),
            (2, "ARTISTS"),
            (3, "LIBRARY"),
            (4, "FLOW"),
        ];
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
        assert!(row_has(&terminal, 0, "DEEZER"));
        assert!(row_has(&terminal, 7, "q quit"));
    }
}
