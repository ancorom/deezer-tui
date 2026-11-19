# Changelog

История версий deezer-tui. Детали по каждой фазе — в [docs/crates/progress.md](docs/crates/progress.md),
спецификации — в [ARCHITECTURE.md](ARCHITECTURE.md).

## [v2.6] — 2026-11-19 — Спектр: три шкалы высоты и настраиваемая высота панели

### Симптом и диагностика
- После v2.5 спектр стал честнее, но рядом с эталонной cava выглядит бледнее: полосы
  не бывают ни нулями, ни потолком (средняя высота 0.70–0.78, полос ниже 0.2 — ни одной,
  σ 0.10–0.18). Проблема в форме, а не в уровне:
  1. окно 52 дБ от 0 dBFS при реальном разбросе полос внутри кадра 20–25 дБ сжимает всё
     в середину 0.2–0.6;
  2. Monstercat `1.35^|i-j|` подтягивает соседей пика до 74 %/55 %/41 % от пика — тёмных
     полос не остаётся вовсе (в cava пространственного фильтра нет);
  3. `INTEGRAL 0.62` и по-полосный AGC (`^0.7`) дополнительно сглаживают пики;
  4. панель спектра занимает 4–6 строк.
- Замер на реальном MP3 (Trivium — «Silence in the Snow», 25 с сигнала, 60 FPS, 15 полос):
  `0 dBFS · 52 dB` сред 0.761 / σ 0.114 / полос <0.2 — 0.00 / >0.85 — 0.22;
  `Track peak · 30 dB` сред 0.489 / σ 0.232 / <0.2 — 0.12 / >0.85 — 0.07;
  `Frame peak · 30 dB` сред 0.647 / σ 0.252 / <0.2 — 0.06 / >0.85 — 0.26.

### DSP (`src/dsp/cavacore.rs`)
- `SpectrumScale` — три шкалы высоты: `Legacy` (0 dBFS, окно 52 дБ — поведение v2.5),
  `TrackPeak` (по умолчанию: окно `REL_WINDOW = 30` дБ от EMA-пика громкости всего трека,
  attack 0.05 / release 0.0005) и `FramePeak` (то же окно от пика текущего кадра — манера cava).
- В относительных шкалах выключены по-полосный AGC, `sens` и `autosens`: нормировка
  остаётся единственным усилением. Пороги: `REL_SILENCE_PEAK = 1e-4` (весь кадр тише
  1e-4 → нули, регрессия `silence_produces_exact_zeros`) и `REL_BAND_GATE = 0.01`
  (полоса в −40 дБ от пика кадра → 0). Monstercat база 2.0 вместо 1.35, `INTEGRAL 0.5`.
- `set_scale_mode()` / `with_scale_mode()` пересобирают веса Monstercat и сбрасывают
  сглаживание (`mem`/`prev`/`slow_peak`/`sensitivity`).

### UI и конфигурация
- Вкладка Settings получила две строки: **Spectrum Height Scale** (`◄►` Track Peak /
  Frame Peak / 0 dBFS) и **Spectrum Panel Rows** (AUTO 4–6 или фиксированные 4…12 строк).
- `config.spectrum_scale` (по умолчанию `track_peak`) и `config.spectrum_rows`
  (по умолчанию `None` = AUTO); переменные окружения `DEEZER_TUI_SPECTRUM_SCALE` и
  `DEEZER_TUI_SPECTRUM_ROWS` (`0` = AUTO); ручная высота ограничивается 2…12 строками.
- Ручная высота ограничивается свободной высотой окна (минус строка плеера); логика
  вынесена в чистую функцию `App::spectrum_rows_for`.
- Конфиг пишется на диск только если был загружен с диска (`App::persist_config`,
  боевой вход — `App::with_loaded_config`): тесты больше не перезаписывают
  `~/.config/deezer-tui/config.toml`.

### Гейт
- `cargo build` без warning'ов; `cargo test` **128/128** (+10: нули в тише для
  относительных шкал, сброс сглаживания при переключении шкалы, эталон трека против
  эталона кадра, контраст TrackPeak против Legacy, разрешение шкалы/строк из конфига и
  окружения, две новые строки Settings, переключение строк, применение шкалы и высоты
  панели в раскладке); `cargo clippy --all-targets -- -D warnings` exit 0.

## [v2.5] — 2026-11-19 — Спектр: динамический диапазон вместо «прыгает только бас»

### Симптом и диагностика
- На электронике бас «отражался» столбцами, а правая треть (средние/высокие частоты)
  оставалась плоской; на не-басовой музыке доминировал самый громкий элемент.
  Численная проверка (15 полос, микс бас 60 Гц + пэд 1 кГц + шляпки 9 кГц):
  `bass 0.51 / mid 0.10 / high 0.014` — ВЧ физически нечего было показывать.
- Причины: (1) энергия полосы как RMS-среднее `Σ|X_k|²/count` — широкие ВЧ-полосы
  (10–20 кГц ≈ 460 бинов) размазывали разреженное содержимое; (2) наклон
  `(f_center/300)^0.25` с clamp 0.55–2.3 — слишком слаб против реального −5 дБ/октаву;
  (3) линейная шкала без компрессии — самый громкий столбик съедал весь диапазон высот;
  (4) асимметричная `autosens` (0.985 вниз / 1.001 вверх) — один громкий кадрец тянул
  весь спектр вниз на секунды, вверх чувствительность возвращалась ~35 с.

### DSP (`src/dsp/cavacore.rs`)
- Энергия полосы: `Σ|X_k| + 0.35·max|X_k|` (равная энергия на октаву + полка транзиентов).
- По-полосный AGC: EMA `band_ref` (0.06/0.008) → `gain = (ref_mean/ref)^0.7`, clamp 0.35–4.5.
- Относительный порог тишины 0.004 (−48 дБ от пика кадра) — шумовая полка и утечка FFT → 0.
- Наклон частот: `(f_center/120)^0.18`, clamp 0.85–2.2 (компенсацию перекоса теперь делает AGC).
- Шкала после усиления: dB-окно 60 → 52 дБ; линейный режим — компрессия `x^0.45`.
- `autosens`: симметричная мёртвая зона 0.4–0.85 по сырого пика, rates 0.99/1.005, clamp 0.25–8.0.
- Monstercat база 1.5 → 1.35 (меньше «стена баса» в соседях), `INTEGRAL` 0.77 → 0.62 (~40 мс).

### Конфиг
- `db_scale` по умолчанию `true` (в `config.rs` — `#[serde(default = "default_db_scale")]`).

### Тесты
- 118/118 (+4 регрессии): `loud_bass_does_not_flatten_high_bands`,
  `pure_bass_keeps_high_bands_dark`, `sustained_lead_does_not_flatten_other_bands`,
  `autosens_recovers_after_loud_passage`.

## [v2.4] — 2026-11-16 — Релиз: sextant-обложки, настоящий Flow по жанрам/настроениям, бесконечный поиск, Track Mix

### Sextant-обложки (6 субпикселей/ячейку)
- `CoverRenderMode::Sextant` (default): сетка 2×3 — 6 субпикселей на знакоместо
  (в 3 раза детальнее полублока ▀); `resized_pixels_sextant` (2w × 3h,
  Lanczos3/CatmullRom/Nearest).
- Пер-ячейка кластеризация: пара пикселей с максимальным цветовым расстоянием
  задаёт fg (светлее) / bg (темнее); каждый субпиксель → ближайший кластер;
  6-бит маска → глиф `SEXTANT_CHARS` (U+1FB00–U+1FB3B; маски 21/42 → `▌`/`▐`,
  0 → space, 63 → `█`); почти монохромная ячейка (dist² ≤ 25) → `█`.
- `HalfBlock` (▀, v2.3) — опциональный режим (`with_render_mode`).
- Высоты: Now Playing 26 → **40** строк, Flow 14 → **24** (квадрат 1:1 сохраняется).

### Track Mix (m → 📻)
- `song.getSearchTrackMix` (SNG_ID, `start_with_input_track`); публичный fallback:
  `/track/{id}` → artist → `/artist/{id}/radio` → `/artist/{id}/top?limit=40`
  → public chart. Авто-резолв track_id «0» через `find_cached_track_id`.

### Flow по жанрам/настроениям
- Жанры (`get_genre_flow`): `/chart/{genre_id}/tracks?limit=50` →
  `/genre/{id}/radios` → `/radio/{id}/tracks?limit=50` → public chart.
- Настроения (`get_mood_flow`): кураторские плейлисты `/search/playlist?q=...`
  → `/playlist/{id}/tracks?limit=50` → поиск треков → public chart.

### Бесконечный поиск
- `search_paged` (`deezer.pageSearch`, start/nb) + `search_public_paged`
  (index/limit); 30/страницу; автозагрузка при `selected + 5 ≥ len` и по
  `PageDown` (±10 строк); дедупликация по `track_id`.
- `sanitize_query`: дефисы/тиры/`_` → пробел, схлопывание whitespace.
- `search_local_tracks`: мгновенные совпадения в `library_tracks` + кэше
  плейлистов (❤) — до ответа remote.

### Тесты
- 114/114 (+4: sanitize_query, локальный поиск с дефисом, пагинация, sextant).

## [v2.3] — 2026-11-16 — Релиз: Cavacore octave rework, TrueColor обложки, Track Menu, Flow Modes

### Добавлено
- **Cavacore octave rework:** диапазон полос 50 Гц–10 кГц → **20 Гц–20 кГц** (полный
  слышимый, октавная шкала: 15 полос = ⅔ октавы/шаг, 30 полос = ⅓); энергия полосы
  `mag = 0.65 * rms + 0.35 * peak` (вместо Σ|c|); `freq_tilt = (f_center/300)^0.25`
  (clamp 0.55–2.3, `f_center = √(f_lo·f_hi)`), `eq = norm_fft * freq_tilt`
  (v2.2-ное деление на `bin_count.sqrt()` убрано); `set_num_bars(desired_bars)` —
  динамический ресайз (FFT-буферы и окна сохраняются, `mem`/`prev` ресайзятся);
  `Spectrum::render` — 1:1 без ресемплинга (`sample_slice` в render-пути убран).
- **TrueColor обложки:** quad subpixel 2×2 (`QUADRANT_GLYPHS`, `best_quadrant_cell`,
  `resized_quadrant`) заменён на `resized_pixels(w, h)` — Lanczos3 (даунскейл) /
  CatmullRom (апскейл) / Nearest (точное совпадение) под сетку w × 2h; рендер —
  полублок `▀` на ячейку: fg = верхний субпиксель, bg = нижний (честные 24-битные
  цвета). Тема: +`BG_ELEVATED #252842`.
- **Track Menu** (`m`/`M`, любой активный трек — Now Playing / Search / Library / Flow):
  ❤️/💔 favorite (динамический), ➕ добавить в плейлист (модал CHOOSE PLAYLIST),
  📻 Song Radio (`radio.getSongRadio` → `BgEvent::TrackRadioLoaded` → очередь + play),
  ℹ️ Track & Stream Info, Esc. Тост-уведомления в статусной строке (3 с, градиент).
- **Flow Modes** (`f` вне Search / `g` глобально; `f` на Search — фокус поиска):
  My Flow + 7 настроений (chill/motivation/party/focus/romance/melancholy/workout) +
  10 жанров (rock/pop/rap/electronic/metal/jazz/classical/reggae/indie/soundtracks)
  через `radio.getUserRadio` (`get_mood_flow`/`get_genre_flow`, fallback — поиск /
  public chart); выбранный режим запоминается (`spawn_fetch_more_flow` читает
  `current_flow_mode_idx`), лейбл режима — `flow_lyrics.flow_mode_label`.

### Фиксы
- Clippy `collapsible_if` (13 шт., `src/api/client.rs` + toast в `app.rs`) — let-chains
  `if let Ok(x) = … && !x.is_empty()` (Rust 2024).

### Тесты
110/110 (+5: `dynamic_resize_maintains_audio_state`, тесты cover-art 1:1,
`playlist_picker_modal_and_add_to_playlist` и др.), clippy clean, 0 build-предупреждений.

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
