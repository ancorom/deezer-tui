#!/usr/bin/env bash
# =============================================================================
#  deezer-tui :: сбор материалов для ревью Фаз 1-2 в 4 файла .md
#
#  Запуск из КОРНЯ проекта:
#      bash collect_review.sh                 # полный сбор (с cargo check/test/clippy)
#      bash collect_review.sh --no-cargo      # не запускать cargo (быстро, только код)
#      bash collect_review.sh --slim          # ужать логи и Cargo.lock (если файлы большие)
#      bash collect_review.sh --with-docs     # дополнительно вложить AGENT/ARCH/PLAN/TODO
#
#  Результат: review/deezer-tui-review-01-meta.md ... -04-logs.md
#  Прикрепите ВСЕ 4 файла к одному сообщению.
#
#  Формат вложения (для автоматического разбора):
#      <!-- FILE path="src/dsp/cavacore.rs" lines=812 bytes=30123 lang=rust -->
#      ### src/dsp/cavacore.rs
#      ````rust
#      ...содержимое без изменений...
#      ````
#      <!-- /FILE -->
# =============================================================================
set -uo pipefail
export CARGO_TERM_COLOR=never CARGO_TERM_PROGRESS_WHEN=never TERM=dumb
export RUST_BACKTRACE=1

SLIM=0; RUN_CARGO=1; WITH_DOCS=0
for a in "$@"; do
  case "$a" in
    --slim) SLIM=1 ;;
    --no-cargo) RUN_CARGO=0 ;;
    --with-docs) WITH_DOCS=1 ;;
    -h|--help) sed -n '2,25p' "$0"; exit 0 ;;
    *) echo "Неизвестный флаг: $a (см. --help)"; exit 2 ;;
  esac
done

[ -f Cargo.toml ] || { echo "ОШИБКА: нет Cargo.toml в $PWD — запустите скрипт из корня проекта."; exit 1; }
command -v python3 >/dev/null 2>&1 && HAVE_PY=1 || HAVE_PY=0

ROOT="$PWD"
OUT="$ROOT/review"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
mkdir -p "$OUT" "$TMP/gen" "$TMP/raw"

MISSING="$TMP/missing.txt"; : > "$MISSING"
TRUNCATED="$TMP/truncated.txt"; : > "$TRUNCATED"

if [ "$SLIM" = 1 ]; then LOG_HEAD=150; LOG_TAIL=80; IDX_HEAD=120; IDX_TAIL=40
else LOG_HEAD=500; LOG_TAIL=250; IDX_HEAD=300; IDX_TAIL=60; fi

# ---------- утилиты ---------------------------------------------------------

# Длина забора из бэктиков, которой нет внутри файла (чтобы код не ломал markdown)
fence_for() {
  local f="$1" n=3 s
  while :; do
    s="$(printf '`%.0s' $(seq 1 "$n"))"
    if [ -f "$f" ] && grep -q "^$s" "$f"; then n=$((n+1)); else printf '%s' "$s"; return; fi
  done
}

# Вычистить секреты и абсолютный домашний путь
redact() {
  sed -E \
    -e 's/((arl|sid|api_token|license_token|checkForm|license|token|Cookie|cookie)["'"'"']?[[:space:]]*[:=][[:space:]]*"?)[^"'"'"',;[:space:]]+/\1REDACTED/gI' \
    -e 's/[A-Fa-f0-9]{40,}/REDACTED_HEX/g' \
    -e "s|${HOME}|~|g"
}

strip_ansi() { sed -E 's/\x1B\[[0-9;]*[a-zA-Z]//g'; }

# clip <src> <dst> <head> <tail> — обрезка с ЯВНЫМ маркером и счётчиком строк
clip() {
  local src="$1" dst="$2" h="$3" t="$4" total
  total="$(wc -l < "$src" | tr -d ' ')"
  if [ "$total" -le $((h + t)) ]; then
    cp "$src" "$dst"
  else
    { head -n "$h" "$src"
      printf '\n... [ОБРЕЗАНО %s строк из %s — запросите нужный фрагмент отдельно] ...\n\n' "$((total - h - t))" "$total"
      tail -n "$t" "$src"
    } > "$dst"
    printf '%s (показано %s+%s из %s строк)\n' "$src" "$h" "$t" "$total" >> "$TRUNCATED"
  fi
}

# emit <body-file> <real-path-or-"-"> <virtual-path> <lang>
#   real-path "-" означает: содержимое уже лежит в $TMP/gen/<virtual-path>
emit() {
  local body="$1" real="$2" vpath="$3" lang="$4" src lines bytes fence
  if [ "$real" = "-" ]; then
    src="$TMP/gen/$vpath"
  else
    src="$real"
  fi
  if [ ! -f "$src" ]; then
    printf '%s\n' "$vpath" >> "$MISSING"
    {
      printf '<!-- FILE path="%s" lines=0 bytes=0 lang=text missing=true -->\n' "$vpath"
      printf '### %s\n\n' "$vpath"
      printf '%s\n' '```text' 'ОТСУТСТВУЕТ В ПРОЕКТЕ (файл не найден)' '```'
      printf '<!-- /FILE -->\n\n'
    } >> "$body"
    return
  fi
  lines="$(wc -l < "$src" | tr -d ' ')"
  bytes="$(wc -c < "$src" | tr -d ' ')"
  fence="$(fence_for "$src")"
  # файл без завершающего перевода строки помечаем, чтобы распаковка была байт-в-байт
  nonl=""
  [ -n "$(tail -c 1 "$src")" ] && nonl=" nonl=true"
  {
    printf '<!-- FILE path="%s" lines=%s bytes=%s lang=%s%s -->\n' "$vpath" "$lines" "$bytes" "$lang" "$nonl"
    printf '### %s\n\n' "$vpath"
    printf '%s%s\n' "$fence" "$lang"
    cat "$src"
    [ -n "$(tail -c 1 "$src")" ] && printf '\n'
    printf '%s\n' "$fence"
    printf '<!-- /FILE -->\n\n'
  } >> "$body"
}

lang_of() {
  case "$1" in
    *.rs) echo rust ;; *.toml) echo toml ;; *.c|*.h) echo c ;;
    *.md) echo markdown ;; *.json) echo json ;; *.sh) echo bash ;;
    *) echo text ;;
  esac
}

# emit_file <body> <path> — обычный файл проекта
emit_file() { emit "$1" "$2" "$2" "$(lang_of "$2")"; }

run() { # run <имя лога> <команда...>
  local name="$1"; shift
  if [ "$RUN_CARGO" = 0 ]; then
    printf 'Сбор логов пропущен (--no-cargo).\n' > "$TMP/raw/$name"
    return
  fi
  if ! command -v "$1" >/dev/null 2>&1; then
    printf 'Команда %s не найдена в PATH.\n' "$1" > "$TMP/raw/$name"
    return
  fi
  echo "   -> $name"
  if command -v timeout >/dev/null 2>&1; then
    timeout 900 "$@" > "$TMP/raw/$name" 2>&1
  else
    "$@" > "$TMP/raw/$name" 2>&1
  fi
  printf '\n[exit code = %s]\n' "$?" >> "$TMP/raw/$name"
}

B1="$TMP/body1.md"; B2="$TMP/body2.md"; B3="$TMP/body3.md"; B4="$TMP/body4.md"
: > "$B1"; : > "$B2"; : > "$B3"; : > "$B4"

echo "== 1/5 окружение и дерево =="
{
  echo "### rustc"; rustc -Vv 2>&1 || echo "rustc не найден"
  echo; echo "### cargo"; cargo -V 2>&1 || echo "cargo не найден"
  echo; echo "### uname"; uname -a
  echo; echo "### os-release"; grep -E '^(NAME|VERSION)=' /etc/os-release 2>/dev/null
  echo; echo "### nproc"; nproc 2>/dev/null
  echo; echo "### project root"; echo "$ROOT" | sed "s|${HOME}|~|g"
} > "$TMP/gen/toolchain.txt"

{
  echo "### Все файлы проекта (без target/, .git/, docs/crates/)"
  find . \( -name target -o -name .git -o -name review -o -path './docs/crates' \) -prune -o -type f -print \
    | sed 's|^\./||' | sort
  echo
  echo "### Строки Rust-кода"
  find . \( -name target -o -name .git -o -name review \) -prune -o -name '*.rs' -print0 \
    | xargs -0 wc -l 2>/dev/null | sort -n
  echo
  echo "### Размеры docs/reference"
  du -sh docs/reference/* 2>/dev/null
} > "$TMP/gen/tree.txt"

{
  if [ -d .git ] && command -v git >/dev/null 2>&1; then
    echo "### git log"; git log --oneline -15 2>&1
    echo; echo "### git status"; git status --short 2>&1 | head -50
    echo; echo "### git diff --stat (HEAD)"; git diff --stat HEAD 2>&1 | tail -40
  else
    echo "Не git-репозиторий или git не установлен."
  fi
} > "$TMP/gen/git.txt"

{
  echo "### ALSA карты"; cat /proc/asound/cards 2>/dev/null || echo "нет /proc/asound/cards"
  echo; echo "### aplay -l"; aplay -l 2>&1 | head -40 || true
  echo; echo "### PipeWire"; pipewire --version 2>&1 | head -5 || echo "pipewire не найден"
  echo; echo "### pw-cli info (default sink, первые 40 строк)"
  pw-cli info 0 2>&1 | head -40 || true
  echo; echo "###_pulse/pipewire pactl"
  pactl info 2>&1 | head -20 || true
  echo; echo "### Переменные окружения аудио/Deezer"
  env | grep -iE '^(DEEZER|PIPEWIRE|PULSE|ALSA|AUDIODEV)' | sed -E 's/(ARL=).*/\1REDACTED/I' || echo "нет"
} > "$TMP/gen/audio-env.txt"

echo "== 2/5 cargo check / test / clippy =="
run check.log  cargo check --all-targets
run test.log   cargo test -- --nocapture
run clippy.log cargo clippy --all-targets -- -D warnings
run deps.txt   cargo tree -e normal,build --depth 1

echo "== 3/5 код =="
# --- Файл 02: ядро DSP + main/config + эталон cava ---
emit_file "$B2" src/main.rs
emit_file "$B2" src/config.rs
if [ -d src/dsp ]; then
  while IFS= read -r f; do emit_file "$B2" "$f"; done \
    < <(find src/dsp -type f -name '*.rs' | sort)
fi
# прочие модули верхнего уровня (error.rs, state.rs, ring.rs, lib.rs, utils/ ...)
while IFS= read -r f; do
  case "$f" in
    src/main.rs|src/config.rs) continue ;;
    src/audio/*|src/ui/*|src/api/*|src/dsp/*) continue ;;
  esac
  emit_file "$B2" "$f"
done < <(find src -type f -name '*.rs' 2>/dev/null | sort)

for f in docs/reference/cava/cavacore.h docs/reference/cava/cavacore.c docs/reference/cava/cavacore_test.c; do
  emit_file "$B2" "$f"
done

# --- Файл 03: audio + ui/api заглушки + тесты ---
for d in src/audio src/ui src/api; do
  if [ -d "$d" ]; then
    while IFS= read -r f; do emit_file "$B3" "$f"; done < <(find "$d" -type f -name '*.rs' | sort)
  fi
done
for d in tests benches examples; do
  if [ -d "$d" ]; then
    while IFS= read -r f; do emit_file "$B3" "$f"; done < <(find "$d" -type f \( -name '*.rs' -o -name '*.toml' \) | sort)
  else
    printf '%s/ (каталог отсутствует)\n' "$d" >> "$MISSING"
  fi
done

echo "== 4/5 логи и конфиг =="
for l in check test clippy; do
  strip_ansi < "$TMP/raw/$l.log" | redact > "$TMP/raw/$l.clean"
  clip "$TMP/raw/$l.clean" "$TMP/gen/$l.log" "$LOG_HEAD" "$LOG_TAIL"
  emit "$B4" - "$l.log" text
done
strip_ansi < "$TMP/raw/deps.txt" | redact > "$TMP/gen/deps.txt"
emit "$B4" - deps.txt text
emit "$B4" - audio-env.txt text

RUNTIME_FOUND=0
for log in deezer-tui.log "$HOME/.local/state/deezer-tui/deezer-tui.log" \
           "$HOME/.cache/deezer-tui/deezer-tui.log" "$HOME/.config/deezer-tui/deezer-tui.log"; do
  if [ -f "$log" ]; then
    tail -n 400 "$log" | strip_ansi | redact > "$TMP/gen/runtime-tail.log"
    emit "$B4" - runtime-tail.log text
    RUNTIME_FOUND=1
    break
  fi
done
[ "$RUNTIME_FOUND" = 0 ] && emit "$B4" /nonexistent "deezer-tui.log" text

CFG="$HOME/.config/deezer-tui/config.toml"
if [ -f "$CFG" ]; then redact < "$CFG" > "$TMP/gen/config.example.toml"
else printf '# файл ~/.config/deezer-tui/config.toml не найден\n' > "$TMP/gen/config.example.toml"; fi

echo "== 5/5 манифест =="
emit_file "$B1" Cargo.toml
emit_file "$B1" rust-toolchain.toml
emit_file "$B1" .cargo/config.toml
emit_file "$B1" build.rs

if [ -f Cargo.lock ]; then
  if [ "$SLIM" = 1 ]; then
    awk '/^name = /{n=$3} /^version = /{v=$3; gsub(/"/,"",n); gsub(/"/,"",v); printf "%-32s %s\n", n, v}' \
      Cargo.lock | sort > "$TMP/gen/Cargo.lock.digest.txt"
    emit "$B1" - Cargo.lock.digest.txt text
  else
    emit_file "$B1" Cargo.lock
  fi
else
  emit "$B1" /nonexistent Cargo.lock toml
fi

emit "$B1" - toolchain.txt text
emit "$B1" - tree.txt text
emit "$B1" - git.txt text
emit "$B1" - config.example.toml toml

if [ -f docs/crates/index.md ]; then
  clip docs/crates/index.md "$TMP/gen/docs-crates-index.md" "$IDX_HEAD" "$IDX_TAIL"
  emit "$B1" - docs-crates-index.md markdown
else
  emit "$B1" /nonexistent docs/crates/index.md markdown
fi

if [ "$WITH_DOCS" = 1 ]; then
  for f in AGENT.md ARCHITECTURE.md PLAN.md TODO.md; do emit_file "$B1" "$f"; done
fi

# ---------- сборка итоговых .md --------------------------------------------
compose() { # compose <body> <outfile> <title> <description>
  local body="$1" out="$2" title="$3" desc="$4"
  {
    printf '# %s\n\n' "$title"
    printf '%s\n\n' "$desc"
    printf 'Проект: **deezer-tui**, ревизия: %s. Собрано скриптом `collect_review.sh`.\n\n' "$(date '+%Y-%m-%d %H:%M:%S %z')"
    printf 'Формат блоков: `<!-- FILE path="..." lines=N bytes=N lang=... -->`, затем заголовок, затем fenced-блок с содержимым файла БЕЗ изменений, затем `<!-- /FILE -->`.\n\n'
    printf '## Содержимое этого файла\n\n'
    grep -o '<!-- FILE path="[^"]*" lines=[0-9]* bytes=[0-9]*' "$body" \
      | sed -E 's/<!-- FILE path="([^"]*)" lines=([0-9]*) bytes=([0-9]*)/- `\1` — \2 строк, \3 байт/'
    printf '\n---\n\n'
    cat "$body"
  } > "$out"
}

F1="$OUT/deezer-tui-review-01-meta.md"
F2="$OUT/deezer-tui-review-02-dsp.md"
F3="$OUT/deezer-tui-review-03-audio-tests.md"
F4="$OUT/deezer-tui-review-04-logs.md"

compose "$B2" "$F2" "deezer-tui review 02 — ядро DSP, config, эталон cava" \
  'Фаза 1: `src/main.rs`, `src/config.rs`, `src/dsp/**`, прочие модули верхнего уровня и эталонные исходники cava для построчной сверки порта.'
compose "$B3" "$F3" "deezer-tui review 03 — аудиодвижок и тесты" \
  'Фаза 2: `src/audio/**`, заглушки `src/ui/**` и `src/api/**`, все интеграционные тесты, benches и examples.'
compose "$B4" "$F4" "deezer-tui review 04 — логи сборки, тестов и рантайма" \
  "Доказательная база: вывод cargo check / test / clippy, дерево зависимостей, аудио-окружение (ALSA/PipeWire) и хвост рантайм-лога. Секреты вычищены."

# сводный индекс + список отсутствующего — в файл 01
{
  printf '## Сводка по всем четырём файлам\n\n'
  for f in "$F2" "$F3" "$F4"; do
    printf -- '- `%s` — %s КБ\n' "$(basename "$f")" "$(( $(wc -c < "$f") / 1024 ))"
  done
  printf '\n## Отсутствует в проекте (запрошено, но не найдено)\n\n'
  if [ -s "$MISSING" ]; then sed 's/^/- /' "$MISSING"; else echo "- (всё найдено)"; fi
  printf '\n## Обрезано при упаковке\n\n'
  if [ -s "$TRUNCATED" ]; then sed "s|$TMP/raw/||; s|^|- |" "$TRUNCATED"; else echo "- (ничего не обрезано)"; fi
  printf '\n---\n\n'
} > "$TMP/index.md"
cat "$TMP/index.md" "$B1" > "$TMP/body1.full.md"
compose "$TMP/body1.full.md" "$F1" "deezer-tui review 01 — манифест, зависимости, окружение" \
  'Метаданные для сверки версий и features: Cargo.toml, Cargo.lock, тулчейн, дерево файлов, git-состояние, пример конфига (ARL вычищен), оглавление docs/crates.'

echo
echo "================= ГОТОВО ================="
for f in "$F1" "$F2" "$F3" "$F4"; do
  printf '%-42s %6s КБ  %5s строк\n' "$(basename "$f")" "$(( $(wc -c < "$f") / 1024 ))" "$(wc -l < "$f" | tr -d ' ')"
done
echo "------------------------------------------"
printf 'Итого: %s КБ в 4 файлах .md\n' "$(( $(cat "$F1" "$F2" "$F3" "$F4" | wc -c) / 1024 ))"
echo
echo "Прикрепите все 4 файла из каталога review/ к ОДНОМУ сообщению."
if [ -s "$MISSING" ]; then
  echo; echo "ВНИМАНИЕ: не найдено (это тоже полезная информация, я учту):"; sed 's/^/  - /' "$MISSING"
fi
BIG=$(find "$OUT" -name 'deezer-tui-review-*.md' -size +900k 2>/dev/null)
if [ -n "$BIG" ]; then
  echo; echo "ВНИМАНИЕ: некоторые файлы >900 КБ. Перезапустите с --slim:"; echo "$BIG"
fi
echo "Проверьте, что в 04-logs.md и config.example.toml нет вашего ARL (скрипт чистит, но глазами лучше)."