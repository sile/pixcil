use crate::{app::App, pixel::PixelPosition};

use super::{Mark, MouseState};

#[derive(Debug, Default)]
pub struct LassoMarker {}

impl Mark for LassoMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        todo!()
    }

    fn marked_pixels(&self) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        todo!()
    }
}
