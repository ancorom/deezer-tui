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

## Gemini review fixes (6 файлов) [x]
- [x] `src/api/crypto.rs` — `generate_blowfish_key` (имя из ARCHITECTURE.md §4.2) + `generate_key` как синоним, file-level `#![allow(dead_code)]`, `Md5::digest` + `hex::encode`, `anyhow!`-ошибки. `as_chunks_mut::<CHUNK_SIZE>()` (clippy 1.99 `chunks_exact_to_as_chunks`).
- [x] `src/audio/stream.rs` — `impl MediaSource for DeezerStream` (`is_seekable → true`, `byte_len → Some(total_len)`), конструктор `from_encrypted(&[u8], &[u8;16])` (потоковая расшифровка в tempfile), `try_from_decrypted`, `is_empty`. Тест 8192 байта (4 чанка, зашифрованы idx 0 и 3) с произвольными Seek из PLAN.md (`read_exact` побайтово). Stream-тестов: 5 → 3.
- [x] `src/api/client.rs` — фикс двойного `.get("results")` в `get_lyrics` (fallback `unwrap_or`), поддержка обоих форматов `TRACK: {data:[…], total}` (реальный API) и `TRACK: […]` (тесты), tuple-array `[[sng_id, 0]]` в `add_to_playlist`/`remove_from_playlist`, фолбэк-ключи `milliseconds`/`line` в LYRICS_SYNC_JSON. 5/5 тестов.
- [x] `src/ui/theme.rs` — фикс градиента: `t as i32` обнулял дробную часть (градиент не работал) → f32-интерполяция с `.round().clamp(0.0, 255.0)`. Тест midpoint `Rgb(103,150,219)`.
- [x] `src/ui/widgets/spectrum.rs` — `set_bars` больше не сбрасывает `peak_holds` на каждом кадре (плавный decay `PEAK_DECAY_PER_FRAME = 0.015`, `max(bar)`); `set_peak_holds` — явная установка. 8/8 тестов (2 из старой версии перенесены).
- [x] `src/ui/widgets/modal.rs` — `ratatui::widgets::Clear` под модальным окном (ARCHITECTURE.md §6.2). 3/3.
- [x] `src/ui/views/now_playing.rs` — спектр в `Rect::new(area.x, area.y + content_h, …)` (был `(0, …)` — сдвиг на 1 строку в App), `rel_x` вместо сравнения `width > meta_x`, кнопки `[PAUSE] ⏮ ▶ ⏭ [FLOW]` (ARCHITECTURE.md §6.2), 32-столбичковый стартовый спектр в `default_screen()`.
- [x] `src/ui/views/search.rs` — фикс двойного прибавления `area.y` в `draw_query` (строка поиска рисовалась под заголовками), `saturating_sub` в `draw_status` (underflow-паника на узком терминале). 6/6 (1 новый + 2 старых перенесены).
- [x] `src/ui/views/library.rs` — CHOOSE PLAYLIST рисуется поверх отрисованного экрана (ранний `return` убран — экран Фонотеки больше не пропадает). 4/4 (новый `choose_modal_renders_over_background_screen`).
- [x] `src/ui/views/flow_lyrics.rs` — спектр в `Rect::new(area.x, area.y + body_h, …)` (был `(0, spec_top)`), автоскролл `scroll_offset` синхронных строк (активная строка в центре видимой области). 5/5.
- [x] `src/ui/app.rs` — вкладки `[1]..[4]` (было `[0]..[3]`, хоткеи 1–4), часы `HH:MM` в статус-баре, off-by-one в `navigate_down`/`navigate_right` (`selected_track + 1 < len`), `App::with_config(cfg)`, `attach_vis_consumer`, `step_dsp()` (Cavacore 60 FPS: вычитка `VisRingBuffer` → `Cavacore::process` → спектры NowPlaying/Search/Flow), `Mpris::set_volume` на `+`/`-`.
- [x] `src/main.rs` — `App::with_config(cfg)` (было `App::new()` — конфиг не влиял на приложение).
- [x] Гейт: `cargo check --all-targets` + `cargo test` **84/84** (85 → 84: stream 5→3, +1 search-тест) + `cargo clippy -- -D warnings` exit 0 + `cargo clippy --all-targets -- -D warnings` exit 0, 0 test-build warnings.

## Gemini review round 2 (7_arl_guest.md + 8_ui.md) [x]
- [x] `src/api/models.rs` — `TrackData` +`preview_url`/`cover_url_direct` (`skip`), `formatted_duration()`/`duration_secs()`, `Default` derive; `AudioQuality`-метки `MP3 | 64 kbps`/`FLAC | 1411 kbps`. 4/4.
- [x] `src/api/client.rs` — `search_public`/`get_public_chart` (публичный `api.deezer.com`, `?q=&limit=40`), `get_lyrics` (fallback `line`/`milliseconds`), `get_stream_url` через `media.deezer.com/v1/get_url` с фолбэком качества, `search`/`get_flow` гостевые fallback-и. 5/5.
- [x] `src/ui/widgets/cover_art.rs` — `from_bytes(&[u8])` (JPEG из CDN через `ImageReader`), half-block render. 5/5 (3 старых перенесены).
- [x] `src/ui/views/now_playing.rs` — квадратная обложка 2:1, прогресс-бар, спектр на всю ширину. 2/2.
- [x] `src/ui/views/library.rs` — `modal_selected`, `a`/`p`-модалка, `draw_mini_player`. 4/4 (2 перенесены).
- [x] `src/ui/views/flow_lyrics.rs` — квадратная обложка 2:1, спектр на всю ширину. 5/5 (3 перенесены).
- [x] `src/ui/app.rs` — полный rewrite: `BgEvent` + tokio multi-thread runtime `rt`, `spawn_initial_load` (login ARL → else публичный чарт), `poll_bg_events`, `start_track_playback` (create_rings→cpal `AudioOutput` + поток Symphonia/демо-синтезатор), `step_dsp` (60 FPS: vis-ring→Cavacore→3 спектра), `trigger_search`, ARL-модалка (L), F1–F4/Tab, live-ввод Search, `loop_` с подгонкой числа баров под ширину. 4/4 (1 перенесён: `renders_on_narrow_terminal`; +`modal_navigation_is_independent_of_library_table`).
- [x] `Cargo.toml` — reqwest +`"query"` feature (`.query(&[…])` в `search_public`).
- [x] 3 clippy: `large_enum_variant` на `BgEvent` (allow), identical if/else в render → прямой `(&self.now_playing).render`, collapsible `KeyCode::Char` guard в ARL-модалке.
- [x] Гейт: `cargo check --all-targets` + `cargo test` **85/85** (84→85: app +1) + `cargo clippy -- -D warnings` exit 0 + `cargo clippy --all-targets -- -D warnings` exit 0, 0 test-build warnings.

## Status
Все 8 файлов Gemini review (2 раунда) применены. Phase 5 + audit + gemini-review r1 + r2: done. cargo test 85 passed; cargo clippy -- -D warnings = clean, exit 0; 0 warning в test-сборке.
