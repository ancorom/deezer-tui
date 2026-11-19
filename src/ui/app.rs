//! Глобальное состояние `deezer-tui v2.2`: связывает TUI (60 FPS), фоновый рантайм `tokio`,
//! потоковый `DeezerStream` в памяти, защиту от гонок переключения `playback_seq`,
//! переиспользование сессии `DeezerClient`, бесконечный `Flow` и адаптивный спектр.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use ratatui::init::DefaultTerminal;
use ratatui::prelude::*;
use rtrb::Consumer;
use tokio::sync::mpsc;

use crate::api::client::DeezerClient;
use crate::api::models::{LyricLine, PlaylistData, TrackData};
use crate::audio::decoder::{
    push_all_cancelable, Decoder, DecoderCommand, StreamTechInfo,
};
use crate::audio::output::AudioOutput;
use crate::audio::prefetch::{GaplessPolicy, PrefetchSlot};
use crate::audio::ring_buffers::{create_rings, AUDIO_RING_BUFFER_CAPACITY};
use crate::audio::stream::DeezerStream;
use crate::config::AppConfig;
use crate::dsp::cavacore::Cavacore;
use crate::mpris::{self, Mpris};
use crate::ui::theme;
use crate::ui::views::flow_lyrics::{FlowLyrics, FlowRow};
use crate::ui::views::library::{Library, LibraryFocus, LibrarySortBy, PlaylistEntry};
use crate::ui::views::now_playing::NowPlaying;
use crate::ui::views::search::{Search, SearchRow};
use crate::ui::views::settings::Settings;
use crate::ui::widgets::{CoverArt, Modal, Spectrum};

/// Предзагруженный в фоне следующий трек из очереди для мгновенного переключения.
pub struct PrefetchedTrack {
    pub track_id: String,
    pub stream: DeezerStream,
    pub is_preview: bool,
    pub cover: Option<CoverArt>,
    pub lyrics: Vec<LyricLine>,
}

/// Очередь воспроизведения для автоматического и ручного (`n`/`p`) переключения треков.
#[derive(Debug, Clone, Default)]
pub struct PlaybackQueue {
    pub tracks: Vec<TrackData>,
    pub current_index: usize,
}

impl PlaybackQueue {
    pub fn set_queue(&mut self, tracks: Vec<TrackData>, index: usize) -> Option<TrackData> {
        if tracks.is_empty() {
            self.tracks.clear();
            self.current_index = 0;
            return None;
        }
        let clamped = index.min(tracks.len() - 1);
        self.tracks = tracks;
        self.current_index = clamped;
        self.tracks.get(clamped).cloned()
    }

    pub fn peek_next(&self) -> Option<TrackData> {
        self.tracks.get(self.current_index + 1).cloned()
    }

    pub fn remaining(&self) -> usize {
        self.tracks
            .len()
            .saturating_sub(self.current_index.saturating_add(1))
    }

    pub fn append_tracks(&mut self, more: &[TrackData]) {
        for t in more {
            if !self.tracks.iter().any(|existing| existing.track_id == t.track_id) {
                self.tracks.push(t.clone());
            }
        }
    }

    pub fn next_track(&mut self) -> Option<TrackData> {
        if self.tracks.is_empty() {
            return None;
        }
        if self.current_index + 1 < self.tracks.len() {
            self.current_index += 1;
            self.tracks.get(self.current_index).cloned()
        } else {
            None
        }
    }

    pub fn prev_track(&mut self) -> Option<TrackData> {
        if self.tracks.is_empty() {
            return None;
        }
        if self.current_index > 0 {
            self.current_index -= 1;
        }
        self.tracks.get(self.current_index).cloned()
    }
}

/// События, приходящие из фонового пула `tokio` и потока декодера в TUI.
#[allow(clippy::large_enum_variant)]
pub enum BgEvent {
    LoggedIn {
        client: DeezerClient,
        user_name: String,
        playlists: Vec<PlaylistData>,
        favorites: Vec<TrackData>,
        flow: Vec<TrackData>,
    },
    AuthFailed(String),
    SearchDone(Vec<TrackData>),
    SearchMoreLoaded(Vec<TrackData>),
    ChartLoaded(Vec<TrackData>),
    PlaylistTracksLoaded {
        playlist_id: String,
        tracks: Vec<TrackData>,
    },
    FlowMoreLoaded(Vec<TrackData>),
    FlowLoadFailed,
    TrackRadioLoaded {
        seed_title: String,
        tracks: Vec<TrackData>,
    },
    FlowModeTracksLoaded {
        mode_label: String,
        tracks: Vec<TrackData>,
    },
    PlaylistModified {
        message: String,
    },
    FavoriteToggled {
        track_id: String,
        is_favorite: bool,
        title: String,
    },
    TrackStarted {
        req_id: u64,
        track: TrackData,
        badge: String,
        tech_info: StreamTechInfo,
        is_preview: bool,
        vis_consumer: Consumer<f32>,
        cover: Option<CoverArt>,
        lyrics: Vec<LyricLine>,
        decoder_tx: Option<std::sync::mpsc::Sender<DecoderCommand>>,
    },
    TrackEnded {
        req_id: u64,
    },
    PlaybackError(String),
}

pub const FLOW_MODES: &[(&str, &str, bool, &str)] = &[
    ("🎵 My Flow (Персональный бесконечный поток)", "my_flow", false, "standard"),
    ("🧘 Настроение: Chill (Релакс и спокойствие)", "mood_chill", false, "chill"),
    ("⚡ Настроение: Motivation (Бодрость и энергия)", "mood_motivation", false, "motivation"),
    ("🎉 Настроение: Party (Вечеринка и драйв)", "mood_party", false, "party"),
    ("🎯 Настроение: Focus (Концентрация и работа)", "mood_focus", false, "focus"),
    ("💖 Настроение: Romance (Романтический вечер)", "mood_romance", false, "romance"),
    ("🌧️ Настроение: Melancholy (Грусть и размышления)", "mood_melancholy", false, "sad"),
    ("🏃 Настроение: Workout (Спорт и тренировки)", "mood_workout", false, "workout"),
    ("🎸 Жанр: Rock (Рок-музыка)", "genre_rock", true, "152"),
    ("🎤 Жанр: Pop (Поп-хиты)", "genre_pop", true, "132"),
    ("🎧 Жанр: Rap & Hip-Hop (Рэп и хип-хоп)", "genre_rap", true, "116"),
    ("🎛️ Жанр: Electronic & Dance (Электроника и клуб)", "genre_electronic", true, "113"),
    ("🤘 Жанр: Metal (Тяжёлый метал)", "genre_metal", true, "464"),
    ("🎷 Жанр: Jazz & Blues (Джаз и блюз)", "genre_jazz", true, "129"),
    ("🎻 Жанр: Classical (Классическая музыка)", "genre_classical", true, "98"),
    ("🌴 Жанр: Reggae (Регги)", "genre_reggae", true, "144"),
    ("🪕 Жанр: Indie & Alternative (Инди)", "genre_indie", true, "85"),
    ("🔥 Жанр: Soundtracks (Музыка из кино)", "genre_soundtracks", true, "173"),
];

pub struct App {
    /// `true` только когда конфиг был прочитан с диска: тесты строят `App`
    /// из `AppConfig::default()` и не должны перезаписывать `~/.config`.
    pub persist_config: bool,
    pub now_playing: NowPlaying,
    pub search: Search,
    pub library: Library,
    pub flow_lyrics: FlowLyrics,
    pub settings: Settings,
    pub global_spectrum: Spectrum,
    pub active_tab: u8,
    pub search_input_focused: bool,
    pub running: bool,
    pub volume: f32,
    pub config: AppConfig,
    pub client: Arc<DeezerClient>,
    pub user_label: Option<String>,
    pub status_message: Option<String>,

    // Живые коллекции треков и активная очередь воспроизведения
    pub search_tracks: Vec<TrackData>,
    pub library_tracks: Vec<TrackData>,
    pub playlist_tracks_cache: HashMap<String, Vec<TrackData>>,
    pub flow_tracks: Vec<TrackData>,
    pub queue: PlaybackQueue,
    pub playing_from_flow: bool,
    pub loading_more_flow: bool,
    pub search_loading_more: bool,
    pub search_end_reached: bool,
    pub search_page_index: usize,

    // Модальные окна и меню действий
    pub show_arl_modal: bool,
    pub arl_input: String,
    pub show_track_info_modal: bool,
    pub show_track_menu: bool,
    pub track_menu_target: Option<TrackData>,
    pub track_menu_selected: usize,
    pub show_playlist_picker: bool,
    pub playlist_picker_selected: usize,
    pub show_flow_mode_modal: bool,
    pub flow_mode_selected: usize,
    pub current_flow_mode_idx: usize,
    pub toast_message: Option<(String, Instant)>,

    // DSP, синхронизация прогресса, защита от гонок и аудиодвижок
    pub cavacore: Cavacore,
    pub vis_consumer: Option<Consumer<f32>>,
    pub audio_output: Option<AudioOutput>,
    pub stop_decode: Arc<AtomicBool>,
    pub paused_flag: Arc<AtomicBool>,
    pub played_frames: Arc<AtomicU64>,
    pub playback_seq: Arc<AtomicU64>,
    pub current_sample_rate: u32,
    pub current_duration_secs: u64,
    pub current_track: Option<TrackData>,
    pub current_tech_info: Option<StreamTechInfo>,
    pub current_is_preview: bool,
    pub decoder_tx: Option<std::sync::mpsc::Sender<DecoderCommand>>,

    // Асинхронные фоновые задачи и кэш мгновенной предзагрузки следующего трека
    pub rt: Arc<tokio::runtime::Runtime>,
    pub bg_tx: mpsc::UnboundedSender<BgEvent>,
    pub bg_rx: mpsc::UnboundedReceiver<BgEvent>,
    pub next_prefetch_cache: Arc<Mutex<Option<PrefetchedTrack>>>,

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

    /// Конфигурация по умолчанию (тесты, демо-режим): запись на диск выключена.
    pub fn with_config(cfg: AppConfig) -> Self {
        let mut app = Self::with_loaded_config(cfg);
        app.persist_config = false;
        app
    }

    /// Боевой путь: конфиг прочитан с диска, поэтому изменения сохраняются.
    pub fn with_loaded_config(cfg: AppConfig) -> Self {
        let rt = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .expect("build tokio runtime"),
        );
        let (bg_tx, bg_rx) = mpsc::unbounded_channel();
        let mpris = Mpris::new().ok();

        let mut global_spectrum = Spectrum::new(64);
        let demo: Vec<f32> = (0..64)
            .map(|i| {
                let x = i as f32 / 64.0;
                ((x * std::f32::consts::PI * 3.0).sin().abs() * 0.65 + 0.1).clamp(0.0, 1.0)
            })
            .collect();
        global_spectrum.set_bars(&demo);

        let cavacore = Cavacore::new(cfg.sample_rate_hz, 64)
            .with_db_scale(cfg.db_scale)
            .with_scale_mode(cfg.effective_spectrum_scale());
        let mut settings = Settings::new();
        settings.sync_from_config(&cfg, None, None);

        let now_playing = NowPlaying {
            show_lyrics: cfg.show_lyrics,
            ..Default::default()
        };
        let flow_lyrics = FlowLyrics {
            show_lyrics: cfg.show_lyrics,
            ..Default::default()
        };

        let app = Self {
            now_playing,
            search: Search::default(),
            library: Library::default(),
            flow_lyrics,
            settings,
            global_spectrum,
            active_tab: 1,
            search_input_focused: false,
            running: true,
            volume: cfg.volume.clamp(0.0, 1.0),
            config: cfg.clone(),
            client: Arc::new(DeezerClient::new().expect("build reqwest client")),
            user_label: None,
            status_message: None,
            search_tracks: Vec::new(),
            library_tracks: Vec::new(),
            playlist_tracks_cache: HashMap::new(),
            flow_tracks: Vec::new(),
            queue: PlaybackQueue::default(),
            playing_from_flow: false,
            loading_more_flow: false,
            search_loading_more: false,
            search_end_reached: false,
            search_page_index: 0,
            show_arl_modal: false,
            arl_input: cfg.effective_arl().unwrap_or_default(),
            show_track_info_modal: false,
            show_track_menu: false,
            track_menu_target: None,
            track_menu_selected: 0,
            show_playlist_picker: false,
            playlist_picker_selected: 0,
            show_flow_mode_modal: false,
            flow_mode_selected: 0,
            current_flow_mode_idx: 0,
            toast_message: None,
            persist_config: false,
            cavacore,
            vis_consumer: None,
            audio_output: None,
            stop_decode: Arc::new(AtomicBool::new(false)),
            paused_flag: Arc::new(AtomicBool::new(true)),
            played_frames: Arc::new(AtomicU64::new(0)),
            playback_seq: Arc::new(AtomicU64::new(0)),
            current_sample_rate: 44_100,
            current_duration_secs: 30,
            current_track: None,
            current_tech_info: None,
            current_is_preview: false,
            decoder_tx: None,
            rt,
            bg_tx,
            bg_rx,
            next_prefetch_cache: Arc::new(Mutex::new(None)),
            prefetch_slot: Arc::new(PrefetchSlot::new()),
            gapless_policy: GaplessPolicy::default(),
            mpris,
        };

        app.spawn_initial_load();
        app
    }

    pub fn set_search_input_focus(&mut self, focused: bool) {
        self.search_input_focused = focused;
        self.search.input_focused = focused;
    }

    fn sync_settings_view(&mut self) {
        self.settings.sync_from_config(
            &self.config,
            self.user_label.as_deref(),
            self.status_message.as_deref(),
        );
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
                            client,
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

    /// Асинхронно подгружает новую порцию треков для бесконечного Deezer Flow.
    pub fn spawn_fetch_more_flow(&mut self) {
        if self.loading_more_flow {
            return;
        }
        self.loading_more_flow = true;
        let tx = self.bg_tx.clone();
        let mut client = (*self.client).clone();
        let arl_opt = self.config.effective_arl();
        let mode_idx = self.current_flow_mode_idx;
        self.rt.spawn(async move {
            if !client.is_authenticated()
                && let Some(arl) = arl_opt
                && !arl.is_empty()
            {
                let _ = client.login_arl(&arl).await;
            }
            let res = if let Some(mode) = FLOW_MODES.get(mode_idx) {
                let (_, code, is_genre, param) = *mode;
                if code == "my_flow" {
                    client.get_flow().await
                } else if is_genre {
                    client.get_genre_flow(param).await
                } else {
                    client.get_mood_flow(param).await
                }
            } else {
                client.get_flow().await
            };

            if let Ok(more) = res
                && !more.is_empty()
            {
                let _ = tx.send(BgEvent::FlowMoreLoaded(more));
            } else {
                let _ = tx.send(BgEvent::FlowLoadFailed);
            }
        });
    }

    pub fn set_toast(&mut self, msg: impl Into<String>) {
        self.toast_message = Some((msg.into(), Instant::now()));
    }

    pub fn find_cached_track_id(&self, title: &str, artist: &str) -> Option<String> {
        let all = self
            .library_tracks
            .iter()
            .chain(self.playlist_tracks_cache.values().flatten());
        for t in all {
            if t.title.eq_ignore_ascii_case(title) && t.artist.eq_ignore_ascii_case(artist) {
                return Some(t.track_id.clone());
            }
        }
        None
    }

    pub fn search_local_tracks(&self, query: &str) -> Vec<TrackData> {
        let q_clean = DeezerClient::sanitize_query(query).to_lowercase();
        let terms: Vec<&str> = q_clean.split_whitespace().collect();
        if terms.is_empty() {
            return Vec::new();
        }

        let mut matches = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let all_local = self
            .library_tracks
            .iter()
            .chain(self.playlist_tracks_cache.values().flatten());

        for track in all_local {
            let title = track.title.to_lowercase();
            let artist = track.artist.to_lowercase();
            let album = track.album.to_lowercase();

            let matches_all = terms.iter().all(|term| {
                title.contains(term) || artist.contains(term) || album.contains(term)
            });

            if matches_all && seen.insert(track.track_id.clone()) {
                matches.push(track.clone());
            }
        }

        matches
    }

    pub fn active_target_track(&self) -> Option<TrackData> {
        let selected_track = match self.active_tab {
            2 => self.search_tracks.get(self.search.selected).cloned(),
            3 => {
                if let Some(track) = self.library_tracks.get(self.library.selected_track) {
                    Some(track.clone())
                } else if let Some(row) = self
                    .library
                    .playlists
                    .get(self.library.selected_playlist)
                    .and_then(|p| p.tracks.get(self.library.selected_track))
                {
                    let real_tid = if !row.track_id.is_empty() && row.track_id != "0" {
                        row.track_id.clone()
                    } else {
                        self.find_cached_track_id(&row.track, &row.artist)
                            .unwrap_or_else(|| "0".into())
                    };
                    Some(TrackData {
                        track_id: real_tid,
                        title: row.track.clone(),
                        artist: row.artist.clone(),
                        album: row.album.clone(),
                        duration: row.time.clone(),
                        ..Default::default()
                    })
                } else {
                    self.current_track.clone()
                }
            }
            4 => self.flow_tracks.get(self.flow_lyrics.selected_flow).cloned(),
            _ => self.current_track.clone(),
        };

        selected_track
            .or_else(|| self.current_track.clone())
            .or_else(|| self.queue.tracks.get(self.queue.current_index).cloned())
    }

    pub fn is_track_favorite(&self, track_id: &str) -> bool {
        if track_id.is_empty() || track_id == "0" {
            return false;
        }
        self.library_tracks.iter().any(|t| t.track_id == track_id)
    }

    pub fn execute_track_menu_action(&mut self) {
        let Some(track) = self.track_menu_target.clone() else {
            self.show_track_menu = false;
            return;
        };

        match self.track_menu_selected {
            0 => {
                // Избранное
                self.show_track_menu = false;
                let client = Arc::clone(&self.client);
                let bg_tx = self.bg_tx.clone();
                let tid = track.track_id.clone();
                let title = track.title.clone();
                let is_fav = self.is_track_favorite(&tid);

                if is_fav {
                    self.library_tracks.retain(|t| t.track_id != tid);
                    self.set_toast(format!("💔 «{title}» удален из Любимых"));
                    self.rt.spawn(async move {
                        let _ = client.remove_favorite(&tid).await;
                        let _ = bg_tx.send(BgEvent::FavoriteToggled {
                            track_id: tid,
                            is_favorite: false,
                            title,
                        });
                    });
                } else {
                    self.library_tracks.push(track.clone());
                    self.set_toast(format!("❤️ «{title}» добавлен в Любимые"));
                    self.rt.spawn(async move {
                        let _ = client.add_favorite(&tid).await;
                        let _ = bg_tx.send(BgEvent::FavoriteToggled {
                            track_id: tid,
                            is_favorite: true,
                            title,
                        });
                    });
                }
            }
            1 => {
                // Добавить в плейлист
                if self.library.playlists.is_empty() {
                    self.show_track_menu = false;
                    self.set_toast("Нет доступных плейлистов");
                } else {
                    self.show_track_menu = false;
                    self.show_playlist_picker = true;
                    self.playlist_picker_selected = 0;
                }
            }
            2 => {
                // Запустить микс по треку
                self.show_track_menu = false;
                let client = Arc::clone(&self.client);
                let bg_tx = self.bg_tx.clone();
                let mut tid = track.track_id.clone();
                let title = track.title.clone();
                let artist = track.artist.clone();
                self.set_toast(format!("📻 Загрузка микса по треку «{title}»..."));
                self.rt.spawn(async move {
                    if tid.is_empty() || tid == "0" {
                        let q = format!("track:\"{}\" artist:\"{}\"", title, artist);
                        if let Ok(res) = client.search_public(&q).await
                            && let Some(first) = res.data.first()
                        {
                            tid = first.track_id.clone();
                        }
                    }
                    let tracks = client.get_track_radio(&tid).await.unwrap_or_default();
                    let _ = bg_tx.send(BgEvent::TrackRadioLoaded {
                        seed_title: title,
                        tracks,
                    });
                });
            }
            3 => {
                // Информация о треке
                self.show_track_menu = false;
                self.show_track_info_modal = true;
            }
            _ => {
                // Закрыть
                self.show_track_menu = false;
            }
        }
    }

    pub fn execute_add_to_selected_playlist(&mut self) {
        self.show_playlist_picker = false;
        let Some(track) = self.track_menu_target.clone() else {
            return;
        };
        let Some(pl) = self.library.playlists.get_mut(self.playlist_picker_selected) else {
            return;
        };
        let pl_id = pl.id.clone();
        let pl_title = pl.title.clone();
        let tid = track.track_id.clone();
        let title = track.title.clone();

        pl.tracks.push(SearchRow {
            track_id: tid.clone(),
            artist: track.artist.clone(),
            album: track.album.clone(),
            track: track.title.clone(),
            time: track.formatted_duration(),
            added: track.formatted_date_add(),
            favorite: false,
        });
        pl.num_tracks = pl.tracks.len() as u64;

        self.set_toast(format!("✓ «{title}» добавлен в «{pl_title}»"));

        let client = Arc::clone(&self.client);
        let bg_tx = self.bg_tx.clone();
        self.rt.spawn(async move {
            let msg = if pl_id == "fav" {
                let _ = client.add_favorite(&tid).await;
                format!("❤️ «{title}» добавлен в Избранное")
            } else {
                let _ = client.add_to_playlist(&pl_id, &[&tid]).await;
                format!("✓ «{title}» сохранён в «{pl_title}»")
            };
            let _ = bg_tx.send(BgEvent::PlaylistModified { message: msg });
        });
    }

    pub fn execute_select_flow_mode(&mut self) {
        self.show_flow_mode_modal = false;
        self.current_flow_mode_idx = self.flow_mode_selected;
        let Some(mode) = FLOW_MODES.get(self.flow_mode_selected) else {
            return;
        };
        let (label, code, is_genre, param) = *mode;
        self.flow_lyrics.flow_mode_label = label.to_string();
        self.set_toast(format!("Загрузка Flow: {label}..."));

        let client = Arc::clone(&self.client);
        let bg_tx = self.bg_tx.clone();
        let mode_label = label.to_string();
        self.rt.spawn(async move {
            let tracks = if code == "my_flow" {
                client.get_flow().await.unwrap_or_default()
            } else if is_genre {
                client.get_genre_flow(param).await.unwrap_or_default()
            } else {
                client.get_mood_flow(param).await.unwrap_or_default()
            };
            let _ = bg_tx.send(BgEvent::FlowModeTracksLoaded {
                mode_label,
                tracks,
            });
        });
    }

    /// Асинхронно подгружает треки выбранного плейлиста через `DeezerClient::get_playlist_tracks`.
    pub fn spawn_load_playlist_tracks(&mut self, playlist_id: String) {
        if playlist_id.is_empty() {
            return;
        }
        if let Some(cached) = self.playlist_tracks_cache.get(&playlist_id).cloned()
            && !cached.is_empty()
        {
            self.apply_playlist_tracks(&playlist_id, cached);
            return;
        }

        self.library.loading = true;
        let tx = self.bg_tx.clone();
        let mut client = (*self.client).clone();
        let arl_opt = self.config.effective_arl();
        self.rt.spawn(async move {
            if !client.is_authenticated()
                && let Some(arl) = arl_opt
                && !arl.is_empty()
            {
                let _ = client.login_arl(&arl).await;
            }
            let res = if playlist_id == "fav" {
                client.get_favorites().await
            } else {
                client.get_playlist_tracks(&playlist_id).await
            };
            if let Ok(tracks) = res {
                let _ = tx.send(BgEvent::PlaylistTracksLoaded {
                    playlist_id,
                    tracks,
                });
            }
        });
    }

    fn apply_playlist_tracks(&mut self, playlist_id: &str, tracks: Vec<TrackData>) {
        self.playlist_tracks_cache
            .insert(playlist_id.to_string(), tracks.clone());

        let is_selected_pl = self
            .library
            .playlists
            .get(self.library.selected_playlist)
            .is_some_and(|p| p.id == playlist_id);

        if is_selected_pl {
            self.library_tracks = tracks;
            self.resort_active_library_playlist();
        } else {
            let rows: Vec<SearchRow> = tracks
                .iter()
                .map(|t| SearchRow {
                    track_id: t.track_id.clone(),
                    artist: t.artist.clone(),
                    album: t.album.clone(),
                    track: t.title.clone(),
                    time: t.formatted_duration(),
                    added: t.formatted_date_add(),
                    favorite: playlist_id == "fav",
                })
                .collect();
            if let Some(entry) = self
                .library
                .playlists
                .iter_mut()
                .find(|p| p.id == playlist_id)
            {
                entry.num_tracks = rows.len() as u64;
                entry.tracks = rows;
            }
        }
    }

    /// Применяет текущую сортировку (`sort_by` + `sort_asc`) к трекам выбранного плейлиста.
    pub fn resort_active_library_playlist(&mut self) {
        let Some(pl_id) = self
            .library
            .playlists
            .get(self.library.selected_playlist)
            .map(|p| p.id.clone())
        else {
            return;
        };

        if let Some(orig) = self.playlist_tracks_cache.get(&pl_id).cloned() {
            let mut sorted = orig;
            match self.library.sort_by {
                LibrarySortBy::Default => {
                    if sorted.iter().any(|t| t.date_add > 0) {
                        sorted.sort_by(|a, b| {
                            let cmp = a.date_add.cmp(&b.date_add);
                            if self.library.sort_asc {
                                cmp
                            } else {
                                cmp.reverse()
                            }
                        });
                    } else if !self.library.sort_asc {
                        sorted.reverse();
                    }
                }
                LibrarySortBy::Title => {
                    sorted.sort_by(|a, b| {
                        let cmp = a.title.to_lowercase().cmp(&b.title.to_lowercase());
                        if self.library.sort_asc {
                            cmp
                        } else {
                            cmp.reverse()
                        }
                    });
                }
                LibrarySortBy::Artist => {
                    sorted.sort_by(|a, b| {
                        let cmp = a.artist.to_lowercase().cmp(&b.artist.to_lowercase());
                        if self.library.sort_asc {
                            cmp
                        } else {
                            cmp.reverse()
                        }
                    });
                }
                LibrarySortBy::Album => {
                    sorted.sort_by(|a, b| {
                        let cmp = a.album.to_lowercase().cmp(&b.album.to_lowercase());
                        if self.library.sort_asc {
                            cmp
                        } else {
                            cmp.reverse()
                        }
                    });
                }
            }

            let is_fav = pl_id == "fav";
            let rows: Vec<SearchRow> = sorted
                .iter()
                .map(|t| SearchRow {
                    track_id: t.track_id.clone(),
                    artist: t.artist.clone(),
                    album: t.album.clone(),
                    track: t.title.clone(),
                    time: t.formatted_duration(),
                    added: t.formatted_date_add(),
                    favorite: is_fav,
                })
                .collect();

            if let Some(entry) = self.library.playlists.get_mut(self.library.selected_playlist) {
                entry.tracks = rows;
                entry.num_tracks = entry.tracks.len() as u64;
            }
            self.library_tracks = sorted;
        } else if let Some(entry) = self.library.playlists.get_mut(self.library.selected_playlist) {
            match self.library.sort_by {
                LibrarySortBy::Default => {
                    if !self.library.sort_asc {
                        entry.tracks.reverse();
                    }
                }
                LibrarySortBy::Title => {
                    entry.tracks.sort_by(|a, b| {
                        let cmp = a.track.to_lowercase().cmp(&b.track.to_lowercase());
                        if self.library.sort_asc {
                            cmp
                        } else {
                            cmp.reverse()
                        }
                    });
                }
                LibrarySortBy::Artist => {
                    entry.tracks.sort_by(|a, b| {
                        let cmp = a.artist.to_lowercase().cmp(&b.artist.to_lowercase());
                        if self.library.sort_asc {
                            cmp
                        } else {
                            cmp.reverse()
                        }
                    });
                }
                LibrarySortBy::Album => {
                    entry.tracks.sort_by(|a, b| {
                        let cmp = a.album.to_lowercase().cmp(&b.album.to_lowercase());
                        if self.library.sort_asc {
                            cmp
                        } else {
                            cmp.reverse()
                        }
                    });
                }
            }
        }
    }

    pub fn attach_vis_consumer(&mut self, consumer: Consumer<f32>) {
        self.vis_consumer = Some(consumer);
    }

    /// Текущая позиция воспроизведения в секундах из реально сыгранных `CPAL` фреймов.
    pub fn elapsed_secs(&self) -> u64 {
        let frames = self.played_frames.load(Ordering::Relaxed);
        let sr = u64::from(self.current_sample_rate.max(1));
        frames / sr
    }

    pub fn format_mm_ss(secs: u64) -> String {
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    /// Формирует строки модального окна `[i] Track & Stream Info`.
    pub fn track_info_lines(&self) -> Vec<String> {
        let track = self.current_track.clone().unwrap_or_else(|| TrackData {
            track_id: "N/A".into(),
            title: self.now_playing.track.clone(),
            artist: self.now_playing.artist.clone(),
            album: "—".into(),
            duration: Self::format_mm_ss(self.current_duration_secs),
            ..Default::default()
        });
        let tech = self.current_tech_info.clone().unwrap_or_default();
        let size_mb = if tech.file_size_bytes > 0 {
            format!(
                "{:.2} MB ({} bytes)",
                tech.file_size_bytes as f64 / (1024.0 * 1024.0),
                tech.file_size_bytes
            )
        } else {
            String::from("0.00 MB (Live / Synth Stream)")
        };
        let mode_str = if self.current_is_preview {
            "Guest Preview (30s Unencrypted CDN)"
        } else if self.user_label.is_some() || self.config.effective_arl().is_some() {
            "HiFi BF_CBC_STRIPE (Blowfish-CBC Decrypted)"
        } else {
            "Guest Preview / Synth Fallback"
        };

        vec![
            format!("Track ID    : {}", track.track_id),
            format!("Title       : {}", track.title),
            format!("Artist      : {}", track.artist),
            format!("Album       : {}", track.album),
            format!("Codec       : {}", tech.codec_name),
            format!(
                "Sample Rate : {} Hz ({:.1} kHz)",
                tech.sample_rate,
                tech.sample_rate as f32 / 1000.0
            ),
            format!("Bit Depth   : {}-bit", tech.bits_per_sample),
            format!("Bitrate     : {} kbps", tech.bitrate_kbps),
            format!("Stream Size : {}", size_mb),
            format!("Mode        : {}", mode_str),
        ]
    }

    /// Обновляет спектр 60 FPS и прогресс воспроизведения строго по `played_frames / sample_rate`.
    pub fn step_dsp(&mut self) {
        let mut samples = Vec::new();
        if let Some(ref mut consumer) = self.vis_consumer {
            while let Ok(s) = consumer.pop() {
                samples.push(s);
            }
        }

        if !samples.is_empty() {
            let bars = self.cavacore.process(&samples);
            self.global_spectrum.set_bars(bars);
            self.now_playing.spectrum.set_bars(bars);
            self.flow_lyrics.spectrum.set_bars(bars);
            let mini_len = bars.len().min(10);
            self.search.spectrum.set_bars(&bars[..mini_len]);
        } else if self.now_playing.paused && self.vis_consumer.is_some() {
            let silence = [0.0f32; 512];
            let bars = self.cavacore.process(&silence);
            self.global_spectrum.set_bars(bars);
            self.now_playing.spectrum.set_bars(bars);
            self.flow_lyrics.spectrum.set_bars(bars);
            let mini_len = bars.len().min(10);
            self.search.spectrum.set_bars(&bars[..mini_len]);
        }

        if !self.now_playing.paused && self.vis_consumer.is_some() {
            let frames = self.played_frames.load(Ordering::Relaxed);
            let sr = f64::from(self.current_sample_rate.max(1));
            let elapsed_s = frames as f64 / sr;
            let total_s = (self.current_duration_secs.max(1)) as f64;
            let prog = (elapsed_s / total_s).clamp(0.0, 1.0) as f32;
            self.now_playing.progress = prog;
            self.search.player.progress = prog;
            self.library.player.progress = prog;
            let pos_ms = (elapsed_s * 1000.0) as u64;
            self.now_playing.position_ms = pos_ms;
            self.flow_lyrics.position_ms = pos_ms;
        }
    }

    /// Обрабатывает события, пришедшие из фонового потока `tokio` и декодера.
    fn poll_bg_events(&mut self) {
        while let Ok(ev) = self.bg_rx.try_recv() {
            match ev {
                BgEvent::LoggedIn {
                    client,
                    user_name,
                    playlists,
                    favorites,
                    flow,
                } => {
                    self.client = Arc::new(client);
                    self.user_label = Some(user_name.clone());
                    self.status_message = Some(format!("Logged in as {user_name}"));
                    self.sync_settings_view();

                    self.library_tracks = favorites.clone();
                    self.playlist_tracks_cache
                        .insert("fav".to_string(), favorites.clone());
                    let fav_rows: Vec<SearchRow> = favorites
                        .iter()
                        .map(|t| SearchRow {
                            track_id: t.track_id.clone(),
                            artist: t.artist.clone(),
                            album: t.album.clone(),
                            track: t.title.clone(),
                            time: t.formatted_duration(),
                            added: t.formatted_date_add(),
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
                        if p.playlist_id != "fav"
                            && !entries.iter().any(|e| e.id == p.playlist_id)
                        {
                            entries.push(PlaylistEntry {
                                id: p.playlist_id,
                                title: p.playlist_title,
                                num_tracks: p.num_tracks,
                                tracks: Vec::new(),
                            });
                        }
                    }
                    self.library.playlists = entries;
                    self.library.selected_playlist = 0;
                    self.library.selected_track = 0;

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
                    self.user_label = None;
                    self.status_message = Some(format!("ARL Error: {err}"));
                    self.sync_settings_view();
                }
                BgEvent::ChartLoaded(tracks) => {
                    self.search.rows = tracks
                        .iter()
                        .map(|t| SearchRow {
                            track_id: t.track_id.clone(),
                            artist: t.artist.clone(),
                            album: t.album.clone(),
                            track: t.title.clone(),
                            time: t.formatted_duration(),
                            added: t.formatted_date_add(),
                            favorite: false,
                        })
                        .collect();
                    self.flow_lyrics.flow = tracks
                        .iter()
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
                                track_id: t.track_id.clone(),
                                artist: t.artist.clone(),
                                album: t.album.clone(),
                                track: t.title.clone(),
                                time: t.formatted_duration(),
                                added: t.formatted_date_add(),
                                favorite: true,
                            })
                            .collect();
                        first_pl.num_tracks = first_pl.tracks.len() as u64;
                        let first_id = first_pl.id.clone();
                        self.playlist_tracks_cache.insert(first_id, tracks.clone());
                    }
                    self.search_tracks = tracks.clone();
                    self.library_tracks = tracks.clone();
                    self.flow_tracks = tracks;
                }
                BgEvent::SearchDone(tracks) => {
                    self.search.loading = false;
                    // Объединяем полученные треки с уже отображенными локальными, удаляя дубликаты
                    for t in tracks {
                        if !self
                            .search_tracks
                            .iter()
                            .any(|existing| existing.track_id == t.track_id)
                        {
                            self.search_tracks.push(t);
                        }
                    }
                    self.search_page_index = self.search_tracks.len();
                    self.search.rows = self
                        .search_tracks
                        .iter()
                        .map(|t| SearchRow {
                            track_id: t.track_id.clone(),
                            artist: t.artist.clone(),
                            album: t.album.clone(),
                            track: t.title.clone(),
                            time: t.formatted_duration(),
                            added: t.formatted_date_add(),
                            favorite: self.is_track_favorite(&t.track_id),
                        })
                        .collect();
                }
                BgEvent::SearchMoreLoaded(more_tracks) => {
                    self.search_loading_more = false;
                    if more_tracks.is_empty() {
                        self.search_end_reached = true;
                    } else {
                        let mut added = 0;
                        for t in more_tracks {
                            if !self
                                .search_tracks
                                .iter()
                                .any(|existing| existing.track_id == t.track_id)
                            {
                                self.search_tracks.push(t.clone());
                                self.search.rows.push(SearchRow {
                                    track_id: t.track_id.clone(),
                                    artist: t.artist.clone(),
                                    album: t.album.clone(),
                                    track: t.title.clone(),
                                    time: t.formatted_duration(),
                                    added: t.formatted_date_add(),
                                    favorite: self.is_track_favorite(&t.track_id),
                                });
                                added += 1;
                            }
                        }
                        if added == 0 {
                            self.search_end_reached = true;
                        } else {
                            self.search_page_index += added;
                        }
                    }
                }
                BgEvent::PlaylistTracksLoaded {
                    playlist_id,
                    tracks,
                } => {
                    self.library.loading = false;
                    self.apply_playlist_tracks(&playlist_id, tracks);
                }
                BgEvent::FlowMoreLoaded(more_tracks) => {
                    self.loading_more_flow = false;
                    let was_stopped_at_end = self.playing_from_flow
                        && self.now_playing.paused
                        && self.queue.remaining() == 0;
                    for t in &more_tracks {
                        if !self
                            .flow_tracks
                            .iter()
                            .any(|existing| existing.track_id == t.track_id)
                        {
                            self.flow_tracks.push(t.clone());
                            self.flow_lyrics.flow.push(FlowRow {
                                title: t.title.clone(),
                                artist: t.artist.clone(),
                            });
                        }
                    }
                    if self.playing_from_flow {
                        self.queue.append_tracks(&more_tracks);
                        if was_stopped_at_end && let Some(next_track) = self.queue.next_track() {
                            self.start_track_playback(next_track);
                        } else {
                            self.spawn_prefetch_next();
                        }
                    }
                }
                BgEvent::FlowLoadFailed => {
                    self.loading_more_flow = false;
                }
                BgEvent::TrackRadioLoaded { seed_title, tracks } => {
                    if !tracks.is_empty() {
                        self.playing_from_flow = false;
                        self.flow_lyrics.flow = tracks
                            .iter()
                            .map(|t| FlowRow {
                                title: t.title.clone(),
                                artist: t.artist.clone(),
                            })
                            .collect();
                        self.flow_tracks = tracks.clone();
                        self.flow_lyrics.selected_flow = 0;
                        if let Some(track) = self.queue.set_queue(tracks, 0) {
                            self.start_track_playback(track);
                        }
                        self.set_toast(format!("📻 Запущен микс по треку «{seed_title}»"));
                    } else {
                        self.set_toast(format!("Не удалось найти микс для «{seed_title}»"));
                    }
                }
                BgEvent::FlowModeTracksLoaded { mode_label, tracks } => {
                    if !tracks.is_empty() {
                        self.flow_lyrics.flow_mode_label = mode_label.clone();
                        self.flow_lyrics.flow = tracks
                            .iter()
                            .map(|t| FlowRow {
                                title: t.title.clone(),
                                artist: t.artist.clone(),
                            })
                            .collect();
                        self.flow_tracks = tracks.clone();
                        self.flow_lyrics.selected_flow = 0;
                        self.playing_from_flow = true;
                        if let Some(track) = self.queue.set_queue(tracks, 0) {
                            self.start_track_playback(track);
                        }
                        self.set_toast(format!("✓ Включен поток: {mode_label}"));
                    } else {
                        self.set_toast(format!("Не удалось загрузить поток: {mode_label}"));
                    }
                }
                BgEvent::PlaylistModified { message } => {
                    self.set_toast(message);
                }
                BgEvent::FavoriteToggled { track_id, is_favorite, title } => {
                    if is_favorite {
                        self.set_toast(format!("❤️ «{title}» добавлен в Любимые"));
                    } else {
                        self.library_tracks.retain(|t| t.track_id != track_id);
                        self.set_toast(format!("💔 «{title}» удален из Любимых"));
                    }
                }
                BgEvent::TrackStarted {
                    req_id,
                    track,
                    badge,
                    tech_info,
                    is_preview,
                    vis_consumer,
                    cover,
                    lyrics,
                    decoder_tx,
                } => {
                    // Отбрасываем устаревшие события от пропущенных треков
                    if req_id != self.playback_seq.load(Ordering::SeqCst) {
                        continue;
                    }
                    self.now_playing.badge = badge;
                    self.now_playing.track = track.title.clone();
                    self.now_playing.artist = format!("{} — {}", track.artist, track.album);
                    self.now_playing.paused = false;
                    self.now_playing.progress = 0.0;
                    if let Some(c) = cover {
                        self.now_playing.cover = Some(c.clone());
                        self.flow_lyrics.cover = Some(c);
                    }
                    self.now_playing.lyrics = lyrics.clone();
                    self.now_playing.position_ms = 0;
                    self.flow_lyrics.lyrics = lyrics;
                    self.flow_lyrics.position_ms = 0;

                    if self.playing_from_flow {
                        if let Some(pos) = self
                            .flow_tracks
                            .iter()
                            .position(|t| t.track_id == track.track_id)
                        {
                            self.flow_lyrics.selected_flow = pos;
                        }
                        if self.queue.remaining() <= 3 {
                            self.spawn_fetch_more_flow();
                        }
                    }

                    self.search.player.track = track.title.clone();
                    self.search.player.artist = track.artist.clone();
                    self.search.player.paused = false;
                    self.library.player.track = track.title.clone();
                    self.library.player.artist = track.artist.clone();
                    self.library.player.paused = false;
                    self.flow_lyrics.paused = false;

                    self.current_sample_rate = tech_info.sample_rate.max(1);
                    self.current_duration_secs = tech_info.duration_secs.max(1);
                    self.current_track = Some(track);
                    self.current_tech_info = Some(tech_info);
                    self.current_is_preview = is_preview;
                    self.decoder_tx = decoder_tx;
                    self.vis_consumer = Some(vis_consumer);
                }
                BgEvent::TrackEnded { req_id } => {
                    if req_id != self.playback_seq.load(Ordering::SeqCst) {
                        continue;
                    }
                    if let Some(next_track) = self.queue.next_track() {
                        self.start_track_playback(next_track);
                        if self.playing_from_flow && self.queue.remaining() <= 3 {
                            self.spawn_fetch_more_flow();
                        }
                    } else if self.playing_from_flow {
                        self.now_playing.paused = true;
                        self.paused_flag.store(true, Ordering::SeqCst);
                        self.spawn_fetch_more_flow();
                    } else {
                        self.now_playing.paused = true;
                        self.paused_flag.store(true, Ordering::SeqCst);
                    }
                }
                BgEvent::PlaybackError(msg) => {
                    self.status_message = Some(msg);
                }
            }
        }
    }

    /// Фоновая предзагрузка следующего трека в очереди (`queue.peek_next()`) сразу после старта текущего.
    fn spawn_prefetch_next(&self) {
        let Some(next_track) = self.queue.peek_next() else {
            return;
        };
        if next_track.track_id.is_empty() || next_track.track_id == "0" {
            return;
        }
        if let Ok(guard) = self.next_prefetch_cache.lock()
            && guard
                .as_ref()
                .is_some_and(|p| p.track_id == next_track.track_id)
        {
            return;
        }

        let cache = Arc::clone(&self.next_prefetch_cache);
        let mut client = (*self.client).clone();
        let arl_opt = self.config.effective_arl();
        let preferred_quality = self.config.preferred_audio_quality();

        self.rt.spawn(async move {
            let mut stream_opt: Option<(DeezerStream, bool)> = None;
            let mut lyrics = Vec::new();

            if !client.is_authenticated()
                && let Some(arl) = arl_opt
                && !arl.is_empty()
            {
                let _ = client.login_arl(&arl).await;
            }

            if client.is_authenticated() {
                if let Ok(lyr) = client.get_lyrics(&next_track.track_id).await {
                    lyrics = lyr.synced;
                }
                let mut stream_url_res = if next_track.has_track_token() {
                    client.get_stream_url(&next_track, preferred_quality).await
                } else {
                    Err(crate::api::models::DeezerError::TrackUnavailable(
                        "no token".into(),
                    ))
                };
                if stream_url_res.is_err()
                    && let Ok(fresh_track) = client.get_track(&next_track.track_id).await
                {
                    stream_url_res = client.get_stream_url(&fresh_track, preferred_quality).await;
                }
                if let Ok((url, _q)) = stream_url_res
                    && let Ok(s) = DeezerStream::fetch(client.http(), &url, &next_track.track_id).await
                {
                    stream_opt = Some((s, false));
                }
            }

            if stream_opt.is_none()
                && let Some(ref preview_url) = next_track.preview_url
                && let Ok(resp) = client.http().get(preview_url).send().await
                && let Ok(bytes) = resp.bytes().await
                && let Ok(s) = DeezerStream::try_from_decrypted(&bytes)
            {
                stream_opt = Some((s, true));
            }

            let Some((stream, is_preview)) = stream_opt else {
                return;
            };

            let cover_url = next_track
                .cover_url_direct
                .clone()
                .or_else(|| mpris::cover_url(&next_track.album_picture));
            let mut cover = None;
            if let Some(url) = cover_url
                && let Ok(resp) = client.http().get(&url).send().await
                && let Ok(bytes) = resp.bytes().await
            {
                cover = CoverArt::from_bytes(&bytes).ok();
            }

            if let Ok(mut guard) = cache.lock() {
                *guard = Some(PrefetchedTrack {
                    track_id: next_track.track_id,
                    stream,
                    is_preview,
                    cover,
                    lyrics,
                });
            }
        });
    }

    /// Запускает воспроизведение выбранного трека (из кэша предзагрузки за 0 мс либо потоково).
    fn start_track_playback(&mut self, track: TrackData) {
        // Останавливаем предыдущий поток декодера и увеличиваем счётчик поколения
        self.stop_decode.store(true, Ordering::SeqCst);
        let req_id = self.playback_seq.fetch_add(1, Ordering::SeqCst) + 1;
        let seq_ref = Arc::clone(&self.playback_seq);

        let stop_flag = Arc::new(AtomicBool::new(false));
        self.stop_decode = Arc::clone(&stop_flag);

        self.paused_flag.store(false, Ordering::SeqCst);
        let paused_flag = Arc::clone(&self.paused_flag);

        let played_frames = Arc::new(AtomicU64::new(0));
        self.played_frames = Arc::clone(&played_frames);

        self.now_playing.track = track.title.clone();
        self.now_playing.artist = format!("{} — {}", track.artist, track.album);
        self.now_playing.lyrics.clear();
        self.now_playing.position_ms = 0;
        self.flow_lyrics.lyrics.clear();
        self.flow_lyrics.position_ms = 0;
        self.now_playing.progress = 0.0;
        self.decoder_tx = None;

        // Освобождаем предыдущий аудиопоток перед открытием нового
        self.audio_output = None;

        let (audio_ring, vis_ring) = create_rings();
        let (mut audio_prod, audio_cons) = audio_ring.split();
        let (vis_prod, vis_cons) = vis_ring.split();

        match AudioOutput::new_with_sync(
            audio_cons,
            Some(vis_prod),
            self.volume,
            Arc::clone(&paused_flag),
            Arc::clone(&played_frames),
        ) {
            Ok(out) => {
                self.current_sample_rate = out.sample_rate();
                self.audio_output = Some(out);
            }
            Err(e) => {
                tracing::warn!("Не удалось открыть аудиоустройство CPAL: {e}");
                self.audio_output = None;
            }
        }

         // Проверяем, был ли этот трек уже предзагружен в `next_prefetch_cache` 
        let prefetched_hit = self.next_prefetch_cache.lock().ok().and_then(|mut guard| {
            if guard.as_ref().is_some_and(|p| p.track_id == track.track_id) {
                guard.take()
            } else {
                None
            }
        });

        // Немедленно запускаем предзагрузку СЛЕДУЮЩЕГО трека из очереди
        self.spawn_prefetch_next();
        if self.playing_from_flow && self.queue.remaining() <= 3 {
            self.spawn_fetch_more_flow();
        }

        let tx = self.bg_tx.clone();
        let mut client = (*self.client).clone();
        let arl_opt = self.config.effective_arl();
        let preferred_quality = self.config.preferred_audio_quality();

        self.rt.spawn(async move {
            if stop_flag.load(Ordering::Relaxed) || seq_ref.load(Ordering::SeqCst) != req_id {
                return;
            }

            let (stream_opt, cover, lyrics) = if let Some(hit) = prefetched_hit {
                (
                    Some((hit.stream, hit.is_preview)),
                    hit.cover,
                    hit.lyrics,
                )
            } else {
                let mut s_opt: Option<(DeezerStream, bool)> = None;
                let mut lyr_vec = Vec::new();

                if !client.is_authenticated()
                    && let Some(arl) = arl_opt
                    && !arl.is_empty()
                {
                    let _ = client.login_arl(&arl).await;
                }

                if stop_flag.load(Ordering::Relaxed) || seq_ref.load(Ordering::SeqCst) != req_id {
                    return;
                }

                if client.is_authenticated() {
                    if let Ok(lyr) = client.get_lyrics(&track.track_id).await {
                        lyr_vec = lyr.synced;
                    }
                    let mut stream_url_res = if track.has_track_token() {
                        client.get_stream_url(&track, preferred_quality).await
                    } else {
                        Err(crate::api::models::DeezerError::TrackUnavailable(
                            "no token".into(),
                        ))
                    };
                    // Если токен устарел или отсутствовал — запрашиваем свежие метаданные трека
                    if stream_url_res.is_err()
                        && let Ok(fresh_track) = client.get_track(&track.track_id).await
                    {
                        let target_id = fresh_track
                            .fallback
                            .as_ref()
                            .filter(|fb| !fb.track_id.is_empty())
                            .map(|fb| fb.track_id.clone());
                        stream_url_res = client.get_stream_url(&fresh_track, preferred_quality).await;
                        if stream_url_res.is_err()
                            && let Some(fb_id) = target_id
                            && let Ok(fb_track) = client.get_track(&fb_id).await
                        {
                            stream_url_res =
                                client.get_stream_url(&fb_track, preferred_quality).await;
                        }
                    }

                    if let Ok((url, _q)) = stream_url_res
                        && let Ok(s) =
                            DeezerStream::fetch_streaming(client.http(), &url, &track.track_id)
                                .await
                    {
                        s_opt = Some((s, false));
                    }
                }

                if s_opt.is_none()
                    && let Some(ref preview_url) = track.preview_url
                    && let Ok(resp) = client.http().get(preview_url).send().await
                    && let Ok(bytes) = resp.bytes().await
                    && let Ok(s) = DeezerStream::try_from_decrypted(&bytes)
                {
                    s_opt = Some((s, true));
                }

                let cover_url = track
                    .cover_url_direct
                    .clone()
                    .or_else(|| mpris::cover_url(&track.album_picture));
                let mut cov = None;
                if let Some(url) = cover_url
                    && let Ok(resp) = client.http().get(&url).send().await
                    && let Ok(bytes) = resp.bytes().await
                {
                    cov = CoverArt::from_bytes(&bytes).ok();
                }

                (s_opt, cov, lyr_vec)
            };

            if stop_flag.load(Ordering::Relaxed) || seq_ref.load(Ordering::SeqCst) != req_id {
                return;
            }

            thread::spawn(move || {
                if stop_flag.load(Ordering::Relaxed) || seq_ref.load(Ordering::SeqCst) != req_id {
                    return;
                }
                match stream_opt {
                    Some((stream, is_preview)) => {
                        let fallback_dur = if is_preview {
                            30
                        } else {
                            track.duration_secs()
                        };
                        match Decoder::new_seekable_with_stop(
                            Box::new(stream),
                            audio_prod,
                            fallback_dur,
                            Arc::clone(&stop_flag),
                        ) {
                            Ok((mut dec, mut tech_info)) => {
                                if stop_flag.load(Ordering::Relaxed)
                                    || seq_ref.load(Ordering::SeqCst) != req_id
                                {
                                    return;
                                }
                                if is_preview {
                                    tech_info.duration_secs = tech_info.duration_secs.min(30);
                                }
                                let badge = tech_info.badge(is_preview);
                                let sample_rate = u64::from(tech_info.sample_rate.max(1));
                                let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<DecoderCommand>();

                                let _ = tx.send(BgEvent::TrackStarted {
                                    req_id,
                                    track: track.clone(),
                                    badge,
                                    tech_info,
                                    is_preview,
                                    vis_consumer: vis_cons,
                                    cover,
                                    lyrics,
                                    decoder_tx: Some(cmd_tx),
                                });

                                while !stop_flag.load(Ordering::Relaxed)
                                    && !dec.audio_producer.is_abandoned()
                                {
                                    while let Ok(DecoderCommand::Seek(target_sec)) =
                                        cmd_rx.try_recv()
                                    {
                                        if let Ok(actual_frames) = dec.seek(target_sec) {
                                            played_frames.store(actual_frames, Ordering::SeqCst);
                                        } else {
                                            played_frames.store(
                                                target_sec.saturating_mul(sample_rate),
                                                Ordering::SeqCst,
                                            );
                                        }
                                    }

                                    if paused_flag.load(Ordering::Relaxed) {
                                        thread::sleep(Duration::from_millis(15));
                                        continue;
                                    }

                                    match dec.process_next() {
                                        Ok(Some(_)) => {}
                                        Ok(None) => {
                                            while !stop_flag.load(Ordering::Relaxed)
                                                && !dec.audio_producer.is_abandoned()
                                                && dec.audio_producer.slots()
                                                    < AUDIO_RING_BUFFER_CAPACITY
                                            {
                                                if paused_flag.load(Ordering::Relaxed) {
                                                    thread::sleep(Duration::from_millis(20));
                                                    continue;
                                                }
                                                thread::sleep(Duration::from_millis(20));
                                            }
                                            if !stop_flag.load(Ordering::Relaxed)
                                                && seq_ref.load(Ordering::SeqCst) == req_id
                                            {
                                                let _ = tx.send(BgEvent::TrackEnded { req_id });
                                            }
                                            break;
                                        }
                                        Err(_) => break,
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(BgEvent::PlaybackError(format!(
                                    "Decode error: {e}"
                                )));
                            }
                        }
                    }
                    None => {
                        let tech_info = StreamTechInfo {
                            codec_name: "SYNTH".into(),
                            sample_rate: 44_100,
                            bits_per_sample: 32,
                            total_frames: 44_100 * 30,
                            duration_secs: 30,
                            bitrate_kbps: 1411,
                            file_size_bytes: 0,
                        };
                        let _ = tx.send(BgEvent::TrackStarted {
                            req_id,
                            track: track.clone(),
                            badge: "SYNTH DEMO | 44.1 kHz".to_string(),
                            tech_info,
                            is_preview: true,
                            vis_consumer: vis_cons,
                            cover,
                            lyrics,
                            decoder_tx: None,
                        });

                        let mut phase = 0usize;
                        let chunk_frames = 1024usize;
                        let mut stereo = vec![0.0f32; chunk_frames * 2];
                        while !stop_flag.load(Ordering::Relaxed) && !audio_prod.is_abandoned() {
                            if paused_flag.load(Ordering::Relaxed) {
                                thread::sleep(Duration::from_millis(20));
                                continue;
                            }
                            for i in 0..chunk_frames {
                                let t = (phase + i) as f32 / 44_100.0;
                                let env = (t * 2.5).sin().abs();
                                let bass = (2.0 * std::f32::consts::PI * 110.0 * t).sin() * 0.45;
                                let mid =
                                    (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25 * env;
                                let high = (2.0 * std::f32::consts::PI * 2200.0 * t).sin()
                                    * 0.15
                                    * (1.0 - env);
                                let s = (bass + mid + high).clamp(-1.0, 1.0);
                                stereo[2 * i] = s;
                                stereo[2 * i + 1] = s;
                            }
                            phase = phase.wrapping_add(chunk_frames);
                            push_all_cancelable(&mut audio_prod, &stereo, &stop_flag);
                            thread::sleep(Duration::from_millis(20));
                        }
                    }
                }
            });
        });
    }

    pub fn play_next_in_queue(&mut self) {
        if let Some(next) = self.queue.next_track() {
            self.start_track_playback(next);
        } else if self.playing_from_flow {
            self.spawn_fetch_more_flow();
        }
    }

    pub fn play_prev_in_queue(&mut self) {
        if let Some(prev) = self.queue.prev_track() {
            self.start_track_playback(prev);
        }
    }

    fn trigger_search(&mut self) {
        let q = self.search.query.trim().to_string();
        if q.is_empty() {
            return;
        }
        self.search.loading = true;
        self.search.selected = 0;
        self.search_page_index = 0;
        self.search_end_reached = false;
        self.search_loading_more = false;

        // 1. Мгновенный поиск совпадений в локальной медиатеке пользователя
        let local_matches = self.search_local_tracks(&q);
        self.search_tracks = local_matches.clone();
        self.search.rows = local_matches
            .iter()
            .map(|t| SearchRow {
                track_id: t.track_id.clone(),
                artist: t.artist.clone(),
                album: t.album.clone(),
                track: t.title.clone(),
                time: t.formatted_duration(),
                added: t.formatted_date_add(),
                favorite: self.is_track_favorite(&t.track_id),
            })
            .collect();
        self.search_page_index = self.search_tracks.len();

        let tx = self.bg_tx.clone();
        let mut client = (*self.client).clone();
        let arl_opt = self.config.effective_arl();
        self.rt.spawn(async move {
            if !client.is_authenticated()
                && let Some(arl) = arl_opt
                && !arl.is_empty()
            {
                let _ = client.login_arl(&arl).await;
            }
            if let Ok(res) = client.search_paged(&q, 0, 30).await {
                let _ = tx.send(BgEvent::SearchDone(res.data));
            } else if let Ok(res) = client.search_public_paged(&q, 0, 30).await {
                let _ = tx.send(BgEvent::SearchDone(res.data));
            } else {
                let _ = tx.send(BgEvent::SearchDone(Vec::new()));
            }
        });
    }

    pub fn spawn_load_more_search(&mut self) {
        let q = self.search.query.trim().to_string();
        if q.is_empty() || self.search_loading_more || self.search_end_reached {
            return;
        }
        self.search_loading_more = true;
        let index = self.search_page_index;
        let tx = self.bg_tx.clone();
        let client = Arc::clone(&self.client);

        self.rt.spawn(async move {
            if let Ok(res) = client.search_paged(&q, index, 30).await {
                let _ = tx.send(BgEvent::SearchMoreLoaded(res.data));
            } else if let Ok(res) = client.search_public_paged(&q, index, 30).await {
                let _ = tx.send(BgEvent::SearchMoreLoaded(res.data));
            } else {
                let _ = tx.send(BgEvent::SearchMoreLoaded(Vec::new()));
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

        let tabs = ["Now Playing", "Search", "Library", "Flow", "Settings"];
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
            format!("ONLINE ({u})")
        } else if self.client.is_authenticated() {
            "ONLINE".to_string()
        } else {
            "GUEST".to_string()
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

    fn draw_global_sidebar(&self, buf: &mut Buffer, area: Rect) {
        if area.width < 4 || area.height < 2 {
            return;
        }
        let header_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        let active_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_START))
            .bold();
        let normal_style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));

        let max_w = area.width.saturating_sub(1) as usize;
        buf.set_string(
            area.x,
            area.y,
            "NAVIGATION [S]".chars().take(max_w).collect::<String>(),
            header_style,
        );

        let nav_items: [(u8, &str); 5] = [
            (1, "[1] Now Playing"),
            (2, "[2] Search"),
            (3, "[3] Library"),
            (4, "[4] Flow"),
            (5, "[5] Settings"),
        ];

        let mut y = area.y + 1;
        for (tab_id, label) in nav_items {
            if y >= area.bottom() {
                break;
            }
            let is_active = self.active_tab == tab_id;
            let line = format!("{} {}", if is_active { "▶" } else { " " }, label);
            buf.set_string(
                area.x,
                y,
                line.chars().take(max_w).collect::<String>(),
                if is_active { active_style } else { normal_style },
            );
            y += 1;

            if tab_id == 3 {
                let visible_pl = self.library.playlists.len().min(6);
                for (idx, pl) in self.library.playlists.iter().take(visible_pl).enumerate() {
                    if y >= area.bottom() {
                        break;
                    }
                    let prefix = if idx + 1 == visible_pl {
                        "    └─ "
                    } else {
                        "    ├─ "
                    };
                    let pl_line = format!("{prefix}{}", pl.title);
                    let pl_style = if self.active_tab == 3 && idx == self.library.selected_playlist
                    {
                        active_style
                    } else {
                        dim_style
                    };
                    buf.set_string(
                        area.x,
                        y,
                        pl_line.chars().take(max_w).collect::<String>(),
                        pl_style,
                    );
                    y += 1;
                }
            }
        }

        let sep_x = area.right().saturating_sub(1);
        for ry in area.top()..area.bottom() {
            buf.set_string(sep_x, ry, "│", dim_style);
        }
    }

    fn draw_global_player_bar(&self, buf: &mut Buffer, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        let y = area.y;
        let cur_str = Self::format_mm_ss(self.elapsed_secs());
        let tot_str = Self::format_mm_ss(self.current_duration_secs);
        let play_icon = if self.now_playing.paused {
            "[▶ Space]"
        } else {
            "[⏸ Space]"
        };
        let vol_pct = (self.volume * 100.0).round() as i32;
        let right_info = format!(
            "  [⏮ p] {play_icon} [⏭ n]  {}  Vol [-/+]: {vol_pct}%",
            self.now_playing.badge
        );

        let left_time = format!("{cur_str} [←] ");
        let right_time = format!(" [→] {tot_str}");
        let fixed_chars =
            left_time.chars().count() + right_time.chars().count() + right_info.chars().count();

        let text_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL));
        let accent_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_START))
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
            .bold();
        let dim_style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL));

        if (area.width as usize) > fixed_chars + 8 {
            let bar_w = (area.width as usize - fixed_chars) as u16;
            let mut x = area.x;
            buf.set_string(x, y, &left_time, text_style);
            x += left_time.chars().count() as u16;

            let cursor =
                (self.now_playing.progress.clamp(0.0, 1.0) * bar_w.saturating_sub(1) as f32) as u16;
            for bx in 0..bar_w {
                if bx < cursor {
                    buf.set_string(x + bx, y, "━", accent_style);
                } else if bx == cursor {
                    buf.set_string(x + bx, y, "●", accent_style);
                } else {
                    buf.set_string(x + bx, y, "─", dim_style);
                }
            }
            x += bar_w;

            buf.set_string(x, y, &right_time, text_style);
            x += right_time.chars().count() as u16;
            buf.set_string(x, y, &right_info, accent_style);
        } else {
            let compact = format!(
                "{cur_str}/{tot_str} [⏮ p] {play_icon} [⏭ n] {}",
                self.now_playing.badge
            );
            let truncated: String = compact.chars().take(area.width as usize).collect();
            buf.set_string(area.x, y, &truncated, accent_style);
        }
    }

    fn draw_bottom_bar(&self, buf: &mut Buffer, area: Rect) {
        let y = area.bottom() - 1;
        let style = Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bg(theme::color_from_hex(theme::BG_TERMINAL));
        let hotkeys =
            "q quit | 1-5 views | [/] search | [y] lyrics | [m] track menu | [f] flow modes | [o/r] sort | [i] info | [s] sidebar | [v] spectrum";
        let truncated: String = hotkeys.chars().take(area.width as usize).collect();
        buf.set_string(area.left(), y, &truncated, style);

        if let Some((msg, instant)) = &self.toast_message
            && instant.elapsed() < Duration::from_millis(3000)
        {
            let toast_style = Style::default()
                .fg(theme::color_from_hex(theme::BG_TERMINAL))
                .bg(theme::color_from_hex(theme::GRAD_END))
                .bold();
            let toast_text = format!(" {msg} ");
            let tw = toast_text.chars().count() as u16;
            if tw < area.width {
                let tx = area.right().saturating_sub(tw);
                buf.set_string(tx, y, &toast_text, toast_style);
            }
        }
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        if area.is_empty() {
            return;
        }
        let bg_style = Style::default()
            .bg(theme::color_from_hex(theme::BG_TERMINAL))
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        buf.set_style(area, bg_style);

        self.draw_top_bar(buf, area);
        self.draw_bottom_bar(buf, area);

        if area.height >= 3 {
            let inner = Rect::new(area.left(), area.top() + 1, area.width, area.height - 2);
            if !inner.is_empty() {
                let player_rows = if inner.height >= 5 { 1u16 } else { 0u16 };
                let spec_rows = Self::spectrum_rows_for(
                    inner.height,
                    self.config.show_spectrum,
                    self.config.effective_spectrum_rows(),
                );
                let footer_h = spec_rows + player_rows;
                let viewport_h = inner.height.saturating_sub(footer_h);

                let main_area = Rect::new(inner.x, inner.y, inner.width, viewport_h);

                let content_area = if !self.config.sidebar_collapsed && main_area.width >= 64 {
                    let sb_w = 22u16;
                    let sb_rect = Rect::new(main_area.x, main_area.y, sb_w, main_area.height);
                    self.draw_global_sidebar(buf, sb_rect);
                    Rect::new(
                        main_area.x + sb_w,
                        main_area.y,
                        main_area.width - sb_w,
                        main_area.height,
                    )
                } else {
                    main_area
                };

                if !content_area.is_empty() {
                    match self.active_tab {
                        1 => (&self.now_playing).render(content_area, buf),
                        2 => (&self.search).render(content_area, buf),
                        3 => (&self.library).render(content_area, buf),
                        4 => (&self.flow_lyrics).render(content_area, buf),
                        5 => (&self.settings).render(content_area, buf),
                        _ => {}
                    }
                }

                if player_rows > 0 {
                    let player_rect = Rect::new(inner.x, inner.y + viewport_h, inner.width, 1);
                    self.draw_global_player_bar(buf, player_rect);
                }

                if spec_rows > 0 {
                    let spec_rect = Rect::new(
                        inner.x,
                        inner.y + viewport_h + player_rows,
                        inner.width,
                        spec_rows,
                    );
                    (&self.global_spectrum).render(spec_rect, buf);
                }

                if self.show_track_info_modal {
                    let lines = self.track_info_lines();
                    let mw = 68.min(inner.width).max(3);
                    let mh = (lines.len() as u16 + 3).min(inner.height).max(3);
                    let mx = inner.x + (inner.width - mw) / 2;
                    let my = inner.y + (inner.height - mh) / 2;
                    let m_area = Rect::new(mx, my, mw, mh);
                    (&Modal::new(
                        "TRACK & STREAM INFO [i / Esc to close]",
                        lines,
                        usize::MAX,
                    ))
                        .render(m_area, buf);
                }

                if self.show_arl_modal {
                    let mw = 60.min(inner.width).max(3);
                    let mh = 6.min(inner.height).max(3);
                    let mx = inner.x + (inner.width - mw) / 2;
                    let my = inner.y + (inner.height - mh) / 2;
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

                if self.show_track_menu {
                    let target_name = self
                        .track_menu_target
                        .as_ref()
                        .map(|t| format!("{} — {}", t.title, t.artist))
                        .unwrap_or_else(|| "Track".into());
                    let is_fav = self
                        .track_menu_target
                        .as_ref()
                        .map(|t| self.is_track_favorite(&t.track_id))
                        .unwrap_or(false);
                    let fav_opt = if is_fav {
                        "💔  Удалить из Любимых треков"
                    } else {
                        "❤️   Добавить в Любимые треки"
                    };
                    let options = vec![
                        fav_opt.to_string(),
                        "➕  Добавить в плейлист...".to_string(),
                        "📻  Запустить Микс по треку".to_string(),
                        "ℹ️   Информация о треке и потоке".to_string(),
                        "✕   Закрыть меню [Esc]".to_string(),
                    ];
                    let title = format!(" МЕНЮ ТРЕКА: {target_name} ");
                    let mw = (title.chars().count() as u16 + 6).min(inner.width).max(48);
                    let mh = (options.len() as u16 + 3).min(inner.height).max(5);
                    let mx = inner.x + (inner.width.saturating_sub(mw)) / 2;
                    let my = inner.y + (inner.height.saturating_sub(mh)) / 2;
                    let m_area = Rect::new(mx, my, mw, mh);
                    (&Modal::new(title, options, self.track_menu_selected)).render(m_area, buf);
                }

                if self.show_playlist_picker {
                    let rows: Vec<String> = self
                        .library
                        .playlists
                        .iter()
                        .enumerate()
                        .map(|(i, p)| format!("{}. {} ({} треков)", i + 1, p.title, p.num_tracks))
                        .collect();
                    let mw = 56.min(inner.width).max(3);
                    let mh = (rows.len() as u16 + 3).clamp(5, inner.height.saturating_sub(2));
                    let mx = inner.x + (inner.width.saturating_sub(mw)) / 2;
                    let my = inner.y + (inner.height.saturating_sub(mh)) / 2;
                    let m_area = Rect::new(mx, my, mw, mh);
                    (&Modal::new(
                        " ВЫБЕРИТЕ ПЛЕЙЛИСТ [Enter: Добавить, Esc: Отмена] ",
                        rows,
                        self.playlist_picker_selected,
                    ))
                        .render(m_area, buf);
                }

                if self.show_flow_mode_modal {
                    let rows: Vec<String> = FLOW_MODES
                        .iter()
                        .map(|(label, _, _, _)| label.to_string())
                        .collect();
                    let mw = 64.min(inner.width).max(3);
                    let mh = (rows.len() as u16 + 3).clamp(6, inner.height.saturating_sub(2));
                    let mx = inner.x + (inner.width.saturating_sub(mw)) / 2;
                    let my = inner.y + (inner.height.saturating_sub(mh)) / 2;
                    let m_area = Rect::new(mx, my, mw, mh);
                    (&Modal::new(
                        " ВЫБОР РЕЖИМА FLOW [Enter: Включить, Esc: Отмена] ",
                        rows,
                        self.flow_mode_selected,
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
                let desired_bars = ((area.width as usize).div_ceil(2)).clamp(8, 160);
                if self.cavacore.num_bars() != desired_bars && area.width >= 16 {
                    self.cavacore.set_num_bars(desired_bars);
                    let resized_bars = self.cavacore.process(&[]);
                    self.global_spectrum = Spectrum::new(desired_bars);
                    self.global_spectrum.set_bars(resized_bars);
                    self.now_playing.spectrum = Spectrum::new(desired_bars);
                    self.now_playing.spectrum.set_bars(resized_bars);
                    self.flow_lyrics.spectrum = Spectrum::new(desired_bars);
                    self.flow_lyrics.spectrum.set_bars(resized_bars);
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

    pub fn handle_key_event(&mut self, k: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

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
                    self.save_config();
                    self.sync_settings_view();
                    self.spawn_initial_load();
                }
                KeyCode::Char(c) if !k.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.arl_input.push(c);
                }
                _ => {}
            }
            return Ok(());
        }

        if self.show_track_info_modal {
            match k.code {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('i') | KeyCode::Char('I') => {
                    self.show_track_info_modal = false;
                }
                _ => {}
            }
            return Ok(());
        }

        if self.show_track_menu {
            let options_count = 5;
            match k.code {
                KeyCode::Esc | KeyCode::Char('m') | KeyCode::Char('M') => {
                    self.show_track_menu = false;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.track_menu_selected > 0 {
                        self.track_menu_selected -= 1;
                    } else {
                        self.track_menu_selected = options_count - 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.track_menu_selected + 1 < options_count {
                        self.track_menu_selected += 1;
                    } else {
                        self.track_menu_selected = 0;
                    }
                }
                KeyCode::Enter => {
                    self.execute_track_menu_action();
                }
                _ => {}
            }
            return Ok(());
        }

        if self.show_playlist_picker {
            let count = self.library.playlists.len();
            match k.code {
                KeyCode::Esc => {
                    self.show_playlist_picker = false;
                    self.show_track_menu = true;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.playlist_picker_selected > 0 {
                        self.playlist_picker_selected -= 1;
                    } else if count > 0 {
                        self.playlist_picker_selected = count - 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.playlist_picker_selected + 1 < count {
                        self.playlist_picker_selected += 1;
                    } else {
                        self.playlist_picker_selected = 0;
                    }
                }
                KeyCode::Enter => {
                    self.execute_add_to_selected_playlist();
                }
                _ => {}
            }
            return Ok(());
        }

        if self.show_flow_mode_modal {
            let count = FLOW_MODES.len();
            match k.code {
                KeyCode::Esc | KeyCode::Char('f') | KeyCode::Char('F') => {
                    self.show_flow_mode_modal = false;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.flow_mode_selected > 0 {
                        self.flow_mode_selected -= 1;
                    } else {
                        self.flow_mode_selected = count - 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.flow_mode_selected + 1 < count {
                        self.flow_mode_selected += 1;
                    } else {
                        self.flow_mode_selected = 0;
                    }
                }
                KeyCode::Enter => {
                    self.execute_select_flow_mode();
                }
                _ => {}
            }
            return Ok(());
        }

        if self.active_tab == 5 && self.settings.editing_arl {
            match k.code {
                KeyCode::Esc => {
                    self.settings.editing_arl = false;
                    self.sync_settings_view();
                }
                KeyCode::Backspace => {
                    self.settings.arl_input.pop();
                }
                KeyCode::Enter => {
                    self.settings.editing_arl = false;
                    let trimmed = self.settings.arl_input.trim().to_string();
                    self.arl_input = trimmed.clone();
                    self.config.arl = if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed)
                    };
                    self.save_config();
                    self.sync_settings_view();
                    self.spawn_initial_load();
                }
                KeyCode::Char(c)
                    if !k.modifiers.contains(KeyModifiers::CONTROL)
                        && !k.modifiers.contains(KeyModifiers::ALT) =>
                {
                    self.settings.arl_input.push(c);
                }
                _ => {}
            }
            return Ok(());
        }

        match k.code {
            KeyCode::F(1) => {
                self.set_search_input_focus(false);
                self.active_tab = 1;
                return Ok(());
            }
            KeyCode::F(2) => {
                self.active_tab = 2;
                return Ok(());
            }
            KeyCode::F(3) => {
                self.set_search_input_focus(false);
                self.active_tab = 3;
                return Ok(());
            }
            KeyCode::F(4) => {
                self.set_search_input_focus(false);
                self.active_tab = 4;
                return Ok(());
            }
            KeyCode::F(5) => {
                self.set_search_input_focus(false);
                self.sync_settings_view();
                self.active_tab = 5;
                return Ok(());
            }
            _ => {}
        }

        if self.active_tab == 2 && self.search_input_focused {
            match k.code {
                KeyCode::Enter => {
                    self.set_search_input_focus(false);
                    self.trigger_search();
                }
                KeyCode::Esc => {
                    self.set_search_input_focus(false);
                }
                KeyCode::Backspace => {
                    self.search.query.pop();
                }
                KeyCode::Char(c)
                    if !k.modifiers.contains(KeyModifiers::CONTROL)
                        && !k.modifiers.contains(KeyModifiers::ALT) =>
                {
                    self.search.query.push(c);
                }
                _ => {}
            }
            return Ok(());
        }

        match k.code {
            KeyCode::Tab => {
                if self.active_tab == 3 && !self.library.show_choose_modal {
                    self.library.toggle_focus();
                } else {
                    self.active_tab = (self.active_tab % 5) + 1;
                    if self.active_tab == 5 {
                        self.sync_settings_view();
                    }
                }
            }
            KeyCode::BackTab => {
                if self.active_tab == 3 && !self.library.show_choose_modal {
                    self.library.toggle_focus();
                } else {
                    self.active_tab = if self.active_tab <= 1 {
                        5
                    } else {
                        self.active_tab - 1
                    };
                    if self.active_tab == 5 {
                        self.sync_settings_view();
                    }
                }
            }
            KeyCode::Char('/') => {
                self.active_tab = 2;
                self.set_search_input_focus(true);
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                if self.active_tab == 2 {
                    self.set_search_input_focus(true);
                } else {
                    self.show_flow_mode_modal = true;
                    self.flow_mode_selected = self.current_flow_mode_idx;
                }
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.show_flow_mode_modal = true;
                self.flow_mode_selected = self.current_flow_mode_idx;
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if let Some(track) = self.active_target_track() {
                    self.track_menu_target = Some(track);
                    self.track_menu_selected = 0;
                    self.show_track_menu = true;
                } else {
                    self.set_toast("Нет выбранного трека для меню");
                }
            }
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('1') => self.active_tab = 1,
            KeyCode::Char('2') => self.active_tab = 2,
            KeyCode::Char('3') => self.active_tab = 3,
            KeyCode::Char('4') => self.active_tab = 4,
            KeyCode::Char('5') | KeyCode::Char(',') => {
                self.sync_settings_view();
                self.active_tab = 5;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.show_track_info_modal = !self.show_track_info_modal;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.toggle_lyrics();
            }
            KeyCode::Char('o') | KeyCode::Char('O')
                if self.active_tab == 3 && !self.library.show_choose_modal =>
            {
                self.library.cycle_sort_by();
                self.resort_active_library_playlist();
            }
            KeyCode::Char('r') | KeyCode::Char('R')
                if self.active_tab == 3 && !self.library.show_choose_modal =>
            {
                self.library.toggle_sort_direction();
                self.resort_active_library_playlist();
            }
            KeyCode::Char('L') => {
                self.sync_settings_view();
                self.active_tab = 5;
                self.settings.selected_row = 1;
                self.settings.editing_arl = true;
            }
            KeyCode::Char('l') => {
                if self.active_tab == 3 && !self.library.show_choose_modal {
                    self.library.focus = LibraryFocus::Tracks;
                } else {
                    self.sync_settings_view();
                    self.active_tab = 5;
                    self.settings.selected_row = 1;
                }
            }
            KeyCode::Char('h') if self.active_tab == 3 && !self.library.show_choose_modal => {
                self.library.focus = LibraryFocus::Playlists;
            }
            KeyCode::Char('a') if self.active_tab == 3 => {
                self.library.modal_selected = self.library.selected_playlist;
                self.library.show_choose_modal = !self.library.show_choose_modal;
            }
            KeyCode::Char(' ') => self.toggle_play_pause(),
            KeyCode::Char('n') | KeyCode::Char('>') => self.play_next_in_queue(),
            KeyCode::Char('p') | KeyCode::Char('<') => self.play_prev_in_queue(),
            KeyCode::Char('v') | KeyCode::Char('V') => self.toggle_spectrum(),
            KeyCode::Char('s') | KeyCode::Char('S') => self.toggle_sidebar(),
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
            KeyCode::PageDown => {
                if self.active_tab == 2 && !self.search.rows.is_empty() {
                    self.search.selected =
                        (self.search.selected + 10).min(self.search.rows.len().saturating_sub(1));
                    if self.search.selected + 5 >= self.search.rows.len() {
                        self.spawn_load_more_search();
                    }
                }
            }
            KeyCode::PageUp => {
                if self.active_tab == 2 && !self.search.rows.is_empty() {
                    self.search.selected = self.search.selected.saturating_sub(10);
                }
            }
            KeyCode::Left => self.navigate_left(),
            KeyCode::Right => self.navigate_right(),
            KeyCode::Enter => self.press_enter(),
            KeyCode::Esc => self.press_escape(),
            _ => {}
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use crossterm::event::{self, Event, KeyEventKind};
        if let Ok(true) = event::poll(Duration::from_nanos(0))
            && let Ok(Event::Key(k)) = event::read()
            && k.kind == KeyEventKind::Press
        {
            self.handle_key_event(k)?;
        }
        Ok(())
    }

    pub fn toggle_play_pause(&mut self) {
        let new_paused = !self.now_playing.paused;
        self.now_playing.paused = new_paused;
        self.search.player.paused = new_paused;
        self.library.player.paused = new_paused;
        self.flow_lyrics.paused = new_paused;
        self.paused_flag.store(new_paused, Ordering::SeqCst);
    }

    pub fn toggle_spectrum(&mut self) {
        self.config.show_spectrum = !self.config.show_spectrum;
        self.settings.show_spectrum = self.config.show_spectrum;
        self.save_config();
    }

    pub fn toggle_lyrics(&mut self) {
        self.config.show_lyrics = !self.config.show_lyrics;
        self.now_playing.show_lyrics = self.config.show_lyrics;
        self.flow_lyrics.show_lyrics = self.config.show_lyrics;
        self.save_config();
    }

    pub fn toggle_db_scale(&mut self) {
        self.config.db_scale = !self.config.db_scale;
        self.settings.db_scale = self.config.db_scale;
        self.cavacore.set_db_scale(self.config.db_scale);
        self.save_config();
    }

    /// Высота панели спектра: AUTO даёт 4–6 строк по высоте окна, ручная высота
    /// из Settings ограничивается свободной высотой (минус строка плеера).
    fn spectrum_rows_for(inner_height: u16, show: bool, want: Option<u16>) -> u16 {
        if !show {
            return 0;
        }
        let player_rows = if inner_height >= 5 { 1u16 } else { 0u16 };
        let auto = if inner_height >= 22 {
            6u16
        } else if inner_height >= 12 {
            4u16
        } else {
            0u16
        };
        match want {
            None => auto,
            Some(n) if inner_height >= 12 => n.min(inner_height.saturating_sub(player_rows)),
            Some(_) => 0,
        }
    }

    /// Сохраняет конфигурацию, только если она была загружена с диска.
    fn save_config(&mut self) {
        if self.persist_config {
            let _ = self.config.save();
        }
    }

    /// Переключает шкалу высот спектра и применяет её к `Cavacore` сразу.
    pub fn cycle_spectrum_scale(&mut self, forward: bool) {
        let next = self.settings.cycle_spectrum_scale(forward);
        self.config.spectrum_scale = next.as_str().to_string();
        self.cavacore.set_scale_mode(next);
        self.save_config();
    }

    /// Переключает высоту панели спектра (AUTO -> 4 -> 6 -> ... -> 12).
    pub fn cycle_spectrum_rows(&mut self, forward: bool) {
        let next = self.settings.cycle_spectrum_rows(forward);
        self.config.spectrum_rows = next;
        self.save_config();
    }

    pub fn toggle_sidebar(&mut self) {
        self.config.sidebar_collapsed = !self.config.sidebar_collapsed;
        self.save_config();
    }

    pub fn seek(&mut self, forward: bool) {
        let step_secs = 5i64;
        let cur_secs = self.elapsed_secs() as i64;
        let max_secs = self.current_duration_secs as i64;
        let target_secs = if forward {
            (cur_secs + step_secs).clamp(0, max_secs) as u64
        } else {
            (cur_secs - step_secs).clamp(0, max_secs) as u64
        };

        let sr = u64::from(self.current_sample_rate.max(1));
        self.played_frames
            .store(target_secs.saturating_mul(sr), Ordering::SeqCst);

        if let Some(ref tx) = self.decoder_tx {
            let _ = tx.send(DecoderCommand::Seek(target_secs));
        }

        let total = self.current_duration_secs.max(1) as f32;
        let next_prog = (target_secs as f32 / total).clamp(0.0, 1.0);
        self.now_playing.progress = next_prog;
        self.search.player.progress = next_prog;
        self.library.player.progress = next_prog;
        let pos_ms = target_secs * 1000;
        self.now_playing.position_ms = pos_ms;
        self.flow_lyrics.position_ms = pos_ms;
    }

    fn adjust_setting(&mut self, forward: bool) {
        match self.settings.selected_row {
            0 => {
                let next_q = self.settings.cycle_quality(forward);
                self.config.preferred_quality = next_q;
                self.save_config();
            }
            1 => {
                self.settings.editing_arl = true;
            }
            2 => {
                self.toggle_spectrum();
            }
            3 => {
                self.toggle_db_scale();
            }
            4 => {
                self.cycle_spectrum_scale(forward);
            }
            5 => {
                self.cycle_spectrum_rows(forward);
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
                if self.library.show_choose_modal {
                    if self.library.modal_selected > 0 {
                        self.library.modal_selected -= 1;
                    }
                } else {
                    match self.library.focus {
                        LibraryFocus::Playlists => {
                            if self.library.selected_playlist > 0 {
                                self.library.selected_playlist -= 1;
                                self.library.selected_track = 0;
                                if let Some(pl_id) = self
                                    .library
                                    .playlists
                                    .get(self.library.selected_playlist)
                                    .map(|p| p.id.clone())
                                {
                                    self.spawn_load_playlist_tracks(pl_id);
                                }
                            }
                        }
                        LibraryFocus::Tracks => {
                            if self.library.selected_track > 0 {
                                self.library.selected_track -= 1;
                            }
                        }
                    }
                }
            }
            4 if self.flow_lyrics.selected_flow > 0 => self.flow_lyrics.selected_flow -= 1,
            5 if self.settings.selected_row > 0 => self.settings.selected_row -= 1,
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
                if self.search.selected + 5 >= self.search.rows.len() {
                    self.spawn_load_more_search();
                }
            }
            3 => {
                if self.library.show_choose_modal {
                    if self.library.modal_selected + 1 < self.library.playlists.len() {
                        self.library.modal_selected += 1;
                    }
                } else {
                    match self.library.focus {
                        LibraryFocus::Playlists => {
                            if self.library.selected_playlist + 1 < self.library.playlists.len() {
                                self.library.selected_playlist += 1;
                                self.library.selected_track = 0;
                                if let Some(pl_id) = self
                                    .library
                                    .playlists
                                    .get(self.library.selected_playlist)
                                    .map(|p| p.id.clone())
                                {
                                    self.spawn_load_playlist_tracks(pl_id);
                                }
                            }
                        }
                        LibraryFocus::Tracks => {
                            if let Some(entry) =
                                self.library.playlists.get(self.library.selected_playlist)
                                && self.library.selected_track + 1 < entry.tracks.len()
                            {
                                self.library.selected_track += 1;
                            }
                        }
                    }
                }
            }
            4 if !self.flow_lyrics.flow.is_empty() => {
                if self.flow_lyrics.selected_flow + 1 < self.flow_lyrics.flow.len() {
                    self.flow_lyrics.selected_flow += 1;
                }
                if self.flow_lyrics.selected_flow + 3 >= self.flow_lyrics.flow.len() {
                    self.spawn_fetch_more_flow();
                }
            }
            5 if self.settings.selected_row < 5 => self.settings.selected_row += 1,
            _ => {}
        }
    }

    fn navigate_left(&mut self) {
        if self.active_tab == 3 && !self.library.show_choose_modal {
            self.library.toggle_focus();
        } else if self.active_tab == 5 && self.settings.selected_row != 1 {
            self.adjust_setting(false);
        } else {
            self.seek(false);
        }
    }

    fn navigate_right(&mut self) {
        if self.active_tab == 3 && !self.library.show_choose_modal {
            self.library.toggle_focus();
        } else if self.active_tab == 5 && self.settings.selected_row != 1 {
            self.adjust_setting(true);
        } else {
            self.seek(true);
        }
    }

    fn press_enter(&mut self) {
        match self.active_tab {
            1 => {
                self.playing_from_flow = true;
                if let Some(track) = self.queue.set_queue(self.flow_tracks.clone(), 0) {
                    self.start_track_playback(track);
                } else {
                    let demo = TrackData {
                        track_id: "3135556".into(),
                        title: "Harder, Better, Faster, Stronger".into(),
                        artist: "Daft Punk".into(),
                        album: "Discovery".into(),
                        duration: "3:44".into(),
                        ..Default::default()
                    };
                    self.queue.set_queue(vec![demo.clone()], 0);
                    self.start_track_playback(demo);
                }
            }
            2 => {
                self.playing_from_flow = false;
                if self.search.rows.is_empty() && self.search.query.trim().is_empty() {
                    self.set_search_input_focus(true);
                } else if !self.search.rows.is_empty()
                    && let Some(track) = self
                        .queue
                        .set_queue(self.search_tracks.clone(), self.search.selected)
                {
                    self.start_track_playback(track);
                } else if !self.search.query.trim().is_empty() {
                    self.trigger_search();
                }
            }
            3 => {
                self.playing_from_flow = false;
                if self.library.show_choose_modal {
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
                } else if self.library.focus == LibraryFocus::Playlists {
                    if let Some(pl_id) = self
                        .library
                        .playlists
                        .get(self.library.selected_playlist)
                        .map(|p| p.id.clone())
                    {
                        self.spawn_load_playlist_tracks(pl_id);
                    }
                    self.library.focus = LibraryFocus::Tracks;
                } else if let Some(track) = self
                    .queue
                    .set_queue(self.library_tracks.clone(), self.library.selected_track)
                {
                    self.start_track_playback(track);
                } else if let Some(row) = self
                    .library
                    .playlists
                    .get(self.library.selected_playlist)
                    .and_then(|p| p.tracks.get(self.library.selected_track))
                {
                    let track = TrackData {
                        track_id: "0".into(),
                        title: row.track.clone(),
                        artist: row.artist.clone(),
                        album: row.album.clone(),
                        duration: row.time.clone(),
                        ..Default::default()
                    };
                    self.queue.set_queue(vec![track.clone()], 0);
                    self.start_track_playback(track);
                }
            }
            4 => {
                self.playing_from_flow = true;
                if let Some(track) = self
                    .queue
                    .set_queue(self.flow_tracks.clone(), self.flow_lyrics.selected_flow)
                {
                    self.start_track_playback(track);
                } else if let Some(row) = self.flow_lyrics.flow.get(self.flow_lyrics.selected_flow)
                {
                    let track = TrackData {
                        track_id: "0".into(),
                        title: row.title.clone(),
                        artist: row.artist.clone(),
                        album: "Deezer Flow".into(),
                        duration: "3:30".into(),
                        ..Default::default()
                    };
                    self.queue.set_queue(vec![track.clone()], 0);
                    self.start_track_playback(track);
                }
            }
            5 => {
                self.adjust_setting(true);
            }
            _ => {}
        }
    }

    fn press_escape(&mut self) {
        if self.show_track_menu {
            self.show_track_menu = false;
        } else if self.show_playlist_picker {
            self.show_playlist_picker = false;
            self.show_track_menu = true;
        } else if self.show_flow_mode_modal {
            self.show_flow_mode_modal = false;
        } else if self.show_arl_modal {
            self.show_arl_modal = false;
        } else if self.show_track_info_modal {
            self.show_track_info_modal = false;
        } else if self.search_input_focused {
            self.set_search_input_focus(false);
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
    use crate::dsp::cavacore::SpectrumScale;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    fn spectrum_panel_height_follows_auto_and_manual_setting() {
        // AUTO: 6 строк на высоком окне, 4 на среднем, 0 на низком.
        assert_eq!(App::spectrum_rows_for(28, true, None), 6);
        assert_eq!(App::spectrum_rows_for(21, true, None), 4);
        assert_eq!(App::spectrum_rows_for(11, true, None), 0);
        assert_eq!(App::spectrum_rows_for(28, false, Some(10)), 0);
        // Ручная высота.
        assert_eq!(App::spectrum_rows_for(28, true, Some(12)), 12);
        assert_eq!(App::spectrum_rows_for(13, true, Some(12)), 12);
        // Ограничение свободной высотой: строка плеера всегда сохраняется.
        assert_eq!(App::spectrum_rows_for(28, true, Some(30)), 27);
        assert_eq!(App::spectrum_rows_for(12, true, Some(12)), 11);
        // На низком окне ручная высота не рисуется вовсе.
        assert_eq!(App::spectrum_rows_for(11, true, Some(6)), 0);
    }

    #[test]
    fn settings_spectrum_rows_are_applied_to_layout() {
        let mut app = App::new();
        app.config.spectrum_rows = Some(12);
        let terminal = render_ui(&app, 120, 30);
        // inner.height = 28 -> панель занимает строки 17..28, верхние столбики видны на 20.
        assert!(row_has(&terminal, 20, "\u{2588}"));
        app.config.spectrum_rows = Some(4);
        let small = render_ui(&app, 120, 30);
        assert!(!row_has(&small, 20, "\u{2588}"));
    }

    #[test]
    fn settings_spectrum_scale_cycles_into_config_and_cavacore() {
        let mut app = App::new();
        app.active_tab = 5;
        app.settings.selected_row = 4;

        app.handle_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)).unwrap();
        assert_eq!(
            app.config.effective_spectrum_scale(),
            SpectrumScale::FramePeak
        );
        assert_eq!(app.cavacore.scale_mode(), SpectrumScale::FramePeak);

        app.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)).unwrap();
        assert_eq!(app.config.effective_spectrum_scale(), SpectrumScale::Legacy);
        assert_eq!(app.cavacore.scale_mode(), SpectrumScale::Legacy);

        app.handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)).unwrap();
        assert_eq!(
            app.config.effective_spectrum_scale(),
            SpectrumScale::FramePeak
        );

        app.settings.selected_row = 5;
        app.handle_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)).unwrap();
        assert_eq!(app.config.effective_spectrum_rows(), Some(4));
        app.handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)).unwrap();
        assert_eq!(app.config.effective_spectrum_rows(), None);
    }

    #[test]
    fn renders_top_and_bottom_bars_with_1_to_5_tabs() {
        let app = App::new();
        let terminal = render_ui(&app, 120, 24);
        assert!(row_has(&terminal, 0, "DEEZER-TUI"));
        assert!(row_has(&terminal, 0, "[1]Now Playing"));
        assert!(row_has(&terminal, 0, "[2]Search"));
        assert!(row_has(&terminal, 0, "[3]Library"));
        assert!(row_has(&terminal, 0, "[4]Flow"));
        assert!(row_has(&terminal, 0, "[5]Settings"));
        assert!(row_has(&terminal, 23, "q quit"));
    }

    #[test]
    fn renders_each_tab_screen_with_global_player_and_sidebar() {
        let expectations: [(u8, &str); 5] = [
            (1, "FLAC | 1411 kbps"),
            (2, "ARTISTS"),
            (3, "LIBRARY"),
            (4, "FLOW"),
            (5, "SETTINGS & AUDIO CONFIGURATION"),
        ];
        for (tab, needle) in expectations {
            let mut app = App::new();
            app.active_tab = tab;
            let terminal = render_ui(&app, 120, 40);
            assert!(row_has(&terminal, 0, "DEEZER-TUI"));
            assert!(screen_has(&terminal, needle));
            assert!(screen_has(&terminal, "NAVIGATION [S]"));
            assert!(screen_has(&terminal, "00:00 [←]"));
            assert!(screen_has(&terminal, "[⏮ p]"));
            assert!(screen_has(&terminal, "Vol [-/+]:"));
        }
    }

    #[test]
    fn toggle_lyrics_works_on_y_hotkey() {
        let mut app = App::new();
        assert!(app.config.show_lyrics);
        app.handle_key_event(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE))
            .unwrap();
        assert!(!app.config.show_lyrics);
        assert!(!app.now_playing.show_lyrics);
        assert!(!app.flow_lyrics.show_lyrics);
    }

    #[test]
    fn library_sorting_hotkeys_o_and_r_work() {
        let mut app = App::new();
        app.active_tab = 3;
        assert_eq!(app.library.sort_by, LibrarySortBy::Default);
        app.handle_key_event(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.library.sort_by, LibrarySortBy::Title);
        app.handle_key_event(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE))
            .unwrap();
        assert!(!app.library.sort_asc);
    }

    #[test]
    fn settings_tab_opens_on_comma_and_updates_config() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new(KeyCode::Char(','), KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.active_tab, 5);
        assert_eq!(app.config.preferred_quality, "FLAC");

        app.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.config.preferred_quality, "MP3_320");
    }

    #[test]
    fn track_info_modal_opens_on_i_and_renders_stream_details() {
        let mut app = App::new();
        app.current_track = Some(TrackData {
            track_id: "3135556".into(),
            title: "Harder, Better, Faster, Stronger".into(),
            artist: "Daft Punk".into(),
            album: "Discovery".into(),
            ..Default::default()
        });
        app.current_tech_info = Some(StreamTechInfo {
            codec_name: "FLAC".into(),
            sample_rate: 44_100,
            bits_per_sample: 16,
            total_frames: 44_100 * 224,
            duration_secs: 224,
            bitrate_kbps: 1411,
            file_size_bytes: 26_214_400,
        });

        app.handle_key_event(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE))
            .unwrap();
        assert!(app.show_track_info_modal);

        let terminal = render_ui(&app, 120, 36);
        assert!(screen_has(&terminal, "TRACK & STREAM INFO"));
        assert!(screen_has(&terminal, "3135556"));
        assert!(screen_has(&terminal, "25.00 MB"));

        app.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE))
            .unwrap();
        assert!(!app.show_track_info_modal);
    }

    #[test]
    fn search_modal_input_focus_and_global_hotkeys() {
        let mut app = App::new();
        app.active_tab = 2;
        assert!(!app.search_input_focused);

        let was_paused = app.now_playing.paused;
        app.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE))
            .unwrap();
        assert_ne!(app.now_playing.paused, was_paused);
        assert!(app.search.query.is_empty());

        app.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE))
            .unwrap();
        assert!(app.search_input_focused);

        for ch in ['d', 'a', 'f', 't', ' ', '1'] {
            app.handle_key_event(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
                .unwrap();
        }
        assert_eq!(app.search.query, "daft 1");
        assert_eq!(app.active_tab, 2);

        app.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE))
            .unwrap();
        assert!(!app.search_input_focused);
        app.handle_key_event(KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.active_tab, 3);
    }

    #[test]
    fn library_two_panel_focus_and_navigation() {
        let mut app = App::new();
        app.active_tab = 3;
        assert_eq!(app.library.focus, LibraryFocus::Tracks);

        assert_eq!(app.library.selected_playlist, 0);
        assert_eq!(app.library.selected_track, 0);
        app.navigate_down();
        assert_eq!(app.library.selected_track, 1);
        app.navigate_down();
        assert_eq!(
            app.library.selected_playlist, 0,
            "плейлист не должен переключаться при прокрутке таблицы треков"
        );

        app.navigate_left();
        assert_eq!(app.library.focus, LibraryFocus::Playlists);
        app.navigate_down();
        assert_eq!(app.library.selected_playlist, 1);
        assert_eq!(app.library.selected_track, 0);
    }

    #[test]
    fn sidebar_and_spectrum_toggles_work() {
        let mut app = App::new();
        assert!(!app.config.sidebar_collapsed);
        app.toggle_sidebar();
        assert!(app.config.sidebar_collapsed);
        let terminal = render_ui(&app, 120, 40);
        assert!(!screen_has(&terminal, "NAVIGATION [S]"));

        assert!(app.config.show_spectrum);
        app.toggle_spectrum();
        assert!(!app.config.show_spectrum);
    }

    #[test]
    fn playback_queue_advances_and_rewinds() {
        let mut q = PlaybackQueue::default();
        let tracks = vec![
            TrackData {
                track_id: "1".into(),
                title: "One".into(),
                ..Default::default()
            },
            TrackData {
                track_id: "2".into(),
                title: "Two".into(),
                ..Default::default()
            },
        ];
        let first = q.set_queue(tracks, 0).unwrap();
        assert_eq!(first.title, "One");
        let second = q.next_track().unwrap();
        assert_eq!(second.title, "Two");
        assert!(q.next_track().is_none());
        let prev = q.prev_track().unwrap();
        assert_eq!(prev.title, "One");
    }

    #[test]
    fn seek_updates_played_frames_and_progress() {
        let mut app = App::new();
        app.current_sample_rate = 44_100;
        app.current_duration_secs = 100;
        app.seek(true);
        assert_eq!(app.elapsed_secs(), 5);
        assert!((app.now_playing.progress - 0.05).abs() < 1e-4);
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
    fn track_context_menu_opens_navigates_and_toggles_favorite() {
        let mut app = App::new();
        app.active_tab = 1;
        app.current_track = Some(TrackData {
            track_id: "42".into(),
            title: "Test Track".into(),
            artist: "Test Artist".into(),
            ..Default::default()
        });

        // Открытие меню по 'm'
        app.handle_key_event(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE))
            .unwrap();
        assert!(app.show_track_menu);
        assert_eq!(app.track_menu_selected, 0);

        // Навигация вниз/вверх
        app.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.track_menu_selected, 1);
        app.handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.track_menu_selected, 0);

        // Добавление в избранное по Enter
        assert!(!app.is_track_favorite("42"));
        app.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .unwrap();
        assert!(!app.show_track_menu);
        assert!(app.is_track_favorite("42"));
        assert!(app.toast_message.is_some());

        // Повторное открытие и закрытие по Esc
        app.handle_key_event(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE))
            .unwrap();
        assert!(app.show_track_menu);
        app.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE))
            .unwrap();
        assert!(!app.show_track_menu);
    }

    #[test]
    fn playlist_picker_modal_and_add_to_playlist() {
        let mut app = App::new();
        app.active_tab = 1;
        app.current_track = Some(TrackData {
            track_id: "99".into(),
            title: "Song 99".into(),
            artist: "Artist 99".into(),
            album: "Album 99".into(),
            ..Default::default()
        });
        app.library.playlists = vec![PlaylistEntry {
            id: "pl_1".into(),
            title: "Road Trip".into(),
            num_tracks: 0,
            tracks: Vec::new(),
        }];

        // Открываем меню трека
        app.handle_key_event(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE))
            .unwrap();
        // Переходим на пункт 1: "Добавить в плейлист..."
        app.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
            .unwrap();
        app.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .unwrap();

        assert!(!app.show_track_menu);
        assert!(app.show_playlist_picker);
        assert_eq!(app.playlist_picker_selected, 0);

        // Выбираем плейлист
        app.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .unwrap();
        assert!(!app.show_playlist_picker);
        assert_eq!(app.library.playlists[0].num_tracks, 1);
        assert_eq!(app.library.playlists[0].tracks[0].track, "Song 99");
    }

    #[test]
    fn flow_mode_modal_selects_mood_and_genre() {
        let mut app = App::new();
        app.active_tab = 4; // Flow view

        // Нажатие 'f' открывает выбор режимов Flow
        app.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE))
            .unwrap();
        assert!(app.show_flow_mode_modal);
        assert_eq!(app.flow_mode_selected, 0);

        // Листаем до Настроения: Chill (индекс 1)
        app.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
            .unwrap();
        assert_eq!(app.flow_mode_selected, 1);

        // Выбираем по Enter
        app.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .unwrap();
        assert!(!app.show_flow_mode_modal);
        assert!(app.flow_lyrics.flow_mode_label.contains("Chill"));
        assert_eq!(app.current_flow_mode_idx, 1);
    }

    #[test]
    fn cavacore_exact_octave_frequencies_15_and_30() {
        use crate::dsp::cavacore::bar_boundaries;

        let b15 = bar_boundaries(15);
        assert_eq!(b15.len(), 16);
        assert!((b15[0] - 20.0).abs() < 1e-3);
        assert!((b15[15] - 20_000.0).abs() < 1e-2);

        // Проверяем соответствие 15 стандартным полосам эквалайзера 2/3 октавы
        // 1: 20-31.7 Гц (~25 Гц центр)
        assert!((b15[1] - 31.7).abs() < 0.5);
        // 2: 31.7-50.2 Гц (~40 Гц центр)
        assert!((b15[2] - 50.2).abs() < 0.5);
        // 3: 50.2-79.6 Гц (~63 Гц центр)
        assert!((b15[3] - 79.6).abs() < 0.5);
        // 4: 79.6-126.2 Гц (~100 Гц центр)
        assert!((b15[4] - 126.2).abs() < 0.5);
        // 9: 796-1262 Гц (~1 кГц центр)
        assert!((b15[9] - 1261.9).abs() < 2.0);
        // 15: 12619-20000 Гц (~16 кГц центр)
        assert!((b15[14] - 12619.1).abs() < 10.0);

        let b30 = bar_boundaries(30);
        assert_eq!(b30.len(), 31);
        assert!((b30[0] - 20.0).abs() < 1e-3);
        assert!((b30[30] - 20_000.0).abs() < 1e-2);
        // Шаг 1/3 октавы (половина шага 2/3)
        assert!((b30[2] - b15[1]).abs() < 1e-3);
        assert!((b30[4] - b15[2]).abs() < 1e-3);
    }

    #[test]
    fn renders_on_narrow_terminal() {
        let app = App::new();
        let terminal = render_ui(&app, 30, 8);
        assert!(row_has(&terminal, 0, "DEEZER"));
        assert!(row_has(&terminal, 7, "q quit"));
    }

    #[test]
    fn search_local_tracks_finds_artist_and_title_with_hyphen() {
        let mut app = App::new();
        let favorite_track = TrackData {
            track_id: "3862908051".into(),
            title: "Перекуп".into(),
            artist: "BRUTALLIZM".into(),
            album: "НА ТАРАН!".into(),
            ..Default::default()
        };
        app.library_tracks.push(favorite_track.clone());

        // 1. Прямой вызов search_local_tracks с дефисом
        let matches = app.search_local_tracks("Перекуп - BRUTALLIZM");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].track_id, "3862908051");
        assert_eq!(matches[0].title, "Перекуп");
        assert_eq!(matches[0].artist, "BRUTALLIZM");

        // 2. Обратный порядок "BRUTALLIZM - Перекуп"
        let matches_rev = app.search_local_tracks("BRUTALLIZM - Перекуп");
        assert_eq!(matches_rev.len(), 1);
        assert_eq!(matches_rev[0].track_id, "3862908051");

        // 3. trigger_search немедленно заполняет search_tracks и search.rows
        app.search.query = "Перекуп - BRUTALLIZM".into();
        app.trigger_search();
        assert_eq!(app.search_tracks.len(), 1);
        assert_eq!(app.search.rows.len(), 1);
        assert_eq!(app.search.rows[0].track_id, "3862908051");
        assert_eq!(app.search.rows[0].track, "Перекуп");
        assert!(app.search.rows[0].favorite);
    }

    #[test]
    fn search_pagination_and_more_loading() {
        let mut app = App::new();
        app.active_tab = 2; // Search tab

        // Создаем 10 треков в поиске
        for i in 0..10 {
            let t = TrackData {
                track_id: format!("id_{i}"),
                title: format!("Track {i}"),
                artist: "Artist".into(),
                album: "Album".into(),
                ..Default::default()
            };
            app.search_tracks.push(t);
            app.search.rows.push(SearchRow {
                track_id: format!("id_{i}"),
                artist: "Artist".into(),
                album: "Album".into(),
                track: format!("Track {i}"),
                time: "3:00".into(),
                added: "2026-11-16".into(),
                favorite: false,
            });
        }
        app.search_page_index = 10;
        assert_eq!(app.search.selected, 0);

        // Листаем вниз до 6-го элемента (selected = 5) -> selected + 5 >= 10
        for _ in 0..5 {
            app.navigate_down();
        }
        assert_eq!(app.search.selected, 5);

        // Эмулируем приход следующей страницы
        let more = vec![TrackData {
            track_id: "id_10".into(),
            title: "Track 10".into(),
            artist: "Artist".into(),
            album: "Album".into(),
            ..Default::default()
        }];
        app.bg_tx.send(BgEvent::SearchMoreLoaded(more)).unwrap();
        app.poll_bg_events();
        assert_eq!(app.search_tracks.len(), 11);
        assert_eq!(app.search.rows.len(), 11);
        assert_eq!(app.search.rows[10].track_id, "id_10");
        assert_eq!(app.search_page_index, 11);
    }
}
