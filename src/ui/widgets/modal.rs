//! A reusable modal widget: a bordered box with a title and a list of rows,
//! one of which is highlighted as the current selection. Used by the library
//! screen for the "CHOOSE PLAYLIST" dialog and by any other screen for a small
//! interactive popup.
//!
//! The layout is adaptive: on a short or narrow terminal the box shrinks and
//! the list is truncated to the rows that fit.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::ui::theme;

/// A single selectable row inside the modal.
#[derive(Debug, Clone)]
pub struct ModalRow {
    /// Text of this row.
    pub text: String,
    /// `true` when this row is the current selection (highlighted).
    pub selected: bool,
}

/// A modal popup: title + a list of selectable rows inside a bordered box.
#[derive(Debug, Default)]
pub struct Modal {
    /// Modal title (top bar of the box).
    pub title: String,
    /// Rows to display (truncated to fit the area at render time).
    pub rows: Vec<ModalRow>,
}

impl Modal {
    /// Build a modal from a title and a list of row strings; row `selected`
    /// is marked as highlighted.
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

    /// Style for a selected row (inverted gradient background).
    fn selected_style() -> Style {
        Style::default()
            .fg(theme::color_from_hex(theme::BG_TERMINAL))
            .bg(theme::color_from_hex(theme::GRAD_END))
    }

    /// Style for a normal row.
    fn row_style() -> Style {
        Style::default().fg(theme::color_from_hex(theme::TEXT_PRIMARY))
    }

    /// Style for the modal title (dim, as the "chrome").
    fn title_style() -> Style {
        Style::default().fg(theme::color_from_hex(theme::TEXT_SECONDARY))
    }

    /// Style for the box border.
    fn border_style() -> Style {
        Style::default().fg(theme::color_from_hex(theme::GRAD_START))
    }

    /// Background style for the cells (matches the card background).
    fn cell_style() -> Style {
        Style::default().bg(theme::color_from_hex(theme::BG_CARD))
    }

    /// Truncate `text` to `max_width` cells.
    fn truncate(text: &str, max_width: usize) -> String {
        if max_width == 0 {
            return String::new();
        }
        text.chars()
            .take(max_width)
            .collect::<String>()
    }
}

impl Widget for &Modal {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width;
        let h = area.height;
        if w < 3 || h < 3 {
            return;
        }
        let x0 = area.x;
        let y0 = area.y;
        let border_style = Modal::border_style();
        let cell_style = Modal::cell_style();

        // Top border row (y0).
        buf.set_string(x0, y0, "╔", border_style);
        for x in 1..w - 1 {
            buf.set_string(x0 + x, y0, "═", border_style);
        }
        buf.set_string(x0 + w - 1, y0, "╗", border_style);
        // Bottom border row (y0 + h - 1).
        buf.set_string(x0, y0 + h - 1, "╚", border_style);
        for x in 1..w - 1 {
            buf.set_string(x0 + x, y0 + h - 1, "═", border_style);
        }
        buf.set_string(x0 + w - 1, y0 + h - 1, "╝", border_style);

        // Title row (y0 + 1) — available when h >= 3 (y0 + 1 < y0 + h - 1).
        if h >= 4 {
            let y = y0 + 1;
            buf.set_string(x0, y, "║", border_style);
            buf.set_string(x0 + w - 1, y, "║", border_style);
            for x in 1..w - 1 {
                buf.set_string(x0 + x, y, " ", cell_style);
            }
            let max_w = (w - 3) as usize;
            buf.set_string(
                x0 + 1,
                y,
                Modal::truncate(&self.title, max_w).as_str(),
                Modal::title_style(),
            );
        }
        // List rows (y0 + 2 ..= y0 + h - 2) — available when h >= 4.
        if h >= 4 {
            let list_start = y0 + 2;
            let max_w = (w - 3) as usize;
            for (i, row) in self.rows.iter().enumerate() {
                let y = list_start + i as u16;
                if y >= y0 + h - 1 {
                    break; // do not overwrite the bottom border row.
                }
                buf.set_string(x0, y, "║", border_style);
                buf.set_string(x0 + w - 1, y, "║", border_style);
                for x in 1..w - 1 {
                    buf.set_string(x0 + x, y, " ", cell_style);
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

    /// Render a modal to a `TestBackend` and return the terminal.
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

    /// Symbol at (x, y).
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
        (0..width)
            .any(|x| at(terminal, x, y) == needle.chars().next().unwrap_or(' '))
            || {
                let row: String = (0..width).map(|x| at(terminal, x, y)).collect();
                row.contains(needle)
            }
    }

    #[test]
    fn renders_box_in_normal_size() {
        let modal = Modal::new(
            "Choose Playlist",
            vec!["Playlists", " 1"],
            0,
        );
        let terminal = render(&modal, 120, 40);
        // Top corners.
        assert_eq!(at(&terminal, 0, 0), '╔');
        assert_eq!(at(&terminal, 119, 0), '╗');
        // Bottom corners.
        assert_eq!(at(&terminal, 0, 39), '╚');
        assert_eq!(at(&terminal, 119, 39), '╝');
        // Title on the first inner row.
        assert!(row_has(&terminal, 1, "Choose"));
    }

    #[test]
    fn renders_on_narrow_size() {
        let modal = Modal::new("Choose", vec!["a", "b"], 1);
        let terminal = render(&modal, 40, 12);
        // No panic; corners drawn.
        assert_eq!(at(&terminal, 0, 0), '╔');
        assert_eq!(at(&terminal, 39, 0), '╗');
        assert_eq!(at(&terminal, 0, 11), '╚');
        assert_eq!(at(&terminal, 39, 11), '╝');
        // Title present.
        assert!(row_has(&terminal, 1, "Cho"));
    }

    #[test]
    fn tiny_area_does_not_panic() {
        let modal = Modal::new("T", vec!["x"], 0);
        let terminal = render(&modal, 4, 3);
        assert_eq!(at(&terminal, 0, 0), '╔');
    }
}