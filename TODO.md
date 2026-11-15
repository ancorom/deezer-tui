# TODO.md — Пошаговый чеклист задач для AI-агента (Редакция: Ноябрь 2026, v2.0)

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
