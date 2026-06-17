/// Core rendering pipeline, ported from btop_draw.cpp `Graph::_create`.
///
/// Takes normalized data (0.0–100.0) and produces a 2D grid of
/// (character, rgb_color) pairs. The widget layer writes these to a
/// ratatui Buffer.

use crate::gradient::ColorGradient;

pub(crate) struct RenderOutput {
    /// Rows top-to-bottom, each containing (char, rgb) pairs left-to-right.
    pub rows: Vec<Vec<(char, (u8, u8, u8))>>,
}

/// Render a graph from normalized data.
///
/// # Arguments
/// - `data` — values in 0.0..=100.0, ordered for left-to-right display
///   (index 0 appears as the leftmost character's left half).
/// - `height` — number of terminal rows.
/// - `symbols` — 25-entry character lookup table from `SymbolSet::table()`.
/// - `gradient` — pre-computed color gradient.
/// - `invert` — flip the graph vertically.
/// - `no_zero` — ensure non-zero values produce at least one dot in the bottom row.
pub(crate) fn render_graph(
    data: &[f64],
    height: usize,
    symbols: &[char; 25],
    gradient: &ColorGradient,
    invert: bool,
    no_zero: bool,
) -> RenderOutput {
    if height == 0 || data.len() < 2 {
        return RenderOutput { rows: vec![] };
    }

    // btop adds a small boost to prevent invisible low-but-nonzero values
    let boost = if height == 1 { 0.3 } else { 0.1 };
    let num_chars = data.len() - 1;
    let mut rows = Vec::with_capacity(height);

    for row in 0..height {
        let (cur_high, cur_low) = if height > 1 {
            (
                (100.0 * (height - row) as f64 / height as f64).round(),
                (100.0 * (height - row - 1) as f64 / height as f64).round(),
            )
        } else {
            (100.0, 0.0)
        };

        // no_zero only applies to the bottom row (btop: `horizon == height - 1`)
        let is_bottom = row == height - 1;
        let clamp_min: u8 = if no_zero && is_bottom { 1 } else { 0 };

        // Multi-row: color per row (top=hot, bottom=cool). Single-row: per character.
        let row_color = if height > 1 {
            let pct = if invert {
                ((row + 1) as f64 * 100.0 / height as f64).round() as u8
            } else {
                (100.0 - row as f64 * 100.0 / height as f64).round() as u8
            };
            Some(gradient.at(pct))
        } else {
            None
        };

        let mut row_cells = Vec::with_capacity(num_chars);

        // Overlapping windows: each character encodes (previous_value, current_value).
        // This is btop's smooth-transition trick — NOT stride-2 pairs.
        let mut last = data[0];
        for &value in &data[1..] {
            let q_last = quantize(last, cur_low, cur_high, boost, clamp_min);
            let q_curr = quantize(value, cur_low, cur_high, boost, clamp_min);

            let ch = symbols[q_last as usize * 5 + q_curr as usize];

            let color = row_color.unwrap_or_else(|| {
                gradient.at(last.max(value).clamp(0.0, 100.0) as u8)
            });

            row_cells.push((ch, color));
            last = value;
        }

        rows.push(row_cells);
    }

    // Invert: reverse row order so the bottom row appears on top
    if invert {
        rows.reverse();
    }

    RenderOutput { rows }
}

/// Quantize a 0–100 value to 0–4 within a row's band, with btop's boost.
fn quantize(value: f64, low: f64, high: f64, boost: f64, clamp_min: u8) -> u8 {
    if value >= high {
        return 4;
    }
    if value <= low {
        return clamp_min;
    }
    let range = high - low;
    if range <= 0.0 {
        return clamp_min;
    }
    let q = ((value - low) * 4.0 / range + boost).round() as i32;
    q.clamp(clamp_min as i32, 4) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::GradientTheme;
    use crate::symbols::SymbolSet;

    #[test]
    fn quantize_edges() {
        assert_eq!(quantize(0.0, 0.0, 100.0, 0.0, 0), 0);
        assert_eq!(quantize(100.0, 0.0, 100.0, 0.0, 0), 4);
        assert_eq!(quantize(50.0, 0.0, 100.0, 0.0, 0), 2);
    }

    #[test]
    fn quantize_with_boost() {
        // Without boost: 5.0 in 0-100 → 5*4/100 = 0.2 → rounds to 0
        assert_eq!(quantize(5.0, 0.0, 100.0, 0.0, 0), 0);
        // With boost 0.3: 0.2 + 0.3 = 0.5 → rounds to 1
        assert_eq!(quantize(5.0, 0.0, 100.0, 0.3, 0), 1);
    }

    #[test]
    fn quantize_no_zero() {
        // Value above low but clamp_min=1 forces minimum of 1
        assert_eq!(quantize(1.0, 0.0, 100.0, 0.0, 1), 1);
        // Value exactly at low still gets clamp_min
        assert_eq!(quantize(0.0, 0.0, 100.0, 0.0, 1), 1);
    }

    #[test]
    fn render_basic() {
        let data = vec![0.0, 25.0, 50.0, 75.0, 100.0];
        let gradient = ColorGradient::from_theme(GradientTheme::Cpu);
        let symbols = SymbolSet::Braille.table(false);
        let output = render_graph(&data, 1, symbols, &gradient, false, false);

        assert_eq!(output.rows.len(), 1);
        assert_eq!(output.rows[0].len(), 4); // 5 data points → 4 characters
    }

    #[test]
    fn render_multirow() {
        let data = vec![0.0, 50.0, 100.0, 50.0, 0.0];
        let gradient = ColorGradient::from_theme(GradientTheme::Cpu);
        let symbols = SymbolSet::Braille.table(false);
        let output = render_graph(&data, 3, symbols, &gradient, false, true);

        assert_eq!(output.rows.len(), 3);
        for row in &output.rows {
            assert_eq!(row.len(), 4);
        }
    }

    #[test]
    fn render_empty_data() {
        let gradient = ColorGradient::from_theme(GradientTheme::Cpu);
        let symbols = SymbolSet::Braille.table(false);
        let output = render_graph(&[], 3, symbols, &gradient, false, false);
        assert!(output.rows.is_empty());

        let output = render_graph(&[50.0], 3, symbols, &gradient, false, false);
        assert!(output.rows.is_empty());
    }
}
