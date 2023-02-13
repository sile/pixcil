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
                    let x_radius = (position.x - center.x).abs();
                    let y_radius = (position.y - center.y).abs();

                    // TODO
                    let x_start = center.x - x_radius;
                    let y_start = center.y - y_radius;
                    let x_end = center.x + x_radius;
                    let y_end = center.y + y_radius;

                    self.marked.clear();
                    for x in x_start..=x_end {
                        self.marked.insert(PixelPosition::from_xy(x, y_start));
                        self.marked.insert(PixelPosition::from_xy(x, y_end));
                    }
                    for y in y_start..=y_end {
                        self.marked.insert(PixelPosition::from_xy(x_start, y));
                        self.marked.insert(PixelPosition::from_xy(x_end, y));
                    }

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
