/// Btop's 5×5 symbol encoding scheme.
///
/// Each character encodes two consecutive data values, each quantized to 0–4.
/// Index = `left_value * 5 + right_value`, where "left" is the older value
/// and "right" is the newer value in time order.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSet {
    /// Braille dot patterns — smoothest, 4×2 dot resolution per cell.
    Braille,
    /// Half-block characters — simpler, 2× vertical resolution.
    Block,
    /// Shade characters (░▒▓█) — TTY-safe, widest terminal support.
    TTY,
}

impl Default for SymbolSet {
    fn default() -> Self {
        Self::Braille
    }
}

impl SymbolSet {
    pub fn table(&self, invert: bool) -> &'static [char; 25] {
        match (self, invert) {
            (Self::Braille, false) => &BRAILLE_UP,
            (Self::Braille, true) => &BRAILLE_DOWN,
            (Self::Block, false) => &BLOCK_UP,
            (Self::Block, true) => &BLOCK_DOWN,
            (Self::TTY, _) => &TTY,
        }
    }
}

// Tables ported from btop_draw.cpp `Symbols::graph_symbols`.
// Index = first_value * 5 + second_value, each in 0..=4.
// First value → left column of character, second → right column.

const BRAILLE_UP: [char; 25] = [
    ' ', '⢀', '⢠', '⢰', '⢸',
    '⡀', '⣀', '⣠', '⣰', '⣸',
    '⡄', '⣄', '⣤', '⣴', '⣼',
    '⡆', '⣆', '⣦', '⣶', '⣾',
    '⡇', '⣇', '⣧', '⣷', '⣿',
];

const BRAILLE_DOWN: [char; 25] = [
    ' ', '⠈', '⠘', '⠸', '⢸',
    '⠁', '⠉', '⠙', '⠹', '⢹',
    '⠃', '⠋', '⠛', '⠻', '⢻',
    '⠇', '⠏', '⠟', '⠿', '⢿',
    '⡇', '⡏', '⡟', '⡿', '⣿',
];

const BLOCK_UP: [char; 25] = [
    ' ', '▗', '▗', '▐', '▐',
    '▖', '▄', '▄', '▟', '▟',
    '▖', '▄', '▄', '▟', '▟',
    '▌', '▙', '▙', '█', '█',
    '▌', '▙', '▙', '█', '█',
];

const BLOCK_DOWN: [char; 25] = [
    ' ', '▝', '▝', '▐', '▐',
    '▘', '▀', '▀', '▜', '▜',
    '▘', '▀', '▀', '▜', '▜',
    '▌', '▛', '▛', '█', '█',
    '▌', '▛', '▛', '█', '█',
];

const TTY: [char; 25] = [
    ' ', '░', '░', '▒', '▒',
    '░', '░', '▒', '▒', '█',
    '░', '▒', '▒', '▒', '█',
    '▒', '▒', '▒', '█', '█',
    '▒', '█', '█', '█', '█',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn braille_corners() {
        assert_eq!(BRAILLE_UP[0], ' ');       // both zero
        assert_eq!(BRAILLE_UP[24], '⣿');      // both max (4*5+4)
        assert_eq!(BRAILLE_UP[4], '⢸');       // left=0, right=4
        assert_eq!(BRAILLE_UP[20], '⡇');      // left=4, right=0
    }

    #[test]
    fn table_selection() {
        let t = SymbolSet::Braille.table(false);
        assert_eq!(t[0], ' ');
        let t = SymbolSet::Braille.table(true);
        assert_eq!(t[0], ' ');
        assert_ne!(t[1], BRAILLE_UP[1]); // inverted uses different chars
    }
}
