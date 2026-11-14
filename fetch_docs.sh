#!/usr/bin/env bash
set -e

echo "[1/3] Генерация Markdown-документации по крейтам..."
mkdir -p docs
# Если в v0.11.0 поддерживается -o, используем его, иначе забираем из дефолтной папки target/doc-md
if cargo doc-md --help | grep -q -- "-o"; then
    cargo doc-md -o docs/crates
else
    cargo doc-md
    rm -rf docs/crates
    mv target/doc-md docs/crates
fi

echo "[2/3] Загрузка оригинального cavacore (cavacore.h, cavacore.c, cavacore_test.c)..."
mkdir -p docs/reference/cava
curl -sSL https://raw.githubusercontent.com/karlstav/cava/master/cavacore.h -o docs/reference/cava/cavacore.h
curl -sSL https://raw.githubusercontent.com/karlstav/cava/master/cavacore.c -o docs/reference/cava/cavacore.c
curl -sSL https://raw.githubusercontent.com/karlstav/cava/master/cavacore_test.c -o docs/reference/cava/cavacore_test.c

echo "[3/3] Загрузка референсов Deezer API и Blowfish (Tatayoyoh/deezer-tui и OpenDeezer)..."
mkdir -p docs/reference/deezer_rust_ref
mkdir -p docs/reference/opendeezer_go_ref

TMP_DIR=$(mktemp -d)

git clone --depth 1 https://github.com/Tatayoyoh/deezer-tui.git "$TMP_DIR/deezer-tui"
cp -r "$TMP_DIR/deezer-tui/crates" docs/reference/deezer_rust_ref/

git clone --depth 1 https://github.com/Cycl0o0/OpenDeezer.git "$TMP_DIR/OpenDeezer"
cp -r "$TMP_DIR/OpenDeezer/internal/deezer" docs/reference/opendeezer_go_ref/

rm -rf "$TMP_DIR"
echo "Готово! Вся документация собрана в docs/"
