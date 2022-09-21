use super::{Mark, MouseState};
use crate::{
    app::App,
    model::config::MinimumPixelSize,
    pixel::{PixelLine, PixelPosition},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct StrokeMarker {
    last: Option<PixelPosition>,
    unit: MinimumPixelSize,
    marked: HashSet<PixelPosition>,
}

impl Mark for StrokeMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        self.unit = app.models().config.minimum_pixel_size;
        let position = self.unit.normalize(position);
        if let Some(last) = self.last {
            self.marked.extend(PixelLine::new(last, position).pixels());
        } else {
            self.marked = [position].into_iter().collect()
        }
        if mouse == MouseState::Pressing {
            self.last = Some(position);
        } else {
            self.last = None;
        }
    }

    fn marked_pixels(&self) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        Box::new(
            self.marked
                .iter()
                .copied()
                .flat_map(|p| self.unit.denormalize_to_region(p).pixels()),
        )
    }
}
