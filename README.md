# deezer-tui

Native Linux TUI client for **Deezer HiFi** — plays FLAC 1411 kbps straight from Deezer's
CDN with an in-terminal 60 FPS spectrum analyzer built on a Rust port of the
[cavacore](docs/reference/cava/) algorithm. Single Rust binary, no GUI toolkit, no
runtime dependencies beyond ALSA/PipeWire.

- **Rust**: Edition 2024, MSRV 1.88 (built with 1.99)
- **UI**: [ratatui 0.30](https://ratatui.rs) + crossterm, TrueColor, 60 FPS render loop
- **Audio**: Symphonia 0.6 decoder → rtrb 0.4 lock-free rings → cpal 0.18 real-time callback
- **Crypto**: Deezer `BF_CBC_STRIPE` (Blowfish-CBC, 2048-byte chunks) on the fly

## Features

| Area | What you get |
| --- | --- |
| Playback | HiFi login by ARL token (no token → guest mode: 30 s MP3 previews); FLAC 1411 kbps, MP3 320/128 fallback; seek ±5 s; gapless prefetch of the next track (starts 20 s before the end) |
| Spectrum | cavacore port: dual FFT (4096 bass / 2048 mid-high), Hann window, log bands 50 Hz–10 kHz, Monstercat spatial filter, gravity falloff, autosens, optional dB scale; adapts bar count to any window width |
| Screens | 5 tabs: **Now Playing** (big cover, lyrics panel), **Search**, **Library** (playlists + sortable track table with ADDED column), **Flow** (infinite recommendations + live lyrics), **Settings** (ARL token, quality, spectrum, dB scale) |
| Covers | 500×500 HD covers rendered as 2×2 quad subpixels (16 quadrant glyphs) — 2× horizontal resolution of classic half-block art |
| System | MPRIS over D-Bus (souvlaki/zbus): hardware media keys, GNOME/KDE now-playing widgets, `playerctl` works out of the box |
| Resilience | Generation counter (`playback_seq`) prevents races on rapid track switching; in-memory decrypted stream buffer — no temp files, no FD sharing |

## Quick start

```sh
# build (release)
cargo build --release

# run
./target/release/deezer-tui
```

Provide your ARL token (Deezer web cookie `arl=...`) either in the config file or the
environment:

```sh
# ~/.config/deezer-tui/config.toml
arl = "YOUR_ARL_TOKEN"

# or
export DEEZER_ARL="YOUR_ARL_TOKEN"
```

Or paste it interactively: launch the app, press `L` (or open `5` → ARL row → `L`).

Terminal 256-color or TrueColor; minimum comfortable size ~80×24, everything is adaptive.

## Key bindings

| Key | Action |
| --- | --- |
| `1`–`5` / `F1`–`F5`, `Tab`/`Shift-Tab` | Switch screens (Now Playing / Search / Library / Flow / Settings) |
| `Space` | Play / pause (works everywhere) |
| `n` / `>`, `p` / `<` | Next / previous track in queue |
| `←` / `→` | Seek ∓5 s (on screens without panel focus) |
| `+`/`=` , `-` | Volume ±5 % |
| `/` (or `f` on Search) | Focus search input; `Enter` — search, `Esc` — unfocus |
| `y` | Toggle lyrics panel |
| `s` | Toggle navigation sidebar |
| `v` | Toggle spectrum |
| `i` | Track & Stream Info modal (codec, sample rate, bitrate, size, mode) |
| `o` / `r` (Library) | Cycle sort criterion (Added/Title/Artist/Album) / flip direction |
| `h` / `l` (Library) | Focus playlists / tracks panel (also `Tab`) |
| `a` (Library) | "Choose playlist" modal (add current track to a playlist) |
| `L` | Jump to Settings → ARL token input |
| `5` / `,` | Settings screen |
| `q` | Quit |
| `Enter`, `Esc` | Play/confirm, close/unfocus |

## Configuration

`~/.config/deezer-tui/config.toml` (created on first run; ARL can also come from
`DEEZER_ARL` env var, env wins):

| Field | Type | Meaning |
| --- | --- | --- |
| `arl` | `String?` | Deezer ARL cookie for HiFi guest login |
| `volume` | `f32` | Master volume, `0.0–1.0` |
| `sample_rate_hz` | `f32` | Sample rate for the Cavacore DSP (default 44100 — Deezer HiFi FLAC standard; cpal output explicitly requests 44100) |
| `preferred_quality` | `String` | `"FLAC"` (1411 kbps), `"MP3_320"`, `"MP3_128"` |
| `show_spectrum` | `bool` | Global spectrum on/off |
| `show_lyrics` | `bool` | Lyrics panel on/off (key `y`) |
| `db_scale` | `bool` | dB scaling in the spectrum |
| `sidebar_collapsed` | `bool` | Sidebar collapsed state |

Logs: `~/.config/deezer-tui/deezer-tui.log` (daily rolling, via tracing; override the
directory with `DEEZER_TUI_LOG_DIR`).

## Architecture (short)

```
tokio pool ── HTTP gw-light.php / media.deezer.com ──► DeezerStream
   (Blowfish-CBC decrypt, in-memory buffer, 256 KB prebuffer)         │
                                                                     ▼
                              Symphonia decoder thread ──► rtrb AudioRing (stereo f32)
                                                                     │      │
                                                                     ▼      ▼ mono
                                          cpal 0.18 RT callback ◄───  VisRing ──► Cavacore
                                          (44.1 kHz, no allocs)              │ bars [0..1]
                                                                     ▼
                              ratatui 60 FPS render ◄── tokio::sync::mpsc BgEvents
```

Full details — see [ARCHITECTURE.md](ARCHITECTURE.md) (Russian): zero-latency pipeline,
Deezer API & Blowfish spec, Cavacore math, UI layout, screen specs. Development
constraints for AI agents — [AGENT.md](AGENT.md).

## Project structure

```
src/
├── main.rs               # logger init, config load, App::run
├── config.rs             # ~/.config/deezer-tui/config.toml (serde/toml)
├── api/
│   ├── client.rs         # gw-light.php client: auth, search, playlists, flow, lyrics, stream URL
│   ├── crypto.rs         # MD5 key derivation + Blowfish-CBC chunk decryption
│   └── models.rs         # API data models (tracks, playlists, lyrics, user)
├── audio/
│   ├── stream.rs         # DeezerStream: Read+Seek+MediaSource over the decrypted in-memory buffer
│   ├── decoder.rs        # Symphonia decode → rtrb, Seek channel, StreamTechInfo
│   ├── output.rs         # cpal device/streams, RT callback, volume, played_frames
│   ├── prefetch.rs       # gapless next-track prefetch (download+decrypt 20 s early)
│   └── ring_buffers.rs  # rtrb AudioRing (65536 stereo) / VisRing (16384 mono)
├── dsp/
│   └── cavacore.rs       # cavacore port: FFT, bands, spatial filter, gravity, autosens
├── mpris.rs              # MPRIS over souvlaki/zbus (media keys, desktop integration)
└── ui/
    ├── app.rs            # AppShell: 5 tabs, global player bar + spectrum, event loop
    ├── theme.rs          # TrueColor palette + turquoise→violet gradient
    ├── widgets/
    │   ├── spectrum.rs   # sub-symbol bar widget with peak holds, width-adaptive
    │   ├── cover_art.rs  # 2×2 quad subpixel cover renderer
    │   └── modal.rs      # popups: Choose Playlist, Track & Stream Info
    └── views/
        ├── now_playing.rs  # cover + metadata + lyrics panel
        ├── search.rs       # search input + results table
        ├── library.rs      # playlists + sortable track table (ADDED column)
        ├── flow_lyrics.rs  # infinite Flow + live lyrics
        └── settings.rs     # ARL, quality, spectrum, dB scale
```

## Development

```sh
cargo test                                   # 105 tests (render tests: 120×40 and 40×12)
cargo clippy --all-targets -- -D warnings   # zero-warning policy
cargo build --release
```

Milestones and per-phase changelog — [CHANGELOG.md](CHANGELOG.md). Per-phase progress:
[docs/crates/progress.md](docs/crates/progress.md). Crate-level API docs: `docs/crates/`.

---

## Резюме (RU)

Нативный Linux TUI-клиент для **Deezer HiFi**: FLAC 1411 кбит/с прямо из CDN Deezer
с расшифровкой Blowfish-CBC «на лету» и спектроанализатором на 60 FPS (порт
cavacore на Rust). Один бинарник, без GUI-toolkit; единственный внешний запрос к
системе — ALSA/PipeWire для вывода звука.

Пять вкладок: **Now Playing** (крупная обложка 500×500 в quad subpixel 2×2 + синхронные
lyrics), **Search**, **Library** (плейлисты + сортируемая таблица треков с колонкой
ADDED), **Flow** (бесконечные рекомендации + живые lyrics), **Settings** (ARL-токен,
качество, спектр, dB-масштаб). Интеграция MPRIS: аппаратные клавиши мультимедиа,
виджеты «сейчас играет» GNOME/KDE, `playerctl`. Gapless-предзагрузка следующего
трека начинается за 20 секунд до конца текущего; счётчик поколений `playback_seq`
устраняет гонки при быстром переключении треков.

Быстрый старт: `cargo build --release && ./target/release/deezer-tui`; ARL-токен —
в `~/.config/deezer-tui/config.toml` (`arl = "..."`) или переменной `DEEZER_ARL`,
либо в интерактивном режиме по клавише `L`. Полная документация:
[ARCHITECTURE.md](ARCHITECTURE.md), ограничения — [AGENT.md](AGENT.md), история
версий — [CHANGELOG.md](CHANGELOG.md).
