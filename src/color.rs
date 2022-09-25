use pagurus_game_std::color::{Color, Rgb, Rgba};

pub const CANVAS_BACKGROUND: Color = rgb(245, 245, 245);
pub const CANVAS_PREVIEW_MODE_BACKGROUND: Color = rgb(200, 200, 200);

pub const WINDOW_BORDER: Color = rgb(83, 80, 76); // W-9
pub const WINDOW_BACKGROUND: Color = BUTTONS_BACKGROUND;

pub const GRID_LINE_1: Color = rgba(0, 0, 0, 20);
pub const GRID_LINE_8: Color = rgba(0, 0, 0, 70);
pub const GRID_LINE_32: Color = rgba(0, 0, 0, 120);

pub const PREVIEW_BACKGROUND: Color = rgba(255, 255, 255, 255);
pub const PREVIEW_BORDER: Color = WINDOW_BORDER;
pub const PREVIEW_FOCUSED_BORDER: Color = rgb(255, 0, 0);

pub const BUTTONS_BACKGROUND: Color = rgb(221, 220, 213); // W-3

pub const FRAME_EDGE: Color = rgb(160, 160, 160);
pub const CURRENT_FRAME_EDGE: Color = rgb(80, 80, 80);

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r, g, b)
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::Rgba(Rgba::new(r, g, b, a))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

impl Hsv {
    pub fn from_rgb(rgb: Rgb) -> Self {
        let r = rgb.r as f64 / 255.0;
        let g = rgb.g as f64 / 255.0;
        let b = rgb.b as f64 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let n = max - min;

        let s = if max == 0.0 { 0.0 } else { n / max };
        let v = max;
        let h = if n == 0.0 {
            0.0
        } else if max == r {
            if g < b {
                6.0 + g / n - b / n
            } else {
                (g - b) / n
            }
        } else if max == g {
            2.0 + b / n - r / n
        } else {
            4.0 + r / n - g / n
        } / 6.0;

        Self { h, s, v }
    }

    pub fn to_rgb(self) -> Rgb {
        if self.s == 0.0 {
            let v = (self.v * 255.0).round() as u8;
            return Rgb::new(v, v, v);
        }

        let mut r = self.v;
        let mut g = self.v;
        let mut b = self.v;
        let s = self.s;
        let h6 = self.h * 6.0;

        let f = h6.fract();
        match h6.floor() as usize {
            1 => {
                r *= 1.0 - s * f;
                b *= 1.0 - s;
            }
            2 => {
                r *= 1.0 - s;
                b *= 1.0 - s * (1.0 - f);
            }
            3 => {
                r *= 1.0 - s;
                g *= 1.0 - s * f;
            }
            4 => {
                r *= 1.0 - s * (1.0 - f);
                g *= 1.0 - s;
            }
            5 => {
                g *= 1.0 - s;
                b *= 1.0 - s * f;
            }
            n => {
                debug_assert!(n == 0 || n == 6, "n: {}", n);
                g *= 1.0 - s * (1.0 - f);
                b *= 1.0 - s;
            }
        }

        Rgb::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_hsv_works() {
        let inputs = [(255, 0, 0), (10, 30, 200), (222, 222, 222)];
        for i in inputs {
            let hsv = Hsv::from_rgb(Rgb::new(i.0, i.1, i.2));
            let Rgb { r, g, b } = hsv.to_rgb();

            dbg!(i);
            dbg!((r, g, b));

            assert!((i32::from(r) - i32::from(i.0)).abs() <= 2);
            assert!((i32::from(g) - i32::from(i.1)).abs() <= 2);
            assert!((i32::from(b) - i32::from(i.2)).abs() <= 2);
        }
    }
}
