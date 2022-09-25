use super::{Mark, MouseState};
use crate::{app::App, pixel::PixelPosition};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct FillMarker {
    marked: HashSet<PixelPosition>,
    cannot_fill: HashSet<PixelPosition>,
}

impl FillMarker {
    fn fill_same_color_area(&mut self, app: &App, position: PixelPosition) {
        self.marked.clear();
        let pixel_canvas = &app.models().pixel_canvas;
        let region = pixel_canvas.region();
        let color = pixel_canvas.get_direct_pixel(position);
        let mut stack = vec![position];
        let mut visited = HashSet::new();
        while let Some(position) = stack.pop() {
            if color.is_none() && !region.contains(position) {
                self.cannot_fill.extend(self.marked.drain());
                break;
            }
            if !visited.insert(position) {
                // Already visited.
                continue;
            }
            if pixel_canvas.get_direct_pixel(position) != color {
                continue;
            }

            self.marked.insert(position);
            stack.extend([
                position.move_x(-1),
                position.move_x(1),
                position.move_y(-1),
                position.move_y(1),
            ]);
        }
    }
}

impl Mark for FillMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, _mouse: MouseState) {
        if self.marked.contains(&position) || self.cannot_fill.contains(&position) {
            return;
        }

        self.fill_same_color_area(app, position);
    }

    fn marked_pixels(&self, _app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        Box::new(self.marked.iter().copied())
    }
}
