use std::collections::HashSet;

use super::{Mark, MouseState};
use crate::{app::App, pixel::PixelPosition};

#[derive(Debug, Default)]
pub struct LineMarker {
    start: Option<PixelPosition>,
    marked: HashSet<PixelPosition>,
}

impl Mark for LineMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        self.marked = [position].into_iter().collect()
    }

    fn marked_pixels(&self) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        Box::new(self.marked.iter().copied())
    }
}
