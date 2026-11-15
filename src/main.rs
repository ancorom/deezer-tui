mod api;
mod audio;
mod config;
mod dsp;
mod mpris;
mod ui;

use anyhow::Result;
use std::env;
use std::path::PathBuf;
use crate::ui::app::App;

/// Инициализация файлового логгера tracing: логи пишутся в ~/.config/deezer-tui/deezer-tui.log
/// (ежедневные роллиng-файлы, 20 МБ на файл).
///
/// В TUI-режиме никогда не вызывайте println!/eprintln! — только tracing.
fn init_file_logger() -> Result<()> {
    let log_dir = env::var("DEEZER_TUI_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(PathBuf::from)
                .map(|p| p.join("config/deezer-tui"))
                .unwrap_or_else(|_| PathBuf::from("config/deezer-tui"))
        });
    let file_appender = tracing_appender::rolling::daily(&log_dir, "deezer-tui");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(file_appender)
        .init();
    tracing::info!(
        "deezer-tui: логгер инициализирован, лог: {}",
        log_dir.display()
    );
    Ok(())
}

fn main() -> Result<()> {
    init_file_logger()?;
    let mut app = App::new();
    app.run()?;
    Ok(())
}
