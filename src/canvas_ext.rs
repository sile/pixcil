use pagurus::spatial::{Position, Region};
use pagurus_game_std::{color::Color, image::Canvas};

pub trait CanvasExt {
    fn draw_vertical_line(&mut self, start: Position, height: u32, color: Color);
    fn draw_horizontal_line(&mut self, start: Position, width: u32, color: Color);
    fn draw_rectangle(&mut self, rectangle: Region, color: Color);
    fn fill_rectangle(&mut self, rectangle: Region, color: Color);
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
}
