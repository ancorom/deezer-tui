# ARCHITECTURE.md — Архитектура и спецификация deezer-tui v2.4 (Редакция: Ноябрь 2026)

## 1. Технологический стек
Проект собирается в единый монолитный бинарник на Rust (Edition 2024, MSRV 1.88). Все зависимости зафиксированы в корневом Cargo.toml:
- Интерфейс и графика: ratatui 0.30.2 (layout-cache, macros), crossterm 0.29.0, image 0.25.10, ratatui-image 11.1.0.
- Аудио и DSP: symphonia 0.6.1 (flac, mp3, aac, isomp4), cpal 0.18.2, rtrb 0.4.1, realfft 3.5.0.
- Сеть и криптография: tokio 1.54, reqwest 0.13.5, blowfish 0.9, cbc 0.1, cipher 0.4, md-5 0.10, hex 0.4, tempfile 3.15.
- Система и конфиги: serde 1.0, serde_json 1.0, toml 1.2.1, souvlaki 0.9.3 (use_zbus), anyhow 1.0, tracing 0.1.

## 2. Структура проекта: что где лежит и для чего

    deezer-tui/
    ├── Cargo.toml                  # Единственный crate: deezer-tui (bin). Edition 2024, MSRV 1.88.
    │                               #   Фичи: pipewire (cpal/pipewire) — включается явно.
    ├── AGENT.md                    # Инструкции/ограничения для AI-разработчика (NEVER VIOLATE).
    ├── ARCHITECTURE.md             # Этот файл: стек, карта модулей, пайплайны, спецификации.
    ├── PLAN.md                     # Этапы разработки и критерии приёмки (исторический план).
    ├── TODO.md                     # Атомарный чеклист задач по фазам (1..13, все [x]).
    ├── README.md                   # Публичный README для GitHub (EN + RU).
    ├── CHANGELOG.md               # История версий v2.0 → v2.4 по фазам.
    ├── .cargo/config.toml          # Локальные настройки cargo (профайлы сборки).
    ├── docs/
    │   ├── crates/                 # cargo-doc-md: markdown-доки по всем крейтам (справка API).
    │   │   └── progress.md         # Milestones по фазам: что сделано, тесты, гейты.
    │   └── reference/
    │       ├── cava/               # Эталонный C-исходник cavacore (cavacore.c/h, тесты).
    │       ├── deezer_rust_ref/    # Рабочий референс авторизации/Flow/Lyrics (Rust).
    │       └── opendeezer_go_ref/  # Рабочий референс Blowfish (Go, OpenDeezer).
    └── src/
        ├── main.rs                 # init_file_logger (tracing→deezer-tui.log), AppConfig::load(),
        │                           #   App::with_config(cfg).run(). Точка входа.
        ├── config.rs               # AppConfig: ~/.config/deezer-tui/config.toml (serde+toml).
        │                           #   Поля: arl, volume, sample_rate_hz, preferred_quality,
        │                           #   show_spectrum, show_lyrics, db_scale, sidebar_collapsed.
        │                           #   load()/save(); ARL: env DEEZER_ARL > файл.
        ├── mpris.rs                # MPRIS (souvlaki/zbus): org.mpris.MediaPlayer2.deezer_tui.
        │                           #   Аппаратные media-клавиши, now-playing для GNOME/KDE,
        │                           #   playerctl. cover_url — 500x500. Поток D-Bus (Mpris::attach).
        ├── api/
        │   ├── mod.rs              # pub-экспорт: client, crypto, models.
        │   ├── crypto.rs           # generate_blowfish_key (MD5+XOR secret), decrypt_chunk (2048 B,
        │                           #   chunk_index%3==0), Blowfish-CBC NoPadding, IV 0..7.
        │   ├── models.rs           # Модели API: TrackData (date_add, formatted_date_add → YYYY-MM-DD,
        │                           #   has_track_token, cover), PlaylistData, SearchResults,
        │                           #   LyricLine/Lyrics, UserData. deserialize_u64_flexible (u64/строка).
        │   └── client.rs           # DeezerClient: login_arl (gw-light getUserData → api_token, sid,
        │                           #   license_token), is_authenticated, session. search,
        │                           #   get_track, get_stream_url (media.deezer.com/v1/get_url),
        │                           #   get_flow, get_lyrics, get_playlists, playlist ops,
        │                           #   get_favorites. cover_big→cover_medium→cover.
        ├── audio/
        │   ├── mod.rs              # pub-экспорт модулей audio.
        │   ├── stream.rs           # DeezerStream: Read+Seek+Send+Sync+MediaSource over
        │                           #   Arc<RwLock<Vec<u8>>> (in-memory, v2.2) + независимый курсор
        │                           #   pos: usize (Clone сбрасывает). from_decrypted/
        │                           #   try_from_decrypted/from_encrypted, fetch (full),
        │                           #   fetch_streaming (256KB prebuffer + bg tokio). read()
        │                           #   проверяет abort_download → Ok(0). Seek ждёт ≤1.5с.
        │   ├── decoder.rs          # Symphonia 0.6: decode FLAC→f32 → rtrb AudioRing (stereo) +
        │                           #   VisRing (mono (L+R)/2). DecoderCommand::Seek, stop_flag,
        │                           #   push_all_cancelable, StreamTechInfo (codec, sr, kbps, size).
        │   ├── output.rs           # cpal 0.18: выбор SampleRate(44100) из supported_output_configs,
        │                           #   RT-callback (no allocs/locks, rtrb::Consumer pop),
        │                           #   AtomicBool paused, AtomicU64 played_frames, громкость.
        │   ├── ring_buffers.rs    # AudioRingBuffer 65536 (stereo interleaved f32),
        │                           #   VisRingBuffer 16384 (mono f32) на rtrb 0.4.
        │   └── prefetch.rs         # Gapless prefetch: за ~20с до конца текущего трека
        │                           #   скачивает+расшифровывает следующий в PrefetchSlot
        │                           #   (Mutex), playback loop читает. Без взаимных блокировок.
        ├── dsp/
        │   ├── mod.rs              # pub-экспорт cavacore.
        │   └── cavacore.rs         # Порт cavacore: dual FFT (4096 bass/2048 mid-high), Hann,
        │                           #   log-octave bands 20Hz–20kHz (15 полос = ⅔ окт/шаг, 30 = ⅓),
        │                           #   энергия полосы 0.65·RMS + 0.35·peak (v2.3), дробная
        │                           #   интерполяция узких полос, eq = norm_fft·(f_center/300)^0.25
        │                           #   (clamp 0.55–2.3), Monstercat spatial filter, gravity
        │                           #   falloff, autosens, dB scale, set_num_bars (ресайз).
        │                           #   process(&[f32]).
        └── ui/
            ├── mod.rs              # pub-экспорт: app, theme, views, widgets.
            ├── theme.rs            # TrueColor палитра (#13151F/#1E2233/#3E445E/#ECEFF4/#94A3B8)
            │                       #   + градиент #4FD1C5→#7F5AF0 (линейная интерполяция).
            ├── app.rs              # AppShell: 5 вкладок, глобальный player bar + спектр,
            │                       #   event loop (crossterm poll), tokio BgEvent-канал,
            │                       #   playback_seq (Arc<AtomicU64>), session reuse (self.client),
            │                       #   PlaybackQueue (n/p), spawn_* (fetch_more_flow, prefetch,
            │                       #   initial_load, search), адаптивный спектр (set_num_bars при ресайзе),
            │                       #   Track Menu (m/M), Flow Modes (f/g, FLOW_MODES), Toast (3 с),
            │                       #   бесконечный поиск (30/стр., PageDown, автозагрузка), search_local_tracks.
            │                       #   div_ceil(width/2).clamp(8,160), player row с хоткеями.
            ├── views/
                ├── now_playing.rs  # Экран 1: обложка (sextant 2×3, до 40 строк) + метаданные +
                │                   #   тех. параметры + панель LYRICS (автопрокрутка, подсветка).
                ├── search.rs       # Экран 2: строка поиска (модальный ввод / / f), 3 колонки
                │                   #   результатов (SearchRow: artist/album/track/time/added/fav),
                │                   #   бесконечный поиск 30/стр. (PageDown, автозагрузка).
                ├── library.rs      # Экран 3: двухпанельный фокус (плейлисты | таблица треков),
                │                   #   LibrarySortBy (Default/Title/Artist/Album, o/r),
                │                   #   колонка ADDED (▲/▼), CHOOSE PLAYLIST модал (a).
                ├── flow_lyrics.rs  # Экран 4: бесконечный Flow (автоподгрузка ≤3) + Live Lyrics
                │                   #   (y toggle), Flow Modes (f/g: moods/genres), обложка
                │                   #   до 24 строк (v2.4), разделитель │.
                ├── settings.rs     # Экран 5: ARL inline (L), quality FLAC/320/128, Show
                                    #   Spectrum, dB Scale. Settings::render (Widget).
            └── widgets/
                ├── spectrum.rs     # Spectrum: subpixel-бары ▁..█, peak-hold '▪', 1:1
                │                   #   рендер (v2.3: без sample_slice).
                ├── cover_art.rs    # CoverArt: sextant 2×3 (6 субпикселей/ячейка,
                │                   #   U+1FB00–U+1FB3B + ▌/▐/█), кластеризация fg/bg,
                │                   #   Lanczos3 (down) / CatmullRom (up); режим HalfBlock (▀).
                └── modal.rs        # Modal: общее модальное окно (Track Menu (m), Flow
                                    #   Modes (f/g), ARL, CHOOSE PLAYLIST, TrackInfo).

## 3. Архитектура аудиопотоков без задержек и рассинхрона (Zero-Latency Audio Pipeline v2.2)
1. CPAL 0.18 Real-Time Callback (Точка синхронизации):
   - Владеет `rtrb::Consumer<f32>` (`AudioRingBuffer`, 65536 сэмплов) и `rtrb::Producer<f32>` (`VisRingBuffer`, 16384 моно-сэмпла).
   - Проверяет атомарный флаг `paused: Arc<AtomicBool>`: если `true`, немедленно заполняет выходной буфер тишиной (`0.0f32`), не потребляя сэмплы из `AudioRingBuffer`. Задержка паузы равна размеру одного аппаратного периода ALSA/PipeWire (≈ 5–10 мс).
   - Когда `paused == false`, вычитывает стерео-пары (L, R) из `AudioRingBuffer`, умножает на громкость `volume`, записывает в аудиокарту, **инкрементирует атомарный счётчик `played_frames: Arc<AtomicU64>`** и отправляет моно-сэмпл (L + R) * 0.5 в `VisRingBuffer`.
   - Благодаря этому спектр `Cavacore` визуализирует ровно тот звук, который звучит в этот момент, и мгновенно затухает при паузе.
2. Symphonia 0.6 Decoder Thread + Seek Channel:
   - Читает байты из DeezerStream, декодирует FLAC (16-bit / 44.1 kHz, 1411 kbps) в поток f32 сэмплов в диапазоне [-1.0, 1.0].
   - Записывает интерливированные стерео-сэмплы в rtrb::Producer<f32> (AudioRingBuffer).
   - Принимает команды `DecoderCommand::Seek(u64)` (целевая секунда) через `std::sync::mpsc::Receiver<DecoderCommand>`.
   - При получении `Seek(sec)` вызывает `format.seek(SeekMode::Accurate, SeekTo::Time { time: Time::from(sec), track_id: Some(track_id) })`, сбрасывает `decoder.reset()` и обновляет `played_frames = sec * sample_rate`.
   - Извлекает из `codec_params` точные параметры потока: `sample_rate`, `bits_per_sample`, `n_frames`, вычисляет реальный битрейт `kbps = (stream_bytes * 8) / (duration_secs * 1000)` и отправляет структуру `StreamTechInfo` в UI.
   - По окончании декодирования пакета (`EOF`) ждёт опустошения `AudioRingBuffer` и шлёт `BgEvent::TrackEnded`, вызывая автоматический переход к следующему треку в очереди `PlaybackQueue`.
3. Tokio Async Runtime (Фоновый пул):
   - Выполняет авторизацию и запросы к API Deezer (и публичный api.deezer.com в GUEST-режиме).
   - Скачивает зашифрованный поток, на лету расшифровывает блоки по 2048 байт и сохраняет их в буфер/временный файл DeezerStream, а также запускает префетч следующего трека за 20 секунд до конца текущего.
   - Загружает и декодирует JPEG-обложки альбомов через image::ImageReader, отправляя готовые буферы в UI через tokio::sync::mpsc.
4. UI & DSP Render Thread (ratatui 0.30 + Cavacore, 60 FPS):
   - Каждые 16.6 мс вычитывает накопившиеся сэмплы из VisRingBuffer во внутренний кольцевой буфер окна FFT (размер 4096 сэмплов), вызывает Cavacore::process(), получает вектор высот столбиков [f32; num_bars] в диапазоне [0.0, 1.0] и отрисовывает кадр терминала.
   - Позиция воспроизведения считается строго по `played_frames / sample_rate` (не по системному таймеру), а длительность — из `StreamTechInfo::duration_secs` (для гостевого превью принудительно 30.0 с).

### 3.1 Изменения v2.1/v2.2: гонки, сессия, очередь, prefetch

1. **In-memory буфер `DeezerStream` (v2.2, `stream.rs`).** Буфер расшифрованных данных — `Arc<RwLock<Vec<u8>>>`, каждый экземпляр владеет независимым курсором `pos: usize` (`Clone` сбрасывает `pos = 0`). Это убило гонку двух FD `File::try_clone()` на один OS file offset: `seek(0)` декодера больше не конфликтует с фоновой записью чанков и не портит FLAC-заголовок при быстрой смене трека (фриз при двойном `n`/`Enter` в Flow). `read()` проверяет `abort_download` → мгновенно `Ok(0)` без зависания. Временных файлов больше нет.
2. **Счётчик поколений `playback_seq: Arc<AtomicU64>` (v2.2, `app.rs`).** Каждый вызов `start_track_playback` получает `req_id = playback_seq.fetch_add(1, SeqCst) + 1`. Все async-задачи и `poll_bg_events` сравнивают свой `req_id` с актуальным `playback_seq.load(SeqCst)`: устаревшая задача выходит, не запуская декодер и не шлюя `BgEvent::TrackStarted`/`TrackEnded` (с `req_id`) — поздний `TrackStarted` мёртвой задачи не затирает активный поток `dead-vis_consumer`'ом. Перед открытием нового потока — `self.audio_output = None`.
3. **Переиспользование сессии (v2.2).** `BgEvent::LoggedIn { client, .. }` сохраняет авторизованный `DeezerClient` в `self.client = Arc::new(client)`. Все фоновые задачи берут `let client = (*self.client).clone()` и вызывают `login_arl` только при `!client.is_authenticated()` — никакого нового клиента + повторного логина на каждый трек. Если `get_stream_url` упал (истёк `TRACK_TOKEN` у трека Flow) — запрашивается свежий `get_track(&track.track_id)` и повторяется `get_stream_url` с фолбэком.
4. **Очередь воспроизведения `PlaybackQueue` (v2.1).** Глобальная очередь треков: `n`/`p` (и `>`/`<`) листают очередь; `spawn_fetch_more_flow` автоподгружает бесконечный Flow при ≤3 треках, сбрасывая `loading_more_flow` по `BgEvent::FlowLoadFailed` (сетевая ошибка больше не оставляет флаг навсегда). Визуальный список Flow 1:1 с `flow_tracks` (обрезка `.take(15)` убрана); выбор активного трека — по позиции `track_id`, а не по индексу.
5. **Gapless prefetch (v2.1, `prefetch.rs`).** За ~20 секунд до конца текущего трека `spawn_prefetch_next` скачивает и расшифровывает следующий трек (`DeezerStream::fetch` целиком) в `PrefetchSlot` (`Mutex` — единственный разделяемый мутабельный ресурс; стороны не блокируют друг друга). `fetch_streaming` (пребуфер 256 КБ + фоновая докачка) — только для интерактивного трека.
6. **Tokio `worker_threads(4)` (v2.2).** Пул расширен 2→4: параллельно живут HTTP-запросы, потоковая докачка, prefetch и декодирование обложек.

## 4. Спецификация внутреннего API Deezer и криптографии Blowfish

### 4.1 Авторизация и получение URL потока
1. Инициализация сессии:
   - Пользователь указывает куку arl в ~/.config/deezer-tui/config.toml (или переменной окружения DEEZER_ARL).
   - Отправляется запрос POST [https://www.deezer.com/ajax/gw-light.php?method=deezer.getUserData&input=3&api_version=1.0&api_token=](https://www.deezer.com/ajax/gw-light.php?method=deezer.getUserData&input=3&api_version=1.0&api_token=) с заголовком Cookie: arl=<ARL>.
   - Из JSON-ответа (results) парсится `UserData`:
     - results.checkForm -> `api_token` для всех последующих вызовов gw-light.php.
     - results.USER.OPTIONS.license_token -> токен лицензии для запроса аудиопотока.
     - results.USER.USER_ID / USER.USER_NAME -> `Session { api_token, license_token, user_id, user_name }`.
     - Валидация: `USER_ID == "0"` или пустой `api_token` -> `DeezerError::Auth` (ARL недействителен/устарел).
2. Получение метаданных трека:
   - Вызов gw-light.php с методом song.getData и телом {"SNG_ID": "<id>"} возвращает SNG_ID, TRACK_TOKEN, ALB_PICTURE и FILESIZE_FLAC.
3. Получение прямой ссылки на зашифрованный FLAC CDN:
   - Запрос POST [https://media.deezer.com/v1/get_url](https://media.deezer.com/v1/get_url) с JSON-телом, запрашивающим cipher "BF_CBC_STRIPE" и форматы "FLAC", "MP3_320", "MP3_128" для указанного TRACK_TOKEN и license_token.
   - Из ответа извлекается data[0].media[0].sources[0].url.

### 4.2 Генерация ключа Blowfish и расшифровка чанков (BF_CBC_STRIPE)
- Секретная константа Deezer: const SECRET: &[u8; 16] = b"g4el58wc0zvf9na1";
- Вектор инициализации (IV): const IV: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
- Алгоритм расчёта 16-байтного ключа и расшифровки чанка 2048 байт на Rust:

    use blowfish::Blowfish;
    use cbc::Decryptor;
    use cipher::{BlockDecryptMut, KeyIvInit, block_padding::NoPadding};
    use md5::{Md5, Digest};

    pub fn generate_blowfish_key(sng_id: &str) -> [u8; 16] {
        let hash = Md5::digest(sng_id.as_bytes());
        let id_md5 = hex::encode(hash);
        let id_bytes = id_md5.as_bytes();
        let secret = b"g4el58wc0zvf9na1";
        let mut key = [0u8; 16];
        for i in 0..16 {
            key[i] = id_bytes[i] ^ id_bytes[i + 16] ^ secret[i];
        }
        key
    }

    /// Расшифровывает один 2048-байтовый чанк на месте, если `chunk_idx % 3 == 0` и `chunk.len() == 2048`.
    pub fn decrypt_chunk(chunk: &mut [u8], chunk_idx: usize, key: &[u8; 16]) -> Result<(), Error> {
        if !chunk_idx.is_multiple_of(3) || chunk.len() != CHUNK_SIZE {
            return Ok(());
        }
        let bf = blowfish::Blowfish::new_from_slice(key)?;
        let cipher = BlowfishCbcDec::inner_iv_init(bf, GenericArray::from_slice(&IV));
        cipher.decrypt_padded_mut::<NoPadding>(chunk)?;
        Ok(())
    }

    /// Расшифровывает весь буфер потока по чанкам `CHUNK_SIZE` (каждый `chunk_idx % 3 == 0`).
    pub fn decrypt_stream(data: &mut [u8], key: &[u8; 16]) -> Result<(), Error> {
        let (chunks, _rest) = data.as_chunks_mut::<CHUNK_SIZE>();
        for (block_idx, block) in chunks.iter_mut().enumerate() {
            decrypt_chunk(block, block_idx, key)?;
        }
        Ok(())
    }

- Правило потоковой расшифровки и кэширования:
  - Аудиопоток делится на блоки фиксированного размера 2048 байт (chunk_index = byte_offset / 2048).
  - Если chunk_index % 3 == 0 И размер прочитанного блока равен ровно 2048 байт, вызывается decrypt_chunk(chunk, &key).
  - Все остальные блоки (chunk_index % 3 != 0 или финальный неполный блок меньше 2048 байт) остаются без изменений.
  - Расшифрованные чанки лежат в in-memory буфере `DeezerStream` (`Arc<RwLock<Vec<u8>>>`, v2.2) с независимым курсором `pos: usize` — мгновенный Seek вперёд/назад без временных файлов и без гонки двух FD на один OS offset.

## 5. Математическая спецификация Cavacore (src/dsp/cavacore.rs)
Полный порт актуального алгоритма из docs/reference/cava/cavacore.c на чистый Rust:
1. Буферы FFT:
   - Для басовых частот (ниже 300 Гц) используется окно N_bass = 4096 сэмплов для высокого частотного разрешения.
   - Для средних и высоких частот используется окно N_mid_high = 2048 сэмплов для быстрой реакции по времени.
2. Оконная функция Ханна:
   Перед подачей в realfft каждый элемент входного буфера умножается на коэффициент:
   w[n] = 0.5 * (1.0 - (2.0 * PI * n as f32 / (N - 1) as f32).cos())
3. Логарифмическая разметка частот по столбикам (num_bars):
   Для диапазона f_min = 20.0 Гц и f_max = 20000.0 Гц (полный слышимый диапазон, v2.3) граничная частота i-го столбика вычисляется как:
   f[i] = f_min * (f_max / f_min).powf(i as f32 / num_bars as f32)
   Октавная шкала (v2.3): f_max/f_min = 10 октав — 15 полос = шаг ⅔ октавы (20–32–50–80…20 000 Гц), 30 полос = ⅓ октавы.
   Частоты переводятся в индексы бинов FFT: bin[i] = (f[i] * N as f32 / sample_rate as f32) as usize, а также применяется частотная эквализация (либо опциональное dB-масштабирование из cavacore 2026) для равномерного заполнения спектра.
   **EQ-веса v2.3:** `eq = norm_fft * freq_tilt`, где `freq_tilt = (f_center / 300.0).powf(0.25).clamp(0.55, 2.3)` и `f_center = (f_lo * f_hi).sqrt()` (геометрический центр полосы) — мягкая компенсация розового шума без взрыва ВЧ; v2.2-ное деление на `bin_count.sqrt()` убрано (компенсируется RMS-энергией ниже). `norm_fft = 4.0 / fft_size` (нормализация окна Ханна).
   **Сбор магнитуд v2.2:** для узких полос (`hi - lo <= 1`, когда полоса ложится в один FFT-бин) берётся дробная линейная интерполяция между соседними бинами: `f_center` → `exact_bin = f_center * N / sample_rate`, `sum = spectrum[b0].norm() * (1 - frac) + spectrum[b0+1].norm() * frac`. Убирает «пары-близнецы» (одинаковые высоты соседних полос с пустыми провалами между ними). Для широких полос — сумма `c.norm()` по `[lo..hi]`. **Энергия полосы v2.3:** амплитуда полосы = `mag = 0.65 * rms + 0.35 * peak` (RMS + пик по сэмплам полосы), в dB-режиме логарифмируется как раньше.
4. Фильтр пространственного сглаживания Monstercat:
   Для каждого столбика i соседние столбики j подтягиваются к локальным пикам:
   bars[i] = bars[i].max(bars[j] / 1.5_f32.powi((i as i32 - j as i32).abs()))
5. Гравитация падения (Falloff) и интегральный фильтр:
   - Если новое значение v_new[i] меньше v_prev[i], столбик опускается с квадратичным ускорением:
     v_fall[i] = v_prev[i] * (1.0 - gravity * fall_step[i].powi(2))
     где fall_step[i] увеличивается на 1.0 каждый кадр падения и сбрасывается в 0.0 при новом пике.
   - Финальное сглаживание:
     out[i] = mem[i] * 0.77 + v[i] * (1.0 - 0.77)
6. Авточувствительность (autosens):
   Если в текущем кадре хотя бы один столбик превысил 1.0, множитель sensitivity умножается на 0.985; если перегрузки не было — умножается на 1.001.
7. Динамическое число полос (v2.3):
   `set_num_bars(desired_bars)` — пересчёт границ, bin-индексов и EQ-весов; `mem`/`prev` ресайзятся (новые полосы обнулены), FFT-буферы и окна Ханна сохраняются (аудио-состояние не сбрасывается); вызывает App при ресайзе окна. `Spectrum::render` отрисовывает полосы 1:1 — без ресемплинга (`sample_slice` убран).

## 6. Спецификация экранов и палитры UI (src/ui/)

### 6.1 Цветовая схема (TrueColor RGB)
- Основной фон терминала: Rgb(19, 21, 31) (#13151F).
- Фон карточек и модальных окон: Rgb(30, 34, 51) (#1E2233).
- Цвет рамок неактивных блоков: Rgb(62, 68, 94) (#3E445E).
- Основной текст: Rgb(236, 239, 244) (#ECEFF4), вторичный текст: Rgb(148, 163, 184) (#94A3B8).
- Градиент спектра и активных элементов: линейная интерполяция от бирюзового Rgb(79, 209, 197) (#4FD1C5) до фиолетового Rgb(127, 90, 240) (#7F5AF0).

### 6.2 Unified Shell Layout v2.2 (единый сквозной каркас)
В отличие от v1.0, где каждая вкладка рисовала собственный плеер и собственный сайдбар, в v2.0+ используется **единый сквозной каркас окна (`AppShell`)**:

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ DEEZER-TUI • [1] Now Playing  [2] Search  [3] Library  [4] Flow  [5] Settings   ONLINE │
├──────────────────┬─────────────────────────────────────────────────────────────────────┤
│ NAVIGATION   [S] │                                                                     │
│                  │                                                                     │
│  [1] Now Playing │                 АКТИВНЫЙ ЭКРАН (VIEWPORT)                          │
│  [2] Search      │   • Экран 1: Обложка 500x500 (quad 2x2) + Метаданные + LYRICS [Y]  │
│  [3] Library     │   • Экран 2: Строка поиска (/ для ввода) + таблица результатов      │
│      ├─ Favorites│   • Экран 3: Плейлисты | треки (… ADDED … TIME), сортировка [o]/[r] │
│      ├─ My Mix   │   • Экран 4: Бесконечный Flow (слева) + Live Lyrics (справа)       │
│      └─ Chill    │   • Экран 5: Настройки (ARL, Качество FLAC/320/128, Спектр, dB)    │
│  [4] Flow        │                                                                     │
│  [5] Settings    │                                                                     │
├──────────────────┴─────────────────────────────────────────────────────────────────────┤
│ 02:14 [←] ━━━━●─── [→] 03:02   [⏮ p] [⏸ Space] [⏭ n]   FLAC  Vol [-/+]: 100%          │
│ ▁▂▃▅▇█▆▄▂▃▅▆█▇▅▃▂▁▂▄▆▇█▆▄▃▂▁▂▃▄▆▇▅▃▂▁  (Глобальный спектр, адаптивен по ширине, [v])  │
│ q quit | 1-5 views | [/] search | [y] lyrics | [m] track menu | [f] flow modes | ...   │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Ключевые UX-инварианты v2.3:
1. **Глобальный плеер и спектр снизу:** Панель прогресса с **подсказками хоткеев** (`02:14 [←] ━━━━●─── [→] 03:02 [⏮ p] [⏸ Space] [⏭ n] FLAC Vol [-/+]: 100%`; в узкой строке — компактный `00:00/03:02 [⏮ p] ▶ [⏭ n]`), реальный битрейт (`FLAC 16-bit/44.1kHz | 1411 kbps`) и спектроанализатор отрисовываются на уровне `App` и доступны на **любой** вкладке. Нижняя строка: `q quit | 1-5 views | [/] search | [y] lyrics | [m] track menu | [f] flow modes | [o/r] sort | [i] info | [s] sidebar | [v] spectrum`.
2. **Мгновенное управление воспроизведением:** Вне режима набора текста клавиши `Space` (Play/Pause), `n`/`>` (Next), `p`/`<` (Prev), `←`/`→` (Seek ± 5 с), `+`/`=`/`-` (Volume ±5 %), `v` (Toggle Spectrum), `s` (Toggle Sidebar), `i` (Track & Stream Info), `y` (Lyrics) работают **везде**.
3. **Модальный фокус строки поиска:** На вкладке `[2] Search` ввод текста активируется клавишей `/` или `f` (и выключается по `Enter` — поиск — или `Esc`). Когда ввод не активен, все хоткеи плеера (`Space`, `1`–`5`, `←`/`→`) работают штатно.
4. **Двухпанельная навигация в Фонотеке (`[3] Library`):** Стрелки `←`/`→`, `Tab` (`h` — плейлисты, `l` — треки) переключают фокус между списком плейлистов и таблицей треков. Сортировка: `o` (критерий Added/Title/Artist/Album по кругу), `r` (направление ▲/▼). `a` — модал CHOOSE PLAYLIST (добавить трек в плейлист).
5. **Адаптивный спектр:** Число полос Cavacore = `div_ceil(area.width / 2).clamp(8, 160)`; при ресайзе окна — `Cavacore::set_num_bars` (FFT-буферы и память сохраняются, пересоздания нет); `Spectrum::render` рисует полосы 1:1 (без ресемплинга, `fit_bars = div_ceil(w/2)`) — весь диапазон 20 Гц–20 кГц умещается в любое окно без обрезки справа.
### 6.3 Виджеты
- Субсимвольный спектр (`Spectrum` / SpectrumWidget):
  Использует массив символов [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']. Полная высота столбика в субпикселях равна `(val * rows as f32 * 8.0) as usize`. Целые клетки заполняются символом '█', верхушка — остатком от деления на 8, а выше удержанного пика ставится точка '▪'. **v2.3:** 1:1-рендер — cavacore-полосы рисуются напрямую (`sample_slice` убран), число полос задаётся в Cavacore (`set_num_bars`), `fit_bars = div_ceil(w/2).min(bars.len())`.
- Рендерер обложек (`CoverArtWidget`):
  **TrueColor 1:1-пиксели (v2.3):** `resized_pixels` ресайзит изображение под пиксельную сетку терминала w × 2h (Lanczos3 при даунскейле, CatmullRom при апскейле, Nearest при точном совпадении); каждая ячейка рисуется полублоком '▀' — fg = верхний субпиксель, bg = нижний субпиксель: честные 24-битные цвета на каждый субпиксель без дITHER-аппроксимаций. Quad subpixel 2×2 v2.2 (`QUADRANT_GLYPHS`, `best_quadrant_cell`) удалён. **Sextant (v2.4, default):** `CoverRenderMode::Sextant` — сетка 2×3 (6 субпикселей/ячейка, 3× разрешение): `resized_pixels_sextant` (2w × 3h, Lanczos3/CatmullRom/Nearest); на ячейку — пара пикселей с максимальным цветовым расстоянием задаёт два кластера (fg = светлее, bg = темнее), каждый субпиксель присоединяется к ближайшему кластеру → 6-бит маска → глиф `SEXTANT_CHARS` (U+1FB00–U+1FB3B; маски 21/42 → `▌`/`▐`, 0 → space, 63 → `█`); почти монохромная ячейка (dist² ≤ 25) → `█`. `HalfBlock` (▀, v2.3) — опциональный режим (`with_render_mode`). Обложки 500×500 (клиент: `cover_big`→`cover_medium`→`cover`; MPRIS — 500×500).
- Модальные окна (`widgets/modal.rs`):
  `ChoosePlaylist` (добавление трека в плейлист), `TrackInfoModal` (`[i]` Track & Stream Info: кодек, sample rate, битрейт, размер, режим — данные из `StreamTechInfo`). **Track Menu (v2.3, `m`/`M`)** — 5 пунктов: ❤️/💔 favorite (динамический статус), ➕ добавить в плейлист, 📻 Song Radio (`song.getSearchTrackMix`; fallback /artist/{id}/radio → /top → public chart → очередь), ℹ️ информация, Esc. **Flow Modes (v2.3, `f`/`g`)** — My Flow / 7 настроений / 10 жанров. **Toast (v2.3):** разовое уведомление в статусной строке, 3 с, градиентный фон.

## 6.4 Экраны (v2.3)
- Экран 01 (Now Playing, `views/now_playing.rs`): Обложка 500×500, sextant 2×3 (высота `clamp(4, 40)` строк, ширина `min(26)` колонок) + метаданные (трек/исполнитель/альбом) + тех. параметры потока (Codec, Sample Rate, Bit Depth, Bitrate kbps, Stream Size MB) + панель LYRICS (автоскролл по таймкоду, подсветка текущей строки бирюзовым `#4FD1C5` с тёмным текстом; `y` — вкл/выкл).
- Экран 02 (Search, `views/search.rs`): Строка поиска — модал ввода по `/` (или `f`), `Enter` — поиск, `Esc` — сброс фокуса. Результаты: таблица `SearchRow` (ARTIST / ALBUM / TRACK / TIME / ADDED / ♥) с колонкой `added: String` (дата добавления в Deezer, YYYY-MM-DD или "—"). Track Menu (`m`) — по выделенной строке. **Бесконечный поиск (v2.4):** 30/страницу (`search_paged`/`search_public_paged`, index/limit), автозагрузка при `selected + 5 ≥ len` и по `PageDown` (±10 строк), дедупликация по `track_id`; `sanitize_query` (дефисы/тиры/`_` → пробел); `search_local_tracks` — мгновенные совпадения в `library_tracks` + кэше плейлистов (❤).
- Экран 03 (Library, `views/library.rs`): Двухпанельный фокус (плейлисты | таблица треков). Колонки: `#`, ТРЕК, ИСПОЛНИТЕЛЬ, АЛЬБОМ, **ADDED** (YYYY-MM-DD, `track.added`, ширина 12, виден при ширине ≥ 68), TIME, ♥. Заголовок `ADDED ▲/▼` на сортировке Default (по дате добавления); `o` — круг по критериям, `r` — направление. Модал CHOOSE PLAYLIST (`a`) — добавление текущего трека в выбранный плейлист.
- Экран 04 (Flow & Live Lyrics, `views/flow_lyrics.rs`): Бесконечный Flow (автоподгрузка при ≤3 треках; обложка `min(24)` строк) + Live Lyrics справа (разделитель `│`), автоскролл, подсветка. При старте трека lyrics очищается; для инструментальных — по центру "No synchronized lyrics available". **Flow Modes (v2.3):** модал `f` (вне Search) / `g` (глобально) — My Flow, 7 настроений (chill/motivation/party/focus/romance/melancholy/workout) и 10 жанров (`get_genre_flow`: /chart/{genre_id}/tracks?limit=50 → /genre/{id}/radios → /radio/{id}/tracks; fallback — public chart) и 7 настроений (`get_mood_flow`: кураторские плейлисты /search/playlist?q=... → /playlist/{id}/tracks; fallback — поиск треков / public chart); выбранный режим запоминается (`spawn_fetch_more_flow` читает `current_flow_mode_idx`).
- Экран 05 (Settings, `views/settings.rs`): ARL-токен **inline** (`L` — перейти к вводу; статус `Deezer HiFi / Premium / Free / Guest`), качество (`FLAC 1411 kbps`, `MP3 320 kbps`, `MP3 128 kbps`), переключатели `Show Spectrum`, `dB Scale`, `Show Lyrics`, `Sidebar Collapsed`.

## 7. MPRIS и конфигурация
### 7.1 MPRIS (`src/mpris.rs`)
- Транспорта: souvlaki 0.9.3 (use_zbus) поверх D-Bus, имя сервиса `org.mpris.MediaPlayer2.deezer_tui`.
- Объявляет now-playing (метаданные + `cover_url` 500×500), положение/прогресс, громкость; обрабатывает аппаратные media-клавиши (Play/Pause/Next/Previous/Stop) и запросы `playerctl`.
- Запускается через `Mpris::attach` (поток D-Bus) на старте App; при отсутствии D-Bus-сессион-шину приложение работает без него (feature-некритично).

### 7.2 Конфигурация и окружение
- Файл конфига: `~/.config/deezer-tui/config.toml` (serde + toml; создаётся при первом сохранении).
  | Поле | Тип | Назначение |
  | --- | --- | --- |
  | `arl` | `Option<String>` | ARL-токен Deezer (кука веб-сайта): HiFi-логин; без токена — guest-режим (30-сек MP3-превью) |
  | `volume` | `f32` | Громкость 0.0–1.0 |
  | `sample_rate_hz` | `f32` | Частота вывода (44100; FLAC ресэмплируется) |
  | `preferred_quality` | `String` | `FLAC` / `MP3_320` / `MP3_128` |
  | `show_spectrum` | `bool` | Спектр вкл/выкл |
  | `show_lyrics` | `bool` | Панель lyrics вкл/выкл |
  | `db_scale` | `bool` | dB-масштаб в спектре |
  | `sidebar_collapsed` | `bool` | Сайдбар свёрнут |
- Переменные окружения: `DEEZER_ARL` (перекрывает `arl` из файла), `DEEZER_TUI_LOG_DIR` (перекрывает каталог логов).
- Лог: `~/.config/deezer-tui/deezer-tui.log` (tracing → tracing-appender daily rolling).

## 8. Полная карта хоткеев (v2.3)
| Клавиша | Действие |
| --- | --- |
| `1`–`5`, `F1`–`F5`, `Tab`/`Shift-Tab` | Вкладки Now Playing / Search / Library / Flow / Settings |
| `Space` | Play/Pause (везде) |
| `n`/`>`, `p`/`<` | Следующий/предыдущий трек в очереди |
| `←`/`→` | Seek ∓5 с (везде, кроме Library-фокуса и Settings-строк) |
| `+`/`=`, `-` | Громкость ±5 % |
| `/`, `f` (на Search) | Фокус строки поиска; `Enter` — поиск, `Esc` — снять |
| `f` (вне Search) / `g` | Модал Flow Modes (настроения/жанры) |
| `m` / `M` | Track Menu (Now Playing / Search / Library / Flow): ❤️ favorite, ➕ плейлист, 📻 Song Radio, ℹ️ info |
| `PageDown` / `PageUp` (Search) | ±10 строк; PageDown у конца — загрузка следующей страницы (30/страницу) |
| `y` | Toggle панели lyrics |
| `s` | Toggle сайдбара навигации |
| `v` | Toggle спектра |
| `i` | Модал Track & Stream Info |
| `o` / `r` (Library) | Сменить критерий сортировки / направление |
| `h` / `l` (Library) | Фокус плейлисты / треки |
| `a` (Library) | Модал CHOOSE PLAYLIST |
| `L` | Перейти к ARL-вводу (Settings) |
| `q` | Выход |
| `Enter`, `Esc` | Play/подтвердить, закрыть/снять фокус |

## 9. Что добавили v2.1/v2.2/v2.3 (кратко)
- **v2.1:** глобальная очередь воспроизведения (`PlaybackQueue`), бесконечный Flow + Live Lyrics, панель lyrics (`y`), `[i]` Track & Stream Info, поиск `/`/`f`, двухпанельная Library с сортировкой (`o`/`r`), `PrefetchedTrack`-кэш (`spawn_prefetch_next`), gapless prefetch, `fetch_streaming` (256 КБ prebuffer), `StreamTechInfo`.
- **v2.2:** in-memory буфер `DeezerStream` (`Arc<RwLock<Vec<u8>>>` + `pos`), `playback_seq: Arc<AtomicU64>`, переиспользование сессии `self.client`, `BgEvent::FlowLoadFailed`, подсказки хоткеев в player bar, адаптивный спектр (`div_ceil(w/2).clamp(8,160)`, `sample_slice`), Cavacore-интерполяция узких полос + `eq=(norm_fft*freq_tilt)/bin_count.sqrt()`, `freq_tilt=(f_hi/100.0)^0.55`, колонка ADDED (Library, `width>=68`), `TrackData::formatted_date_add()` → YYYY-MM-DD, SearchRow + `added`, quad subpixel 2×2 (`QUADRANT_GLYPHS`, `resized_quadrant`, `best_quadrant_cell`), обложки 500×500, now_playing cover `clamp(4,26)`, flow_lyrics `min(14)`, tokio `worker_threads(4)`.
- **v2.3:** Cavacore: октавная шкала 20 Гц–20 кГц (15 полос = ⅔ окт, 30 = ⅓ окт), энергия полосы `0.65·RMS + 0.35·peak`, `freq_tilt = (f_center/300)^0.25` (clamp 0.55–2.3), `eq = norm_fft·freq_tilt`, `set_num_bars` (динамический ресайз, FFT-буферы сохраняются), спектр 1:1 без ресемплинга; обложки — TrueColor 1:1-пиксели (Lanczos3, '▀' fg/bg, quad 2×2 удалён); Track Menu (`m`/`M`: favorite, плейлист, `radio.getSongRadio`, info) + Toast (3 с); Flow Modes (`f`/`g`: My Flow / 7 moods / 10 жанров через `radio.getUserRadio`, режим запоминается); тема: `BG_ELEVATED #252842`.
- **v2.4:** Sextant-обложки (2×3, 6 субпикселей/ячейка, U+1FB00–U+1FB3B + ▌/▐/█, кластеризация fg/bg, `CoverRenderMode`, обложки now_playing до 40 строк, flow до 24); Track Mix — `song.getSearchTrackMix` + публичный каскад (/track → /artist/{id}/radio → /top → public chart), авто-резолв track_id; Flow Modes — жанры: /chart/{genre_id}/tracks + /genre/{id}/radios → /radio/{id}/tracks, настроения: кураторские плейлисты /search/playlist → /playlist/{id}/tracks; поиск: `search_paged`/`search_public_paged` (30/страницу, PageDown, автозагрузка, дедупликация), `sanitize_query` (дефисы), `search_local_tracks` (медиатека + плейлисты, ❤).
