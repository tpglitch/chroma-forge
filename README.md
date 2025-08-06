# chroma-forge

A comprehensive color conversion library for Rust that provides seamless conversion between various color formats including RGB, Hex, HSL, HSV, CMYK, and Minecraft color codes.

## Features

- **Multiple Color Formats**: RGB/RGBA, Hex, HSL, HSV, CMYK
- **Minecraft Color Codes**: Legacy codes (§, &) and modern hex formats
- **Color Utilities**: Luminance calculation, dark/light detection, contrasting colors
- **Color Manipulation**: Blending, darkening, lightening
- **Zero Dependencies**: Pure Rust implementation
- **Comprehensive Error Handling**: Custom error types with detailed messages
- **Extensive Testing**: Full test coverage for all conversion methods

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chroma-forge = "1.0.0"
```

## Quick Start

```rust
use chroma_forge::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create colors from different formats
    let hex_color = Color::from_hex("#FF5733")?;
    let rgb_color = Color::from_rgb(255, 87, 51);
    let hsl_color = Color::from_hsl(10.0, 100.0, 60.0)?;

    // Convert between formats
    println!("Hex: {}", hex_color.to_hex());        // #FF5733
    println!("RGB: {}", hex_color.to_rgb());        // rgb(255, 87, 51)
    println!("HSL: {}", hex_color.to_hsl());        // hsl(10.0°, 100.0%, 60.0%)

    Ok(())
}
```

## Color Format Support

### RGB/RGBA

```rust
let color = Color::from_rgb(255, 87, 51);
let color_with_alpha = Color::from_rgba(255, 87, 51, 0.8);
let rgb = color.to_rgb();
```

### Hex Colors

Supports multiple hex formats:

```rust
let color1 = Color::from_hex("#FF5733")?;     // Full format
let color2 = Color::from_hex("FF5733")?;      // Without #
let color3 = Color::from_hex("#F53")?;        // Short format
let color4 = Color::from_hex("#FF5733CC")?;   // With alpha

let hex = color.to_hex();                     // #FF5733
let hex_alpha = color.to_hex_alpha();         // #FF5733FF
```

### HSL (Hue, Saturation, Lightness)

```rust
let color = Color::from_hsl(220.0, 100.0, 50.0)?; // Blue
let hsl = color.to_hsl();
```

### HSV (Hue, Saturation, Value)

```rust
let color = Color::from_hsv(220.0, 100.0, 100.0)?; // Blue
let hsv = color.to_hsv();
```

### CMYK (Cyan, Magenta, Yellow, Key/Black)

```rust
let color = Color::from_cmyk(100.0, 0.0, 0.0, 0.0)?; // Cyan
let cmyk = color.to_cmyk();
```

## Minecraft Color Codes

chroma-forge provides comprehensive support for Minecraft's color system, including both legacy and modern formats.

### Legacy Color Codes

Supports all 16 standard Minecraft colors with both § and & prefixes:

```rust
// Legacy format parsing
let red = Color::from_minecraft_code("§c")?;       // Red
let blue = Color::from_minecraft_code("&1")?;      // Dark Blue
let yellow = Color::from_minecraft_code("e")?;     // Yellow (no prefix)

// Convert to legacy format (finds closest match)
let mc_code = color.to_minecraft_code();           // "§c"
```

### Modern Hex Formats

Supports Minecraft's modern hex color formats:

```rust
// Modern hex format
let color1 = Color::from_minecraft_code("&#FF5733")?;

// Alternate hex format
let color2 = Color::from_minecraft_code("&x&F&F&5&7&3&3")?;

// Convert to modern formats
let mc_hex = color.to_minecraft_hex();             // "&#FF5733"
let mc_alt = color.to_minecraft_hex_alt();         // "&x&F&F&5&7&3&3"
```

### Minecraft Color Constants

Pre-defined constants for all Minecraft colors:

```rust
let colors = [
    Color::MC_BLACK,        // §0
    Color::MC_DARK_BLUE,    // §1
    Color::MC_DARK_GREEN,   // §2
    Color::MC_DARK_AQUA,    // §3
    Color::MC_DARK_RED,     // §4
    Color::MC_DARK_PURPLE,  // §5
    Color::MC_GOLD,         // §6
    Color::MC_GRAY,         // §7
    Color::MC_DARK_GRAY,    // §8
    Color::MC_BLUE,         // §9
    Color::MC_GREEN,        // §a
    Color::MC_AQUA,         // §b
    Color::MC_RED,          // §c
    Color::MC_LIGHT_PURPLE, // §d
    Color::MC_YELLOW,       // §e
    Color::MC_WHITE,        // §f
];
```

## Color Utilities

### Luminance and Brightness

```rust
let color = Color::from_hex("#FF5733")?;

let luminance = color.luminance();              // 0.0 to 1.0
let is_dark = color.is_dark();                  // true/false
let is_light = color.is_light();                // true/false

// Get contrasting color for text overlay
let contrast = color.contrasting_text_color();  // Black or white
```

### Color Manipulation

```rust
let color = Color::from_hex("#4080FF")?;

// Darken/lighten by percentage
let darker = color.darken(30.0);                // 30% darker
let lighter = color.lighten(30.0);              // 30% lighter

// Blend two colors
let red = Color::RED;
let blue = Color::BLUE;
let purple = red.blend(&blue, 0.5);             // 50% blend
```

## Color Constants

Common colors are available as constants:

```rust
let colors = [
    Color::BLACK,
    Color::WHITE,
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::YELLOW,
    Color::CYAN,
    Color::MAGENTA,
];
```

## Error Handling

chroma-forge uses a comprehensive error system:

```rust
use chroma_forge::{Color, ColorError};

match Color::from_hex("invalid") {
    Ok(color) => println!("Color: {}", color.to_hex()),
    Err(ColorError::InvalidHexFormat) => println!("Invalid hex format"),
    Err(ColorError::InvalidHexLength) => println!("Invalid hex length"),
    Err(e) => println!("Other error: {}", e),
}
```

Error types include:

- `InvalidHexFormat`
- `InvalidHexLength`
- `InvalidRgbValue`
- `InvalidHslValue`
- `InvalidHsvValue`
- `InvalidCmykValue`
- `InvalidMinecraftCode`

## Examples

### Basic Color Conversion

```rust
use chroma_forge::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color = Color::from_hex("#FF5733")?;

    println!("Original: {}", color.to_hex());
    println!("RGB: {}", color.to_rgb());
    println!("HSL: {}", color.to_hsl());
    println!("HSV: {}", color.to_hsv());
    println!("CMYK: {}", color.to_cmyk());

    Ok(())
}
```

### Minecraft Integration

```rust
use chroma_forge::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse various Minecraft color formats
    let colors = [
        Color::from_minecraft_code("§c")?,           // Legacy red
        Color::from_minecraft_code("&#FF5733")?,     // Modern hex
        Color::from_minecraft_code("&x&F&F&5&7&3&3")?, // Alt hex
    ];

    for color in colors {
        println!("Color: {} -> MC: {}",
                color.to_hex(),
                color.to_minecraft_code());
    }

    Ok(())
}
```

### Color Analysis

```rust
use chroma_forge::Color;

fn analyze_color(hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let color = Color::from_hex(hex)?;

    println!("Analyzing color: {}", color.to_hex());
    println!("Luminance: {:.2}", color.luminance());
    println!("Is dark: {}", color.is_dark());
    println!("Best text color: {}", color.contrasting_text_color().to_hex());
    println!("Darker version: {}", color.darken(20.0).to_hex());
    println!("Lighter version: {}", color.lighten(20.0).to_hex());

    Ok(())
}
```

## Performance

chroma-forge is designed for performance with:

- Zero-copy string parsing where possible
- Efficient mathematical operations
- Minimal memory allocations
- No external dependencies

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Make sure to:

1. Add tests for new functionality
2. Update documentation
3. Follow Rust best practices
4. Ensure all tests pass

## License

This project is licensed under MIT License

## Changelog

### 1.0.0

- Initial release
- RGB, Hex, HSL, HSV, CMYK support
- Minecraft color code support
- Color manipulation utilities
- Comprehensive error handling
- Full test coverage
