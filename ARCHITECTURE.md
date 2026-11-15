# ARCHITECTURE.md — Архитектура и спецификация deezer-tui v2.0 (Редакция: Ноябрь 2026)

## 1. Технологический стек
Проект собирается в единый монолитный бинарник на Rust (Edition 2024, MSRV 1.88). Все зависимости зафиксированы в корневом Cargo.toml:
- Интерфейс и графика: ratatui 0.30.2 (layout-cache, macros), crossterm 0.29.0, image 0.25.10, ratatui-image 11.1.0.
- Аудио и DSP: symphonia 0.6.1 (flac, mp3, aac, isomp4), cpal 0.18.2, rtrb 0.4.1, realfft 3.5.0.
- Сеть и криптография: tokio 1.54, reqwest 0.13.5, blowfish 0.9, cbc 0.1, cipher 0.4, md-5 0.10, hex 0.4, tempfile 3.15.
- Система и конфиги: serde 1.0, serde_json 1.0, toml 1.2.1, souvlaki 0.9.3 (use_zbus), anyhow 1.0, tracing 0.1.

## 2. Структура директорий проекта

    deezer-tui/
    ├── Cargo.toml
    ├── AGENT.md
    ├── ARCHITECTURE.md
    ├── PLAN.md
    ├── TODO.md
    ├── docs/
    │   ├── crates/                 # Документация cargo-doc-md 0.11 по всем крейтам
    │   └── reference/              # Оригинальный cavacore.c, cavacore_test.c и референсы Deezer
    └── src/
        ├── main.rs                 # Инициализация логгера, потоков, ratatui::init() и ratatui::restore()
        ├── config.rs               # Чтение/сохранение ~/.config/deezer-tui/config.toml (v2: preferred_quality, show_spectrum, db_scale, sidebar_collapsed)
        ├── api/
        │   ├── mod.rs
        │   ├── client.rs           # Клиент gw-light.php и media.deezer.com (v2: get_playlist_tracks)
        │   ├── crypto.rs           # Ключи MD5 и расшифровка Blowfish-CBC чанков по 2048 байт
        │   └── models.rs           # Структуры Track, Album, Artist, Playlist, SyncedLyrics
        ├── audio/
        │   ├── mod.rs
        │   ├── stream.rs           # DeezerStream (Read + Seek + MediaSource) с кэшем чанков и расшифровкой
        │   ├── decoder.rs          # Symphonia 0.6 + DecoderCommand::Seek + StreamTechInfo (v2)
        │   └── output.rs           # CPAL 0.18 + paused/played_frames/vis_producer в callback (v2)
        ├── dsp/
        │   ├── mod.rs
        │   └── cavacore.rs         # Порт алгоритма Cava на Rust + realfft 3.5
        └── ui/
            ├── mod.rs
            ├── app.rs              # AppShell: единый каркас, роутинг [1]..[5], глобальный плеер+спектр
            ├── theme.rs            # Палитра TrueColor и расчёт бирюзово-фиолетового градиента
            ├── widgets/
            │   ├── spectrum.rs     # Виджет спектра на символах ▁▂▃▄▅▆▇█ с точками пиков
            │   ├── cover_art.rs    # Рендеринг обложки через полублоки ▀ и ratatui-image
            │   └── modal.rs        # Попап CHOOSE PLAYLIST + Track Info (v2)
            └── views/
                ├── now_playing.rs  # Экран 01: Now Playing
                ├── search.rs       # Экран 02: Search & Discover (модальный ввод, v2)
                ├── library.rs      # Экран 03: Library & Overlays (двухпанельный фокус, v2)
                ├── flow_lyrics.rs  # Экран 04: Flow & Live Lyrics
                └── settings.rs     # Экран 05: Settings (v2)

## 3. Архитектура аудиопотоков без задержек и рассинхрона (Zero-Latency Audio Pipeline v2.0)
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

## 4. Спецификация внутреннего API Deezer и криптографии Blowfish

### 4.1 Авторизация и получение URL потока
1. Инициализация сессии:
   - Пользователь указывает куку arl в ~/.config/deezer-tui/config.toml (или переменной окружения DEEZER_ARL).
   - Отправляется запрос POST [https://www.deezer.com/ajax/gw-light.php?method=deezer.getUserData&input=3&api_version=1.0&api_token=](https://www.deezer.com/ajax/gw-light.php?method=deezer.getUserData&input=3&api_version=1.0&api_token=) с заголовком Cookie: arl=<ARL>.
   - Из JSON-ответа извлекаются:
     - results.checkForm -> сохраняется как api_token для всех последующих вызовов gw-light.php.
     - results.SESSION_ID -> кука sid.
     - results.USER.OPTIONS.license_token -> токен лицензии для запроса аудиопотока.
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

    pub fn decrypt_chunk(chunk: &mut [u8], key: &[u8; 16]) {
        if chunk.len() == 2048 {
            let iv = [0u8, 1, 2, 3, 4, 5, 6, 7];
            let dec = Decryptor::<Blowfish>::new_from_slices(key, &iv).expect("valid key/iv");
            let _ = dec.decrypt_padded_mut::<NoPadding>(chunk);
        }
    }

- Правило потоковой расшифровки и кэширования:
  - Аудиопоток делится на блоки фиксированного размера 2048 байт (chunk_index = byte_offset / 2048).
  - Если chunk_index % 3 == 0 И размер прочитанного блока равен ровно 2048 байт, вызывается decrypt_chunk(chunk, &key).
  - Все остальные блоки (chunk_index % 3 != 0 или финальный неполный блок меньше 2048 байт) остаются без изменений.
  - Расшифрованные чанки записываются в кэш-файл tempfile, что позволяет декодеру symphonia выполнять мгновенный Seek вперёд и назад.

## 5. Математическая спецификация Cavacore (src/dsp/cavacore.rs)
Полный порт актуального алгоритма из docs/reference/cava/cavacore.c на чистый Rust:
1. Буферы FFT:
   - Для басовых частот (ниже 300 Гц) используется окно N_bass = 4096 сэмплов для высокого частотного разрешения.
   - Для средних и высоких частот используется окно N_mid_high = 2048 сэмплов для быстрой реакции по времени.
2. Оконная функция Ханна:
   Перед подачей в realfft каждый элемент входного буфера умножается на коэффициент:
   w[n] = 0.5 * (1.0 - (2.0 * PI * n as f32 / (N - 1) as f32).cos())
3. Логарифмическая разметка частот по столбикам (num_bars):
   Для диапазона f_min = 50.0 Гц и f_max = 10000.0 Гц граничная частота i-го столбика вычисляется как:
   f[i] = f_min * (f_max / f_min).powf(i as f32 / num_bars as f32)
   Частоты переводятся в индексы бинов FFT: bin[i] = (f[i] * N as f32 / sample_rate as f32) as usize, а также применяется частотная эквализация (либо опциональное dB-масштабирование из cavacore 2026) для равномерного заполнения спектра.
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

## 6. Спецификация экранов и палитры UI (src/ui/)

### 6.1 Цветовая схема (TrueColor RGB)
- Основной фон терминала: Rgb(19, 21, 31) (#13151F).
- Фон карточек и модальных окон: Rgb(30, 34, 51) (#1E2233).
- Цвет рамок неактивных блоков: Rgb(62, 68, 94) (#3E445E).
- Основной текст: Rgb(236, 239, 244) (#ECEFF4), вторичный текст: Rgb(148, 163, 184) (#94A3B8).
- Градиент спектра и активных элементов: линейная интерполяция от бирюзового Rgb(79, 209, 197) (#4FD1C5) до фиолетового Rgb(127, 90, 240) (#7F5AF0).

### 6.2 Unified Shell Layout v2.0 (единый сквозной каркас)
В отличие от v1.0, где каждая вкладка рисовала собственный плеер и собственный сайдбар, в v2.0 используется **единый сквозной каркас окна (`AppShell`)**:

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ DEEZER-TUI • [1] Now Playing  [2] Search  [3] Library  [4] Flow  [5] Settings   ONLINE │
├──────────────────┬─────────────────────────────────────────────────────────────────────┤
│ NAVIGATION   [S] │                                                                     │
│                  │                                                                     │
│  [1] Now Playing │                 АКТИВНЫЙ ЭКРАН (VIEWPORT)                          │
│  [2] Search      │   • Экран 1: Большая обложка + Метаданные + Тех. параметры потока  │
│  [3] Library     │   • Экран 2: Строка поиска (/ для ввода) + 3 колонки результатов   │
│      ├─ Favorites│   • Экран 3: Таблица треков выбранного плейлиста + фильтр         │
│      ├─ My Mix   │   • Экран 4: Deezer Flow (слева) + Синхронный Lyrics (справа)      │
│      └─ Chill    │   • Экран 5: Настройки (ARL-токен, Качество FLAC/320/128, Спектр)  │
│  [4] Flow        │                                                                     │
│  [5] Settings    │                                                                     │
├──────────────────┴─────────────────────────────────────────────────────────────────────┤
│ 00:46 ━━━━━━━●───────────────── 03:44   [⏮ Prev] [⏸ Pause] [⏭ Next]   FLAC | 1411 kbps │
│ ▁▂▃▅▇█▆▄▂▃▅▆█▇▅▃▂▁▂▄▆▇█▆▄▃▂▁▂▃▄▆▇▅▃▂▁▂▃▅▆▇▅▃▂▁  (Глобальный спектр, [V] вкл/выкл)      │
│ [Space] Play/Pause  [n/p] Next/Prev  [←/→] Seek  [/] Search  [S] Sidebar  [V] Spectrum│
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Ключевые UX-инварианты v2.0:
1. **Глобальный плеер и спектр снизу:** Панель прогресса (`MM:SS / MM:SS`), кнопки управления (`⏮ ⏯ ⏭`), реальный битрейт (`FLAC 16-bit/44.1kHz | 1411 kbps`) и спектроанализатор отрисовываются на уровне `App` и доступны на **любой** вкладке.
2. **Мгновенное управление воспроизведением:** Вне режима набора текста клавиши `Space` (Play/Pause), `n` (Next), `p` (Prev), `←`/`→` (Seek ± 5 с), `+`/`-` (Volume), `v` (Toggle Spectrum), `s` (Toggle Sidebar), `i` (Track Info Modal) работают **везде**.
3. **Модальный фокус строки поиска:** На вкладке `[2] Search` ввод текста активируется клавишей `/` или `f` (и выключается по `Enter` или `Esc`). Когда ввод не активен, все хоткеи плеера (`Space`, `1`–`5`, `←`/`→`) работают штатно.
4. **Двухпанельная навигация в Фонотеке (`[3] Library`):** Стрелки `←` / `→` (или `Tab`) переключают фокус между списком плейлистов и таблицей треков. При выборе плейлиста загружаются его треки через `playlist.getSongs`.

### 6.3 Виджеты
- Субсимвольный спектр (SpectrumWidget):
  Использует массив символов [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']. Полная высота столбика в субпикселях равна (val * rows as f32 * 8.0) as usize. Целые клетки заполняются символом '█', верхушка — остатком от деления на 8, а выше удерживаемого пика ставится точка '▪'.
- Рендерер обложек (CoverArtWidget):
  По умолчанию рендерит пиксельную сетку из полублоков '▀', где fg равен RGB-цвету верхнего пикселя (x, 2*y), а bg — RGB-цвету нижнего пикселя (x, 2*y + 1), либо переключается на HD-протокол Kitty/Sixel через ratatui-image 11.1.0.

### 6.4 Экраны
- Экран 01 (Now Playing): Большая обложка + метаданные + тех. параметры потока (Codec, Sample Rate, Bit Depth, Bitrate kbps, Stream Size MB).
- Экран 02 (Search & Discover): Строка поиска (`/` для ввода) + 3 колонки результатов (ARTISTS, ALBUMS, TRACKS).
- Экран 03 (Library & Overlays): Двухпанельный фокус — слева дерево плейлистов, справа таблица треков (#, ТРЕК, ИСПОЛНИТЕЛЬ, АЛЬБОМ, ВРЕМЯ, ♥) + фильтр. Попап CHOOSE PLAYLIST по клавише добавления.
- Экран 04 (Flow & Live Lyrics): Слева обложка и управление Deezer Flow, справа блок LYRICS с автоскроллом по таймкоду и выделением текущей строки бирюзовым фоном (#4FD1C5) с тёмным текстом. При старте нового трека `lyrics` всегда очищается; для инструментальных треков — по центру статус "No synchronized lyrics available".
- Экран 05 (Settings): Ввод/обновление ARL-токена со статусом подписки (`Deezer HiFi / Premium / Free / Guest`), выбор качества (`FLAC 1411 kbps`, `MP3 320 kbps`, `MP3 128 kbps`), переключатели `Show Spectrum` и `dB Scale`.
