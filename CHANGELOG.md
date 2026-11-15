# Changelog

История версий deezer-tui. Детали по каждой фазе — в [docs/crates/progress.md](docs/crates/progress.md),
спецификации — в [ARCHITECTURE.md](ARCHITECTURE.md).

## [v2.2] — 2026-11-16 — Релиз: дебаг по диагнозу (4 проблемы)

### Фиксы
- **Фриз при быстром `n`/`Enter` в Flow** — `DeezerStream` переведён с `File::try_clone()`
  на in-memory буфер `Arc<RwLock<Vec<u8>>>` + независимый курсор `pos: usize`;
  добавлен счётчик поколений `playback_seq: Arc<AtomicU64>` — устаревшие
  `TrackStarted`/`TrackEnded` отбрасываются, гонка двух FD на один OS offset устранена.
- **EQ не масштабируется, «провалы» между барами** — Cavacore: дробная линейная
  интерполяция между соседними FFT-бинами для узких полос; новый вес
  `eq = (norm_fft * freq_tilt) / bin_count.sqrt()`, `freq_tilt = (f_hi/100)^0.55`;
  `Spectrum` — плавное ресемплирование `sample_slice` + `fit_bars = div_ceil(w/2).min(bars.len())`.
- **Не было колонки ADDED в Library** — `TrackData::formatted_date_add()` (YYYY-MM-DD,
  civil calendar), колонка `ADDED` (ширина 12, видна при ширине ≥ 68) с сортировкой
  `ADDED ▲/▼`, `SearchRow + added`.
- **Player-строка без подсказок + мелкие обложки 264×264** — player row с хоткеями
  `02:14 [←] ━━●━ [→] 03:02 [⏮ p] [⏸ Space] [⏭ n] FLAC Vol [-/+]: 100%`, нижняя строка
  хоткеев; обложки 500×500, quad subpixel 2×2 (`QUADRANT_GLYPHS`, `best_quadrant_cell`).

### Добавлено
- Переиспользование сессии `self.client` (один `DeezerClient`, `login_arl` только при
  `!is_authenticated()`), `BgEvent::FlowLoadFailed`, `.take(15)` убран (flow 1:1).
- Адаптивный спектр `div_ceil(width/2).clamp(8, 160)` + ресемплинг при ресайзе.
- Обложки Now Playing `clamp(4, 26)`, Flow `min(14)`.
- tokio `worker_threads(4)`.

### Тесты
105/105 (число не изменилось — тесты обновлены in-place), clippy clean.

## [v2.1] — 2026-11-15 — Релиз: масштабный дебаг и доработки

### Фиксы (5 diagnosed-багов)
- **CPU-зомби при смене трека** — `push_all_cancelable` (`stop_flag`/`is_abandoned`),
  `stop_flag` в `Decoder`.
- **Тишина на старте трека** — `fetch_streaming` (пребуфер 256 КБ + background tokio).
- **Пустая Library/плейлисты** — `get_playlists` поэлементно из `deezer.pageProfile` +
  публичный fallback, алиасы `PlaylistData`, `deserialize_u64_flexible`.
- **Дубль Now Playing** — Now Playing: обложка + метаданные + LYRICS (глобальный плеер).
- **Flow-макет** — убран второй спектр, `FLOW • Infinite Recommendations [y: Lyrics]`.

### Добавлено
- `TrackData.date_add` (rename `DATE_ADD`, alias `TIMELINE`) + алиасы плейлистов.
- `show_lyrics: bool` (default true).
- Now Playing: панель LYRICS `[Y to Hide]` (автопрокрутка, подсветка).
- Flow: панель `LYRICS [Y to Hide]`, автопрокрутка.
- Library: `LibrarySortBy` (Default/Title/Artist/Album), `o`/`r`, бейдж `[Sort [o/r]]`.
- App: `PrefetchedTrack`-кэш, автоподгрузка Flow при ≤3 треках, `o`/`r`, адаптивный спектр.

### Тесты
105/105 (99→105), clippy clean.

## [v2.0] — 2026-11 — Релиз: Unified Shell + Zero-Latency Pipeline

### Фаза 6–7 — Unified Shell Layout + Zero-Latency Audio Pipeline
- `AudioOutput::new_with_sync` — синхронный вывод (пауза → тишина, stereo `pop()`,
  mono → vis, `played_frames`), CPAL-колбэк владеет Arc-клубами.
- `DecoderCommand::Seek`, `StreamTechInfo` (+`Default`, `badge`), `new_seekable`.
- Config: `preferred_quality`, `show_spectrum`, `db_scale`, `sidebar_collapsed`.
- `AppShell`: 5 вкладок, глобальный сайдбар `[S]`, глобальный плеер-бар, глобальный
  спектроанализатор, `PlaybackQueue`, `step_dsp` 60 FPS, `start_track_playback`.

### Фаза 8 — Modal Search Focus + Two-Panel Library + Playlist Loading
- `search.rs`: модал фокуса ввода `/`/`f`, `[EDITING: Enter/Esc]`, scroll-offset.
- `client.rs`: `get_playlist_tracks` (авторизованный `playlist.getSongs` + публичный fallback).
- `library.rs`: `LibraryFocus` (Playlists/Tracks), двухпанельный фокус, маркеры `▶`/`•`.
- `app.rs`: `handle_key_event` единая точка, `search_input_focused`, `playlist_tracks_cache`.

### Фаза 9 — Screen [5] Settings + Modal [i] Track & Stream Info
- `settings.rs` (новый): Preferred Audio Quality, Deezer ARL Token, Show Spectrum, dB Scale.
- `app.rs`: вкладка `[5]` (`5`/`,`/F5), редактор ARL inline, модал `[i] Track & Stream Info`.

### Тесты
99/99, clippy clean.

## [v1.x / Phase 1–5] — 2026-11 — Начальные фазы

- **Phase 1–2** — каркас, API layer (crypto, stream, models, client), theme.
- **Phase 3–4** — audio engine (stream, decoder, output), DSP cavacore.
- **Phase 5** — UI views (now_playing, search, library, flow_lyrics).
- **Audit `1_2_deezer_tui.md`** — переписывание Phase 1–2 под аудит.
- **Gemini review round 1** — фиксы 6 файлов (crypto/stream/client/theme/spectrum/modal/views/app/main).
- **Gemini review round 2** — ARL guest + audio engine wiring (files 7–8).

## [0.1] — Initial commit
Первый рабочий запуск: `5863fdc Initial commit: deezer tui client`.
