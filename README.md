# deezer-tui

A native terminal (TUI) music client for Deezer, written in Rust. It streams the
real Deezer CDN — FLAC 1411 kbps on Premium/HiFi accounts, MP3 320/128 otherwise,
30-second previews when you give it no token — decodes the encrypted stream on the
fly and renders a 60 FPS spectrum analyzer, HD cover art and synced lyrics inside
the terminal. Single binary, no GUI toolkit, no web view, no daemon.

Current status: **alpha, version 0.2.7**. It works for everyday listening on Linux
and macOS, but the Deezer API surface is unofficial and untested against every
account type. Release tags use semver `0.x.y`; the phase numbers in `TODO.md`,
`PLAN.md` and `ARCHITECTURE.md` are internal development phases (Phase 17 = 0.2.7).

> **Disclaimer.** This is an unofficial Deezer client, not affiliated with or endorsed
> by Deezer. It reads your own `arl` session cookie and decrypts media you already have
> access to (`BF_CBC_STRIPE`) for local playback only. Use it with your own account, at
> your own risk; respect Deezer's terms of service in your region.

## Features

- **Playback** — FLAC 1411 kbps / MP3 320 / MP3 128, ±5 s seek, volume 0–100 %,
  gapless prefetch: the next track starts downloading 20 s before the current one ends.
- **Spectrum analyzer** — a Rust port of cava's `cavacore`: dual FFT (4096 samples for
  bass, 2048 for mids/highs), Hann window, log-octave bands from 20 Hz to 20 kHz
  (15 bars = ⅔ octave, 30 bars = ⅓ octave), per-band AGC so mids stay visible next to
  a loud bass, a relative silence gate that removes the noise floor, gravity falloff
  and three switchable height scales (see [Spectrum](#spectrum)).
- **Screens** — Now Playing (big cover + lyrics), Search, Library (playlists and a
  sortable track table), Flow (infinite recommendations with live lyrics), Settings.
- **From any track** — Track Menu (`m`: favorite, add to playlist, Song Radio, info)
  and Flow Modes (`f`: My Flow, 7 moods, 10 genres).
- **Search** — instant local matching against your library and playlists, plus
  infinite Deezer pagination (30 results per page, `PageDown` loads more).
- **Cover art** — 500×500 covers drawn with Unicode sextants (2×3 subpixels per cell,
  three times the resolution of half-block art) with per-cell colour clustering and
  Lanczos3 downscale.
- **MPRIS over D-Bus** — hardware media keys, GNOME/KDE "now playing" widgets and
  `playerctl` work out of the box.
- **Audio pipeline** — Symphonia 0.6 decoder → rtrb 0.4 lock-free rings → cpal 0.18
  real-time callback (no locks, no allocations, no I/O inside the callback).

## Requirements

| Requirement | Detail |
| --- | --- |
| Rust | 1.88 or newer (Edition 2024; built and tested with 1.99) |
| OS | Linux (ALSA or PipeWire) and macOS (CoreAudio). Windows: through WSL2 |
| Build deps (Linux) | C compiler, `pkg-config`, ALSA headers (`libasound2-dev` on Debian/Ubuntu, `alsa-lib-devel` on Fedora, `alsa-lib` on Arch) |
| Terminal | TrueColor (24-bit) strongly recommended, 256 colours minimum; a font with Unicode sextants (U+1FB00–U+1FB3B) for HD covers; comfortable size ~100×30, everything adapts down to 80×24 |
| Deezer account | Premium or HiFi for FLAC and full-length tracks. Without any token the app runs in guest mode: 30-second MP3 previews, no lyrics |

## Install

### Option A — zero-setup script (Linux and macOS)

Installs the system packages, Rust (via rustup, if missing or older than 1.88),
fetches the source and builds the binary into `~/.local/bin`:

```sh
curl -fsSL https://raw.githubusercontent.com/ancorom/deezer-tui/main/install.sh | bash
```

If you prefer to read the script before running it:

```sh
git clone https://github.com/ancorom/deezer-tui && cd deezer-tui
bash install.sh
```

Script options:

| Flag | Effect |
| --- | --- |
| `--ref <tag-or-branch>` | Build a specific release tag or branch instead of `main` |
| `--pipewire` | Build with cpal's native PipeWire backend instead of ALSA |
| `--no-deps` | Skip system package installation (you installed the deps yourself) |
| `--dir <path>` | Where to clone/build the source (default `~/.local/share/deezer-tui-src`) |
| `--prefix <path>` | Where to install the binary (default `~/.local/bin`) |
| `--dry-run` | Print every command without running any of them |

### Option B — build from source

```sh
git clone https://github.com/ancorom/deezer-tui
cd deezer-tui
cargo build --release            # or: cargo build --release --features pipewire
./target/release/deezer-tui
```

### Option C — cargo install

```sh
cargo install --git https://github.com/ancorom/deezer-tui --rev v0.2.7
```

## Getting your ARL token

The token is your Deezer session cookie. The app never asks for your password.

1. Log in to [deezer.com](https://www.deezer.com) in your browser (Chrome, Firefox, Edge).
2. Open DevTools: `F12` (or `Ctrl+Shift+I` / `Cmd+Opt+I` on macOS).
3. Go to **Application** (Chrome) / **Storage** (Firefox) → **Cookies** → `https://www.deezer.com`.
4. Find the cookie named **`arl`** — a 32-character hex string. Copy its value
   (do **not** copy `arl_ui`, it is a different cookie).
5. Paste it into `~/.config/deezer-tui/config.toml` as `arl = "…"`, or launch the app
   and press `L` to paste it interactively (it is saved to the config file).

The cookie rotates when you log out or Deezer refreshes the session. When the app
starts reporting `401` / "invalid credentials", repeat steps 2–5.

## Configuration

Config file: `~/.config/deezer-tui/config.toml` (path override: `DEEZER_TUI_CONFIG`).
It is read at startup and rewritten whenever you change a setting or paste the ARL
token — but the app does not invent it: create the file yourself the first time.
Environment variables override the file.

```toml
arl = "0123456789abcdef0123456789abcdef"
volume = 0.8
preferred_quality = "FLAC"          # FLAC | MP3_320 | MP3_128
spectrum_scale = "track_peak"       # track_peak | frame_peak | legacy
spectrum_spread = 0.0               # 0.0 = off (cava-like), 1.35 = Monstercat look
spectrum_rows = 5                   # omit for AUTO (4–6 rows depending on height)
show_lyrics = true
db_scale = true                     # Legacy height scale: dB window vs linear amplitude
```

| Field | Type | Default | Notes |
| --- | --- | --- | --- |
| `arl` | string | *(unset → guest mode)* | Deezer `arl` cookie; enables full tracks and HiFi |
| `volume` | float | `0.8` | `0.0`–`1.0`, live with `+`/`-` |
| `sample_rate_hz` | float | `44100.0` | Deezer streams are 44.1 kHz; change only if your output device refuses it |
| `preferred_quality` | string | `FLAC` | `FLAC`, `MP3_320`, `MP3_128`; falls back automatically when the account lacks HiFi |
| `show_spectrum` | bool | `true` | Bottom spectrum panel; `v` toggles it live |
| `show_lyrics` | bool | `true` | Synchronized lyrics panel; `y` toggles it live |
| `db_scale` | bool | `true` | **Legacy** height scale only: 32 dB logarithmic window vs linear amplitude with `x^0.45` |
| `spectrum_scale` | string | `track_peak` | `track_peak`, `frame_peak`, `legacy` (see below) |
| `spectrum_spread` | float | `0.0` | Neighbor pull-up base. `0.0` = off (bands independent, as in cava); `2.0`/`3.0` soft; `1.35` = the classic Monstercat look |
| `spectrum_rows` | integer or omitted | *(AUTO)* | Panel height: AUTO picks 4–6 rows from the terminal height; explicit values are clamped to 2–12 |
| `sidebar_collapsed` | bool | `false` | Left navigation sidebar collapsed; `s` toggles it |

Environment overrides:

| Variable | Values | Overrides |
| --- | --- | --- |
| `DEEZER_ARL` | token | `arl` |
| `DEEZER_TUI_SPECTRUM_SCALE` | `legacy` / `track_peak` / `frame_peak` | `spectrum_scale` |
| `DEEZER_TUI_SPECTRUM_SPREAD` | `off`, `0`, or a number (`1.05`–`4.0`) | `spectrum_spread` |
| `DEEZER_TUI_SPECTRUM_ROWS` | `0` (= AUTO) or `2`–`12` | `spectrum_rows` |
| `DEEZER_TUI_CONFIG` | path | config file location |
| `DEEZER_TUI_LOG_DIR` | path | log directory (default `~/.config/deezer-tui/`, rolling daily file `deezer-tui.log`) |

## Spectrum

The analyzer draws 15 or 30 log-octave bars. Bar *height* is always normalized to
one of three scales, cycled with `←`/`→` on the **Settings** tab (row *Spectrum
Height Scale*):

| Scale | Reference | Window | Behaviour |
| --- | --- | --- | --- |
| `track_peak` (**default**) | the track's slow long-term loudness peak | 26 dB below it | Stable contrast across songs; quiet bands stay quiet |
| `frame_peak` | the current frame's peak, held ~0.1 s before it is allowed to fall | 26 dB below it | cava-style: the loudest bar touches the ceiling, loudness changes stay visible |
| `legacy` | absolute 0 dBFS | 32 dB | Reference-style absolute levels, no peak tracking |

Two more knobs on the same tab: **Spectrum Neighbor Spread** (off by default —
the Monstercat filter that pulled neighbouring bars toward the dominant one is a
near no-op on dense real spectra, so it is opt-in) and **Spectrum Panel Rows**
(AUTO 4–6, or a fixed 2–12 rows for finer vertical detail).

## Key bindings

| Key | Action |
| --- | --- |
| `q` | Quit |
| `Space` | Play / pause (works on every screen) |
| `n` / `>`, `p` / `<` | Next / previous track in the queue |
| `←` / `→` | Seek ∓5 s (when no panel needs the arrows) |
| `+` / `=`, `-` | Volume ±5 % |
| `1`–`5`, `F1`–`F5`, `Tab` / `Shift-Tab` | Switch screens: Now Playing, Search, Library, Flow, Settings |
| `/` | Focus the search input (`Enter` searches, `Esc` releases focus) |
| `PageDown` / `PageUp` | Jump ±10 rows in a table; near the end of the search results, load the next page |
| `f` / `g` | Flow Modes modal: My Flow, 7 moods, 10 genres |
| `m` | Track Menu: favorite, add to playlist, Song Radio, info |
| `y` | Toggle the lyrics panel |
| `s` | Toggle the navigation sidebar |
| `v` | Toggle the spectrum analyzer |
| `i` | Track & stream info modal (codec, sample rate, bitrate, stream mode) |
| `L` | Jump to Settings → ARL token input |
| `5` / `,` | Settings screen |
| `Enter` / `Esc` | Play / confirm, close / release focus |
| `↑` / `↓`, `j` / `k` | Move in lists, panels and settings rows |
| `o` / `r` (Library) | Cycle the sort criterion (Added, Title, Artist, Album) / flip the direction |
| `h` / `l` (Library) | Focus the playlists panel / the tracks panel (also `Tab`) |
| `a` (Library) | "Choose playlist" modal — add the current track to a playlist |

Settings rows are edited with `←`/`→` (cycle a value) or `Enter` (open / edit).

## Development

```sh
cargo build                      # debug build
cargo test                       # unit + render tests (TestBackend)
cargo clippy --all-targets -- -D warnings
cargo build --release
```

The gate for every change is: clean `cargo build` with zero warnings, all tests
green, clippy `-D warnings` clean. Spectrum changes are verified by measuring the
real renderer output on real MP3 files (15 bars, 60 FPS) — the numbers behind each
release are recorded in `CHANGELOG.md`.

Layout: `src/dsp/` (cavacore port), `src/audio/` (Symphonia decoder, encrypted
stream, cpal output, lock-free rings, prefetch), `src/api/` (Deezer API client,
`BF_CBC_STRIPE` crypto, models), `src/ui/` (views, widgets, theme, app loop),
`src/mpris.rs` (D-Bus integration).
Long-term design notes are in [ARCHITECTURE.md](ARCHITECTURE.md) (Russian), the
release history in [CHANGELOG.md](CHANGELOG.md), open work in [TODO.md](TODO.md),
and agent-facing hard constraints in [AGENT.md](AGENT.md).

## Troubleshooting

| Symptom | Fix |
| --- | --- |
| Build fails with `Could not find library alsa` | Install ALSA headers and `pkg-config`, or build with `--features pipewire` |
| No sound, callback underruns | Your output device does not offer 44.1 kHz — set `sample_rate_hz` to the device rate, or pick another device |
| `401` / "invalid credentials" | The `arl` cookie expired: re-copy it (press `L` in the app) |
| Covers look like mosaic garbage | Terminal without TrueColor or a font without Unicode sextants — use a different terminal/font |
| Only 30-second previews | No token loaded: check `config.toml` or `DEEZER_ARL`, then press `L` |
| Bars all pinned to the top | Switch the height scale (`←`/`→` on the Settings tab) or widen the window by using `legacy` |
| Nothing but noise floor in the bars | Guest-mode previews are short and quiet; with a token the gate removes the floor |
