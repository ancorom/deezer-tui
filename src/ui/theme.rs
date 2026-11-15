//! Палитра TrueColor (`Color::Rgb`) и функция линейной интерполяции градиента.
//!
//! Цвета из `ARCHITECTURE.md` (раздел 6.1):
//! - Основной фон терминала: `#13151F` (`Rgb(19, 21, 31)`)
//! - Фон карточек и модальных окон: `#1E2233` (`Rgb(30, 34, 51)`)
//! - Рамки неактивных блоков: `#3E445E` (`Rgb(62, 68, 94)`)
//! - Основной текст: `#ECEFF4` (`Rgb(236, 239, 244)`)
//! - Вторичный текст: `#94A3B8` (`Rgb(148, 163, 184)`)
//! - Градиент: от `#4FD1C5` (`Rgb(79, 209, 197)`) до `#7F5AF0` (`Rgb(127, 90, 240)`)

#![allow(dead_code)]

use ratatui::style::{Color, Style};

pub const BG_TERMINAL: u32 = 0x13151F;
pub const BG_CARD: u32 = 0x1E2233;
pub const BORDER: u32 = 0x3E445E;
pub const TEXT_PRIMARY: u32 = 0xECEFF4;
pub const TEXT_SECONDARY: u32 = 0x94A3B8;
pub const GRAD_START: u32 = 0x4FD1C5;
pub const GRAD_END: u32 = 0x7F5AF0;

/// Конвертирует `0xRRGGBB` в `Color::Rgb(r, g, b)`.
pub const fn color_from_hex(h: u32) -> Color {
    Color::from_u32(h)
}

/// Линейная интерполяция цвета между `GRAD_START` (`#4FD1C5`) и `GRAD_END` (`#7F5AF0`).
#[inline]
pub fn gradient_color(t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let (sr, sg, sb) = hex_to_rgb(GRAD_START);
    let (er, eg, eb) = hex_to_rgb(GRAD_END);
    let r = (sr as f32 + (er as f32 - sr as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8;
    let g = (sg as f32 + (eg as f32 - sg as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8;
    let b = (sb as f32 + (eb as f32 - sb as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8;
    Color::Rgb(r, g, b)
}

const fn hex_to_rgb(h: u32) -> (u8, u8, u8) {
    ((h >> 16) as u8, (h >> 8) as u8, h as u8)
}

pub fn style_primary() -> Style {
    Style::default()
        .fg(color_from_hex(TEXT_PRIMARY))
        .bg(color_from_hex(BG_TERMINAL))
}

pub fn style_secondary() -> Style {
    Style::default()
        .fg(color_from_hex(TEXT_SECONDARY))
        .bg(color_from_hex(BG_TERMINAL))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_endpoints() {
        assert_eq!(gradient_color(0.0), color_from_hex(GRAD_START));
        assert_eq!(gradient_color(1.0), color_from_hex(GRAD_END));
    }

    #[test]
    fn gradient_midpoint_is_smoothly_interpolated() {
        let mid = gradient_color(0.5);
        assert_ne!(mid, color_from_hex(GRAD_START));
        assert_ne!(mid, color_from_hex(GRAD_END));
        assert_eq!(mid, Color::Rgb(103, 150, 219));
    }

    #[test]
    fn gradient_clamped() {
        assert_eq!(gradient_color(-0.5), gradient_color(0.0));
        assert_eq!(gradient_color(1.7), gradient_color(1.0));
    }

    #[test]
    fn palette_constants_are_distinct() {
        let p = [BG_TERMINAL, BG_CARD, BORDER, TEXT_PRIMARY, TEXT_SECONDARY];
        assert_eq!(p.len(), p.windows(2).filter(|w| w[0] != w[1]).count() + 1);
    }
}
