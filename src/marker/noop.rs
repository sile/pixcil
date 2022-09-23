use super::{Mark, MouseState};
use crate::{app::App, pixel::PixelPosition};

#[derive(Debug, Default)]
pub struct NoopMarker {}

impl Mark for NoopMarker {
    fn mark(&mut self, _app: &App, _position: PixelPosition, _mouse: MouseState) {}

    fn marked_pixels(&self, _app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        Box::new(std::iter::empty())
    }
}
