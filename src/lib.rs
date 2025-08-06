//! # Chroma Forge
//!
//! A comprehensive color conversion library for Rust that provides seamless
//! conversion between various color formats including RGB, Hex, HSL, HSV, and CMYK.
//!
//! ## Examples
//!
//! ```rust
//! use chroma_forge::Color;
//!
//! let color = Color::from_hex("#FF5733").unwrap();
//! println!("RGB: {}", color.to_rgb());
//! println!("HSL: {}", color.to_hsl());
//! ```

use std::fmt;

/// Represents a color with various conversion capabilities
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: f32, // Alpha channel (0.0 to 1.0)
}

/// RGB color representation
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// HSL (Hue, Saturation, Lightness) color representation
#[derive(Debug, Clone, PartialEq)]
pub struct Hsl {
    pub h: f32, // 0.0 to 360.0
    pub s: f32, // 0.0 to 100.0
    pub l: f32, // 0.0 to 100.0
}

/// HSV (Hue, Saturation, Value) color representation
#[derive(Debug, Clone, PartialEq)]
pub struct Hsv {
    pub h: f32, // 0.0 to 360.0
    pub s: f32, // 0.0 to 100.0
    pub v: f32, // 0.0 to 100.0
}

/// CMYK (Cyan, Magenta, Yellow, Key/Black) color representation
#[derive(Debug, Clone, PartialEq)]
pub struct Cmyk {
    pub c: f32, // 0.0 to 100.0
    pub m: f32, // 0.0 to 100.0
    pub y: f32, // 0.0 to 100.0
    pub k: f32, // 0.0 to 100.0
}

/// Custom error type for color conversion operations
#[derive(Debug, PartialEq)]
pub enum ColorError {
    InvalidHexFormat,
    InvalidHexLength,
    InvalidRgbValue,
    InvalidHslValue,
    InvalidHsvValue,
    InvalidCmykValue,
    InvalidMinecraftCode,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::InvalidHexFormat => write!(f, "Invalid hex color format"),
            ColorError::InvalidHexLength => write!(f, "Invalid hex color length"),
            ColorError::InvalidRgbValue => write!(f, "RGB values must be between 0 and 255"),
            ColorError::InvalidHslValue => write!(f, "Invalid HSL values"),
            ColorError::InvalidHsvValue => write!(f, "Invalid HSV values"),
            ColorError::InvalidCmykValue => write!(f, "CMYK values must be between 0 and 100"),
            ColorError::InvalidMinecraftCode => write!(f, "Invalid Minecraft color code"),
        }
    }
}

impl std::error::Error for ColorError {}

impl Color {
    /// Create a new Color from RGB values
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    /// Create a new Color from RGBA values
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Color {
            r,
            g,
            b,
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create a Color from a hex string (e.g., "#FF5733" or "FF5733")
    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            3 => {
                // Short form: #RGB -> #RRGGBB
                let chars: Vec<char> = hex.chars().collect();
                let r = u8::from_str_radix(&format!("{}{}", chars[0], chars[0]), 16)
                    .map_err(|_| ColorError::InvalidHexFormat)?;
                let g = u8::from_str_radix(&format!("{}{}", chars[1], chars[1]), 16)
                    .map_err(|_| ColorError::InvalidHexFormat)?;
                let b = u8::from_str_radix(&format!("{}{}", chars[2], chars[2]), 16)
                    .map_err(|_| ColorError::InvalidHexFormat)?;
                Ok(Color::from_rgb(r, g, b))
            }
            6 => {
                // Full form: #RRGGBB
                let r =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidHexFormat)?;
                let g =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidHexFormat)?;
                let b =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidHexFormat)?;
                Ok(Color::from_rgb(r, g, b))
            }
            8 => {
                // With alpha: #RRGGBBAA
                let r =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidHexFormat)?;
                let g =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidHexFormat)?;
                let b =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidHexFormat)?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| ColorError::InvalidHexFormat)? as f32
                    / 255.0;
                Ok(Color::from_rgba(r, g, b, a))
            }
            _ => Err(ColorError::InvalidHexLength),
        }
    }

    /// Create a Color from HSL values
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Result<Self, ColorError> {
        if s < 0.0 || s > 100.0 || l < 0.0 || l > 100.0 {
            return Err(ColorError::InvalidHslValue);
        }

        let h = h % 360.0;
        let s = s / 100.0;
        let l = l / 100.0;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r_prime, g_prime, b_prime) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let r = ((r_prime + m) * 255.0).round() as u8;
        let g = ((g_prime + m) * 255.0).round() as u8;
        let b = ((b_prime + m) * 255.0).round() as u8;

        Ok(Color::from_rgb(r, g, b))
    }

    /// Create a Color from Minecraft color code (e.g., "§c", "&4", "&#FF5733")
    pub fn from_minecraft_code(code: &str) -> Result<Self, ColorError> {
        if code.is_empty() {
            return Err(ColorError::InvalidMinecraftCode);
        }

        // Handle modern hex format: &#RRGGBB or &x&R&R&G&G&B&B
        if code.starts_with("&#") && code.len() == 8 {
            return Color::from_hex(&code[2..]);
        }

        // Handle alternate hex format: &x&R&R&G&G&B&B
        if code.starts_with("&x") && code.len() == 14 {
            let hex_chars: String = code
                .chars()
                .skip(2)
                .enumerate()
                .filter_map(|(i, c)| if i % 2 == 1 { Some(c) } else { None })
                .collect();
            return Color::from_hex(&hex_chars);
        }

        // Handle legacy color codes
        let color_char = if code.starts_with('§') || code.starts_with('&') {
            code.chars()
                .nth(1)
                .ok_or(ColorError::InvalidMinecraftCode)?
        } else if code.len() == 1 {
            code.chars().next().unwrap()
        } else {
            return Err(ColorError::InvalidMinecraftCode);
        };

        match color_char {
            '0' => Ok(Color::from_rgb(0, 0, 0)),             // Black
            '1' => Ok(Color::from_rgb(0, 0, 170)),           // Dark Blue
            '2' => Ok(Color::from_rgb(0, 170, 0)),           // Dark Green
            '3' => Ok(Color::from_rgb(0, 170, 170)),         // Dark Aqua
            '4' => Ok(Color::from_rgb(170, 0, 0)),           // Dark Red
            '5' => Ok(Color::from_rgb(170, 0, 170)),         // Dark Purple
            '6' => Ok(Color::from_rgb(255, 170, 0)),         // Gold
            '7' => Ok(Color::from_rgb(170, 170, 170)),       // Gray
            '8' => Ok(Color::from_rgb(85, 85, 85)),          // Dark Gray
            '9' => Ok(Color::from_rgb(85, 85, 255)),         // Blue
            'a' | 'A' => Ok(Color::from_rgb(85, 255, 85)),   // Green
            'b' | 'B' => Ok(Color::from_rgb(85, 255, 255)),  // Aqua
            'c' | 'C' => Ok(Color::from_rgb(255, 85, 85)),   // Red
            'd' | 'D' => Ok(Color::from_rgb(255, 85, 255)),  // Light Purple
            'e' | 'E' => Ok(Color::from_rgb(255, 255, 85)),  // Yellow
            'f' | 'F' => Ok(Color::from_rgb(255, 255, 255)), // White
            _ => Err(ColorError::InvalidMinecraftCode),
        }
    }

    /// Create a Color from HSV values
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Result<Self, ColorError> {
        if s < 0.0 || s > 100.0 || v < 0.0 || v > 100.0 {
            return Err(ColorError::InvalidHsvValue);
        }

        let h = h % 360.0;
        let s = s / 100.0;
        let v = v / 100.0;

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let r = ((r_prime + m) * 255.0).round() as u8;
        let g = ((g_prime + m) * 255.0).round() as u8;
        let b = ((b_prime + m) * 255.0).round() as u8;

        Ok(Color::from_rgb(r, g, b))
    }

    /// Create a Color from CMYK values
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32) -> Result<Self, ColorError> {
        if c < 0.0
            || c > 100.0
            || m < 0.0
            || m > 100.0
            || y < 0.0
            || y > 100.0
            || k < 0.0
            || k > 100.0
        {
            return Err(ColorError::InvalidCmykValue);
        }

        let c = c / 100.0;
        let m = m / 100.0;
        let y = y / 100.0;
        let k = k / 100.0;

        let r = (255.0 * (1.0 - c) * (1.0 - k)).round() as u8;
        let g = (255.0 * (1.0 - m) * (1.0 - k)).round() as u8;
        let b = (255.0 * (1.0 - y) * (1.0 - k)).round() as u8;

        Ok(Color::from_rgb(r, g, b))
    }

    /// Convert to RGB
    pub fn to_rgb(&self) -> Rgb {
        Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to hex string with alpha
    pub fn to_hex_alpha(&self) -> String {
        let alpha = (self.a * 255.0).round() as u8;
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, alpha)
    }

    /// Convert to HSL
    pub fn to_hsl(&self) -> Hsl {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let l = (max + min) / 2.0;

        let (h, s) = if delta == 0.0 {
            (0.0, 0.0)
        } else {
            let s = if l < 0.5 {
                delta / (max + min)
            } else {
                delta / (2.0 - max - min)
            };

            let h = match max {
                x if x == r => ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0,
                x if x == g => ((b - r) / delta + 2.0) * 60.0,
                _ => ((r - g) / delta + 4.0) * 60.0,
            };

            (h, s * 100.0)
        };

        Hsl { h, s, l: l * 100.0 }
    }

    /// Convert to HSV
    pub fn to_hsv(&self) -> Hsv {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let v = max;

        let (h, s) = if delta == 0.0 {
            (0.0, 0.0)
        } else {
            let s = delta / max;

            let h = match max {
                x if x == r => ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0,
                x if x == g => ((b - r) / delta + 2.0) * 60.0,
                _ => ((r - g) / delta + 4.0) * 60.0,
            };

            (h, s * 100.0)
        };

        Hsv { h, s, v: v * 100.0 }
    }

    /// Convert to CMYK
    pub fn to_cmyk(&self) -> Cmyk {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let k = 1.0 - r.max(g.max(b));

        if k == 1.0 {
            Cmyk {
                c: 0.0,
                m: 0.0,
                y: 0.0,
                k: 100.0,
            }
        } else {
            let c = (1.0 - r - k) / (1.0 - k) * 100.0;
            let m = (1.0 - g - k) / (1.0 - k) * 100.0;
            let y = (1.0 - b - k) / (1.0 - k) * 100.0;

            Cmyk {
                c,
                m,
                y,
                k: k * 100.0,
            }
        }
    }

    /// Get luminance (brightness) of the color
    pub fn luminance(&self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        0.299 * r + 0.587 * g + 0.114 * b
    }

    /// Check if the color is considered "dark" (luminance < 0.5)
    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.5
    }

    /// Check if the color is considered "light" (luminance >= 0.5)
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    /// Get a contrasting color (black or white) for text overlay
    pub fn contrasting_text_color(&self) -> Color {
        if self.is_dark() {
            Color::from_rgb(255, 255, 255) // White
        } else {
            Color::from_rgb(0, 0, 0) // Black
        }
    }

    /// Blend this color with another color
    pub fn blend(&self, other: &Color, ratio: f32) -> Color {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv_ratio = 1.0 - ratio;

        let r = (self.r as f32 * inv_ratio + other.r as f32 * ratio).round() as u8;
        let g = (self.g as f32 * inv_ratio + other.g as f32 * ratio).round() as u8;
        let b = (self.b as f32 * inv_ratio + other.b as f32 * ratio).round() as u8;
        let a = self.a * inv_ratio + other.a * ratio;

        Color::from_rgba(r, g, b, a)
    }

    /// Convert to Minecraft legacy color code (closest match)
    pub fn to_minecraft_code(&self) -> String {
        let distances: Vec<(f32, char)> = vec![
            (self.color_distance(&Color::from_rgb(0, 0, 0)), '0'), // Black
            (self.color_distance(&Color::from_rgb(0, 0, 170)), '1'), // Dark Blue
            (self.color_distance(&Color::from_rgb(0, 170, 0)), '2'), // Dark Green
            (self.color_distance(&Color::from_rgb(0, 170, 170)), '3'), // Dark Aqua
            (self.color_distance(&Color::from_rgb(170, 0, 0)), '4'), // Dark Red
            (self.color_distance(&Color::from_rgb(170, 0, 170)), '5'), // Dark Purple
            (self.color_distance(&Color::from_rgb(255, 170, 0)), '6'), // Gold
            (self.color_distance(&Color::from_rgb(170, 170, 170)), '7'), // Gray
            (self.color_distance(&Color::from_rgb(85, 85, 85)), '8'), // Dark Gray
            (self.color_distance(&Color::from_rgb(85, 85, 255)), '9'), // Blue
            (self.color_distance(&Color::from_rgb(85, 255, 85)), 'a'), // Green
            (self.color_distance(&Color::from_rgb(85, 255, 255)), 'b'), // Aqua
            (self.color_distance(&Color::from_rgb(255, 85, 85)), 'c'), // Red
            (self.color_distance(&Color::from_rgb(255, 85, 255)), 'd'), // Light Purple
            (self.color_distance(&Color::from_rgb(255, 255, 85)), 'e'), // Yellow
            (self.color_distance(&Color::from_rgb(255, 255, 255)), 'f'), // White
        ];

        let closest = distances
            .iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();
        format!("§{}", closest.1)
    }

    /// Convert to Minecraft modern hex color code format
    pub fn to_minecraft_hex(&self) -> String {
        format!("&#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to Minecraft alternate hex format (&x&R&R&G&G&B&B)
    pub fn to_minecraft_hex_alt(&self) -> String {
        let hex = format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b);
        let mut result = String::from("&x");
        for ch in hex.chars() {
            result.push('&');
            result.push(ch);
        }
        result
    }

    /// Helper function to calculate color distance for closest match
    fn color_distance(&self, other: &Color) -> f32 {
        let dr = self.r as f32 - other.r as f32;
        let dg = self.g as f32 - other.g as f32;
        let db = self.b as f32 - other.b as f32;
        (dr * dr + dg * dg + db * db).sqrt()
    }

    /// Darken the color by a percentage
    pub fn darken(&self, percentage: f32) -> Color {
        let factor = 1.0 - (percentage / 100.0).clamp(0.0, 1.0);
        let r = (self.r as f32 * factor).round() as u8;
        let g = (self.g as f32 * factor).round() as u8;
        let b = (self.b as f32 * factor).round() as u8;

        Color::from_rgba(r, g, b, self.a)
    }

    /// Lighten the color by a percentage
    pub fn lighten(&self, percentage: f32) -> Color {
        let factor = (percentage / 100.0).clamp(0.0, 1.0);
        let r = (self.r as f32 + (255.0 - self.r as f32) * factor).round() as u8;
        let g = (self.g as f32 + (255.0 - self.g as f32) * factor).round() as u8;
        let b = (self.b as f32 + (255.0 - self.b as f32) * factor).round() as u8;

        Color::from_rgba(r, g, b, self.a)
    }
}

// Display implementations for easy printing
impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({:.1}°, {:.1}%, {:.1}%)", self.h, self.s, self.l)
    }
}

impl fmt::Display for Hsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({:.1}°, {:.1}%, {:.1}%)", self.h, self.s, self.v)
    }
}

impl fmt::Display for Cmyk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmyk({:.1}%, {:.1}%, {:.1}%, {:.1}%)",
            self.c, self.m, self.y, self.k
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

// Common color constants
impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 255,
        b: 255,
        a: 1.0,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 1.0,
    };

    // Minecraft color constants
    pub const MC_BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    pub const MC_DARK_BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 170,
        a: 1.0,
    };
    pub const MC_DARK_GREEN: Color = Color {
        r: 0,
        g: 170,
        b: 0,
        a: 1.0,
    };
    pub const MC_DARK_AQUA: Color = Color {
        r: 0,
        g: 170,
        b: 170,
        a: 1.0,
    };
    pub const MC_DARK_RED: Color = Color {
        r: 170,
        g: 0,
        b: 0,
        a: 1.0,
    };
    pub const MC_DARK_PURPLE: Color = Color {
        r: 170,
        g: 0,
        b: 170,
        a: 1.0,
    };
    pub const MC_GOLD: Color = Color {
        r: 255,
        g: 170,
        b: 0,
        a: 1.0,
    };
    pub const MC_GRAY: Color = Color {
        r: 170,
        g: 170,
        b: 170,
        a: 1.0,
    };
    pub const MC_DARK_GRAY: Color = Color {
        r: 85,
        g: 85,
        b: 85,
        a: 1.0,
    };
    pub const MC_BLUE: Color = Color {
        r: 85,
        g: 85,
        b: 255,
        a: 1.0,
    };
    pub const MC_GREEN: Color = Color {
        r: 85,
        g: 255,
        b: 85,
        a: 1.0,
    };
    pub const MC_AQUA: Color = Color {
        r: 85,
        g: 255,
        b: 255,
        a: 1.0,
    };
    pub const MC_RED: Color = Color {
        r: 255,
        g: 85,
        b: 85,
        a: 1.0,
    };
    pub const MC_LIGHT_PURPLE: Color = Color {
        r: 255,
        g: 85,
        b: 255,
        a: 1.0,
    };
    pub const MC_YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 85,
        a: 1.0,
    };
    pub const MC_WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        let color = Color::from_hex("#FF5733").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);
    }

    #[test]
    fn test_rgb_to_hex() {
        let color = Color::from_rgb(255, 87, 51);
        assert_eq!(color.to_hex(), "#FF5733");
    }

    #[test]
    fn test_short_hex() {
        let color = Color::from_hex("#F53").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 85);
        assert_eq!(color.b, 51);
    }

    #[test]
    fn test_hsl_conversion() {
        let color = Color::from_hsl(10.0, 100.0, 60.0).unwrap();
        let hsl = color.to_hsl();
        assert!((hsl.h - 10.0).abs() < 1.0);
        assert!((hsl.s - 100.0).abs() < 1.0);
        assert!((hsl.l - 60.0).abs() < 1.0);
    }

    #[test]
    fn test_color_blending() {
        let red = Color::RED;
        let blue = Color::BLUE;
        let purple = red.blend(&blue, 0.5);

        assert_eq!(purple.r, 127);
        assert_eq!(purple.g, 0);
        assert_eq!(purple.b, 127);
    }

    #[test]
    fn test_luminance() {
        assert!(Color::WHITE.is_light());
        assert!(Color::BLACK.is_dark());
    }

    #[test]
    fn test_darken_lighten() {
        let color = Color::from_rgb(100, 100, 100);
        let darker = color.darken(50.0);
        let lighter = color.lighten(50.0);

        assert!(darker.r < color.r);
        assert!(lighter.r > color.r);
    }

    #[test]
    fn test_cmyk_conversion() {
        let color = Color::from_cmyk(0.0, 100.0, 100.0, 0.0).unwrap(); // Should be red
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_minecraft_legacy_codes() {
        // Test legacy color codes
        let red = Color::from_minecraft_code("§c").unwrap();
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 85);
        assert_eq!(red.b, 85);

        let blue = Color::from_minecraft_code("&1").unwrap();
        assert_eq!(blue.r, 0);
        assert_eq!(blue.g, 0);
        assert_eq!(blue.b, 170);

        let yellow = Color::from_minecraft_code("e").unwrap();
        assert_eq!(yellow.r, 255);
        assert_eq!(yellow.g, 255);
        assert_eq!(yellow.b, 85);
    }

    #[test]
    fn test_minecraft_hex_codes() {
        // Test modern hex format
        let color = Color::from_minecraft_code("&#FF5733").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);

        // Test alternate hex format
        let color2 = Color::from_minecraft_code("&x&F&F&5&7&3&3").unwrap();
        assert_eq!(color2.r, 255);
        assert_eq!(color2.g, 87);
        assert_eq!(color2.b, 51);
    }

    #[test]
    fn test_minecraft_code_conversion() {
        let color = Color::from_rgb(255, 85, 85);
        let mc_code = color.to_minecraft_code();
        assert_eq!(mc_code, "§c");

        let hex_code = color.to_minecraft_hex();
        assert_eq!(hex_code, "&#FF5555");

        let alt_hex = color.to_minecraft_hex_alt();
        assert_eq!(alt_hex, "&x&F&F&5&5&5&5");
    }

    #[test]
    fn test_minecraft_constants() {
        assert_eq!(Color::MC_RED.r, 255);
        assert_eq!(Color::MC_RED.g, 85);
        assert_eq!(Color::MC_RED.b, 85);

        assert_eq!(Color::MC_GOLD.r, 255);
        assert_eq!(Color::MC_GOLD.g, 170);
        assert_eq!(Color::MC_GOLD.b, 0);
    }
}
