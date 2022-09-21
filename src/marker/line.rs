use super::{Mark, MouseState};
use crate::{
    app::App,
    model::config::MinimumPixelSize,
    pixel::{PixelLine, PixelPosition},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct LineMarker {
    start: Option<PixelPosition>,
    unit: MinimumPixelSize,
    marked: HashSet<PixelPosition>,
}

impl Mark for LineMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        self.unit = app.models().config.minimum_pixel_size;
        let position = self.unit.normalize(position);
        match mouse {
            MouseState::Neutral => {
                self.start = None;
                self.marked = [position].into_iter().collect();
            }
            MouseState::Pressing => {
                if let Some(start) = self.start {
                    self.marked = PixelLine::new(start, position).pixels().collect();
                } else {
                    self.start = Some(position);
                    self.marked = [position].into_iter().collect()
                }
            }
            MouseState::Clicked => {
                if let Some(start) = self.start {
                    self.marked = PixelLine::new(start, position).pixels().collect();
                    self.start = None;
                } else {
                    self.marked = [position].into_iter().collect()
                }
            }
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
