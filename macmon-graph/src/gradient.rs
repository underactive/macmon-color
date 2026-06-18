/// 101-stop color gradient, ported from btop_theme.cpp `generateGradients`.
///
/// Pre-computed at creation time: each entry maps a 0–100 percentage to an RGB
/// color. At render time, color lookup is a single array index.

#[derive(Debug, Clone)]
pub struct ColorGradient {
    stops: [(u8, u8, u8); 101],
}

impl Default for ColorGradient {
    fn default() -> Self {
        Self::from_theme(GradientTheme::Cpu)
    }
}

impl ColorGradient {
    pub fn new(start: (u8, u8, u8), mid: Option<(u8, u8, u8)>, end: (u8, u8, u8)) -> Self {
        let mut stops = [(0u8, 0u8, 0u8); 101];

        if let Some(mid) = mid {
            for i in 0..=50 {
                stops[i] = lerp_rgb(start, mid, i as f64 / 50.0);
            }
            for i in 51..=100 {
                stops[i] = lerp_rgb(mid, end, (i - 50) as f64 / 50.0);
            }
        } else {
            for i in 0..=100 {
                stops[i] = lerp_rgb(start, end, i as f64 / 100.0);
            }
        }

        Self { stops }
    }

    pub fn from_theme(theme: GradientTheme) -> Self {
        Self::from_palette(theme, ColorPalette::Btop)
    }

    pub fn from_palette(theme: GradientTheme, palette: ColorPalette) -> Self {
        let (start, mid, end) = palette.colors(theme);
        Self::new(start, Some(mid), end)
    }

    #[inline]
    pub fn at(&self, value: u8) -> (u8, u8, u8) {
        self.stops[value.min(100) as usize]
    }
}

fn lerp_rgb(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    (
        (a.0 as f64 + (b.0 as f64 - a.0 as f64) * t).round() as u8,
        (a.1 as f64 + (b.1 as f64 - a.1 as f64) * t).round() as u8,
        (a.2 as f64 + (b.2 as f64 - a.2 as f64) * t).round() as u8,
    )
}

/// Graph color palette — cycles with the T key in macmon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPalette {
    /// btop's default theme gradients.
    #[default]
    Btop,
    /// 80's synthwave: neon cyan, magenta, and sunset tones.
    Synthwave,
    /// CRT terminal: phosphor green, amber, and alert red.
    Hacker,
    /// Early Mac OS X: Bondi blue, sky aqua, and pearl highlights.
    Aqua,
}

impl ColorPalette {
    pub fn colors(self, theme: GradientTheme) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        match self {
            Self::Btop => theme.btop_colors(),
            Self::Synthwave => theme.synthwave_colors(),
            Self::Hacker => theme.hacker_colors(),
            Self::Aqua => theme.aqua_colors(),
        }
    }
}

/// Predefined gradient themes from btop's Default_theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientTheme {
    /// Green → Yellow → Red
    Cpu,
    /// Blue → Purple → Pink
    Temp,
    /// Dark green → Bright green → Bright yellow
    Free,
    /// Dark blue → Cyan → Bright blue
    Cached,
    /// Dark yellow → Warm → Amber
    Available,
    /// Dark red → Rose → Bright red
    Used,
    /// Deep blue → Medium blue → Light blue
    Download,
    /// Dark purple → Medium purple → Light purple
    Upload,
    /// Green → Yellow → Red (process variant)
    Process,
}

impl GradientTheme {
    /// Returns (start, mid, end) RGB tuples from btop's hex theme values.
    pub fn colors(self) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        self.btop_colors()
    }

    fn btop_colors(self) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        match self {
            Self::Cpu => ((0x77, 0xca, 0x9b), (0xcb, 0xc0, 0x6c), (0xdc, 0x4c, 0x4c)),
            Self::Temp => ((0x48, 0x97, 0xd4), (0x54, 0x74, 0xe8), (0xff, 0x40, 0xb6)),
            Self::Free => ((0x38, 0x4f, 0x21), (0xb5, 0xe6, 0x85), (0xdc, 0xff, 0x85)),
            Self::Cached => ((0x16, 0x33, 0x50), (0x74, 0xe6, 0xfc), (0x26, 0xc5, 0xff)),
            Self::Available => ((0x4e, 0x3f, 0x0e), (0xff, 0xd7, 0x7a), (0xff, 0xb8, 0x14)),
            Self::Used => ((0x59, 0x2b, 0x26), (0xd9, 0x62, 0x6d), (0xff, 0x47, 0x69)),
            Self::Download => ((0x29, 0x1f, 0x75), (0x4f, 0x43, 0xa3), (0xb0, 0xa9, 0xde)),
            Self::Upload => ((0x62, 0x06, 0x65), (0x7d, 0x41, 0x80), (0xdc, 0xaf, 0xde)),
            Self::Process => ((0x80, 0xd0, 0xa3), (0xdc, 0xd1, 0x79), (0xd4, 0x54, 0x54)),
        }
    }

    fn synthwave_colors(self) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        match self {
            // Cyan → magenta → hot pink
            Self::Cpu => ((0x05, 0xd9, 0xe8), (0xd3, 0x00, 0xc5), (0xff, 0x2a, 0x6d)),
            // Deep purple → neon magenta → cyan
            Self::Temp => ((0x2b, 0x0b, 0x3f), (0xff, 0x00, 0xff), (0x05, 0xd9, 0xe8)),
            // Dark violet → electric cyan → neon pink
            Self::Free => ((0x1a, 0x05, 0x33), (0x00, 0xf0, 0xff), (0xff, 0x6e, 0xc7)),
            // Deep indigo → electric blue → bright cyan
            Self::Cached => ((0x24, 0x00, 0x46), (0x43, 0x61, 0xee), (0x4c, 0xc9, 0xf0)),
            // Magenta dusk → sunset orange → hot yellow
            Self::Available => ((0x5c, 0x00, 0x4d), (0xff, 0x6b, 0x35), (0xff, 0xd6, 0x0a)),
            // Dark plum → hot pink → neon red
            Self::Used => ((0x3d, 0x0c, 0x44), (0xff, 0x00, 0x6e), (0xff, 0x0a, 0x54)),
            // Midnight purple → violet → sky cyan
            Self::Download => ((0x10, 0x00, 0x2b), (0x7b, 0x2c, 0xbf), (0x00, 0xb4, 0xd8)),
            // Dark magenta → hot pink → mint cyan
            Self::Upload => ((0x4a, 0x04, 0x4e), (0xff, 0x0a, 0x81), (0x72, 0xef, 0xdd)),
            // Neon teal → magenta → hot pink
            Self::Process => ((0x0a, 0xff, 0x99), (0xd9, 0x00, 0xff), (0xff, 0x2a, 0x6d)),
        }
    }

    fn hacker_colors(self) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        match self {
            // Dark green → terminal green → matrix phosphor
            Self::Cpu => ((0x00, 0x2b, 0x00), (0x00, 0xb3, 0x00), (0x39, 0xff, 0x14)),
            // Deep amber → CRT orange → hot phosphor
            Self::Temp => ((0x1a, 0x12, 0x00), (0xcc, 0x77, 0x00), (0xff, 0xcc, 0x00)),
            // Shadow green → mid green → bright free-space glow
            Self::Free => ((0x00, 0x1a, 0x0a), (0x00, 0x99, 0x44), (0x00, 0xff, 0x88)),
            // Dark teal → data cyan → bright stream
            Self::Cached => ((0x00, 0x16, 0x20), (0x00, 0x77, 0x99), (0x00, 0xe5, 0xff)),
            // Burnt amber → warm gold → bright reserve
            Self::Available => ((0x1a, 0x10, 0x00), (0xaa, 0x77, 0x00), (0xff, 0xbb, 0x33)),
            // Dark red → alert red → critical flash
            Self::Used => ((0x1a, 0x00, 0x00), (0xcc, 0x11, 0x00), (0xff, 0x22, 0x44)),
            // Deep blue → ingress cyan → bright receive
            Self::Download => ((0x00, 0x1a, 0x2b), (0x00, 0x66, 0x99), (0x00, 0xcc, 0xff)),
            // Dark violet → terminal purple → bright transmit
            Self::Upload => ((0x1a, 0x00, 0x22), (0x77, 0x00, 0xaa), (0xbb, 0x44, 0xff)),
            // Dim green → process green → bright activity
            Self::Process => ((0x00, 0x22, 0x00), (0x00, 0xaa, 0x44), (0x66, 0xff, 0x99)),
        }
    }

    fn aqua_colors(self) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        match self {
            // Teal → warm gold → soft coral (Aqua traffic-light load)
            Self::Cpu => ((0x00, 0x66, 0x66), (0xff, 0xc1, 0x50), (0xff, 0x5f, 0x57)),
            // Deep Bondi → iMac blue → sky aqua
            Self::Temp => ((0x00, 0x33, 0x55), (0x00, 0x93, 0xcb), (0x87, 0xce, 0xfa)),
            // Sea teal → mint aqua → frost green
            Self::Free => ((0x00, 0x44, 0x44), (0x40, 0xc0, 0xc0), (0xaf, 0xff, 0xe6)),
            // Navy → classic Mac blue → ice highlight
            Self::Cached => ((0x00, 0x22, 0x44), (0x00, 0x66, 0xcc), (0x99, 0xcc, 0xff)),
            // Slate → pearl blue → warm cream reserve
            Self::Available => ((0x44, 0x55, 0x66), (0xb4, 0xc8, 0xdc), (0xff, 0xf8, 0xdc)),
            // Plum shadow → rose → soft coral alert
            Self::Used => ((0x44, 0x22, 0x33), (0xcc, 0x66, 0x80), (0xff, 0x66, 0x66)),
            // Deep ocean → Bondi blue → bright aqua stream
            Self::Download => ((0x00, 0x22, 0x33), (0x00, 0x93, 0xcb), (0x66, 0xd9, 0xff)),
            // Deep violet → Mac purple → lavender transmit
            Self::Upload => ((0x33, 0x22, 0x55), (0x80, 0x66, 0xcc), (0xcc, 0xb3, 0xff)),
            // Bondi teal → activity cyan → bright mint
            Self::Process => ((0x00, 0x44, 0x55), (0x00, 0x99, 0xb4), (0x66, 0xff, 0xe6)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_endpoints() {
        let g = ColorGradient::new((0, 0, 0), None, (255, 255, 255));
        assert_eq!(g.at(0), (0, 0, 0));
        assert_eq!(g.at(100), (255, 255, 255));
        assert_eq!(g.at(50), (128, 128, 128));
    }

    #[test]
    fn gradient_with_mid() {
        let g = ColorGradient::new((0, 0, 0), Some((255, 0, 0)), (255, 255, 255));
        assert_eq!(g.at(0), (0, 0, 0));
        assert_eq!(g.at(50), (255, 0, 0));
        assert_eq!(g.at(100), (255, 255, 255));
    }

    #[test]
    fn cpu_gradient_endpoints() {
        let g = ColorGradient::from_theme(GradientTheme::Cpu);
        assert_eq!(g.at(0), (0x77, 0xca, 0x9b));    // green
        assert_eq!(g.at(50), (0xcb, 0xc0, 0x6c));   // yellow
        assert_eq!(g.at(100), (0xdc, 0x4c, 0x4c));   // red
    }

    #[test]
    fn clamps_above_100() {
        let g = ColorGradient::from_theme(GradientTheme::Cpu);
        assert_eq!(g.at(255), g.at(100));
    }

    #[test]
    fn synthwave_cpu_gradient_endpoints() {
        let g = ColorGradient::from_palette(GradientTheme::Cpu, ColorPalette::Synthwave);
        assert_eq!(g.at(0), (0x05, 0xd9, 0xe8));
        assert_eq!(g.at(50), (0xd3, 0x00, 0xc5));
        assert_eq!(g.at(100), (0xff, 0x2a, 0x6d));
    }

    #[test]
    fn hacker_cpu_gradient_endpoints() {
        let g = ColorGradient::from_palette(GradientTheme::Cpu, ColorPalette::Hacker);
        assert_eq!(g.at(0), (0x00, 0x2b, 0x00));
        assert_eq!(g.at(50), (0x00, 0xb3, 0x00));
        assert_eq!(g.at(100), (0x39, 0xff, 0x14));
    }

    #[test]
    fn aqua_cpu_gradient_endpoints() {
        let g = ColorGradient::from_palette(GradientTheme::Cpu, ColorPalette::Aqua);
        assert_eq!(g.at(0), (0x00, 0x66, 0x66));
        assert_eq!(g.at(50), (0xff, 0xc1, 0x50));
        assert_eq!(g.at(100), (0xff, 0x5f, 0x57));
    }
}
