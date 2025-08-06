use chroma_forge::Color;

fn main() {
    println!("Chroma Forge Color Conversion Examples\n");

    // Create colors from different formats
    let hex_color = Color::from_hex("#FF5733").unwrap();
    let rgb_color = Color::from_rgb(64, 128, 255);
    let hsl_color = Color::from_hsl(220.0, 100.0, 62.5).unwrap();

    println!("Hex Color #FF5733:");
    println!("  RGB: {}", hex_color.to_rgb());
    println!("  HSL: {}", hex_color.to_hsl());
    println!("  HSV: {}", hex_color.to_hsv());
    println!("  CMYK: {}", hex_color.to_cmyk());
    println!("  Luminance: {:.2}", hex_color.luminance());
    println!("  Is Dark: {}", hex_color.is_dark());

    println!("\nRGB Color (64, 128, 255):");
    println!("  Hex: {}", rgb_color.to_hex());
    println!("  HSL: {}", rgb_color.to_hsl());

    println!("\nHSL Color (220°, 100%, 62.5%):");
    println!("  Hex: {}", hsl_color.to_hex());
    println!("  RGB: {}", hsl_color.to_rgb());

    // Color manipulation
    println!("\nColor Manipulation:");
    let original = Color::from_hex("#4080FF").unwrap();
    println!("  Original: {}", original.to_hex());
    println!("  Darkened 30%: {}", original.darken(30.0).to_hex());
    println!("  Lightened 30%: {}", original.lighten(30.0).to_hex());

    // Color blending
    let red = Color::RED;
    let blue = Color::BLUE;
    let blend = red.blend(&blue, 0.7);
    println!("  Red + Blue (70% blue): {}", blend.to_hex());

    // Contrasting colors
    println!("\nContrasting Colors:");
    println!(
        "  Dark color {} needs: {}",
        Color::BLACK.to_hex(),
        Color::BLACK.contrasting_text_color().to_hex()
    );
    println!(
        "  Light color {} needs: {}",
        Color::WHITE.to_hex(),
        Color::WHITE.contrasting_text_color().to_hex()
    );

    // Minecraft color codes
    println!("\nMinecraft Color Codes:");
    let mc_colors = [
        ("§0 Black", Color::MC_BLACK),
        ("§1 Dark Blue", Color::MC_DARK_BLUE),
        ("§2 Dark Green", Color::MC_DARK_GREEN),
        ("§3 Dark Aqua", Color::MC_DARK_AQUA),
        ("§4 Dark Red", Color::MC_DARK_RED),
        ("§5 Dark Purple", Color::MC_DARK_PURPLE),
        ("§6 Gold", Color::MC_GOLD),
        ("§7 Gray", Color::MC_GRAY),
        ("§8 Dark Gray", Color::MC_DARK_GRAY),
        ("§9 Blue", Color::MC_BLUE),
        ("§a Green", Color::MC_GREEN),
        ("§b Aqua", Color::MC_AQUA),
        ("§c Red", Color::MC_RED),
        ("§d Light Purple", Color::MC_LIGHT_PURPLE),
        ("§e Yellow", Color::MC_YELLOW),
        ("§f White", Color::MC_WHITE),
    ];

    for (name, color) in mc_colors.iter().take(8) {
        // Show first 8
        println!(
            "  {}: {} -> {}",
            name,
            color.to_hex(),
            color.to_minecraft_hex()
        );
    }

    // Custom color to Minecraft code
    let custom = Color::from_hex("#8A2BE2").unwrap(); // Blue Violet
    println!("\nCustom Color Conversion:");
    println!(
        "  {} -> Closest MC code: {}",
        custom.to_hex(),
        custom.to_minecraft_code()
    );
    println!(
        "  {} -> MC hex: {}",
        custom.to_hex(),
        custom.to_minecraft_hex()
    );
    println!(
        "  {} -> MC alt hex: {}",
        custom.to_hex(),
        custom.to_minecraft_hex_alt()
    );

    // Parse Minecraft codes
    println!("\nParsing Minecraft Codes:");
    let codes = ["§c", "&1", "&#FF5733", "&x&F&F&5&7&3&3"];
    for code in codes.iter() {
        match Color::from_minecraft_code(code) {
            Ok(color) => println!("  {} -> {}", code, color.to_hex()),
            Err(e) => println!("  {} -> Error: {}", code, e),
        }
    }
}
