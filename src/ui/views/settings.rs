//! Экран 05 (Settings): выбор качества аудиопотока (`FLAC`, `MP3_320`, `MP3_128`),
//! ввод и сохранение `ARL`-токена со статусом подписки, переключатели `Show Spectrum`,
//! `dB Scale`, шкала высот спектра (`Track Peak` / `Frame Peak` / `0 dBFS`) и высота
//! панели спектра (`AUTO` или 4–12 строк).

#![allow(dead_code)]

use ratatui::prelude::*;

use crate::config::AppConfig;
use crate::dsp::cavacore::{SpectrumScale, MONSTERCAT_BASE};
use crate::ui::theme;

pub const QUALITY_CODES: [&str; 3] = ["FLAC", "MP3_320", "MP3_128"];

/// Порядок переключения шкалы высот спектра стрелками `←`/`→`.
pub const SPECTRUM_SCALE_CHOICES: [SpectrumScale; 3] = [
    SpectrumScale::TrackPeak,
    SpectrumScale::FramePeak,
    SpectrumScale::Legacy,
];

/// Порядок переключения базы фильтра соседних полос: `0.0` — выключен (как в
/// эталонной cava), дальше мягкие базы; `MONSTERCAT_BASE` (1.35) — «плато» v2.5.
pub const SPECTRUM_SPREAD_CHOICES: [f32; 4] = [0.0, 3.0, 2.0, MONSTERCAT_BASE];

/// Порядок переключения высоты панели спектра: `None` — авто (4–6 строк).
pub const SPECTRUM_ROW_CHOICES: [Option<u16>; 6] =
    [None, Some(4), Some(6), Some(8), Some(10), Some(12)];

#[derive(Debug, Clone)]
pub struct Settings {
    pub selected_row: usize,
    pub preferred_quality: String,
    pub arl_input: String,
    pub editing_arl: bool,
    pub account_status: String,
    pub show_spectrum: bool,
    pub db_scale: bool,
    pub spectrum_scale: SpectrumScale,
    pub spectrum_rows: Option<u16>,
    pub spectrum_spread: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        Self {
            selected_row: 0,
            preferred_quality: String::from("FLAC"),
            arl_input: String::new(),
            editing_arl: false,
            account_status: String::from("GUEST (30s MP3 Previews)"),
            show_spectrum: true,
            db_scale: false,
            spectrum_scale: SpectrumScale::TrackPeak,
            spectrum_rows: None,
            spectrum_spread: 0.0,
        }
    }

    /// Синхронизирует состояние экрана настроек с `AppConfig` и текущим статусом авторизации.
    pub fn sync_from_config(
        &mut self,
        cfg: &AppConfig,
        user_label: Option<&str>,
        status_message: Option<&str>,
    ) {
        self.preferred_quality = cfg.preferred_quality.clone();
        if !self.editing_arl {
            self.arl_input = cfg.effective_arl().unwrap_or_default();
        }
        self.show_spectrum = cfg.show_spectrum;
        self.db_scale = cfg.db_scale;
        self.spectrum_scale = cfg.effective_spectrum_scale();
        self.spectrum_rows = cfg.effective_spectrum_rows();
        self.spectrum_spread = cfg.effective_spectrum_spread();

        self.account_status = if let Some(user) = user_label {
            format!("CONNECTED as {user} (Deezer HiFi / BF_CBC_STRIPE ready)")
        } else if let Some(msg) = status_message {
            msg.to_string()
        } else if cfg.effective_arl().is_some() {
            String::from("ARL configured (verifying session…)")
        } else {
            String::from("GUEST (30s MP3 Previews — paste ARL token for full FLAC)")
        };
    }

    /// Переключает предпочитаемое качество звука по кругу (`FLAC` <-> `MP3_320` <-> `MP3_128`).
    pub fn cycle_quality(&mut self, forward: bool) -> String {
        let current_idx = QUALITY_CODES
            .iter()
            .position(|&q| q.eq_ignore_ascii_case(self.preferred_quality.trim()))
            .unwrap_or(0);
        let next_idx = if forward {
            (current_idx + 1) % QUALITY_CODES.len()
        } else if current_idx == 0 {
            QUALITY_CODES.len() - 1
        } else {
            current_idx - 1
        };
        self.preferred_quality = QUALITY_CODES[next_idx].to_string();
        self.preferred_quality.clone()
    }

    pub fn quality_display_label(&self) -> &'static str {
        match self.preferred_quality.trim().to_uppercase().as_str() {
            "MP3_320" => "MP3 HQ (320 kbps)",
            "MP3_128" => "MP3 Standard (128 kbps)",
            _ => "FLAC (16-bit / 44.1 kHz, 1411 kbps)",
        }
    }

    /// Переключает шкалу высот спектра по кругу и возвращает новую.
    pub fn cycle_spectrum_scale(&mut self, forward: bool) -> SpectrumScale {
        let idx = SPECTRUM_SCALE_CHOICES
            .iter()
            .position(|m| *m == self.spectrum_scale)
            .unwrap_or(0);
        let next = if forward {
            (idx + 1) % SPECTRUM_SCALE_CHOICES.len()
        } else if idx == 0 {
            SPECTRUM_SCALE_CHOICES.len() - 1
        } else {
            idx - 1
        };
        self.spectrum_scale = SPECTRUM_SCALE_CHOICES[next];
        self.spectrum_scale
    }

    /// Переключает высоту панели спектра по кругу (`AUTO` -> 4 -> 6 -> ... -> 12 -> AUTO).
    pub fn cycle_spectrum_rows(&mut self, forward: bool) -> Option<u16> {
        let idx = SPECTRUM_ROW_CHOICES
            .iter()
            .position(|r| *r == self.spectrum_rows)
            .unwrap_or(0);
        let next = if forward {
            (idx + 1) % SPECTRUM_ROW_CHOICES.len()
        } else if idx == 0 {
            SPECTRUM_ROW_CHOICES.len() - 1
        } else {
            idx - 1
        };
        self.spectrum_rows = SPECTRUM_ROW_CHOICES[next];
        self.spectrum_rows
    }

    /// Переключает базу фильтра соседних полос по кругу и возвращает новую.
    pub fn cycle_spectrum_spread(&mut self, forward: bool) -> f32 {
        let idx = SPECTRUM_SPREAD_CHOICES
            .iter()
            .position(|b| (*b - self.spectrum_spread).abs() < 1e-3)
            .unwrap_or(0);
        let next = if forward {
            (idx + 1) % SPECTRUM_SPREAD_CHOICES.len()
        } else if idx == 0 {
            SPECTRUM_SPREAD_CHOICES.len() - 1
        } else {
            idx - 1
        };
        self.spectrum_spread = SPECTRUM_SPREAD_CHOICES[next];
        self.spectrum_spread
    }

    /// Подпись текущей базы фильтра соседних полос для экрана Settings.
    pub fn spectrum_spread_label(&self) -> String {
        if self.spectrum_spread <= 0.0 {
            String::from("◄ OFF \u{00b7} cava ►")
        } else {
            format!("◄ {:.2} ►", self.spectrum_spread)
        }
    }

    /// Подпись текущей высоты панели для экрана Settings.
    pub fn spectrum_rows_label(&self) -> String {
        match self.spectrum_rows {
            None => String::from("◄ AUTO (4–6) ►"),
            Some(n) => format!("◄ {n} rows ►"),
        }
    }

    pub fn masked_arl(&self) -> String {
        let trimmed = self.arl_input.trim();
        if trimmed.is_empty() {
            return String::from("<not set — press Enter to paste ARL cookie>");
        }
        let chars: Vec<char> = trimmed.chars().collect();
        if chars.len() <= 8 {
            "*".repeat(chars.len())
        } else {
            let prefix: String = chars[..4].iter().collect();
            let suffix: String = chars[chars.len() - 4..].iter().collect();
            format!("{prefix}••••••••{suffix} ({} chars)", chars.len())
        }
    }

    fn truncate(text: &str, max_width: usize) -> String {
        text.chars().take(max_width).collect()
    }
}

impl Widget for &Settings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 4 || h < 3 {
            return;
        }

        let x0 = area.x;
        let mut y = area.y;
        let max_w = w as usize;

        let title_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_END))
            .bold();
        let dim_style = Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY));
        let normal_style = Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY));
        let active_style = Style::default()
            .fg(theme::color_from_hex(theme::GRAD_START))
            .bold();

        buf.set_string(
            x0,
            y,
            Settings::truncate("SETTINGS & AUDIO CONFIGURATION", max_w),
            title_style,
        );
        y += 1;

        if y < area.bottom() {
            let status_line = format!("Account Status: {}", self.account_status);
            buf.set_string(
                x0,
                y,
                Settings::truncate(&status_line, max_w),
                dim_style,
            );
            y += 2;
        }

        let arl_value = if self.editing_arl {
            format!("{}█  [Enter: Save & Login | Esc: Cancel]", self.arl_input)
        } else {
            self.masked_arl()
        };

        let items: [(&str, String, &str); 7] = [
            (
                "Preferred Audio Quality",
                format!("◄ {} ►", self.quality_display_label()),
                "Use ←/→ or Enter to switch between FLAC 1411, MP3 320, MP3 128",
            ),
            (
                "Deezer ARL Cookie",
                arl_value,
                "Press Enter to edit/paste ARL token from browser cookies",
            ),
            (
                "Show Spectrum Analyzer",
                if self.show_spectrum {
                    String::from("[ON]")
                } else {
                    String::from("[OFF]")
                },
                "Toggle real-time Cavacore FFT visualizer (or press [v] anytime)",
            ),
            (
                "Spectrum Logarithmic Scale (dB)",
                if self.db_scale {
                    String::from("[ON]")
                } else {
                    String::from("[OFF]")
                },
                "Legacy scale only: linear amplitude vs 32 dB logarithmic scale",
            ),
            (
                "Spectrum Height Scale",
                format!("◄ {} ►", self.spectrum_scale.label()),
                "◄► Track Peak: track loudness reference | Frame Peak: cava-like | 0 dBFS: v2.5",
            ),
            (
                "Spectrum Neighbor Spread",
                self.spectrum_spread_label(),
                "◄► OFF (cava-like, bands stay independent) | 3.0 / 2.0 soft | 1.35 legacy plateau",
            ),
            (
                "Spectrum Panel Rows",
                self.spectrum_rows_label(),
                "◄► AUTO (4–6 rows) or fixed 4..12 rows for finer visual detail",
            ),
        ];

        for (idx, (label, val, hint)) in items.iter().enumerate() {
            if y >= area.bottom() {
                break;
            }
            let selected = self.selected_row == idx;
            let prefix = if selected { "▶ " } else { "  " };
            let line = format!("{prefix}{label}:  {val}");
            buf.set_string(
                x0,
                y,
                Settings::truncate(&line, max_w),
                if selected { active_style } else { normal_style },
            );
            y += 1;

            if y < area.bottom() {
                let hint_line = format!("    {hint}");
                buf.set_string(
                    x0,
                    y,
                    Settings::truncate(&hint_line, max_w),
                    dim_style,
                );
                y += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn render(screen: &Settings, width: u16, height: u16) -> Terminal<TestBackend> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(screen, frame.area());
            })
            .unwrap();
        terminal
    }

    fn at(terminal: &Terminal<TestBackend>, x: u16, y: u16) -> char {
        terminal
            .backend()
            .buffer()
            .cell((x, y))
            .expect("buffer cell out of bounds")
            .symbol()
            .chars()
            .next()
            .unwrap_or(' ')
    }

    fn row_has(terminal: &Terminal<TestBackend>, y: u16, needle: &str) -> bool {
        let width = terminal.size().unwrap().width;
        let row: String = (0..width).map(|x| at(terminal, x, y)).collect();
        row.contains(needle)
    }

    #[test]
    fn renders_in_normal_size() {
        let screen = Settings::new();
        let terminal = render(&screen, 120, 30);
        assert!(row_has(&terminal, 0, "SETTINGS"));
        assert!(row_has(&terminal, 1, "Account Status:"));
        assert!(row_has(&terminal, 3, "Preferred Audio Quality"));
        assert!(row_has(&terminal, 3, "FLAC (16-bit / 44.1 kHz, 1411 kbps)"));
        assert!(row_has(&terminal, 5, "Deezer ARL Cookie"));
        assert!(row_has(&terminal, 7, "Show Spectrum Analyzer"));
        assert!(row_has(&terminal, 9, "Spectrum Logarithmic Scale (dB)"));
    }

    #[test]
    fn renders_spectrum_scale_and_panel_rows() {
        let screen = Settings::new();
        let terminal = render(&screen, 120, 30);
        assert!(row_has(&terminal, 11, "Spectrum Height Scale"));
        assert!(row_has(&terminal, 11, "Track peak \u{00b7} 26 dB"));
        assert!(row_has(&terminal, 13, "Spectrum Neighbor Spread"));
        assert!(row_has(&terminal, 13, "OFF \u{00b7} cava"));
        assert!(row_has(&terminal, 15, "Spectrum Panel Rows"));
        assert!(row_has(&terminal, 15, "AUTO (4\u{2013}6)"));
    }

    #[test]
    fn cycles_spectrum_scale_and_panel_rows() {
        let mut screen = Settings::new();
        assert_eq!(screen.cycle_spectrum_scale(true), SpectrumScale::FramePeak);
        assert_eq!(screen.cycle_spectrum_scale(true), SpectrumScale::Legacy);
        assert_eq!(screen.cycle_spectrum_scale(true), SpectrumScale::TrackPeak);
        assert_eq!(screen.cycle_spectrum_scale(false), SpectrumScale::Legacy);

        assert_eq!(screen.cycle_spectrum_spread(true), 3.0);
        assert_eq!(screen.cycle_spectrum_spread(true), 2.0);
        assert_eq!(screen.cycle_spectrum_spread(false), 3.0);
        assert_eq!(screen.cycle_spectrum_spread(false), 0.0);
        assert_eq!(screen.spectrum_spread_label(), "\u{25c4} OFF \u{00b7} cava \u{25ba}");
        assert_eq!(screen.cycle_spectrum_spread(true), 3.0);
        assert_eq!(screen.spectrum_spread_label(), "\u{25c4} 3.00 \u{25ba}");

        assert_eq!(screen.cycle_spectrum_rows(true), Some(4));
        assert_eq!(screen.cycle_spectrum_rows(true), Some(6));
        assert_eq!(screen.cycle_spectrum_rows(false), Some(4));
        assert_eq!(screen.cycle_spectrum_rows(false), None);
        assert_eq!(screen.spectrum_rows_label(), "\u{25c4} AUTO (4\u{2013}6) \u{25ba}");

        screen.spectrum_rows = Some(10);
        assert_eq!(screen.spectrum_rows_label(), "\u{25c4} 10 rows \u{25ba}");
        assert_eq!(screen.cycle_spectrum_rows(true), Some(12));
        assert_eq!(screen.cycle_spectrum_rows(true), None);
    }

    #[test]
    fn cycles_audio_quality_forward_and_backward() {
        let mut screen = Settings::new();
        assert_eq!(screen.preferred_quality, "FLAC");
        assert_eq!(screen.cycle_quality(true), "MP3_320");
        assert_eq!(screen.cycle_quality(true), "MP3_128");
        assert_eq!(screen.cycle_quality(true), "FLAC");
        assert_eq!(screen.cycle_quality(false), "MP3_128");
    }

    #[test]
    fn masks_arl_token_and_shows_editor() {
        let mut screen = Settings::new();
        screen.arl_input = "0123456789abcdef0123456789abcdef".into();
        assert!(screen.masked_arl().contains("0123••••••••cdef"));

        screen.editing_arl = true;
        let terminal = render(&screen, 120, 20);
        assert!(row_has(&terminal, 5, "[Enter: Save & Login | Esc: Cancel]"));
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let screen = Settings::new();
        let terminal = render(&screen, 12, 4);
        assert!(row_has(&terminal, 0, "SETTINGS"));
    }
}
