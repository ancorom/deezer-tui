mod api;
mod audio;
mod config;
mod dsp;
mod mpris;
mod ui;

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ui::app::App;

/// Инициализация файлового логгера tracing: логи пишутся в ~/.config/deezer-tui/deezer-tui.log
/// (ежедневная ротация файлов).
///
/// В TUI-режиме никогда не вызывайте println!/eprintln! — только макросы tracing.
fn init_file_logger() -> Result<()> {
    let log_dir = env::var("DEEZER_TUI_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(PathBuf::from)
                .map(|p| p.join(".config/deezer-tui"))
                .unwrap_or_else(|_| PathBuf::from(".config/deezer-tui"))
        });

    fs::create_dir_all(&log_dir)
        .with_context(|| format!("не удалось создать директорию логов {}", log_dir.display()))?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, "deezer-tui.log");
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_writer(file_appender)
        .try_init();

    tracing::info!(
        "deezer-tui: файловый логгер инициализирован в {}",
        log_dir.display()
    );
    Ok(())
}

fn main() -> Result<()> {
    init_file_logger()?;
    let cfg = config::AppConfig::load();
    tracing::info!(
        volume = cfg.volume,
        sample_rate_hz = cfg.sample_rate_hz,
        has_arl = cfg.effective_arl().is_some(),
        "конфигурация загружена"
    );
    let mut app = App::with_loaded_config(cfg);
    app.run()?;
    Ok(())
}
