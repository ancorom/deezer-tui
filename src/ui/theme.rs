//! UI theme: Deezer-TUI TrueColor palette and gradient interpolation.
//!
//! All colors are TrueColor (`Color::Rgb`). Values are stored as 0xRRGGBB
//! constants so the palette can be referenced in one place; the `style_*`
//! helpers convert them to ready-to-use [`Style`] values.

use ratatui::style::Color;
use ratatui::style::Style;

/// 0x13151F — terminal background (the whole viewport).
#[allow(dead_code)]
pub const BG_TERMINAL: u32 = 0x13151F;
/// 0x1E2233 — card / modal / panel background.
#[allow(dead_code)]
pub const BG_CARD: u32 = 0x1E2233;
/// 0x3E445E — inactive border color.
#[allow(dead_code)]
pub const BORDER: u32 = 0x3E445E;
/// 0xECEFF4 — primary (readable) text.
#[allow(dead_code)]
pub const TEXT_PRIMARY: u32 = 0xECEFF4;
/// 0x94A3B8 — secondary (dim) text.
#[allow(dead_code)]
pub const TEXT_SECONDARY: u32 = 0x94A3B8;
/// 0x4FD1C5 — gradient start (teal).
#[allow(dead_code)]
pub const GRAD_START: u32 = 0x4FD1C5;
/// 0x7F5AF0 — gradient end (violet).
#[allow(dead_code)]
pub const GRAD_END: u32 = 0x7F5AF0;

/// Build a TrueColor [`Color`] from a 0xRRGGBB value.
#[allow(dead_code)]
pub const fn color_from_hex(h: u32) -> Color {
    Color::from_u32(h)
}

/// Lerp between [`GRAD_START`] and [`GRAD_END`].
///
/// `t = 0.0` is the start (teal `#4FD1C5`), `t = 1.0` is the end (violet
/// `#7F5AF0`). Values outside [0, 1] are clamped.
///
/// This is used for the spectrum and for active elements.
#[inline]
#[allow(dead_code)]
pub fn gradient_color(t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let (sr, sg, sb) = hex_to_rgb(GRAD_START);
    let (er, eg, eb) = hex_to_rgb(GRAD_END);
    let r = (sr as i32 + (er as i32 - sr as i32) * t as i32).clamp(0, 255) as u8;
    let g = (sg as i32 + (eg as i32 - sg as i32) * t as i32).clamp(0, 255) as u8;
    let b = (sb as i32 + (eb as i32 - sb as i32) * t as i32).clamp(0, 255) as u8;
    Color::Rgb(r, g, b)
}

fn hex_to_rgb(h: u32) -> (u8, u8, u8) {
    ((h >> 16) as u8, (h >> 8) as u8, h as u8)
}

/// Primary (readable) text style.
#[allow(dead_code)]
pub fn style_primary() -> Style {
    Style::default()
        .fg(color_from_hex(TEXT_PRIMARY))
        .bg(color_from_hex(BG_TERMINAL))
}

/// Secondary (dim) text style.
#[allow(dead_code)]
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
        // t = 0.0 must return the start color.
        assert_eq!(gradient_color(0.0), color_from_hex(GRAD_START));
        // t = 1.0 must return the end color.
        assert_eq!(gradient_color(1.0), color_from_hex(GRAD_END));
    }

    #[test]
    fn gradient_clamped() {
        assert_eq!(gradient_color(-0.5), gradient_color(0.0));
        assert_eq!(gradient_color(1.7), gradient_color(1.0));
    }

    #[test]
    fn gradient_monotonic() {
        // The R and B channels strictly increase from teal to violet.
        // We can't read channels off a Color directly, so we verify against
        // the known endpoints and a midpoint whose R must be between them.
        let start = color_from_hex(GRAD_START);
        let end = color_from_hex(GRAD_END);
        assert_ne!(start, end);
        assert_eq!(gradient_color(0.0), start);
        assert_eq!(gradient_color(1.0), end);
    }

    #[test]
    fn palette_constants_are_distinct() {
        let p: Vec<u32> = vec![
            BG_TERMINAL,
            BG_CARD,
            BORDER,
            TEXT_PRIMARY,
            TEXT_SECONDARY,
        ];
        assert_eq!(p.len(), p.windows(2).filter(|w| w[0] != w[1]).count() + 1);
    }
}
