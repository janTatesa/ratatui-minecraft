use catppuccin::{FlavorColors, PALETTE, Rgb};
use ratatui::style::Color;
use valence::text::color::RgbColor;
const COLORS: FlavorColors = PALETTE.mocha.colors;
pub const BACKGROUND: RgbColor = catppuccin_to_valence_color(COLORS.base);
pub const FOREGROUND: RgbColor = catppuccin_to_valence_color(COLORS.text);
pub enum ColorType {
    Foreground,
    Background,
}

pub const fn ratatui_to_valence_color(color: Color, color_type: ColorType) -> RgbColor {
    catppuccin_to_valence_color(match color {
        Color::White => COLORS.text,
        Color::Black => COLORS.mantle,
        Color::Red | Color::LightRed => COLORS.red,
        Color::Green | Color::LightGreen => COLORS.green,
        Color::Yellow | Color::LightYellow => COLORS.yellow,
        Color::Blue | Color::LightBlue => COLORS.blue,
        Color::Magenta | Color::LightMagenta => COLORS.mauve,
        Color::Cyan | Color::LightCyan => COLORS.teal,
        Color::Gray => COLORS.subtext1,
        Color::Indexed(index @ 0..=15) => {
            return ratatui_to_valence_color(color_from_index(index), color_type);
        }
        Color::Rgb(r, g, b) => return RgbColor { r, g, b },
        _ => match color_type {
            ColorType::Foreground => COLORS.text,
            ColorType::Background => COLORS.base,
        },
    })
}

const fn catppuccin_to_valence_color(color: catppuccin::Color) -> RgbColor {
    let Rgb { r, g, b } = color.rgb;
    RgbColor { r, g, b }
}

const fn color_from_index(index: u8) -> Color {
    match index {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::Gray,
        8 => Color::DarkGray,
        9 => Color::LightRed,
        10 => Color::LightGreen,
        11 => Color::LightYellow,
        12 => Color::LightBlue,
        13 => Color::LightMagenta,
        14 => Color::LightCyan,
        _ => Color::White,
    }
}
