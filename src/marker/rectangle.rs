use super::{Mark, MouseState};
use crate::{
    app::App,
    pixel::{PixelPosition, PixelRegion},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct RectangleMarker {
    start: Option<PixelPosition>,
    marked: HashSet<PixelPosition>,
    fill: bool,
}

impl RectangleMarker {
    pub fn fill() -> Self {
        Self {
            fill: true,
            ..Self::default()
        }
    }
}

impl Mark for RectangleMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        let unit = app.models().config.minimum_pixel_size;
        let position = unit.normalize(position);
        match mouse {
            MouseState::Neutral => {
                self.start = None;
                self.marked = [position].into_iter().collect();
            }
            MouseState::Pressing | MouseState::Clicked => {
                if let Some(start) = self.start {
                    let end = PixelPosition::from_xy(
                        start.x.max(position.x) + 1,
                        start.y.max(position.y) + 1,
                    );
                    let start =
                        PixelPosition::from_xy(start.x.min(position.x), start.y.min(position.y));
                    let region = PixelRegion::new(start, end);
                    if self.fill {
                        self.marked = region.pixels().collect();
                    } else {
                        self.marked = region.edges().collect();
                    }
                    if mouse == MouseState::Clicked {
                        self.start = None;
                    }
                } else {
                    self.start = Some(position);
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
