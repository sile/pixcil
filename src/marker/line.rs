use std::collections::HashSet;

use super::{Mark, MouseState};
use crate::{
    app::App,
    model::config::Unit,
    pixel::{PixelLine, PixelPosition},
};

#[derive(Debug, Default)]
pub struct LineMarker {
    start: Option<PixelPosition>,
    unit: Unit,
    marked: HashSet<PixelPosition>,
}

impl Mark for LineMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        self.unit = app.models().config.unit;
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
