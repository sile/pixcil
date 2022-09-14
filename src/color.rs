use pagurus_game_std::color::{Color, Rgba};

pub const CANVAS_BACKGROUND: Color = rgb(247, 246, 240); // W-0

pub const WINDOW_BORDER: Color = rgb(83, 80, 76); // W-9

pub const GRID_LINE_1: Color = rgba(0, 0, 0, 20);
pub const GRID_LINE_8: Color = rgba(0, 0, 0, 70);
pub const GRID_LINE_32: Color = rgba(0, 0, 0, 120);

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r, g, b)
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::Rgba(Rgba::new(r, g, b, a))
}
