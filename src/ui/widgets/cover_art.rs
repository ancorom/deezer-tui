use std::io::Cursor;
use std::path::Path;

use image::{self, imageops, DynamicImage, RgbaImage};
use ratatui::prelude::*;
use ratatui::style::{Color, Style};

#[allow(dead_code)]
const HALF_BLOCK: char = '\u{2580}';

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
        let mut rgba = RgbaImage::new(16, 16);
        for y in 0..16 {
            for x in 0..16 {
                let mix = ((x + y) as f32) / 30.0;
                let nr = (r as f32 * (1.0 - mix * 0.4) + 127.0 * (mix * 0.4)) as u8;
                let ng = (g as f32 * (1.0 - mix * 0.5) + 90.0 * (mix * 0.5)) as u8;
                let nb = (b as f32 * (1.0 - mix * 0.3) + 240.0 * (mix * 0.3)) as u8;
                rgba[(x, y)] = image::Rgba([nr, ng, nb, 255u8]);
            }
        }
        Self::from_image(rgba)
    }

    fn resized_for(&self, w: u32, h: u32) -> RgbaImage {
        let pw = w.max(1);
        let ph = (h * 2).max(1);
        self.img
            .resize_exact(pw, ph, imageops::FilterType::Lanczos3)
            .to_rgba8()
    }
}

impl Widget for &CoverArt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w == 0 || h == 0 {
            return;
        }
        let rgba = self.resized_for(w as u32, h as u32);
        for cy in 0..h {
            let y = area.y + cy;
            let p_top = (2 * cy) as u32;
            for cx in 0..w {
                let x = area.x + cx;
                let top = &rgba[(cx as u32, p_top)];
                let bot = &rgba[(cx as u32, p_top + 1)];
                let style = Style::default()
                    .fg(Color::Rgb(top[0], top[1], top[2]))
                    .bg(Color::Rgb(bot[0], bot[1], bot[2]));
                buf.set_string(x, y, "▀", style);
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
        // 2-wide x 2-tall pixel image with a distinct colour per pixel:
        // (0,0)=red (top-left), (1,0)=green (top-right),
        // (0,1)=blue (bottom-left), (1,1)=yellow (bottom-right).
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
        // 2x2 image rendered into a 4-wide x 1-tall area (4x2 pixels) -> upsampled.
        let mut img = RgbaImage::new(2, 2);
        img[(0, 0)] = image::Rgba([255u8, 0, 0, 255]);
        img[(1, 0)] = image::Rgba([0, 0, 255u8, 255]);
        img[(0, 1)] = image::Rgba([0, 0, 0, 255]);
        img[(1, 1)] = image::Rgba([0, 255u8, 0, 255]);
        let cover = CoverArt::from_image(img);
        let terminal = render_terminal(&cover, 4, 1);
        for x in 0..4 {
            let (sym, fg, _) = cell_info(&terminal, x, 0);
            assert_eq!(sym, HALF_BLOCK);
            assert!(fg.is_some());
        }
    }
}
