use super::{Mark, MouseState, stroke::StrokeMarker};
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

        Box::new(pixels.into_iter())
    }
}
