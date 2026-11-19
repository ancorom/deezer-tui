# TODO.md — Пошаговый чеклист задач для AI-агента (Редакция: Ноябрь 2026, v2.7 — фазы 1..16 закрыты)

Выполняй задачи строго сверху вниз по одной за итерацию. После прохождения cargo test и cargo clippy --all-targets -- -D warnings меняй [ ] на [x].

## Фаза 1: Инициализация и математическое ядро Cavacore
- [x] Создать Cargo.toml с актуальными зависимостями 2026 года из ARCHITECTURE.md и базовую структуру модулей в src/.
- [x] Реализовать src/config.rs (чтение и сохранение arl, громкости и настроек в ~/.config/deezer-tui/config.toml через toml 1.2) и настроить файловый логгер tracing в src/main.rs.
- [x] Реализовать src/dsp/cavacore.rs: структура Cavacore, инициализация окон Ханна (4096 для басов и 2048 для СЧ/ВЧ), планировщик realfft 3.5 и логарифмическое распределение частот (50–10000 Гц) по столбикам.
- [x] Добавить в src/dsp/cavacore.rs фильтр сглаживания Monstercat, параболическую гравитацию падения (falloff), интегральное сглаживание (0.77), опциональный dB-скейлинг и авточувствительность (autosens).
- [x] Написать unit-тесты для Cavacore по образцу docs/reference/cava/cavacore_test.c (тишина, синусоида 100 Гц, синусоида 5000 Гц, плавный спад после пика).

## Фаза 2: Локальный аудиодвижок (Symphonia 0.6 + CPAL 0.18 + rtrb 0.4)
- [x] Реализовать src/audio/decoder.rs: декодирование аудиопотока через symphonia 0.6 (FLAC/MP3), конвертация буферов в f32 и запись в два lock-free буфера rtrb::Producer<f32> (AudioRingBuffer для стерео и VisRingBuffer для моно).
- [x] Реализовать src/audio/output.rs: инициализация cpal 0.18 с явным выбором частоты SampleRate(44100) (вместо нового дефолта 48 кГц), чтение из rtrb::Consumer<f32> без блокировок и аллокаций, атомарная регулировка громкости.
- [x] Написать интеграционный тест передачи декодированных сэмплов через rtrb в Cavacore.

## Фаза 3: Графическое ядро (Ratatui 0.30) и Экран 01 (Now Playing)
- [x] Реализовать src/ui/theme.rs: константы цветов (#13151F, #1E2233, #3E445E, #ECEFF4, #94A3B8) и функцию интерполяции градиента от #4FD1C5 до #7F5AF0.
- [x] Реализовать src/ui/widgets/spectrum.rs: отрисовка столбиков символами [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'] с интервалом в 1 пробел и точками удержания пиков сверху.
- [x] Реализовать src/ui/widgets/cover_art.rs: чтение через image::ImageReader, масштабирование DynamicImage и рендеринг через полублоки ▀ (fg — верхний пиксель, bg — нижний пиксель).
- [x] Сверстать глобальный верхний статус-бар (DEEZER-TUI, вкладки [1]..[4], CONNECTED, VOL, часы) и нижний бар хоткеев в src/ui/app.rs с инициализацией терминала через ratatui::init() и ratatui::restore().
- [x] Сверстать src/ui/views/now_playing.rs (Экран 01: обложка, бейдж FLAC | 1411 kbps, название трека, артист, прогресс-бар ●, кнопки управления и спектр во всю нижнюю половину).
- [x] Написать рендер-тест Экрана 01 на ratatui::backend::TestBackend.

## Фаза 4: Криптография Blowfish и API Deezer (reqwest 0.13)
- [x] Реализовать src/api/crypto.rs: генерация 16-байтного ключа из SNG_ID (MD5 hex + XOR с g4el58wc0zvf9na1) и функция расшифровки чанка 2048 байт через cbc::Decryptor<blowfish::Blowfish> с IV [0,1,2,3,4,5,6,7].
- [x] Написать unit-тесты для src/api/crypto.rs (проверка длины ключа, детерминированности и обратимости шифрования/дешифрования).
- [x] Реализовать src/audio/stream.rs: структура DeezerStream, реализующая Read + Seek + Send + Sync + MediaSource, расшифровывающая каждый чанк chunk_idx % 3 == 0 размером ровно 2048 байт с сохранением в tempfile для мгновенного Seek.
- [x] Написать unit-тест для DeezerStream с произвольными операциями SeekFrom::Start и невыровненными чтениями read().
- [x] Реализовать src/api/models.rs и src/api/client.rs на базе reqwest 0.13: авторизация по куке arl в gw-light.php (deezer.getUserData), получение URL потока в [media.deezer.com/v1/get_url](https://media.deezer.com/v1/get_url), методы поиска, плейлистов, лайков, Flow и Lyrics.

## Фаза 5: Экраны 02, 03, 04 и финализация
- [x] Сверстать src/ui/views/search.rs (Экран 02: сайдбар NAVIGATION с мини-спектром, строка ввода поиска и 3 колонки ARTISTS, ALBUMS, TRACKS + нижний мини-плеер).
- [x] Сверстать src/ui/views/library.rs и src/ui/widgets/modal.rs (Экран 03: дерево плейлистов слева, таблица треков справа и модальное окно CHOOSE PLAYLIST по клавише добавления в плейлист).
- [x] Сверстать src/ui/views/flow_lyrics.rs (Экран 04: блок Deezer Flow, синхронная прокрутка LYRICS по таймкоду декодера с бирюзовой подсветкой активной строки и полный спектр снизу).
- [x] Подключить D-Bus MPRIS через souvlaki 0.9 (use_zbus) и префетч следующего трека за 20 секунд до окончания текущего для Gapless-воспроизведения.
- [x] Прогнать финальный cargo test и cargo clippy -- -D warnings.

## Фаза 6: Синхронизация звука и спектра, мгновенная пауза, Seek и очередь треков

* [x] **Шаг 6.1 (`src/audio/output.rs`):** Добавить в `AudioOutput` атомарный флаг `paused: Arc<AtomicBool>`, счётчик реально сыгранных стерео-фреймов `played_frames: Arc<AtomicU64>` и передачу `vis_producer: Option<Producer<f32>>` внутрь real-time коллбэка `CPAL`. Когда `paused == true`, коллбэк пишет `0.0` в буфер карты, не читая `AudioRingBuffer`. Когда `paused == false`, коллбэк пишет моно-сэмплы `(L + R) * 0.5` в `vis_producer` и увеличивает `played_frames`. Сохранить `fill_buffer_from_consumer` и `AudioOutput::new` для совместимости, добавив `AudioOutput::new_with_sync` и unit-тесты.

* [x] **Шаг 6.2 (`src/audio/decoder.rs`):** Добавить `DecoderCommand::Seek(u64)` и структуру `StreamTechInfo` (`codec_name`, `sample_rate`, `bits_per_sample`, `total_frames`, `duration_secs`, `bitrate_kbps`, `file_size_bytes`) с методом `badge(is_preview: bool)`. Реализовать `Decoder::new_seekable(source: Box<dyn MediaSource>, audio_producer: Producer<f32>, fallback_duration_secs: u64) -> Result<(Self, StreamTechInfo)>` (передача MediaSource напрямую в `MediaSourceStream::new` без `ReadOnlySource` — для Seek) и `Decoder::seek(&mut self, target_secs: u64)` через `format.seek(SeekMode::Accurate, SeekTo::Time { time: Time::from(target_secs), track_id: Some(self.track_id) })` + `decoder.reset()`.

* [x] **Шаг 6.3 (`src/ui/app.rs`):** Заменить таймер `Instant::now()` на расчёт прогресса из `played_frames / sample_rate` и `StreamTechInfo::duration_secs`. Добавить `PlaybackQueue` (автоматический переход на следующий трек при `BgEvent::TrackEnded`, хоткеи `n` — следующий трек, `p` — предыдущий, `←`/`→` — отправка `DecoderCommand::Seek`). Всегда очищать `flow_lyrics.lyrics` при старте нового трека.

## Фаза 7: Единый сквозной плеер, сворачиваемый сайдбар и переключатель спектра

* [x] **Шаг 7.1 (`src/config.rs`):** Добавить в `AppConfig` сохраняемые поля: `preferred_quality: String` (`"FLAC"`, `"MP3_320"`, `"MP3_128"`), `show_spectrum: bool` (по умолчанию `true`), `db_scale: bool` (`false`), `sidebar_collapsed: bool` (`false`).

* [x] **Шаг 7.2 (`src/ui/app.rs`):** Сделать нижнюю панель плеера (прогресс `MM:SS / MM:SS`, кнопки `[⏮] [⏸/▶] [⏭]`, реальный бейдж качества из `StreamTechInfo`, громкость) и спектр глобальными в `App::render`, убрав дублирующиеся мини-плееры из `search.rs` и `library.rs`. Добавить клавишу `v` (вкл/выкл спектра) и `s` (свернуть/развернуть левый сайдбар навигации).

## Фаза 8: Удобная навигация в Search и Library + подгрузка плейлистов

* [x] **Шаг 8.1 (`src/ui/views/search.rs` и `src/ui/app.rs`):** Ввести режим фокуса поисковой строки `search_input_focused: bool`. Включать ввод текста по `/` или `f`, выключать по `Enter` (запуск поиска) или `Esc`. Когда `search_input_focused == false`, клавиши `Space`, `1`–`5`, `n`, `p`, `←`/`→`, `v`, `s` работают как глобальные хоткеи плеера.

* [x] **Шаг 8.2 (`src/api/client.rs` и `src/ui/views/library.rs`):** Добавить в `DeezerClient` метод `get_playlist_tracks(&self, playlist_id: &str)` (вызов `playlist.getSongs` в `gw-light.php` и публичный `https://api.deezer.com/playlist/{id}/tracks`). Разделить фокус на экране `Library` между списком плейлистов и таблицей треков (переключение по `Tab` или `←`/`→`), подгружая треки выбранного плейлиста.

## Фаза 9: Вкладка `[5] Settings` и окно деталей потока `[i] Track Info`

* [x] **Шаг 9.1 (`src/ui/views/settings.rs`):** Создать экран `[5] Settings` (открывается по `5` или `,`): выбор качества (`FLAC 1411 kbps`, `MP3 320 kbps`, `MP3 128 kbps`), ввод и сохранение `ARL`-токена (убрать надпись `[press L for ARL]` из топ-бара в настройки), переключатели `Show Spectrum` и `dB Scale`.

* [x] **Шаг 9.2 (`src/ui/app.rs`):** Добавить модальное окно технической информации о треке и потоке по клавише `i` (`Track ID`, `Artist`, `Album`, `Codec`, `Sample Rate`, `Bit Depth`, `Bitrate kbps`, `Stream Size MB`, `Mode: HiFi BF_CBC_STRIPE / Guest Preview`).


## Фаза 10: Релиз v2.1 — масштабный дебаг и доработки

* [x] **Шаг 10.1 (`src/api/models.rs`):** Добавить `TrackData.date_add: u64` (rename `DATE_ADD`, alias `TIMELINE`, `deserialize_u64_flexible`) и расширить алиасы `PlaylistData` (`id`, `title`, `nb_tracks`, `COUNT`, `CREATED_AT`, `creation_date` + `deserialize_u64_flexible` для `NB_SONG`/`DATE_ADD`), чтобы плейлисты с строковыми датами и числами в API не терялись при десериализации.

* [x] **Шаг 10.2 (`src/api/client.rs`):** Заменить `get_playlists` — собирать плейлисты из `deezer.pageProfile` поэлементно (без whole-array `unwrap`) и дублировать/дополнять через публичный fallback `https://api.deezer.com/user/{id}/playlists?limit=100` с дедупликацией по `playlist_id`.

* [x] **Шаг 10.3 (`src/audio/stream.rs`):** Streaming-декод: `fetch_streaming` с общим temp-файлом и `written_len: AtomicU64` — декодер стартует после пребуфера 256 КБ, загрузка продолжается в фоновом tokio-задаче; `Read` ждёт дописанных байтов, `Seek` ждёт не более 1.5 с и клампится к `written_len` до окончания загрузки.

* [x] **Шаг 10.4 (`src/audio/decoder.rs`):** Убить CPU-зомби: `push_all_cancelable` с проверкой `stop_flag.load() || dst.is_abandoned()` и `thread::sleep(5 мс)` вместо busy-spin `yield_now()`; `Decoder::new_seekable_with_stop` с общим `stop_flag`; `push_all_blocking` оставлена как обёртка.

* [x] **Шаг 10.5 (`src/config.rs`):** Добавить сохраняемое поле `show_lyrics: bool` (по умолчанию `true`) для глобального переключателя `y`.

* [x] **Шаг 10.6 (`src/ui/views/now_playing.rs`):** Разбить Now Playing на левую панель (крупная обложка + метаданные + подсказки) и панель `LYRICS [Y]` с переносом строк, подсветкой активной строки и автопрокруткой; прогресс-бар/кнопки/спектр рендерятся глобально в `App::render`.

* [x] **Шаг 10.7 (`src/ui/views/flow_lyrics.rs`):** Убрать второй спектр, добавить вертикальный разделитель `│`, `wrap_text` для длинных названий, header `FLOW • Infinite Recommendations [y: Lyrics ON/OFF]`, автопрокрутку списка Flow и подсветку активной строки лирики.

* [x] **Шаг 10.8 (`src/ui/views/library.rs`):** Сортировка треков плейлиста (`o` — критерий `Added/Title/Artist/Album` по кругу, `r` — направление `▲/▼`), сортируемые заголовки колонок, бейдж `[Sort [o/r]: ...]` в заголовке и расширенные отступы колонок.

* [x] **Шаг 10.9 (`src/ui/app.rs`):** `PrefetchedTrack`-кэш мгновенной предзагрузки следующего трека (`spawn_prefetch_next` после старта текущего), `spawn_fetch_more_flow` для бесконечного Flow (автоподгрузка при ≤3 треках), `resort_active_library_playlist` с кэшем исходных треков, хоткеи `y` (лирика), `o`/`r` (сортировка), адаптивный спектр 6–4 строки, глобальные `show_lyrics`/`position_ms`.

## Фаза 11: Релиз v2.2 — дебаг и доработки по диагнозу Gemini

* [x] **Шаг 11.1 (`src/audio/stream.rs`):** Заменить `File::try_clone()` на in-memory буфер `Arc<RwLock<Vec<u8>>>` с независимым курсором чтения `pos: usize` у каждого экземпляра (убирает гонку двух FD на один OS offset, портившую FLAC-заголовок при быстрой смене трека); проверять `abort_download` внутри `read()` → мгновенный `Ok(0)`.

* [x] **Шаг 11.2 (`src/api/models.rs`, `src/api/client.rs`, `src/mpris.rs`):** `TrackData::formatted_date_add()` → `YYYY-MM-DD` (Unix-секунды и строка `YYYYMMDDHHMMSS`, `—` при 0); `parse_public_tracks`: `cover_big` → `cover_medium` → `cover`; MPRIS-обложка 500x500 вместо 264x264.

* [x] **Шаг 11.3 (`src/ui/widgets/cover_art.rs`):** Quad subpixel рендеринг **2x2 на ячейку** (16 глифов `QUADRANT_GLYPHS`, `resized_quadrant` 2x2 суперсэмплинг, `best_quadrant_cell` — 4-битная маска fg/bg с минимизацией цветов) = 2x горизонтальное разрешение обложек.

* [x] **Шаг 11.4 (`src/dsp/cavacore.rs`, `src/ui/widgets/spectrum.rs`):** Дробная интерполяция между соседними FFT-бинами для узких полос (убирает «пары-близнецы» и провалы); `eq = (norm_fft * freq_tilt) / bin_count.sqrt()`, `freq_tilt = (f_hi/100)^0.55` (RMS + подъём СЧ/ВЧ); `Spectrum::render` ресемплирует полосы ровно под ширину окна (`sample_slice`, `div_ceil(width/2)`) — весь диапазон умещается при любом ресайзе.

* [x] **Шаг 11.5 (`src/ui/views/search.rs`, `src/ui/views/library.rs`):** `SearchRow + pub added: String`; колонка **ADDED** (`YYYY-MM-DD`) в Library: `ADDED ▲/▼` при Default-сортировке (сортировка по `date_add`), ячейка `added` или `—`.

* [x] **Шаг 11.6 (`src/ui/views/now_playing.rs`, `src/ui/views/flow_lyrics.rs`):** Увеличенные обложки: NowPlaying до 26 строк (104x52 субпикселя 2x2), FlowLyrics до 14 (56x28).

* [x] **Шаг 11.7 (`src/ui/app.rs`):** `playback_seq: Arc<AtomicU64>` (req_id против гонок при быстром n/Enter, устаревшие `TrackStarted`/`TrackEnded` отбрасываются), переиспользование авторизованного `self.client` (без `login_arl` на каждый трек; свежий `get_track` при истёкшем `TRACK_TOKEN`), `BgEvent::FlowLoadFailed` + `.take(15)` убран (flow 1:1 с `flow_tracks`), player row с хоткеями `[←]` `[→]` `[⏮ p]` `[⏸ Space]` `[⏭ n]` `Vol [-/+]`, адаптивный спектр `div_ceil(width/2).clamp(8, 160)`, tokio `worker_threads(4)`.

## Фаза 12: Релиз v2.3 — Cavacore octave rework, TrueColor обложки, Track Menu, Flow Modes

* [x] **Шаг 12.1 (`src/dsp/cavacore.rs`):** Octave rework — диапазон полос 50 Гц–10 кГц → **20 Гц–20 кГц** (полный слышимый; октавная шкала: 15 полос = ⅔ октавы/шаг, 30 полос = ⅓), энергия полосы `mag = 0.65 * rms + 0.35 * peak` (вместо Σ|c|), `freq_tilt = (f_center/300)^0.25` (clamp 0.55–2.3, `f_center = √(f_lo·f_hi)`), `eq = norm_fft * freq_tilt` (v2.2-ное деление на `bin_count.sqrt()` убрано), `set_num_bars(desired_bars)` — динамический ресайз (FFT-буферы и окна сохраняются, `mem`/`prev` ресайзятся). Тест `dynamic_resize_maintains_audio_state`.

* [x] **Шаг 12.2 (`src/ui/widgets/cover_art.rs`, `src/ui/theme.rs`, `src/ui/widgets/spectrum.rs`):** TrueColor обложки — quad subpixel 2×2 (`QUADRANT_GLYPHS`, `best_quadrant_cell`, `resized_quadrant`) заменён на `resized_pixels(w, h)`: Lanczos3 (даунскейл) / CatmullRom (апскейл) / Nearest (точное совпадение) под сетку w × 2h; рендер — полублок `▀` на ячейку (fg = верхний субпиксель, bg = нижний, честные 24-битные цвета). Тема: `+BG_ELEVATED #252842`. `Spectrum::render` — 1:1 без ресемплинга (`sample_slice` из render-пути убран). Тесты cover-art 1:1.

* [x] **Шаг 12.3 (`src/ui/app.rs`):** **Track Menu** (`m`/`M`, любой активный трек — Now Playing / Search / Library / Flow): ❤️/💔 favorite (динамический), ➕ добавить в плейлист (модал `CHOOSE PLAYLIST`), 📻 Song Radio (`radio.getSongRadio` → `BgEvent::TrackRadioLoaded` → очередь + play), ℹ️ Track & Stream Info, Esc. Тост-уведомления в статусной строке (3 с, градиент). Тесты `track_context_menu_opens_navigates_and_toggles_favorite`, `playlist_picker_modal_and_add_to_playlist`.

* [x] **Шаг 12.4 (`src/api/client.rs`, `src/ui/app.rs`, `src/ui/views/flow_lyrics.rs`):** **Flow Modes** (`f` вне Search / `g` глобально; `f` на Search — фокус поиска): My Flow + 7 настроений (chill/motivation/party/focus/romance/melancholy/workout) + 10 жанров (rock/pop/rap/electronic/metal/jazz/classical/reggae/indie/soundtracks) через `radio.getUserRadio` (`get_mood_flow` / `get_genre_flow`, fallback — поиск / public chart); выбранный режим запоминается (`spawn_fetch_more_flow` читает `current_flow_mode_idx`), лейбл режима — `flow_lyrics.flow_mode_label`.

* [x] **Шаг 12.5 (`src/api/client.rs`, `src/ui/app.rs`):** Clippy `collapsible_if` (13 шт.) → let-chains `if let Ok(x) = … && !x.is_empty()` (Rust 2024). Гейт: `cargo test` **110/110** (+5), `cargo clippy --all-targets -- -D warnings` clean.

## Фаза 13: Релиз v2.4 — sextant-обложки, Track Mix, настоящие жанровые/настроений Flow, бесконечный поиск

* [x] **Шаг 13.1 (`src/ui/widgets/cover_art.rs`):** `CoverRenderMode::Sextant` (default) — сетка 2×3, **6 субпикселей на знакоместо** (в 3 раза детальнее полублока ▀); `resized_pixels_sextant` (2w × 3h, Lanczos3/CatmullRom/Nearest); пер-ячейка кластеризация: пара пикселей с максимальным цветовым расстоянием задаёт fg (светлее) / bg (темнее), каждый субпиксель → ближайший кластер, 6-бит маска → глиф `SEXTANT_CHARS` (U+1FB00–U+1FB3B; маски 21/42 → `▌`/`▐`, 0 → space, 63 → `█`); почти монохромная ячейка (`dist² ≤ 25`) → `█`. `HalfBlock` (▀, v2.3) — опциональный режим (`with_render_mode`). Высоты: Now Playing 26 → **40** строк, Flow 14 → **24** (квадрат 1:1 сохраняется). +1 тест sextant.

* [x] **Шаг 13.2 (`src/api/client.rs`):** **Track Mix** (`m` → 📻): `song.getSearchTrackMix` (SNG_ID, `start_with_input_track`) + публичный fallback `/track/{id}` → artist → `/artist/{id}/radio` → `/artist/{id}/top?limit=40` → public chart; авто-резолв `track_id` «0» через `find_cached_track_id`.

* [x] **Шаг 13.3 (`src/api/client.rs`):** Настоящие Flow по жанрам/настроениям: жанры (`get_genre_flow`) — `/chart/{genre_id}/tracks?limit=50` → `/genre/{id}/radios` → `/radio/{id}/tracks?limit=50` → public chart; настроения (`get_mood_flow`) — кураторские плейлисты `/search/playlist?q=...` → `/playlist/{id}/tracks?limit=50` → поиск треков → public chart.

* [x] **Шаг 13.4 (`src/api/client.rs`, `src/ui/views/search.rs`, `src/ui/app.rs`):** **Бесконечный поиск** — `search_paged` (`deezer.pageSearch`, start/nb) + `search_public_paged` (index/limit), 30/страницу, автозагрузка при `selected + 5 ≥ len` и по `PageDown` (±10 строк), дедупликация по `track_id`; `sanitize_query` (дефисы/тиры/`_` → пробел, схлопывание whitespace); `search_local_tracks` — мгновенные совпадения в `library_tracks` + кэше плейлистов (❤) до ответа remote. +3 теста (`sanitize_query`, локальный поиск с дефисом, пагинация).

* [x] **Шаг 13.5:** Гейт релиза v2.4: `cargo test` **114/114** (+4), `cargo clippy --all-targets -- -D warnings` clean, `cargo build --release`.

## Фаза 14: v2.5 — динамический диапазон спектра (полосы «прыгают», а не только бас)

* [x] **Шаг 14.1 (`src/dsp/cavacore.rs`):** Энергия полосы `Σ|X_k| + 0.35·max|X_k|` вместо RMS-среднего (равная энергия на октаву + полка транзиентов); наклон `(f_center/120)^0.18` (clamp 0.85–2.2) вместо `(f_center/300)^0.25` (0.55–2.3); шкала (dB или линейная) применяется **после** усиления.

* [x] **Шаг 14.2 (`src/dsp/cavacore.rs`):** По-полосный AGC (`band_ref` EMA 0.06/0.008 → `gain = (ref_mean/ref)^0.7`, clamp 0.35–4.5) + относительный порог тишины `0.004` (шумовая полка и утечка FFT → 0); dB-окно 60 → 52 дБ, линейный режим — компрессия `x^0.45`; `autosens` с симметричной мёртвой зоной 0.4–0.85 по сырого пика (rates 0.99/1.005, clamp 0.25–8.0) вместо асимметричных 0.985/1.001; Monstercat 1.5 → 1.35; `INTEGRAL` 0.77 → 0.62.

* [x] **Шаг 14.3 (`src/config.rs`, тесты):** `db_scale` по умолчанию `true` (`#[serde(default = "default_db_scale")]`); +4 регрессии: `loud_bass_does_not_flatten_high_bands`, `pure_bass_keeps_high_bands_dark`, `sustained_lead_does_not_flatten_other_bands`, `autosens_recovers_after_loud_passage`. Гейт: `cargo test` **118/118**, `cargo clippy --all-targets -- -D warnings` exit 0, build без warning'ов.

## Фаза 15: v2.6 — три шкалы высоты спектра и настраиваемая высота панели

* [x] **Шаг 15.1 (`src/dsp/cavacore.rs`):** `SpectrumScale` (`Legacy` / `TrackPeak` / `FramePeak`) + относительное окно `REL_WINDOW = 30` дБ: `TrackPeak` нормируется от EMA-пика громкости всего трека (`TRACK_PEAK_ATTACK 0.05` / `TRACK_PEAK_RELEASE 0.0005`), `FramePeak` — от пика кадра; в обоих отключены по-полосный AGC, `sens` и `autosens`, пороги `REL_SILENCE_PEAK 1e-4` и `REL_BAND_GATE 0.01`, Monstercat `REL_SPREAD_BASE 2.0`, `INTEGRAL 0.5`; API `with_scale_mode`/`set_scale_mode` со сбросом сглаживания.

* [x] **Шаг 15.2 (`src/config.rs`, `src/ui/views/settings.rs`, `src/ui/app.rs`):** `config.spectrum_scale` (по умолчанию `track_peak`) и `config.spectrum_rows` (`None` = AUTO) с env-переопределениями `DEEZER_TUI_SPECTRUM_SCALE`/`DEEZER_TUI_SPECTRUM_ROWS`; строки Settings **Spectrum Height Scale** и **Spectrum Panel Rows** (`◄►`/`Enter`), ручная высота ограничивается свободной высотой окна через `App::spectrum_rows_for`; запись конфига только при загрузке с диска (`App::persist_config`).

* [x] **Шаг 15.3 (тесты):** +10 тестов (нули в тише для относительных шкал, сброс сглаживания при смене шкалы, трек-эталон против кадр-эталона, контраст TrackPeak против Legacy, разрешение шкалы/строк, две строки Settings и их циклы, применение шкалы и высоты панели в раскладке). Гейт: `cargo build` без warning'ов, `cargo test` **128/128**, `cargo clippy --all-targets -- -D warnings` exit 0.

## Фаза 16

* [x] **Шаг 16.1 (`src/dsp/cavacore.rs`):** соседний фильтр Monstercat выключен по умолчанию (`SPREAD_OFF`, `SPREAD_MIN_BASE 1.05`, `SPREAD_MAX 4.0`, `rebuild_spread_weights`, `set_spread_base`/`spread_base`/`with_spread_base`); окна нормировки `REL_WINDOW` 30 → 26 дБ и `DB_DYNAMIC_RANGE` 52 → 32 дБ через поле `db_window`; `integral`/`gravity` — поля по шкалам (0.62/0.70/0.72 и 0.025/0.05).

* [x] **Шаг 16.2 (`src/dsp/cavacore.rs`):** `FramePeak` опирается на `frame_ref` с несимметричной опорой (подъём не ограничен, спуск `FRAME_PEAK_RELEASE 0.12` ≈ 0.1 с удержания) вместо мгновенного пика кадра; метки шкал `0 dBFS · 32 dB` / `Track peak · 26 dB` / `Frame peak · 26 dB`.

* [x] **Шаг 16.3 (`src/config.rs`, `src/ui/views/settings.rs`, `src/ui/app.rs`):** `config.spectrum_spread` + `effective_spectrum_spread()` + `spread_from_setting()` (env `DEEZER_TUI_SPECTRUM_SPREAD`, `off`/`0` = выключен), строка Settings «Spectrum Neighbor Spread» (`SPECTRUM_SPREAD_CHOICES`), `App::cycle_spectrum_spread` с применением к `Cavacore` и сохранением конфига.

* [x] **Шаг 16.4 (замер и тесты):** зонд на реальных MP3 (three трека, 15 полос, 40–65 с, 60 FPS, метрики `сред/σ/Δ/%<0.2/%>0.85/плато/bleed`) — плато `FramePeak` 0.82–0.83 → 0.06–0.07, `Δ` 0.031 → 0.024, `Legacy` плато 1.00 → 0.51–0.73; зонд удалён, +3 регрессии в `cavacore.rs` (+7 в `config.rs`, циклы строки в `settings.rs`/`app.rs`). Гейт: `cargo build` без warning'ов, `cargo test` **131/131**, `cargo clippy --all-targets -- -D warnings` exit 0.


