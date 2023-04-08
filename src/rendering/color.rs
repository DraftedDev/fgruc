/// A struct for to and from conversion of most color formats including hex, rgb, rgba, etc.
/// The Color Data is actually stored as a RGBA8888 u32.
#[derive(Copy, Clone, Debug)]
pub struct UniColor(u32);

impl UniColor {

    /// Creates a new UniColor from an RGB tuple.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let u32_color = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF;
        UniColor(u32_color)
    }

    /// Converts a UniColor to an RGB tuple.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let r = (self.0 >> 24) as u8;
        let g = (self.0 >> 16) as u8;
        let b = (self.0 >> 8) as u8;
        (r, g, b)
    }

    /// Creates a new UniColor from an RGBA tuple.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let u32_color = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32);
        UniColor(u32_color)
    }

    /// Converts a UniColor to an RGBA tuple.
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        let r = (self.0 >> 24) as u8;
        let g = (self.0 >> 16) as u8;
        let b = (self.0 >> 8) as u8;
        let a = (self.0 & 0xFF) as u8;
        (r, g, b, a)
    }

    /// Creates a new UniColor from a hex string.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = hex[0..1].repeat(2);
                let g = hex[1..2].repeat(2);
                let b = hex[2..3].repeat(2);
                let hex = format!("{}{}{}", r, g, b);
                u32::from_str_radix(&hex, 16).ok().map(|v| UniColor(v << 8 | 0xFF))
            }
            6 => u32::from_str_radix(hex, 16).ok().map(|v| UniColor(v << 8 | 0xFF)),
            _ => None,
        }
    }

    /// Converts a UniColor to a hex string.
    pub fn to_hex(&self) -> String {
        format!("{:#08x}", self.0 >> 8)
    }

    /// Creates a new UniColor from a CMYK tuple.
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32) -> Self {
        let r = (1.0 - c) * (1.0 - k);
        let g = (1.0 - m) * (1.0 - k);
        let b = (1.0 - y) * (1.0 - k);
        Self::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }

    /// Converts a UniColor to a CMYK tuple.
    pub fn to_cmyk(&self) -> (f32, f32, f32, f32) {
        let r = (self.0 >> 16) as f32 / 255.0;
        let g = (self.0 >> 8 & 0xff) as f32 / 255.0;
        let b = (self.0 & 0xff) as f32 / 255.0;

        let k = 1.0 - r.max(g).max(b);
        let c = (1.0 - r - k) / (1.0 - k);
        let m = (1.0 - g - k) / (1.0 - k);
        let y = (1.0 - b - k) / (1.0 - k);

        (c, m, y, k)
    }

    /// Converts a UniColor to its HSL representation.
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let (r, g, b) = self.to_rgb();
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let chroma = max - min;
        let lightness = (max + min) / 2.0;
        let saturation = if chroma == 0.0 {
            0.0
        } else {
            chroma / (1.0 - (2.0 * lightness - 1.0).abs())
        };
        let hue = if chroma == 0.0 {
            0.0
        } else if max == r {
            ((g - b) / chroma) % 6.0
        } else if max == g {
            ((b - r) / chroma) + 2.0
        } else {
            ((r - g) / chroma) + 4.0
        };
        let hue = hue * 60.0;
        if hue < 0.0 {
            (hue + 360.0, saturation, lightness)
        } else {
            (hue, saturation, lightness)
        }
    }

    /// Creates a UniColor from its HSL representation.
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h = h / 60.0;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let (r1, g1, b1) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let m = l - c / 2.0;
        let r = (r1 + m) * 255.0 + 0.5;
        let g = (g1 + m) * 255.0 + 0.5;
        let b = (b1 + m) * 255.0 + 0.5;
        UniColor::from_rgb(r as u8, g as u8, b as u8)
    }

    pub fn as_bytes(&self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    /// Computes the midpoint between two colors.
    pub fn midpoint(&self, other: &Self) -> Self {
        let (r1, g1, b1, a1) = self.to_rgba();
        let (r2, g2, b2, a2) = other.to_rgba();

        let r = (r1 as f32 + r2 as f32) / 2.0;
        let g = (g1 as f32 + g2 as f32) / 2.0;
        let b = (b1 as f32 + b2 as f32) / 2.0;
        let a = (a1 as f32 + a2 as f32) / 2.0;

        UniColor::from_rgba(
            r.round() as u8,
            g.round() as u8,
            b.round() as u8,
            a.round() as u8,
        )
    }

    /// Computes a linear interpolation between two colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.max(0.0).min(1.0);
        let (r1, g1, b1, a1) = self.to_rgba();
        let (r2, g2, b2, a2) = other.to_rgba();

        let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t).round() as u8;
        let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t).round() as u8;
        let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t).round() as u8;
        let a = (a1 as f32 * (1.0 - t) + a2 as f32 * t).round() as u8;

        UniColor::from_rgba(r, g, b, a)
    }

    /// Computes the squared distance between two colors in RGBA space.
    pub fn distance_squared(&self, other: &Self) -> u32 {
        let (r1, g1, b1, a1) = self.to_rgba();
        let (r2, g2, b2, a2) = other.to_rgba();

        let dr = r1 as i32 - r2 as i32;
        let dg = g1 as i32 - g2 as i32;
        let db = b1 as i32 - b2 as i32;
        let da = a1 as i32 - a2 as i32;

        (dr * dr + dg * dg + db * db + da * da) as u32
    }

}