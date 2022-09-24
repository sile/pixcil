use super::{Mark, MouseState};
use crate::{app::App, pixel::PixelPosition};

#[derive(Debug, Default)]
pub struct PickMarker {
    point: Option<PixelPosition>,
}

impl Mark for PickMarker {
    fn mark(&mut self, _app: &App, position: PixelPosition, _mouse: MouseState) {
        self.point = Some(position);
    }

    fn marked_pixels(&self, _app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        Box::new(self.point.iter().copied())
    }
}
