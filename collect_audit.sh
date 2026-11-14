#!/usr/bin/env bash
# Сбор материалов для ревью Фаз 1-2 deezer-tui.
# Запускать из КОРНЯ проекта: bash collect_audit.sh
# На выходе: deezer-tui-audit.tar.gz (пришлите его в чат)

set -uo pipefail
OUT="audit"
mkdir -p "$OUT"

echo "== toolchain =="
{ rustc -Vv; echo; cargo -V; } > "$OUT/toolchain.txt" 2>&1

echo "== file tree =="
{
  find . \( -name target -o -name .git -o -path './docs/crates' \) -prune -o -type f -print | sort
  echo
  echo "== lines of rust code =="
  find . \( -name target -o -name .git \) -prune -o -name '*.rs' -print0 | xargs -0 wc -l | sort -n
} > "$OUT/tree.txt" 2>&1

echo "== cargo check =="
cargo check --all-targets --message-format short > "$OUT/check.log" 2>&1
echo "   exit=$? -> $OUT/check.log"

echo "== cargo test =="
cargo test -- --nocapture > "$OUT/test.log" 2>&1
echo "   exit=$? -> $OUT/test.log"

echo "== cargo clippy =="
cargo clippy --all-targets -- -D warnings > "$OUT/clippy.log" 2>&1
echo "   exit=$? -> $OUT/clippy.log"

echo "== deps =="
cargo tree -e normal,build --depth 1 > "$OUT/deps.txt" 2>&1

# Хвост рантайм-лога, если он есть (проверьте, что там нет токенов!)
for log in deezer-tui.log "$HOME/.local/state/deezer-tui/deezer-tui.log" "$HOME/.cache/deezer-tui/deezer-tui.log"; do
  if [ -f "$log" ]; then tail -n 300 "$log" > "$OUT/runtime-tail.log"; break; fi
done

# Пример конфига БЕЗ секрета
CFG="$HOME/.config/deezer-tui/config.toml"
if [ -f "$CFG" ]; then
  sed -E 's/(arl[[:space:]]*=[[:space:]]*).*/\1"REDACTED"/I; s/(sid|token|license_token)([[:space:]]*=[[:space:]]*).*/\1\2"REDACTED"/I' \
    "$CFG" > "$OUT/config.example.toml"
fi

FILES=()
for p in Cargo.toml Cargo.lock rust-toolchain.toml rust-toolchain .cargo/config.toml build.rs \
         src tests benches examples \
         docs/reference/cava docs/crates/index.md \
         AGENT.md ARCHITECTURE.md PLAN.md TODO.md "$OUT"; do
  [ -e "$p" ] && FILES+=("$p")
done

tar czf deezer-tui-audit.tar.gz "${FILES[@]}"
echo
echo "Готово: $(du -h deezer-tui-audit.tar.gz | cut -f1) deezer-tui-audit.tar.gz"
echo "ВАЖНО: перед отправкой убедитесь, что в audit/config.example.toml и audit/runtime-tail.log нет вашего ARL."