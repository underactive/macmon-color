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
        let (start, mid, end) = theme.colors();
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
}
