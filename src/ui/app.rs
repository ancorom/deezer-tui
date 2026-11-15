//! Глобальное состояние `deezer-tui`: связывает TUI (60 FPS), фоновый рантайм `tokio`,
//! декодер `Symphonia`, аудиовывод `CPAL` и спектроанализатор `Cavacore`.

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use ratatui::init::DefaultTerminal;
use ratatui::prelude::*;
use rtrb::Consumer;
use tokio::sync::mpsc;

use crate::api::client::DeezerClient;
use crate::api::models::{AudioQuality, LyricLine, PlaylistData, TrackData};
use crate::audio::decoder::{write_interleaved_stereo_to_rings, Decoder};
use crate::audio::output::AudioOutput;
use crate::audio::prefetch::{GaplessPolicy, PrefetchSlot};
use crate::audio::ring_buffers::create_rings;
use crate::audio::stream::DeezerStream;
use crate::config::AppConfig;
use crate::dsp::cavacore::Cavacore;
use crate::mpris::{self, Mpris};
use crate::ui::theme;
use crate::ui::views::flow_lyrics::{FlowLyrics, FlowRow};
use crate::ui::views::library::{Library, PlaylistEntry};
use crate::ui::views::now_playing::NowPlaying;
use crate::ui::views::search::{Search, SearchRow};
use crate::ui::widgets::{CoverArt, Modal};

/// События, приходящие из фонового пула `tokio` и потока декодера в TUI.
#[allow(clippy::large_enum_variant)]
pub enum BgEvent {
    LoggedIn {
        user_name: String,
        playlists: Vec<PlaylistData>,
        favorites: Vec<TrackData>,
        flow: Vec<TrackData>,
    },
    AuthFailed(String),
    SearchDone(Vec<TrackData>),
    ChartLoaded(Vec<TrackData>),
    TrackStarted {
        track: TrackData,
        badge: String,
        vis_consumer: Consumer<f32>,
        cover: Option<CoverArt>,
        lyrics: Vec<LyricLine>,
    },
    PlaybackError(String),
}

pub struct App {
    pub now_playing: NowPlaying,
    pub search: Search,
    pub library: Library,
    pub flow_lyrics: FlowLyrics,
    pub active_tab: u8,
    pub running: bool,
    pub volume: f32,
    pub config: AppConfig,
    pub client: Arc<DeezerClient>,
    pub user_label: Option<String>,
    pub status_message: Option<String>,

    // Живые коллекции треков для запуска воспроизведения по Enter
    pub search_tracks: Vec<TrackData>,
    pub library_tracks: Vec<TrackData>,
    pub flow_tracks: Vec<TrackData>,

    // Модальное окно ввода ARL-токена (клавиша L)
    pub show_arl_modal: bool,
    pub arl_input: String,

    // DSP и аудиодвижок
    pub cavacore: Cavacore,
    pub vis_consumer: Option<Consumer<f32>>,
    pub audio_output: Option<AudioOutput>,
    pub stop_decode: Arc<AtomicBool>,
    pub paused_flag: Arc<AtomicBool>,
    pub track_started_at: Option<Instant>,
    pub current_duration_secs: u64,

    // Асинхронные фоновые задачи
    pub rt: Arc<tokio::runtime::Runtime>,
    pub bg_tx: mpsc::UnboundedSender<BgEvent>,
    pub bg_rx: mpsc::UnboundedReceiver<BgEvent>,

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
        let rt = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("build tokio runtime"),
        );
        let (bg_tx, bg_rx) = mpsc::unbounded_channel();
        let mpris = Mpris::new().ok();

        let app = Self {
            now_playing: NowPlaying::default(),
            search: Search::default(),
            library: Library::default(),
            flow_lyrics: FlowLyrics::default(),
            active_tab: 1,
            running: true,
            volume: cfg.volume.clamp(0.0, 1.0),
            config: cfg.clone(),
            client: Arc::new(DeezerClient::new().expect("build reqwest client")),
            user_label: None,
            status_message: None,
            search_tracks: Vec::new(),
            library_tracks: Vec::new(),
            flow_tracks: Vec::new(),
            show_arl_modal: false,
            arl_input: cfg.effective_arl().unwrap_or_default(),
            cavacore: Cavacore::new(cfg.sample_rate_hz, 64),
            vis_consumer: None,
            audio_output: None,
            stop_decode: Arc::new(AtomicBool::new(false)),
            paused_flag: Arc::new(AtomicBool::new(true)),
            track_started_at: None,
            current_duration_secs: 30,
            rt,
            bg_tx,
            bg_rx,
            prefetch_slot: Arc::new(PrefetchSlot::new()),
            gapless_policy: GaplessPolicy::default(),
            mpris,
        };

        app.spawn_initial_load();
        app
    }

    fn spawn_initial_load(&self) {
        let tx = self.bg_tx.clone();
        let arl_opt = self.config.effective_arl();
        self.rt.spawn(async move {
            let mut client = match DeezerClient::new() {
                Ok(c) => c,
                Err(_) => return,
            };
            if let Some(arl) = arl_opt
                && !arl.is_empty()
            {
                match client.login_arl(&arl).await {
                    Ok(session) => {
                        let playlists = client.get_playlists().await.unwrap_or_default();
                        let favorites = client.get_favorites().await.unwrap_or_default();
                        let flow = client.get_flow().await.unwrap_or_default();
                        let _ = tx.send(BgEvent::LoggedIn {
                            user_name: session.user_name,
                            playlists,
                            favorites,
                            flow,
                        });
                        return;
                    }
                    Err(e) => {
                        let _ = tx.send(BgEvent::AuthFailed(e.to_string()));
                    }
                }
            }
            if let Ok(chart) = client.get_public_chart().await {
                let _ = tx.send(BgEvent::ChartLoaded(chart));
            }
        });
    }

    pub fn attach_vis_consumer(&mut self, consumer: Consumer<f32>) {
        self.vis_consumer = Some(consumer);
    }

    /// Обновляет спектр 60 FPS и прогресс воспроизведения.
    pub fn step_dsp(&mut self) {
        let mut samples = Vec::new();
        if let Some(ref mut consumer) = self.vis_consumer {
            while let Ok(s) = consumer.pop() {
                samples.push(s);
                if samples.len() >= 2048 {
                    break;
                }
            }
        }

        if !samples.is_empty() {
            let bars = self.cavacore.process(&samples);
            self.now_playing.spectrum.set_bars(bars);
            self.flow_lyrics.spectrum.set_bars(bars);
            let mini_len = bars.len().min(10);
            self.search.spectrum.set_bars(&bars[..mini_len]);
        } else if self.now_playing.paused {
            let silence = [0.0f32; 512];
            if self.vis_consumer.is_some() {
                let bars = self.cavacore.process(&silence);
                self.now_playing.spectrum.set_bars(bars);
                self.flow_lyrics.spectrum.set_bars(bars);
                let mini_len = bars.len().min(10);
                self.search.spectrum.set_bars(&bars[..mini_len]);
            }
        }

        if !self.now_playing.paused
            && let Some(started) = self.track_started_at
        {
            let elapsed = started.elapsed();
            let total = Duration::from_secs(self.current_duration_secs.max(1));
            let prog = (elapsed.as_secs_f32() / total.as_secs_f32()).clamp(0.0, 1.0);
            self.now_playing.progress = prog;
            self.search.player.progress = prog;
            self.library.player.progress = prog;
            self.flow_lyrics.position_ms = elapsed.as_millis() as u64;
        }
    }

    /// Обрабатывает события, пришедшие из фонового потока `tokio`.
    fn poll_bg_events(&mut self) {
        while let Ok(ev) = self.bg_rx.try_recv() {
            match ev {
                BgEvent::LoggedIn {
                    user_name,
                    playlists,
                    favorites,
                    flow,
                } => {
                    self.user_label = Some(user_name.clone());
                    self.status_message = Some(format!("Logged in as {user_name}"));
                    if !favorites.is_empty() {
                        self.library_tracks = favorites.clone();
                        let fav_rows: Vec<SearchRow> = favorites
                            .iter()
                            .map(|t| SearchRow {
                                artist: t.artist.clone(),
                                album: t.album.clone(),
                                track: t.title.clone(),
                                time: t.formatted_duration(),
                                favorite: true,
                            })
                            .collect();
                        let mut entries = vec![PlaylistEntry {
                            id: "fav".into(),
                            title: "Favorites".into(),
                            num_tracks: fav_rows.len() as u64,
                            tracks: fav_rows,
                        }];
                        for p in playlists {
                            entries.push(PlaylistEntry {
                                id: p.playlist_id,
                                title: p.playlist_title,
                                num_tracks: p.num_tracks,
                                tracks: Vec::new(),
                            });
                        }
                        self.library.playlists = entries;
                    }
                    if !flow.is_empty() {
                        self.flow_lyrics.flow = flow
                            .iter()
                            .map(|t| FlowRow {
                                title: t.title.clone(),
                                artist: t.artist.clone(),
                            })
                            .collect();
                        self.flow_tracks = flow;
                    }
                }
                BgEvent::AuthFailed(err) => {
                    self.status_message = Some(format!("ARL Error: {err}"));
                }
                BgEvent::ChartLoaded(tracks) => {
                    self.search.rows = tracks
                        .iter()
                        .map(|t| SearchRow {
                            artist: t.artist.clone(),
                            album: t.album.clone(),
                            track: t.title.clone(),
                            time: t.formatted_duration(),
                            favorite: false,
                        })
                        .collect();
                    self.flow_lyrics.flow = tracks
                        .iter()
                        .take(12)
                        .map(|t| FlowRow {
                            title: t.title.clone(),
                            artist: t.artist.clone(),
                        })
                        .collect();
                    if let Some(first_pl) = self.library.playlists.first_mut() {
                        first_pl.tracks = tracks
                            .iter()
                            .take(15)
                            .map(|t| SearchRow {
                                artist: t.artist.clone(),
                                album: t.album.clone(),
                                track: t.title.clone(),
                                time: t.formatted_duration(),
                                favorite: true,
                            })
                            .collect();
                        first_pl.num_tracks = first_pl.tracks.len() as u64;
                    }
                    self.search_tracks = tracks.clone();
                    self.library_tracks = tracks.clone();
                    self.flow_tracks = tracks;
                }
                BgEvent::SearchDone(tracks) => {
                    self.search.loading = false;
                    self.search.selected = 0;
                    self.search.rows = tracks
                        .iter()
                        .map(|t| SearchRow {
                            artist: t.artist.clone(),
                            album: t.album.clone(),
                            track: t.title.clone(),
                            time: t.formatted_duration(),
                            favorite: false,
                        })
                        .collect();
                    self.search_tracks = tracks;
                }
                BgEvent::TrackStarted {
                    track,
                    badge,
                    vis_consumer,
                    cover,
                    lyrics,
                } => {
                    self.now_playing.badge = badge;
                    self.now_playing.track = track.title.clone();
                    self.now_playing.artist = format!("{} — {}", track.artist, track.album);
                    self.now_playing.paused = false;
                    self.now_playing.progress = 0.0;
                    if let Some(c) = cover {
                        self.now_playing.cover = Some(c.clone());
                        self.flow_lyrics.cover = Some(c);
                    }
                    if !lyrics.is_empty() {
                        self.flow_lyrics.lyrics = lyrics;
                    }
                    self.search.player.track = track.title.clone();
                    self.search.player.artist = track.artist.clone();
                    self.search.player.paused = false;
                    self.library.player.track = track.title.clone();
                    self.library.player.artist = track.artist.clone();
                    self.library.player.paused = false;
                    self.flow_lyrics.paused = false;
                    self.current_duration_secs = track.duration_secs();
                    self.track_started_at = Some(Instant::now());
                    self.vis_consumer = Some(vis_consumer);
                }
                BgEvent::PlaybackError(msg) => {
                    self.status_message = Some(msg);
                }
            }
        }
    }

    /// Запускает реальное воспроизведение выбранного трека (ARL HiFi поток, либо 30с превью, либо синтезатор).
    fn start_track_playback(&mut self, track: TrackData) {
        self.stop_decode.store(true, Ordering::SeqCst);
        let stop_flag = Arc::new(AtomicBool::new(false));
        self.stop_decode = Arc::clone(&stop_flag);
        self.paused_flag.store(false, Ordering::SeqCst);
        let paused_flag = Arc::clone(&self.paused_flag);

        let (audio_ring, vis_ring) = create_rings();
        let (mut audio_prod, audio_cons) = audio_ring.split();
        let (mut vis_prod, vis_cons) = vis_ring.split();

        // Открываем реальный звуковой вывод CPAL 44.1 kHz
        match AudioOutput::new(audio_cons, self.volume) {
            Ok(out) => {
                self.audio_output = Some(out);
            }
            Err(e) => {
                tracing::warn!("Не удалось открыть аудиоустройство CPAL: {e}");
            }
        }

        let tx = self.bg_tx.clone();
        let arl_opt = self.config.effective_arl();

        self.rt.spawn(async move {
            let mut client = DeezerClient::new().unwrap();
            let mut stream_opt: Option<(DeezerStream, String)> = None;
            let mut lyrics = Vec::new();

            if let Some(arl) = arl_opt
                && !arl.is_empty()
                && client.login_arl(&arl).await.is_ok()
            {
                if let Ok(lyr) = client.get_lyrics(&track.track_id).await {
                    lyrics = lyr.synced;
                }
                let full_track = if track.has_track_token() {
                    Ok(track.clone())
                } else {
                    client.get_track(&track.track_id).await
                };
                if let Ok(ft) = full_track
                    && let Ok((url, q)) = client.get_stream_url(&ft, AudioQuality::Flac).await
                    && let Ok(s) = DeezerStream::fetch(client.http(), &url, &ft.track_id).await
                {
                    stream_opt = Some((s, q.label().to_string()));
                }
            }

            // Если нет ARL или трек из гостевого режима — качаем 30-секундное MP3-превью с CDN Deezer
            if stream_opt.is_none()
                && let Some(ref preview_url) = track.preview_url
                && let Ok(resp) = client.http().get(preview_url).send().await
                && let Ok(bytes) = resp.bytes().await
                && let Ok(s) = DeezerStream::try_from_decrypted(&bytes)
            {
                stream_opt = Some((s, "MP3 PREVIEW | 128 kbps (GUEST)".to_string()));
            }

            // Скачиваем JPEG-обложку альбома
            let cover_url = track
                .cover_url_direct
                .clone()
                .or_else(|| mpris::cover_url(&track.album_picture));
            let mut cover = None;
            if let Some(url) = cover_url
                && let Ok(resp) = client.http().get(&url).send().await
                && let Ok(bytes) = resp.bytes().await
            {
                cover = CoverArt::from_bytes(&bytes).ok();
            }

            let badge = stream_opt
                .as_ref()
                .map(|(_, b)| b.clone())
                .unwrap_or_else(|| "SYNTH DEMO | 44.1 kHz".to_string());

            let _ = tx.send(BgEvent::TrackStarted {
                track: track.clone(),
                badge,
                vis_consumer: vis_cons,
                cover,
                lyrics,
            });

            // Запускаем поток декодирования Symphonia (или гармонический демо-синтезатор при оффлайне)
            thread::spawn(move || {
                match stream_opt {
                    Some((stream, _)) => {
                        if let Ok(mut dec) = Decoder::new(Box::new(stream), audio_prod, vis_prod) {
                            while !stop_flag.load(Ordering::Relaxed) {
                                if paused_flag.load(Ordering::Relaxed) {
                                    thread::sleep(Duration::from_millis(20));
                                    continue;
                                }
                                match dec.process_next() {
                                    Ok(Some(_)) => {}
                                    _ => break,
                                }
                            }
                        }
                    }
                    None => {
                        // Fallback: музыкальный аккорд, чтобы звук и спектр работали даже без интернета
                        let mut phase = 0usize;
                        let chunk_frames = 1024usize;
                        let mut stereo = vec![0.0f32; chunk_frames * 2];
                        while !stop_flag.load(Ordering::Relaxed) {
                            if paused_flag.load(Ordering::Relaxed) {
                                thread::sleep(Duration::from_millis(20));
                                continue;
                            }
                            for i in 0..chunk_frames {
                                let t = (phase + i) as f32 / 44_100.0;
                                let env = (t * 2.5).sin().abs();
                                let bass = (2.0 * std::f32::consts::PI * 110.0 * t).sin() * 0.45;
                                let mid = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25 * env;
                                let high = (2.0 * std::f32::consts::PI * 2200.0 * t).sin() * 0.15
                                    * (1.0 - env);
                                let s = (bass + mid + high).clamp(-1.0, 1.0);
                                stereo[2 * i] = s;
                                stereo[2 * i + 1] = s;
                            }
                            phase = phase.wrapping_add(chunk_frames);
                            write_interleaved_stereo_to_rings(
                                &stereo,
                                &mut audio_prod,
                                &mut vis_prod,
                            );
                            thread::sleep(Duration::from_millis(20));
                        }
                    }
                }
            });
        });
    }

    fn trigger_search(&mut self) {
        let q = self.search.query.trim().to_string();
        if q.is_empty() {
            return;
        }
        self.search.loading = true;
        let tx = self.bg_tx.clone();
        let arl_opt = self.config.effective_arl();
        self.rt.spawn(async move {
            let mut client = match DeezerClient::new() {
                Ok(c) => c,
                Err(_) => return,
            };
            if let Some(arl) = arl_opt
                && !arl.is_empty()
            {
                let _ = client.login_arl(&arl).await;
            }
            if let Ok(res) = client.search(&q).await {
                let _ = tx.send(BgEvent::SearchDone(res.data));
            } else if let Ok(res) = client.search_public(&q).await {
                let _ = tx.send(BgEvent::SearchDone(res.data));
            } else {
                let _ = tx.send(BgEvent::SearchDone(Vec::new()));
            }
        });
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

    fn draw_top_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.top();
        let title_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
            .bold();
        let dim_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL));
        let primary_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
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

        let status = if let Some(ref u) = self.user_label {
            format!("CONNECTED ({u})")
        } else if self.client.is_authenticated() {
            "CONNECTED".to_string()
        } else {
            "GUEST [press L for ARL]".to_string()
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

    fn draw_bottom_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.bottom() - 1;
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL));
        let hotkeys = if self.active_tab == 2 {
            "type to search | Enter search/play | Up/Down select | F1-F4/Tab screen | L login ARL | Esc back"
        } else if self.active_tab == 3 {
            "q quit | 1-4/Tab screen | Enter play | a add to playlist | Up/Down select | L login ARL"
        } else {
            "q quit | 1-4/Tab screen | space play/pause | Enter play | <- -> seek | + - vol | L login ARL"
        };
        let truncated: String = hotkeys.chars().take(area.width as usize).collect();
        buf.set_string(area.left(), y, &truncated, style);
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        if area.is_empty() {
            return;
        }
        // Заливаем весь экран фирменным тёмным фоном #13151F
        let bg_style = Style::default()
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        buf.set_style(area, bg_style);

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

                if self.show_arl_modal {
                    let mw = 60.min(screen_area.width).max(3);
                    let mh = 6.min(screen_area.height).max(3);
                    let mx = screen_area.x + (screen_area.width - mw) / 2;
                    let my = screen_area.y + (screen_area.height - mh) / 2;
                    let m_area = Rect::new(mx, my, mw, mh);
                    let display_arl = if self.arl_input.is_empty() {
                        "<paste your Deezer ARL cookie here & press Enter>".to_string()
                    } else {
                        self.arl_input.clone()
                    };
                    (&Modal::new(
                        "DEEZER ARL LOGIN (Enter = Save, Esc = Cancel)",
                        vec![display_arl],
                        0,
                    ))
                        .render(m_area, buf);
                }
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        let res = self.loop_(&mut terminal);
        self.stop_decode.store(true, Ordering::SeqCst);
        ratatui::restore();
        res
    }

    fn loop_(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let interval = Duration::from_millis(16);
        let mut last = Instant::now();
        while self.running {
            self.poll_bg_events();
            self.step_dsp();
            let _ = terminal.draw(|frame| {
                let area = frame.area();
                let desired_bars = ((area.width as usize) / 2).clamp(16, 96);
                if self.cavacore.num_bars() != desired_bars && area.width >= 40 {
                    self.cavacore = Cavacore::new(self.config.sample_rate_hz, desired_bars);
                }
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
        use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
        if let Ok(true) = event::poll(Duration::from_nanos(0))
            && let Ok(Event::Key(k)) = event::read()
            && k.kind == KeyEventKind::Press
        {
            // 1. Если открыто окно ввода ARL-куки
            if self.show_arl_modal {
                match k.code {
                    KeyCode::Esc => self.show_arl_modal = false,
                    KeyCode::Backspace => {
                        self.arl_input.pop();
                    }
                    KeyCode::Enter => {
                        self.show_arl_modal = false;
                        let trimmed = self.arl_input.trim().to_string();
                        self.config.arl = if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed)
                        };
                        let _ = self.config.save();
                        self.spawn_initial_load();
                    }
                    KeyCode::Char(c)
                        if !k.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        self.arl_input.push(c);
                    }
                    _ => {}
                }
                return Ok(());
            }

            // 2. Глобальные функциональные клавиши переключения экранов (работают даже при вводе в Search)
            match k.code {
                KeyCode::F(1) => {
                    self.active_tab = 1;
                    return Ok(());
                }
                KeyCode::F(2) => {
                    self.active_tab = 2;
                    return Ok(());
                }
                KeyCode::F(3) => {
                    self.active_tab = 3;
                    return Ok(());
                }
                KeyCode::F(4) => {
                    self.active_tab = 4;
                    return Ok(());
                }
                KeyCode::Tab => {
                    self.active_tab = (self.active_tab % 4) + 1;
                    return Ok(());
                }
                KeyCode::BackTab => {
                    self.active_tab = if self.active_tab <= 1 {
                        4
                    } else {
                        self.active_tab - 1
                    };
                    return Ok(());
                }
                _ => {}
            }

            // 3. На вкладке 2 (Search) печатаемые символы и Backspace идут в поисковую строку
            if self.active_tab == 2 {
                match k.code {
                    KeyCode::Backspace => {
                        self.search.query.pop();
                        return Ok(());
                    }
                    KeyCode::Char('L') if k.modifiers.contains(KeyModifiers::SHIFT) => {
                        self.show_arl_modal = true;
                        return Ok(());
                    }
                    KeyCode::Char(c)
                        if !k.modifiers.contains(KeyModifiers::CONTROL)
                            && !k.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        // Если строка поиска пуста и нажата цифра 1..4 — переключаем вкладку
                        if self.search.query.is_empty() && matches!(c, '1' | '2' | '3' | '4') {
                            self.active_tab = c as u8 - b'0';
                            return Ok(());
                        }
                        self.search.query.push(c);
                        return Ok(());
                    }
                    KeyCode::Up => {
                        self.navigate_up();
                        return Ok(());
                    }
                    KeyCode::Down => {
                        self.navigate_down();
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.press_enter();
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if !self.search.query.is_empty() {
                            self.search.query.clear();
                        } else {
                            self.active_tab = 1;
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }

            // 4. Стандартные хоткеи на вкладках 1, 3, 4
            match k.code {
                KeyCode::Char('q') => self.running = false,
                KeyCode::Char('1') => self.active_tab = 1,
                KeyCode::Char('2') => self.active_tab = 2,
                KeyCode::Char('3') => self.active_tab = 3,
                KeyCode::Char('4') => self.active_tab = 4,
                KeyCode::Char('l') | KeyCode::Char('L') => {
                    self.show_arl_modal = true;
                }
                KeyCode::Char('a') | KeyCode::Char('p') if self.active_tab == 3 => {
                    self.library.modal_selected = self.library.selected_playlist;
                    self.library.show_choose_modal = !self.library.show_choose_modal;
                }
                KeyCode::Char(' ') => self.toggle_play_pause(),
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    self.volume = (self.volume + 0.05).min(1.0);
                    if let Some(ref out) = self.audio_output {
                        out.set_volume(self.volume);
                    }
                    if let Some(ref mpris) = self.mpris {
                        let _ = mpris.set_volume(f64::from(self.volume));
                    }
                }
                KeyCode::Char('-') => {
                    self.volume = (self.volume - 0.05).max(0.0);
                    if let Some(ref out) = self.audio_output {
                        out.set_volume(self.volume);
                    }
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
        let new_paused = !self.now_playing.paused;
        self.now_playing.paused = new_paused;
        self.search.player.paused = new_paused;
        self.library.player.paused = new_paused;
        self.flow_lyrics.paused = new_paused;
        self.paused_flag.store(new_paused, Ordering::SeqCst);
    }

    fn seek(&mut self, forward: bool) {
        let delta = if forward { 0.05 } else { -0.05 };
        let next_prog = (self.now_playing.progress + delta).clamp(0.0, 1.0);
        self.now_playing.progress = next_prog;
        self.search.player.progress = next_prog;
        self.library.player.progress = next_prog;
        let delta_ms = if forward { 2000 } else { -2000 };
        self.flow_lyrics.position_ms =
            (self.flow_lyrics.position_ms as i64 + delta_ms).max(0) as u64;
    }

    fn navigate_up(&mut self) {
        match self.active_tab {
            2 => {
                if self.search.selected > 0 {
                    self.search.selected -= 1;
                }
            }
            3 => {
                // Если открыта модалка CHOOSE PLAYLIST — двигаем ТОЛЬКО курсор модалки!
                if self.library.show_choose_modal {
                    if self.library.modal_selected > 0 {
                        self.library.modal_selected -= 1;
                    }
                } else if self.library.selected_track > 0 {
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
                // Если открыта модалка CHOOSE PLAYLIST — двигаем ТОЛЬКО курсор модалки!
                if self.library.show_choose_modal {
                    if self.library.modal_selected + 1 < self.library.playlists.len() {
                        self.library.modal_selected += 1;
                    }
                } else if let Some(entry) =
                    self.library.playlists.get(self.library.selected_playlist)
                {
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
        if self.active_tab == 3 && !self.library.show_choose_modal {
            if self.library.selected_playlist > 0 {
                self.library.selected_playlist -= 1;
                self.library.selected_track = 0;
            }
        } else {
            self.seek(false);
        }
    }

    fn navigate_right(&mut self) {
        if self.active_tab == 3 && !self.library.show_choose_modal {
            if self.library.selected_playlist + 1 < self.library.playlists.len() {
                self.library.selected_playlist += 1;
                self.library.selected_track = 0;
            }
        } else {
            self.seek(true);
        }
    }

    fn press_enter(&mut self) {
        match self.active_tab {
            1 => {
                if let Some(first) = self.flow_tracks.first().cloned() {
                    self.start_track_playback(first);
                } else {
                    self.start_track_playback(TrackData {
                        track_id: "3135556".into(),
                        title: "Harder, Better, Faster, Stronger".into(),
                        artist: "Daft Punk".into(),
                        album: "Discovery".into(),
                        duration: "3:44".into(),
                        ..Default::default()
                    });
                }
            }
            2 => {
                // Если ввели новый запрос — выполняем поиск; если выбран трек — запускаем его
                if !self.search.query.trim().is_empty() && self.search.rows.is_empty() {
                    self.trigger_search();
                } else if let Some(track) = self.search_tracks.get(self.search.selected).cloned() {
                    if !self.search.query.trim().is_empty() {
                        self.trigger_search();
                    }
                    self.start_track_playback(track);
                } else if !self.search.query.trim().is_empty() {
                    self.trigger_search();
                }
            }
            3 => {
                if self.library.show_choose_modal {
                    // Копируем выбранный трек в целевой плейлист и закрываем модалку
                    let src_pl = self.library.selected_playlist;
                    let src_tr = self.library.selected_track;
                    let dst_pl = self.library.modal_selected;
                    let cloned_row = self
                        .library
                        .playlists
                        .get(src_pl)
                        .and_then(|p| p.tracks.get(src_tr))
                        .cloned();
                    if let Some(row) = cloned_row
                        && let Some(target) = self.library.playlists.get_mut(dst_pl)
                    {
                        target.tracks.push(row);
                        target.num_tracks = target.tracks.len() as u64;
                    }
                    self.library.show_choose_modal = false;
                } else if let Some(track) =
                    self.library_tracks.get(self.library.selected_track).cloned()
                {
                    self.start_track_playback(track);
                } else if let Some(row) = self
                    .library
                    .playlists
                    .get(self.library.selected_playlist)
                    .and_then(|p| p.tracks.get(self.library.selected_track))
                {
                    self.start_track_playback(TrackData {
                        track_id: "0".into(),
                        title: row.track.clone(),
                        artist: row.artist.clone(),
                        album: row.album.clone(),
                        duration: row.time.clone(),
                        ..Default::default()
                    });
                }
            }
            4 => {
                if let Some(track) = self.flow_tracks.get(self.flow_lyrics.selected_flow).cloned() {
                    self.start_track_playback(track);
                } else if let Some(row) = self.flow_lyrics.flow.get(self.flow_lyrics.selected_flow)
                {
                    self.start_track_playback(TrackData {
                        track_id: "0".into(),
                        title: row.title.clone(),
                        artist: row.artist.clone(),
                        album: "Deezer Flow".into(),
                        duration: "3:30".into(),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        }
    }

    fn press_escape(&mut self) {
        if self.show_arl_modal {
            self.show_arl_modal = false;
        } else if self.library.show_choose_modal {
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
        let terminal = render_ui(&app, 110, 20);
        assert!(row_has(&terminal, 0, "DEEZER-TUI"));
        assert!(row_has(&terminal, 0, "[1]Now Playing"));
        assert!(row_has(&terminal, 0, "[2]Search"));
        assert!(row_has(&terminal, 0, "[3]Library"));
        assert!(row_has(&terminal, 0, "[4]Flow"));
        assert!(row_has(&terminal, 19, "q quit"));
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
            assert!(row_has(&terminal, 0, "DEEZER-TUI"));
            assert!(screen_has(&terminal, needle));
        }
    }

    #[test]
    fn modal_navigation_is_independent_of_library_table() {
        let mut app = App::new();
        app.active_tab = 3;
        app.library.selected_track = 0;
        app.library.show_choose_modal = true;
        app.library.modal_selected = 0;

        app.navigate_down();
        assert_eq!(app.library.modal_selected, 1);
        assert_eq!(
            app.library.selected_track, 0,
            "фоновая таблица треков не должна двигаться при открытом модальном окне!"
        );
    }
    #[test]
    fn renders_on_narrow_terminal() {
        let app = App::new();
        let terminal = render_ui(&app, 30, 8);
        assert!(row_has(&terminal, 0, "DEEZER"));
        assert!(row_has(&terminal, 7, "q quit"));
    }
}
