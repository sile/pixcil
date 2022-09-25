use super::{Mark, MouseState};
use crate::{app::App, pixel::PixelPosition};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct EllipseMarker {
    center: Option<PixelPosition>,
    marked: HashSet<PixelPosition>,
}

impl Mark for EllipseMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        let unit = app.models().config.minimum_pixel_size;
        let position = unit.normalize(position);
        match mouse {
            MouseState::Neutral => {
                self.center = None;
                self.marked = [position].into_iter().collect();
            }
            MouseState::Pressing | MouseState::Clicked => {
                if let Some(center) = self.center {
                    let _x_radius = (position.x - center.x).abs();
                    let _y_radius = (position.y - center.y).abs();

                    // TODO

                    if mouse == MouseState::Clicked {
                        self.center = None;
                    }
                } else {
                    self.center = Some(position);
                    self.marked = [position].into_iter().collect()
                }
            }
        }
    }

    fn marked_pixels(&self, app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        let unit = app.models().config.minimum_pixel_size;
        Box::new(
            self.marked
                .iter()
                .copied()
                .flat_map(move |p| unit.denormalize_to_region(p).pixels()),
        )
    }
}
