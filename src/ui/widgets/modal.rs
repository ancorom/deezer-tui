//! Модальное всплывающее окно (`CHOOSE PLAYLIST`) с очисткой подложки через `ratatui::widgets::Clear`.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Clear, Widget};

use crate::ui::theme;

#[derive(Debug, Clone)]
pub struct ModalRow {
    pub text: String,
    pub selected: bool,
}

#[derive(Debug, Default)]
pub struct Modal {
    pub title: String,
    pub rows: Vec<ModalRow>,
}

impl Modal {
    pub fn new(
        title: impl Into<String>,
        rows: impl IntoIterator<Item = impl Into<String>>,
        selected: usize,
    ) -> Self {
        Self {
            title: title.into(),
            rows: rows
                .into_iter()
                .enumerate()
                .map(|(i, t)| ModalRow {
                    text: t.into(),
                    selected: i == selected,
                })
                .collect(),
        }
    }

    fn selected_style() -> Style {
        Style::default()
            .fg(theme::color_from_hex(theme::BG_TERMINAL))
            .bg(theme::color_from_hex(theme::GRAD_END))
    }

    fn row_style() -> Style {
        Style::default()
            .fg(theme::color_from_hex(theme::TEXT_PRIMARY))
            .bg(theme::color_from_hex(theme::BG_CARD))
    }

    fn title_style() -> Style {
        Style::default()
            .fg(theme::color_from_hex(theme::TEXT_SECONDARY))
            .bg(theme::color_from_hex(theme::BG_CARD))
    }

    fn border_style() -> Style {
        Style::default()
            .fg(theme::color_from_hex(theme::GRAD_START))
            .bg(theme::color_from_hex(theme::BG_CARD))
    }

    fn cell_style() -> Style {
        Style::default().bg(theme::color_from_hex(theme::BG_CARD))
    }

    fn truncate(text: &str, max_width: usize) -> String {
        text.chars().take(max_width).collect()
    }
}

impl Widget for &Modal {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 3 || h < 3 {
            return;
        }
        // Очищаем область под модальным окном через ratatui::widgets::Clear
        Clear.render(area, buf);

        let x0 = area.x;
        let y0 = area.y;
        let border_style = Modal::border_style();
        let cell_style = Modal::cell_style();

        buf.set_string(x0, y0, "╔", border_style);
        for x in 1..w - 1 {
            buf.set_string(x0 + x, y0, "═", border_style);
        }
        buf.set_string(x0 + w - 1, y0, "╗", border_style);

        buf.set_string(x0, y0 + h - 1, "╚", border_style);
        for x in 1..w - 1 {
            buf.set_string(x0 + x, y0 + h - 1, "═", border_style);
        }
        buf.set_string(x0 + w - 1, y0 + h - 1, "╝", border_style);

        for y in (y0 + 1)..(y0 + h - 1) {
            buf.set_string(x0, y, "║", border_style);
            buf.set_string(x0 + w - 1, y, "║", border_style);
            for x in 1..w - 1 {
                buf.set_string(x0 + x, y, " ", cell_style);
            }
        }

        if h >= 4 {
            let max_w = (w - 2) as usize;
            buf.set_string(
                x0 + 1,
                y0 + 1,
                Modal::truncate(&self.title, max_w).as_str(),
                Modal::title_style(),
            );

            let list_start = y0 + 2;
            for (i, row) in self.rows.iter().enumerate() {
                let y = list_start + i as u16;
                if y >= y0 + h - 1 {
                    break;
                }
                let row_style = if row.selected {
                    Modal::selected_style()
                } else {
                    Modal::row_style()
                };
                buf.set_string(
                    x0 + 1,
                    y,
                    Modal::truncate(&row.text, max_w).as_str(),
                    row_style,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn render(modal: &Modal, width: u16, height: u16) -> Terminal<TestBackend> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(modal, frame.area());
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
    fn renders_box_in_normal_size() {
        let modal = Modal::new("Choose Playlist", vec!["Playlists", " 1"], 0);
        let terminal = render(&modal, 120, 40);
        assert_eq!(at(&terminal, 0, 0), '╔');
        assert_eq!(at(&terminal, 119, 0), '╗');
        assert_eq!(at(&terminal, 0, 39), '╚');
        assert_eq!(at(&terminal, 119, 39), '╝');
        assert!(row_has(&terminal, 1, "Choose"));
    }

    #[test]
    fn renders_on_narrow_size() {
        let modal = Modal::new("Choose", vec!["a", "b"], 1);
        let terminal = render(&modal, 40, 12);
        assert_eq!(at(&terminal, 0, 0), '╔');
        assert_eq!(at(&terminal, 39, 0), '╗');
        assert_eq!(at(&terminal, 0, 11), '╚');
        assert_eq!(at(&terminal, 39, 11), '╝');
        assert!(row_has(&terminal, 1, "Cho"));
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let modal = Modal::new("T", vec!["x"], 0);
        let terminal = render(&modal, 4, 3);
        assert_eq!(at(&terminal, 0, 0), '╔');
    }
}
