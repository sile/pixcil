use super::{stroke::StrokeMarker, Mark, MouseState};
use crate::{
    app::App,
    pixel::{PixelPosition, PixelRegion},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct LassoMarker {
    stroke: StrokeMarker,
    last: MouseState,
}

impl Mark for LassoMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        self.stroke.mark(app, position, mouse);
        self.last = mouse;
    }

    fn marked_pixels(&self, app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        if self.last != MouseState::Clicked {
            return self.stroke.marked_pixels(app);
        }

        let stroke = self.stroke.marked_pixels(app).collect::<HashSet<_>>();
        let region = PixelRegion::from_positions(stroke.iter().copied());
        let mut pixels = region.pixels().collect::<HashSet<_>>();
        let mut stack = region.edges().collect::<Vec<_>>();
        while let Some(p) = stack.pop() {
            if stroke.contains(&p) {
                continue;
            }
            if !pixels.remove(&p) {
                // Already visited or out-of-region
                continue;
            }
            stack.extend([p.move_x(-1), p.move_x(1), p.move_y(-1), p.move_y(1)]);
        }

        let pixel_canvas = &app.models().pixel_canvas;
        let mut drawn_pixels = Vec::new();
        for p in pixels {
            if pixel_canvas.get_pixel(p).is_some() {
                drawn_pixels.push(p);
            }
        }
        Box::new(drawn_pixels.into_iter())
    }
}