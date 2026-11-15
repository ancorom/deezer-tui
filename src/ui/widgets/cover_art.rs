use std::io::Cursor;
use std::path::Path;

use image::{self, imageops, DynamicImage, RgbaImage};
use ratatui::prelude::*;
use ratatui::style::{Color, Style};

#[allow(dead_code)]
const HALF_BLOCK: char = '\u{2580}';

/// Таблица символов квадрантов 2x2 по 4-битной маске `(tl: 1, tr: 2, bl: 4, br: 8)`.
const QUADRANT_GLYPHS: [&str; 16] = [
    " ", "▘", "▝", "▀", "▖", "▌", "▞", "▛", "▗", "▚", "▐", "▜", "▄", "▙", "▟", "█",
];

#[derive(Clone)]
pub struct CoverArt {
    img: DynamicImage,
}

impl std::fmt::Debug for CoverArt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CoverArt({}x{})", self.img.width(), self.img.height())
    }
}

impl CoverArt {
    /// Загружает обложку из файла на диске через `image::ImageReader`.
    #[allow(dead_code)]
    pub fn new(path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let img: DynamicImage = image::ImageReader::open(path)?.decode()?;
        Ok(Self { img })
    }

    /// Декодирует JPEG/PNG обложку из буфера памяти через `image::ImageReader`.
    pub fn from_bytes(bytes: &[u8]) -> image::ImageResult<Self> {
        let reader = image::ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        let img = reader.decode()?;
        Ok(Self { img })
    }

    pub fn from_image(img: RgbaImage) -> Self {
        Self {
            img: DynamicImage::from(img),
        }
    }

    /// Генерирует стильную градиентную обложку-плейсхолдер до загрузки реального JPEG.
    pub fn solid(r: u8, g: u8, b: u8) -> Self {
        let mut rgba = RgbaImage::new(32, 32);
        for y in 0..32 {
            for x in 0..32 {
                let mix = ((x + y) as f32) / 62.0;
                let nr = (r as f32 * (1.0 - mix * 0.4) + 127.0 * (mix * 0.4)) as u8;
                let ng = (g as f32 * (1.0 - mix * 0.5) + 90.0 * (mix * 0.5)) as u8;
                let nb = (b as f32 * (1.0 - mix * 0.3) + 240.0 * (mix * 0.3)) as u8;
                rgba[(x, y)] = image::Rgba([nr, ng, nb, 255u8]);
            }
        }
        Self::from_image(rgba)
    }

    fn resized_quadrant(&self, w: u32, h: u32) -> RgbaImage {
        let pw = (w * 2).max(1);
        let ph = (h * 2).max(1);
        let filter = if self.img.width() <= w && self.img.height() <= ph {
            imageops::FilterType::Nearest
        } else {
            imageops::FilterType::CatmullRom
        };
        self.img.resize_exact(pw, ph, filter).to_rgba8()
    }

    #[inline]
    fn color_dist_sq(a: [u8; 3], b: [f32; 3]) -> f32 {
        let dr = a[0] as f32 - b[0];
        let dg = a[1] as f32 - b[1];
        let db = a[2] as f32 - b[2];
        dr * dr + dg * dg + db * db
    }

    /// Подбирает оптимальный квадрантный символ 2x2 и пару цветов `(fg, bg)` с минимальной ошибкой.
    #[inline]
    fn best_quadrant_cell(px: [[u8; 3]; 4]) -> (&'static str, Color, Color) {
        // Если верхняя пара и нижняя пара горизонтально однородны — используем классический полублок '▀'
        if px[0] == px[1] && px[2] == px[3] {
            return (
                "▀",
                Color::Rgb(px[0][0], px[0][1], px[0][2]),
                Color::Rgb(px[2][0], px[2][1], px[2][2]),
            );
        }

        let mut best_mask = 0b0011usize;
        let mut best_err = f32::MAX;
        let mut best_fg = [px[0][0], px[0][1], px[0][2]];
        let mut best_bg = [px[2][0], px[2][1], px[2][2]];

        // Проверяем все разбиения 4 субпикселей на 2 непустых кластера (маски 1..=7)
        for mask in [0b0011usize, 0b0101, 0b0001, 0b0010, 0b0100, 0b0111, 0b0110] {
            let mut fg_sum = [0.0f32; 3];
            let mut bg_sum = [0.0f32; 3];
            let mut fg_cnt = 0.0f32;
            let mut bg_cnt = 0.0f32;

            for (i, p) in px.iter().enumerate() {
                if (mask & (1 << i)) != 0 {
                    fg_sum[0] += p[0] as f32;
                    fg_sum[1] += p[1] as f32;
                    fg_sum[2] += p[2] as f32;
                    fg_cnt += 1.0;
                } else {
                    bg_sum[0] += p[0] as f32;
                    bg_sum[1] += p[1] as f32;
                    bg_sum[2] += p[2] as f32;
                    bg_cnt += 1.0;
                }
            }

            let fg_avg = [fg_sum[0] / fg_cnt, fg_sum[1] / fg_cnt, fg_sum[2] / fg_cnt];
            let bg_avg = [bg_sum[0] / bg_cnt, bg_sum[1] / bg_cnt, bg_sum[2] / bg_cnt];

            let mut err = 0.0f32;
            for (i, p) in px.iter().enumerate() {
                if (mask & (1 << i)) != 0 {
                    err += Self::color_dist_sq(*p, fg_avg);
                } else {
                    err += Self::color_dist_sq(*p, bg_avg);
                }
            }

            // Небольшой приоритет горизонтальному полублоку при равной ошибке
            if mask == 0b0011 {
                err *= 0.95;
            }

            if err < best_err {
                best_err = err;
                best_mask = mask;
                best_fg = [
                    fg_avg[0].round().clamp(0.0, 255.0) as u8,
                    fg_avg[1].round().clamp(0.0, 255.0) as u8,
                    fg_avg[2].round().clamp(0.0, 255.0) as u8,
                ];
                best_bg = [
                    bg_avg[0].round().clamp(0.0, 255.0) as u8,
                    bg_avg[1].round().clamp(0.0, 255.0) as u8,
                    bg_avg[2].round().clamp(0.0, 255.0) as u8,
                ];
            }
        }

        (
            QUADRANT_GLYPHS[best_mask],
            Color::Rgb(best_fg[0], best_fg[1], best_fg[2]),
            Color::Rgb(best_bg[0], best_bg[1], best_bg[2]),
        )
    }
}

impl Widget for &CoverArt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        let rgba = self.resized_quadrant(w as u32, h as u32);
        for cy in 0..h {
            let y = area.y + cy;
            let py = (cy * 2) as u32;
            for cx in 0..w {
                let x = area.x + cx;
                let px = (cx * 2) as u32;
                let tl = rgba[(px, py)];
                let tr = rgba[(px + 1, py)];
                let bl = rgba[(px, py + 1)];
                let br = rgba[(px + 1, py + 1)];
                let block = [
                    [tl[0], tl[1], tl[2]],
                    [tr[0], tr[1], tr[2]],
                    [bl[0], bl[1], bl[2]],
                    [br[0], br[1], br[2]],
                ];
                let (glyph, fg, bg) = CoverArt::best_quadrant_cell(block);
                let style = Style::default().fg(fg).bg(bg);
                buf.set_string(x, y, glyph, style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::style::Color;

    fn render_terminal(cover: &CoverArt, w: u16, h: u16) -> Terminal<TestBackend> {
        let backend = TestBackend::new(w, h);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(cover, frame.area());
            })
            .unwrap();
        terminal
    }

    fn cell_info(
        terminal: &Terminal<TestBackend>,
        x: u16,
        y: u16,
    ) -> (char, Option<Color>, Option<Color>) {
        let buf = terminal.backend().buffer();
        let cell = buf.cell((x, y)).unwrap();
        let style = cell.style();
        (
            cell.symbol().chars().next().unwrap_or(' '),
            style.fg,
            style.bg,
        )
    }

    #[test]
    fn renders_half_block() {
        let cover = CoverArt::from_image(RgbaImage::new(4, 4));
        let terminal = render_terminal(&cover, 2, 1);
        let (sym, fg, bg) = cell_info(&terminal, 0, 0);
        assert_eq!(sym, HALF_BLOCK);
        assert!(fg.is_some());
        assert!(bg.is_some());
    }

    #[test]
    fn top_and_bottom_pixels_drive_fg_bg() {
        let mut img = RgbaImage::new(1, 2);
        img[(0, 0)] = image::Rgba([255u8, 0, 0, 255]);
        img[(0, 1)] = image::Rgba([0, 0, 255u8, 255]);
        let cover = CoverArt::from_image(img);
        let terminal = render_terminal(&cover, 1, 1);
        let (sym, fg, bg) = cell_info(&terminal, 0, 0);
        assert_eq!(sym, HALF_BLOCK);
        assert_eq!(fg, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(bg, Some(Color::Rgb(0, 0, 255)));
    }

    #[test]
    fn two_columns_two_rows() {
        let mut img = RgbaImage::new(2, 2);
        img[(0, 0)] = image::Rgba([255u8, 0, 0, 255]);
        img[(1, 0)] = image::Rgba([0, 255u8, 0, 255]);
        img[(0, 1)] = image::Rgba([0, 0, 255u8, 255]);
        img[(1, 1)] = image::Rgba([255u8, 255, 0, 255]);
        let cover = CoverArt::from_image(img);
        let terminal = render_terminal(&cover, 2, 1);
        assert_eq!(cell_info(&terminal, 0, 0).1, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(cell_info(&terminal, 0, 0).2, Some(Color::Rgb(0, 0, 255)));
        assert_eq!(cell_info(&terminal, 1, 0).1, Some(Color::Rgb(0, 255, 0)));
        assert_eq!(cell_info(&terminal, 1, 0).2, Some(Color::Rgb(255, 255, 0)));
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let cover = CoverArt::from_image(RgbaImage::new(3, 3));
        let _ = render_terminal(&cover, 1, 1);
        let _ = render_terminal(&cover, 3, 2);
    }

    #[test]
    fn scales_to_fit() {
        let mut img = RgbaImage::new(2, 2);
        img[(0, 0)] = image::Rgba([255u8, 0, 0, 255]);
        img[(1, 0)] = image::Rgba([0, 0, 255u8, 255]);
        img[(0, 1)] = image::Rgba([0, 0, 0, 255]);
        img[(1, 1)] = image::Rgba([0, 255u8, 0, 255]);
        let cover = CoverArt::from_image(img);
        let terminal = render_terminal(&cover, 4, 1);
        for x in 0..4 {
            let (_, fg, _) = cell_info(&terminal, x, 0);
            assert!(fg.is_some());
        }
    }
}
