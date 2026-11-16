use std::io::Cursor;
use std::path::Path;

use image::{self, imageops, DynamicImage, RgbaImage};
use ratatui::prelude::*;
use ratatui::style::{Color, Style};

pub const HALF_BLOCK: &str = "\u{2580}";

/// Таблица сопоставления 6-битных масок (0..63) символам Unicode 13 Sextant (`\u{1FB00}` .. `\u{1FB3B}`),
/// плюс `\u{258C}` (left half) для маски 21, `\u{2590}` (right half) для маски 42,
/// пробел для 0 и `\u{2588}` (full block) для 63.
/// Сетка субпикселей 2x3:
/// [0] [1]
/// [2] [3]
/// [4] [5]
pub const SEXTANT_CHARS: [char; 64] = [
    ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎',
    '🬏', '🬐', '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝',
    '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬',
    '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CoverRenderMode {
    /// Секстантная сетка 2x3 (6 субпикселей на знакоместо — ровно в 3 раза детальнее полублока ▀)
    #[default]
    Sextant,
    /// Классический полублок ▀ (1x2 субпикселя на знакоместо)
    HalfBlock,
}

#[derive(Clone)]
pub struct CoverArt {
    img: DynamicImage,
    mode: CoverRenderMode,
}

impl std::fmt::Debug for CoverArt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoverArt({}x{}, {:?})",
            self.img.width(),
            self.img.height(),
            self.mode
        )
    }
}

impl CoverArt {
    /// Загружает обложку из файла на диске через `image::ImageReader`.
    #[allow(dead_code)]
    pub fn new(path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let img: DynamicImage = image::ImageReader::open(path)?.decode()?;
        Ok(Self {
            img,
            mode: CoverRenderMode::default(),
        })
    }

    /// Декодирует JPEG/PNG обложку из буфера памяти через `image::ImageReader`.
    pub fn from_bytes(bytes: &[u8]) -> image::ImageResult<Self> {
        let reader = image::ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        let img = reader.decode()?;
        Ok(Self {
            img,
            mode: CoverRenderMode::default(),
        })
    }

    pub fn from_image(img: RgbaImage) -> Self {
        Self {
            img: DynamicImage::from(img),
            mode: CoverRenderMode::default(),
        }
    }

    pub fn with_render_mode(mut self, mode: CoverRenderMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn set_render_mode(&mut self, mode: CoverRenderMode) {
        self.mode = mode;
    }

    pub fn render_mode(&self) -> CoverRenderMode {
        self.mode
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

    /// Ресайз изображения под классическую полублочную сетку 1x2 (верх/низ).
    pub fn resized_pixels_half_block(&self, w: u32, h: u32) -> RgbaImage {
        let pw = w.max(1);
        let ph = (h * 2).max(1);
        let filter = if self.img.width() == pw && self.img.height() == ph {
            imageops::FilterType::Nearest
        } else if self.img.width() < pw || self.img.height() < ph {
            imageops::FilterType::CatmullRom
        } else {
            imageops::FilterType::Lanczos3
        };
        self.img.resize_exact(pw, ph, filter).to_rgba8()
    }

    /// Ресайз изображения под секстантную сетку 2x3 (6 субпикселей на ячейку).
    pub fn resized_pixels_sextant(&self, w: u32, h: u32) -> RgbaImage {
        let pw = (w * 2).max(1);
        let ph = (h * 3).max(1);
        let filter = if self.img.width() == pw && self.img.height() == ph {
            imageops::FilterType::Nearest
        } else if self.img.width() < pw || self.img.height() < ph {
            imageops::FilterType::CatmullRom
        } else {
            imageops::FilterType::Lanczos3
        };
        self.img.resize_exact(pw, ph, filter).to_rgba8()
    }

    pub fn resized_pixels(&self, w: u32, h: u32) -> RgbaImage {
        match self.mode {
            CoverRenderMode::Sextant => self.resized_pixels_sextant(w, h),
            CoverRenderMode::HalfBlock => self.resized_pixels_half_block(w, h),
        }
    }

    fn render_half_block(&self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        let rgba = self.resized_pixels_half_block(w as u32, h as u32);
        for cy in 0..h {
            let y = area.y + cy;
            let py_top = (cy * 2) as u32;
            let py_bot = py_top + 1;
            for cx in 0..w {
                let x = area.x + cx;
                let px = cx as u32;
                let top = rgba[(px, py_top)];
                let bot = rgba[(px, py_bot)];

                let fg = Color::Rgb(top[0], top[1], top[2]);
                let bg = Color::Rgb(bot[0], bot[1], bot[2]);
                let style = Style::default().fg(fg).bg(bg);
                buf.set_string(x, y, HALF_BLOCK, style);
            }
        }
    }

    #[allow(clippy::manual_checked_ops)]
    fn render_sextant(&self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        let rgba = self.resized_pixels_sextant(w as u32, h as u32);

        for cy in 0..h {
            let y = area.y + cy;
            let py0 = (cy * 3) as u32;
            let py1 = py0 + 1;
            let py2 = py0 + 2;

            for cx in 0..w {
                let x = area.x + cx;
                let px0 = (cx * 2) as u32;
                let px1 = px0 + 1;

                let p0 = rgba[(px0, py0)];
                let p1 = rgba[(px1, py0)];
                let p2 = rgba[(px0, py1)];
                let p3 = rgba[(px1, py1)];
                let p4 = rgba[(px0, py2)];
                let p5 = rgba[(px1, py2)];
                let pixels = [p0, p1, p2, p3, p4, p5];

                // Находим пару пикселей с максимальным цветовым расстоянием
                let mut max_dist = -1i32;
                let mut best_pair = (0, 0);
                for i in 0..6 {
                    for j in (i + 1)..6 {
                        let dr = i32::from(pixels[i][0]) - i32::from(pixels[j][0]);
                        let dg = i32::from(pixels[i][1]) - i32::from(pixels[j][1]);
                        let db = i32::from(pixels[i][2]) - i32::from(pixels[j][2]);
                        let dist = dr * dr + dg * dg + db * db;
                        if dist > max_dist {
                            max_dist = dist;
                            best_pair = (i, j);
                        }
                    }
                }

                if max_dist <= 25 {
                    // Ячейка практически одного цвета
                    let r = (pixels.iter().map(|p| u32::from(p[0])).sum::<u32>() / 6) as u8;
                    let g = (pixels.iter().map(|p| u32::from(p[1])).sum::<u32>() / 6) as u8;
                    let b = (pixels.iter().map(|p| u32::from(p[2])).sum::<u32>() / 6) as u8;
                    let col = Color::Rgb(r, g, b);
                    buf.set_string(x, y, "█", Style::default().fg(col).bg(col));
                    continue;
                }

                // Упорядочиваем по яркости (светлее -> FG/1, темнее -> BG/0)
                let lum = |p: &image::Rgba<u8>| {
                    i32::from(p[0]) * 299 + i32::from(p[1]) * 587 + i32::from(p[2]) * 114
                };
                let (c_a, c_b) = if lum(&pixels[best_pair.0]) >= lum(&pixels[best_pair.1]) {
                    (pixels[best_pair.0], pixels[best_pair.1])
                } else {
                    (pixels[best_pair.1], pixels[best_pair.0])
                };

                let mut mask = 0u8;
                let mut sum_a = [0u32; 3];
                let mut count_a = 0u32;
                let mut sum_b = [0u32; 3];
                let mut count_b = 0u32;

                for (k, p) in pixels.iter().enumerate() {
                    let d_a = {
                        let dr = i32::from(p[0]) - i32::from(c_a[0]);
                        let dg = i32::from(p[1]) - i32::from(c_a[1]);
                        let db = i32::from(p[2]) - i32::from(c_a[2]);
                        dr * dr + dg * dg + db * db
                    };
                    let d_b = {
                        let dr = i32::from(p[0]) - i32::from(c_b[0]);
                        let dg = i32::from(p[1]) - i32::from(c_b[1]);
                        let db = i32::from(p[2]) - i32::from(c_b[2]);
                        dr * dr + dg * dg + db * db
                    };

                    if d_a <= d_b {
                        mask |= 1 << k;
                        sum_a[0] += u32::from(p[0]);
                        sum_a[1] += u32::from(p[1]);
                        sum_a[2] += u32::from(p[2]);
                        count_a += 1;
                    } else {
                        sum_b[0] += u32::from(p[0]);
                        sum_b[1] += u32::from(p[1]);
                        sum_b[2] += u32::from(p[2]);
                        count_b += 1;
                    }
                }

                let fg = if count_a > 0 {
                    Color::Rgb(
                        (sum_a[0] / count_a) as u8,
                        (sum_a[1] / count_a) as u8,
                        (sum_a[2] / count_a) as u8,
                    )
                } else {
                    Color::Rgb(c_a[0], c_a[1], c_a[2])
                };

                let bg = if count_b > 0 {
                    Color::Rgb(
                        (sum_b[0] / count_b) as u8,
                        (sum_b[1] / count_b) as u8,
                        (sum_b[2] / count_b) as u8,
                    )
                } else {
                    Color::Rgb(c_b[0], c_b[1], c_b[2])
                };

                let ch = SEXTANT_CHARS[mask as usize];
                let mut s = [0u8; 4];
                let str_ch = ch.encode_utf8(&mut s);
                buf.set_string(x, y, str_ch, Style::default().fg(fg).bg(bg));
            }
        }
    }
}

impl Widget for &CoverArt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        match self.mode {
            CoverRenderMode::Sextant => self.render_sextant(area, buf),
            CoverRenderMode::HalfBlock => self.render_half_block(area, buf),
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
        let cell = terminal
            .backend()
            .buffer()
            .cell((x, y))
            .expect("cell should exist");
        let sym = cell.symbol().chars().next().unwrap_or(' ');
        (sym, Some(cell.fg), Some(cell.bg))
    }

    #[test]
    fn renders_half_block() {
        let cover =
            CoverArt::from_image(RgbaImage::new(4, 4)).with_render_mode(CoverRenderMode::HalfBlock);
        let terminal = render_terminal(&cover, 2, 1);
        let (sym, fg, bg) = cell_info(&terminal, 0, 0);
        assert_eq!(sym, '▀');
        assert!(fg.is_some());
        assert!(bg.is_some());
    }

    #[test]
    fn renders_sextant_3x_detailed() {
        let mut img = RgbaImage::new(2, 3);
        // Верхние 2 субпикселя белые (bit 0 и 1 -> mask 3 -> U+1FB02 '🬂')
        img[(0, 0)] = image::Rgba([255, 255, 255, 255]);
        img[(1, 0)] = image::Rgba([255, 255, 255, 255]);
        // Нижние 4 субпикселя черные
        img[(0, 1)] = image::Rgba([0, 0, 0, 255]);
        img[(1, 1)] = image::Rgba([0, 0, 0, 255]);
        img[(0, 2)] = image::Rgba([0, 0, 0, 255]);
        img[(1, 2)] = image::Rgba([0, 0, 0, 255]);

        let cover = CoverArt::from_image(img).with_render_mode(CoverRenderMode::Sextant);
        let terminal = render_terminal(&cover, 1, 1);
        let (sym, fg, bg) = cell_info(&terminal, 0, 0);
        assert_eq!(sym, '🬂');
        assert_eq!(fg, Some(Color::Rgb(255, 255, 255)));
        assert_eq!(bg, Some(Color::Rgb(0, 0, 0)));
    }

    #[test]
    fn top_and_bottom_pixels_drive_fg_bg_in_half_block() {
        let mut img = RgbaImage::new(1, 2);
        img[(0, 0)] = image::Rgba([255u8, 0, 0, 255]);
        img[(0, 1)] = image::Rgba([0, 0, 255u8, 255]);
        let cover =
            CoverArt::from_image(img).with_render_mode(CoverRenderMode::HalfBlock);
        let terminal = render_terminal(&cover, 1, 1);
        let (sym, fg, bg) = cell_info(&terminal, 0, 0);
        assert_eq!(sym, '▀');
        assert_eq!(fg, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(bg, Some(Color::Rgb(0, 0, 255)));
    }

    #[test]
    fn two_columns_two_rows_half_block() {
        let mut img = RgbaImage::new(2, 2);
        img[(0, 0)] = image::Rgba([255u8, 0, 0, 255]);
        img[(1, 0)] = image::Rgba([0, 255u8, 0, 255]);
        img[(0, 1)] = image::Rgba([0, 0, 255u8, 255]);
        img[(1, 1)] = image::Rgba([255u8, 255, 0, 255]);
        let cover =
            CoverArt::from_image(img).with_render_mode(CoverRenderMode::HalfBlock);
        let terminal = render_terminal(&cover, 2, 1);
        assert_eq!(cell_info(&terminal, 0, 0).1, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(cell_info(&terminal, 0, 0).2, Some(Color::Rgb(0, 0, 255)));
        assert_eq!(cell_info(&terminal, 1, 0).1, Some(Color::Rgb(0, 255, 0)));
        assert_eq!(cell_info(&terminal, 1, 0).2, Some(Color::Rgb(255, 255, 0)));
    }

    #[test]
    fn scales_to_fit() {
        let cover = CoverArt::from_image(RgbaImage::new(10, 10));
        let terminal = render_terminal(&cover, 5, 2);
        let (sym, _, _) = cell_info(&terminal, 4, 1);
        assert!(sym != ' ');
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let cover = CoverArt::from_image(RgbaImage::new(10, 10));
        let _ = render_terminal(&cover, 0, 0);
        let _ = render_terminal(&cover, 1, 1);
    }
}
