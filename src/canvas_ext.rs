use pagurus::spatial::{Position, Region};
use pagurus_game_std::{
    color::Color,
    image::{Canvas, Sprite},
};

use crate::asset::Alphabet;

pub trait CanvasExt {
    fn draw_vertical_line(&mut self, start: Position, height: u32, color: Color);
    fn draw_horizontal_line(&mut self, start: Position, width: u32, color: Color);
    fn draw_rectangle(&mut self, rectangle: Region, color: Color);
    fn fill_rectangle(&mut self, rectangle: Region, color: Color);
    fn draw_sprite_with_alpha(&mut self, sprite: &Sprite, alpha: u8);
    fn draw_string(&mut self, text: &[Alphabet], margin: u32, sprites: &[Sprite; 26]);
}

impl<'a> CanvasExt for Canvas<'a> {
    fn draw_vertical_line(&mut self, start: Position, height: u32, color: Color) {
        for y in start.y..start.y + height as i32 {
            self.draw_pixel(Position::from_xy(start.x, y), color);
        }
    }

    fn draw_horizontal_line(&mut self, start: Position, width: u32, color: Color) {
        for x in start.x..start.x + width as i32 {
            self.draw_pixel(Position::from_xy(x, start.y), color);
        }
    }

    fn draw_rectangle(&mut self, rectangle: Region, color: Color) {
        if rectangle.is_empty() {
            return;
        }
        self.draw_horizontal_line(rectangle.start(), rectangle.size.width, color);
        self.draw_horizontal_line(
            rectangle.start().move_y(rectangle.size.height as i32 - 1),
            rectangle.size.width,
            color,
        );
        self.draw_vertical_line(
            rectangle.start().move_y(1),
            rectangle.size.height - 2,
            color,
        );
        self.draw_vertical_line(
            rectangle
                .start()
                .move_x(rectangle.size.width as i32 - 1)
                .move_y(1),
            rectangle.size.height - 2,
            color,
        );
    }

    fn fill_rectangle(&mut self, rectangle: Region, color: Color) {
        self.mask_region(rectangle).fill_color(color);
    }

    fn draw_sprite_with_alpha(&mut self, sprite: &Sprite, alpha: u8) {
        for (pos, mut pixel) in sprite.pixels() {
            if pixel.a != 0 {
                pixel.a = alpha;
            }
            self.draw_pixel(pos, Color::Rgba(pixel));
        }
    }

    fn draw_string(&mut self, text: &[Alphabet], margin: u32, sprites: &[Sprite; 26]) {
        let mut offset = Position::ORIGIN;
        for &x in text {
            let i = x as usize;
            self.offset(offset).draw_sprite(&sprites[i]);
            offset.x += (sprites[i].size().width + margin) as i32;
        }
    }
}
