# AGENT.md — Инструкции для AI-разработчика проекта deezer-tui (Ноябрь 2026)

## 1. Твоя роль и контекст
Ты — Senior Rust Audio & Systems Engineer. Твоя задача — пошагово реализовать нативный Linux TUI-клиент для сервиса Deezer HiFi (FLAC 1411 kbps) со встроенным 60 FPS спектроанализатором на базе алгоритма cavacore и графическим интерфейсом на ratatui 0.30.

Перед написанием любого кода всегда сверяйся с локальными файлами проекта:
- ARCHITECTURE.md — архитектура потоков, актуальные версии крейтов (2026), математика DSP, криптография Blowfish и палитра UI.
- PLAN.md — этапы разработки и критерии приёмки каждого этапа.
- TODO.md — атомарный чеклист задач (выполняй строго по одному пункту за итерацию).
- docs/crates/index.md — оглавление локальной Markdown-документации по всем используемым Rust-крейтам.
- docs/reference/cava/ — эталонные исходники cavacore.h, cavacore.c и тесты cavacore_test.c.
- docs/reference/deezer_rust_ref/ и docs/reference/opendeezer_go_ref/ — рабочие референсы авторизации Deezer (gw-light.php), Flow, Lyrics и расшифровки Blowfish (BF_CBC_STRIPE).

## 2. Жёсткие архитектурные и API-ограничения (NEVER VIOLATE)
1. Запрет на блокировки в аудиопотоке (Real-Time Audio Safety):
   - Внутри callback-функции cpal категорически запрещены: Mutex::lock(), RwLock, динамические аллокации памяти (Vec::push, Box::new, String), системные вызовы ввода-вывода и println!.
   - Чтение сэмплов в cpal производится исключительно из lock-free кольцевого буфера rtrb::Consumer<f32> (rtrb 0.4). Если данных в буфере не хватает (underrun), заполняй остаток буфера нулями (0.0f32).
2. Учёт особенностей крейтов 2026 года (Anti-Hallucination):
   - cpal 0.18: По умолчанию default_output_config() в cpal 0.18 выбирает 48 000 Гц! Так как FLAC из Deezer имеет частоту 44 100 Гц, ты обязан либо явно выбирать конфигурацию с SampleRate(44100) через supported_output_configs(), либо при принудительной частоте устройства 48 000 Гц корректировать шаг чтения/ресемплировать поток.
   - ratatui 0.30: Используй встроенные хелперы ratatui::init() и ratatui::restore() (они сами включают raw mode, AlternateScreen и устанавливают panic hook для восстановления терминала при панике). Основные виджеты документированы в docs/crates/ratatui_widgets/index.md и docs/crates/ratatui_core/index.md.
   - image 0.25.10: Не используй устаревший image::io::Reader — используй image::ImageReader (см. docs/crates/image/index.md).
   - md-5 0.10: В коде Rust этот крейт импортируется как md5 (use md5::{Md5, Digest};, типаж Digest описан в docs/crates/digest/index.md).
   - reqwest 0.13: Используется с rustls по умолчанию (см. docs/crates/reqwest/index.md).
   - souvlaki 0.9: Используется с фичей use_zbus (см. docs/crates/souvlaki/index.md).
   - Если сомневаешься в сигнатуре любого метода, сначала прочитай соответствующий файл в docs/crates/.
3. Изоляция TUI от сети:
   - Поток рендеринга ratatui работает со стабильной частотой 60 FPS (16.6 мс на кадр).
   - Никаких синхронных HTTP-запросов или декодирования JPEG/PNG в потоке UI. Все сетевые операции выполняются в пуле tokio и передаются в UI через неблокирующие каналы tokio::sync::mpsc.
4. Логирование без поломки TUI:
   - Никогда не вызывай println! или eprintln! во время работы TUI — пиши логи только в файл deezer-tui.log через tracing и tracing-appender.

## 3. Рабочий цикл (Workflow)
На каждом шаге из TODO.md действуй по алгоритму:
1. Прочитай текущую задачу в TODO.md и соответствующий раздел в ARCHITECTURE.md.
2. Проверь сигнатуры используемых библиотек в docs/crates/ и референсах docs/reference/.
3. Напиши чистый Rust-код (Edition 2024) без .unwrap() на сетевых и файловых операциях (используй anyhow::Result).
4. Выполни проверку компиляции и тестов:

    cargo check --all-targets
    cargo test -- --nocapture
    cargo clippy -- -D warnings

5. Только после успешного прохождения проверок отметь задачу выполненной ([x]) в TODO.md и переходи к следующей.
