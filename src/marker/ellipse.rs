use super::{Mark, MouseState};
use crate::{
    app::App,
    pixel::{PixelPosition, PixelRegion},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct EllipseMarker {
    start: Option<PixelPosition>,
    marked: HashSet<PixelPosition>,
}

impl Mark for EllipseMarker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        let unit = app.models().config.minimum_pixel_size;
        let position = unit.normalize(position);
        match mouse {
            MouseState::Neutral => {
                self.start = None;
                self.marked = [position].into_iter().collect();
            }
            MouseState::Pressing | MouseState::Clicked => {
                self.marked.clear();
                if let Some(start) = self.start {
                    let mut region = PixelRegion::from_positions([start, position].into_iter());
                    region.start.x -= 1;
                    region.start.y -= 1;

                    let x_radius = (region.end.x as f32 - region.start.x as f32) / 2.0;
                    let y_radius = (region.end.y as f32 - region.start.y as f32) / 2.0;
                    let x_radius2 = x_radius.powi(2);
                    let y_radius2 = y_radius.powi(2);
                    let center_x = x_radius + region.start.x as f32;
                    let center_y = y_radius + region.start.y as f32;

                    let ratio = |xi, yi| {
                        let mut count = 0;
                        for xj in 0..=10 {
                            for yj in 0..=10 {
                                let xv = (xi as f32 + 0.1 * xj as f32).powi(2) / x_radius2;
                                let yv = (yi as f32 + 0.1 * yj as f32).powi(2) / y_radius2;
                                if xv + yv <= 1.0 {
                                    count += 1;
                                }
                            }
                        }
                        count as f32 / (11 * 11) as f32
                    };

                    let mut xi = x_radius.fract();
                    let mut yi = y_radius - 1.0;
                    while xi < x_radius && yi >= 0.0 {
                        let px = (center_x + xi) as i16;
                        let mx = (center_x - xi) as i16;
                        let py = (center_y + yi) as i16;
                        let my = (center_y - yi) as i16;
                        self.marked.insert(PixelPosition::from_xy(px, py));
                        self.marked.insert(PixelPosition::from_xy(mx, my));
                        self.marked.insert(PixelPosition::from_xy(px, my));
                        self.marked.insert(PixelPosition::from_xy(mx, py));

                        if ratio(xi + 1.0, yi) >= 0.5 {
                            xi += 1.0;
                        } else if ratio(xi + 1.0, yi - 1.0) >= 0.5 {
                            xi += 1.0;
                            yi -= 1.0;
                        } else {
                            yi -= 1.0;
                        }
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
