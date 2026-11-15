# Progress

## Phase 4 — API layer (crypto, stream, models, client) [x]
- [x] `src/api/crypto.rs` — 16-byte key from SNG_ID (MD5 hex + XOR `g4el58wc0zvf9na1`), `decrypt_chunk` via `cbc::Decryptor<blowfish::Blowfish>` IV `[0..8]`. 7 crypto tests green; `#[allow(dead_code)]` on public items.
- [x] `src/audio/stream.rs` — `DeezerStream` (Read + Seek + Send + Sync + MediaSource), per-chunk `chunk_idx % 3 == 0` decryption, `tempfile` for instant Seek. 4 stream tests green.
- [x] `src/api/models.rs` — `DeezerError` (thiserror), `AudioQuality`, `UserData`, `UserInfo`, `UserOptions`, `UserOffer`, `TrackData`, `TrackFallback`, `SearchResults`, `PlaylistData`, `LyricLine`, `Lyrics`, `deserialize_id_to_string` (by-value `D`). All pub items `#[allow(dead_code)]`.
- [x] `src/api/client.rs` — `Session`, `DeezerClient` (reqwest 0.13). `login_arl`, `gw_call`, `search`, favorites, playlists (add/create/remove), `get_track`, `get_flow`, `get_lyrics`, `get_stream_url`. All pub items `#[allow(dead_code)]`. Тесты парсинга JSON Deezer: `TRACK`-массив → `Vec<TrackData>` (числовой SNG_ID→String, TRACK_TOKEN, NB_LISTEN), `getUserData` → `UserData` (USER_ID/OPTIONS/OFFER), `LYRICS_SYNC_JSON` → `Vec<LyricLine>`.
- [x] `src/api/mod.rs` — `pub mod client; pub mod crypto; pub mod models;`.

## Phase 5 — UI views [x]
- [x] `src/ui/widgets/modal.rs` — `Modal`/`ModalRow`, manual box drawing (top/bottom/side borders), title + truncated list rows, `Modal::selected_style()` (inverted gradient). Render tests at 120x40, 40x12, tiny (4x3). 59/59 tests green, clippy `-D warnings` clean.
- [x] `src/ui/widgets/mod.rs` — `pub use modal::Modal;` (with `#[allow(unused_imports, dead_code)]`).
- [x] `src/ui/views/search.rs` — NAVIGATION sidebar (HOME/SEARCH/LIBRARY/FLOW + 10-bar mini-`Spectrum` внизу) слева при ширине ≥ 48, query field, three-column results table (ARTISTS / ALBUMS / TRACKS), compact mini-player at the bottom. Render tests at 120x40, 40x12, tiny (15x8) + сайдбар-тест (NAVIGATION + бары спектра). 82/82, clippy `-D warnings` clean.
- [x] `src/ui/views/library.rs` — `PlaylistEntry` + `Library` (tree с ▶-выделением и num_tracks, таблица `# / TRACK / ARTIST / ALBUM / TIME / ♥` выбранного плейлиста (♥ только у favorite), кнопка CHOOSE PLAYLIST, `Modal`-chooser в центре, мини-плеер). 4 render-теста (120x40, 40x12, modal, tiny 15x8). 82/82 тестов, clippy `-D warnings` clean.
- [x] `src/ui/views/flow_lyrics.rs` — `FlowLyrics` (Экран 04: обложка `CoverArt` + транспорт `⏮ ▶/❚❚ ⏭` + DEEZER FLOW-лист слева, синхронные LYRICS справа с активной строкой (бирюзовый фон `#4FD1C5`, тёмный текст) по `position_ms`, `Spectrum` снизу). `active_lyric_index()`. 5 тестов (120x40, 40x12, active-line, turquoise bg, tiny). 82/82, clippy clean.
- [x] MPRIS via souvlaki 0.9 `use_zbus` in `src/mpris.rs` — `Mpris` (Mutex\<MediaControls\>): `new`, `attach` (forward events), `set_track` (metadata), `set_playback` (status+position), `set_volume`, `cover_url`. DBUS_NAME `deezer_tui`, DISPLAY_NAME. 4 теста (new, ThreadNotRunning, cover_url, attach+drop). 75/75, clippy clean.
- [x] Gapless `src/audio/prefetch.rs` — `GaplessPolicy` (default threshold 20 s, `should_prefetch`), `PreparedTrack` (DeezerStream + `into_reader`), `PrefetchSlot` (Mutex slot: store/take/is_ready). 4 теста. 79/79, clippy clean.
- [x] Wire views into `App::render` (by `active_tab`) + keys (arrows/Enter/Esc/space/+/-) — `App` owns `search`, `library`, `flow_lyrics`, `volume`, `client: Arc<DeezerClient>`; top bar GUEST/CONNECTED + dynamic VOL. Тест `renders_each_tab_screen` (все 4 таба рендерят свой экран). 80/80, clippy clean.
- [x] Remove `#[allow(dead_code)]` from `DeezerClient::new`/`is_authenticated` (теперь используются `App`); `MediaControls`/MPRIS — без dead-code warnings (allows на местах).
- [x] Final `cargo test` + `cargo clippy -- -D warnings`. **85/85 passed, clippy `-D warnings` = clean (exit 0), 0 test-build warnings.**

## Audit `1_2_deezer_tui.md` fixes (Phase 1–2 rewrite) [x]
- [x] Перезаписаны 6 файлов по готовым реворкам аудита: `main.rs` (логгер `~/.config/deezer-tui` + `tracing_appender::rolling::daily` + `mod mpris`/`App::run` сохранены + лог config), `config.rs` (`load_from`/`save_to`, `PartialEq`, NaN→0.8, `effective_arl`), `dsp/cavacore.rs` (FFTW bass/mid-high, monstercat, parabolic gravity, autosens, 5 тестов), `audio/ring_buffers.rs` (`split()`, `Default`, сохранён оригинальный round-trip тест), `audio/decoder.rs` (`process_next`/`run`/`into_producers`, 4 теста), `audio/output.rs` (cpal F32-stereo поиск конфига, `stream.play()`, atomic volume, 4 теста).
- [x] Фикс API symphonia 0.6: `decoded.spec().channels().count()` вместо `decoded.channels()` (decoder.rs).
- [x] Фикс 3 doc-тестов (автор не прогонял — код не компилировался под наши версии зависимостей): decoder 100Гц/5000Гц — 9 кадров до сходимости интегрального фильтра (0.77) + реалистичные пороги (0.2 / 0.15); cavacore decay — параболическая гравитация стартует с нулевого наклона и IIR догоняет уровень, поэтому 2 первых кадра тишины допускаются в пределах пика (≤1.08·peak), дальше монотонный спад.
- [x] 14 clippy-замечаний в тестовом коде (bin-gate их не видел): `u32::from` no-op (output), `1 * CHUNK_SIZE` ×4 (crypto), `loop{match}` → `while let` ×2 (ring_buffers), `at(&terminal,…)` в `row_has` ×6 (app/flow_lyrics/library/search/modal), `iter().any` → `contains` (now_playing).
- [x] Гейт: `cargo check --all-targets` + `cargo test` **85/85** + `cargo clippy -- -D warnings` exit 0 + `cargo clippy --all-targets -- -D warnings` exit 0, 0 test-build warnings.

## Status
Phase 5: все вью + `src/mpris.rs` + `src/audio/prefetch.rs` + подключение вью к `App` (active_tab + клавиши + `DeezerClient` в `App`) — done. cargo test 85 passed; cargo clippy -- -D warnings = clean, exit 0; 0 warning в test-сборке.
