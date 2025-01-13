use std::fmt;

struct Color(u8, u8, u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Color {
    fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color(r, g, b))
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    fn blend(&self, other: &Color, ratio: f32) -> Color {
        let r = (self.0 as f32 * (1.0 - ratio) + other.0 as f32 * ratio).round() as u8;
        let g = (self.1 as f32 * (1.0 - ratio) + other.1 as f32 * ratio).round() as u8;
        let b = (self.2 as f32 * (1.0 - ratio) + other.2 as f32 * ratio).round() as u8;
        Color(r, g, b)
    }
}

fn main() {
    let color1 = Color(255, 0, 0);
    let color2 = Color(0, 0, 255);
    println!("Color 1: {}", color1);
    println!("Color 2: {}", color2);

    let blended_color = color1.blend(&color2, 0.5);
    println!("Blended Color: {}", blended_color);

    if let Some(hex_color) = Color::from_hex("#FF5733") {
        println!("Hex to Color: {}", hex_color);
        println!("Color to Hex: {}", hex_color.to_hex());
    } else {
        println!("Invalid hex color.");
    }
}