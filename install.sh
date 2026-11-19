#!/usr/bin/env bash
#
# deezer-tui — from-zero installer for Linux and macOS.
#
# Installs the system packages needed to compile (C toolchain, pkg-config,
# ALSA headers), installs Rust via rustup when it is missing or too old,
# fetches the source (git clone, or a tarball when git is unavailable),
# builds a release binary and installs it into ~/.local/bin.
#
#   bash install.sh                 # build current checkout / clone main
#   bash install.sh --ref v0.2.7    # build a specific tag or branch
#   bash install.sh --pipewire      # build with cpal's PipeWire backend
#   bash install.sh --dry-run       # print what would run, change nothing
#   bash install.sh --no-deps       # skip system package installation
#
set -euo pipefail

REPO="ancorom/deezer-tui"
REF="${DEEZER_TUI_REF:-main}"
BIN="deezer-tui"
SRC_DIR="${DEEZER_TUI_SRC:-$HOME/.local/share/deezer-tui-src}"
INSTALL_DIR="${HOME}/.local/bin"
MSRV=188          # 1.88, required by Cargo.toml (rust-version)
DRY_RUN=0
NO_DEPS=0
EXTRA_FEATURES=""

log()  { printf '\033[1;36m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarn:\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

have() { command -v "$1" >/dev/null 2>&1; }

run() {
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '    [dry-run] %s\n' "$*"
  else
    "$@"
  fi
}

run_as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    run "$@"
  elif have sudo; then
    run sudo "$@"
  else
    die "need root privileges to install packages (no sudo found); run with --no-deps or as root"
  fi
}

usage() {
  sed -n '2,20p' "$0" | sed 's/^#\{0,1\} //'
  exit 0
}

# ---------------------------------------------------------------- args -----
while [ $# -gt 0 ]; do
  case "$1" in
    --ref)        REF="${2:?--ref needs a value}"; shift 2 ;;
    --pipewire)   EXTRA_FEATURES="--features pipewire"; shift ;;
    --dir)        SRC_DIR="${2:?--dir needs a value}"; shift 2 ;;
    --prefix)     INSTALL_DIR="${2:?--prefix needs a value}"; shift 2 ;;
    --no-deps)    NO_DEPS=1; shift ;;
    --dry-run)    DRY_RUN=1; shift ;;
    -h|--help)    usage ;;
    *)            die "unknown option: $1 (see --help)" ;;
  esac
done

OS="$(uname -s)"

# ------------------------------------------------------- system packages -----
pkg_kind() {
  case "$OS" in
    Darwin) echo brew; return ;;
    Linux)
      if have apt-get; then echo apt
      elif have dnf; then echo dnf
      elif have pacman; then echo pacman
      elif have zypper; then echo zypper
      else echo none; fi
      ;;
    *) echo none ;;
  esac
}

install_packages() {
  local kind; kind="$(pkg_kind)"
  case "$kind" in
    apt)
      log "apt: refreshing index and installing build deps"
      run_as_root apt-get update
      run_as_root apt-get install -y --no-install-recommends \
        ca-certificates curl git build-essential pkg-config libasound2-dev
      ;;
    dnf)
      log "dnf: installing build deps"
      run_as_root dnf install -y curl git gcc gcc-c++ pkgconf-pkg-config alsa-lib-devel
      ;;
    pacman)
      log "pacman: installing build deps"
      run_as_root pacman -Sy --needed --noconfirm \
        curl git base-devel pkgconf alsa-lib
      ;;
    zypper)
      log "zypper: installing build deps"
      run_as_root zypper --non-interactive install \
        curl git gcc gcc-c++ pkg-config alsa-devel
      ;;
    brew)
      log "brew: installing build deps"
      [ "$DRY_RUN" -eq 1 ] && { printf '    [dry-run] brew install curl git\n'; return; }
      have brew || die "Homebrew not found — install it from https://brew.sh, then re-run"
      brew install curl git
      # macOS needs no extra headers: cpal uses the CoreAudio backend.
      ;;
    *)
      warn "no supported package manager found; install git, curl, a C compiler and ALSA headers yourself"
      ;;
  esac
}

if [ "$NO_DEPS" -eq 0 ]; then
  install_packages
fi

# ------------------------------------------------------------- rustup ------
cargo_version_major_minor() {
  cargo --version 2>/dev/null | awk '{print $2}' | awk -F. '{printf "%d%02d\n", $1, $2}'
}

if ! have cargo; then
  log "Rust not found — installing rustup (stable)"
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '    [dry-run] curl rustup + rustup default stable\n'
  else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
  fi
  # shellcheck disable=SC1091
  [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
else
  local_ver="$(cargo_version_major_minor)"
  if [ "${local_ver:-0}" -lt "$MSRV" ]; then
    log "Rust $(cargo --version | awk '{print $2}') is older than MSRV 1.88 — updating toolchain"
    run rustup update stable
    run rustup default stable
    # shellcheck disable=SC1091
    [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
  else
    log "Rust OK: $(cargo --version | awk '{print $2}')"
  fi
fi

# ------------------------------------------------------------- source ------
acquire_source() {
  if [ -f "Cargo.toml" ] && grep -q '^name = "deezer-tui"' Cargo.toml; then
    SRC_DIR="$(pwd)"
    log "using existing checkout: $SRC_DIR"
    return
  fi
  if have git; then
    log "git clone --depth 1 --branch $REF https://github.com/$REPO"
    rm -rf "$SRC_DIR"
    run git clone --depth 1 --branch "$REF" "https://github.com/$REPO" "$SRC_DIR"
  else
    log "downloading tarball of $REF"
    TMP="$(mktemp -d)"
    run curl -fsSL "https://github.com/$REPO/archive/$REF.tar.gz" -o "$TMP/src.tar.gz"
    run mkdir -p "$SRC_DIR"
    run tar -xzf "$TMP/src.tar.gz" -C "$SRC_DIR" --strip-components=1
    rm -rf "$TMP"
  fi
}

acquire_source
cd "$SRC_DIR"

# -------------------------------------------------------------- build ------
log "cargo build --release ${EXTRA_FEATURES}"
run cargo build --release $EXTRA_FEATURES

BIN_PATH="$SRC_DIR/target/release/$BIN"
[ "$DRY_RUN" -eq 1 ] || [ -f "$BIN_PATH" ] || die "build finished but $BIN_PATH is missing"

log "installing binary to $INSTALL_DIR"
run mkdir -p "$INSTALL_DIR"
run cp "$BIN_PATH" "$INSTALL_DIR/$BIN"
run chmod +x "$INSTALL_DIR/$BIN"

if ! printf '%s' "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
  warn "$INSTALL_DIR is not in PATH — add it with:"
  printf '    echo export PATH="\$HOME/.local/bin:\$PATH" >> ~/.bashrc\n'
fi

if [ "$DRY_RUN" -eq 1 ]; then
  log "dry run complete — nothing was changed"
  exit 0
fi

log "done: $(command -v "$BIN" || echo "$INSTALL_DIR/$BIN")"
cat <<'EPILOGUE'

Next steps
  1. Get your Deezer ARL token (browser cookie) — README.md, "Getting your ARL token".
  2. mkdir -p ~/.config/deezer-tui
     printf 'arl = "YOUR_ARL_TOKEN"\n' > ~/.config/deezer-tui/config.toml
  3. Run it in a TrueColor terminal (256 colours minimum):
     deezer-tui

No token? The app still runs in guest mode: 30-second MP3 previews.
EPILOGUE
