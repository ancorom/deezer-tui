#!/usr/bin/env bash
set -e

OUT="ALL_CRATES_DOCS.md"
> "$OUT"

append_crate() {
    local crate="$1"
    local dir="docs/crates/$crate"
    if [ ! -d "$dir" ]; then
        return
    fi

    echo -e "\n================================================================================" >> "$OUT"
    echo -e "## КРЕЙТ: $crate" >> "$OUT"
    echo -e "================================================================================\n" >> "$OUT"

    # Сначала добавляем корневой index.md крейта (если есть)
    if [ -f "$dir/index.md" ]; then
        echo -e "### Файл: $dir/index.md\n" >> "$OUT"
        cat "$dir/index.md" >> "$OUT"
        echo -e "\n" >> "$OUT"
    fi

    # Затем рекурсивно добавляем все остальные .md файлы внутри папки крейта
    find "$dir" -type f -name "*.md" ! -path "$dir/index.md" | sort | while read -r md_file; do
        echo -e "\n--------------------------------------------------------------------------------" >> "$OUT"
        echo -e "### Файл: $md_file\n" >> "$OUT"
        cat "$md_file" >> "$OUT"
        echo -e "\n" >> "$OUT"
    done
}

echo "# Полная документация всех крейтов из docs/crates/ (разбитая по компонентам)" >> "$OUT"

echo -e "\n\n# КОМПОНЕНТ 1: ПОЛЬЗОВАТЕЛЬСКИЙ ИНТЕРФЕЙС И ГРАФИКА (TUI & Graphics)" >> "$OUT"
for c in ratatui ratatui_core ratatui_widgets ratatui_crossterm ratatui_macros ratatui_image crossterm image; do
    append_crate "$c"
done

echo -e "\n\n# КОМПОНЕНЕНТ 2: АУДИОДВИЖОК И DSP СПЕКТРОАНАЛИЗАТОР (Audio & DSP)" >> "$OUT"
for c in cpal symphonia symphonia_core symphonia_metadata rtrb realfft rustfft; do
    append_crate "$c"
done

echo -e "\n\n# КОМПОНЕНТ 3: СЕТЬ, СТРИМИНГ И КРИПТОГРАФИЯ (Network & Crypto)" >> "$OUT"
for c in reqwest tokio bytes blowfish cbc cipher digest md_5 hex; do
    append_crate "$c"
done

echo -e "\n\n# КОМПОНЕНТ 4: СИСТЕМА, КОНФИГУРАЦИЯ, ЛОГИ И MPRIS (System & Config)" >> "$OUT"
for c in anyhow serde serde_json toml tempfile souvlaki tracing tracing_appender tracing_subscriber; do
    append_crate "$c"
done

echo "Готово! Статистика собранного файла:"
ls -lh "$OUT"
wc -l "$OUT"
