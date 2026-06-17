/// ratatui Widget wrapping the render pipeline.
///
/// Follows ratatui's builder/borrow pattern: the widget borrows data
/// from the caller and renders in a single pass. This is how Sparkline
/// works — the widget never owns the data buffer.

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Widget};

use crate::gradient::ColorGradient;
use crate::render;
use crate::symbols::SymbolSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Newest data on the right (btop default).
    LeftToRight,
    /// Newest data on the left (macmon default).
    RightToLeft,
}

impl Default for Direction {
    fn default() -> Self {
        Self::RightToLeft
    }
}

pub struct ColorGraph<'a> {
    data: &'a [f64],
    max_value: f64,
    gradient: ColorGradient,
    symbol_set: SymbolSet,
    direction: Direction,
    invert: bool,
    no_zero: bool,
    block: Option<Block<'a>>,
}

impl Default for ColorGraph<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ColorGraph<'a> {
    pub fn new() -> Self {
        Self {
            data: &[],
            max_value: 100.0,
            gradient: ColorGradient::default(),
            symbol_set: SymbolSet::default(),
            direction: Direction::default(),
            invert: false,
            no_zero: true,
            block: None,
        }
    }

    pub fn data(mut self, data: &'a [f64]) -> Self {
        self.data = data;
        self
    }

    pub fn max_value(mut self, max: f64) -> Self {
        self.max_value = max;
        self
    }

    pub fn gradient(mut self, gradient: ColorGradient) -> Self {
        self.gradient = gradient;
        self
    }

    pub fn symbol_set(mut self, s: SymbolSet) -> Self {
        self.symbol_set = s;
        self
    }

    pub fn direction(mut self, d: Direction) -> Self {
        self.direction = d;
        self
    }

    pub fn invert(mut self, b: bool) -> Self {
        self.invert = b;
        self
    }

    pub fn no_zero(mut self, b: bool) -> Self {
        self.no_zero = b;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Widget for ColorGraph<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the optional border/title block, get inner area for the graph
        let graph_area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else {
            area
        };

        if graph_area.is_empty() || self.data.is_empty() {
            return;
        }

        let width = graph_area.width as usize;
        let height = graph_area.height as usize;
        let max_val = if self.max_value > 0.0 { self.max_value } else { 1.0 };

        // Take at most width+1 data points (produces width characters via overlapping windows)
        let max_points = width + 1;
        let data_len = self.data.len().min(max_points);
        let data_slice = &self.data[..data_len];

        // Normalize raw data → 0.0..=100.0
        let mut normalized: Vec<f64> = data_slice
            .iter()
            .map(|&v| (v * 100.0 / max_val).clamp(0.0, 100.0))
            .collect();

        // Direction: data[0] = newest.
        // RTL → iterate in order (newest = leftmost character). Already correct.
        // LTR → reverse so oldest = leftmost, newest = rightmost.
        if self.direction == Direction::LeftToRight {
            normalized.reverse();
        }

        let symbols = self.symbol_set.table(self.invert);
        let output = render::render_graph(
            &normalized,
            height,
            symbols,
            &self.gradient,
            self.invert,
            self.no_zero,
        );

        // Write rendered cells to the ratatui buffer
        for (row_idx, row) in output.rows.iter().enumerate() {
            // Pad alignment: RTL left-aligns, LTR right-aligns
            let x_offset = if self.direction == Direction::LeftToRight {
                width.saturating_sub(row.len()) as u16
            } else {
                0
            };

            for (col_idx, &(ch, (r, g, b))) in row.iter().enumerate() {
                let x = graph_area.x + x_offset + col_idx as u16;
                let y = graph_area.y + row_idx as u16;

                if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
                    if ch != ' ' {
                        cell.set_char(ch)
                            .set_style(Style::new().fg(Color::Rgb(r, g, b)));
                    }
                }
            }
        }
    }
}
