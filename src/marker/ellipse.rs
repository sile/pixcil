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
                self.marked.clear();
                if let Some(center) = self.center {
                    let x_radius = (position.x - center.x).abs();
                    let y_radius = (position.y - center.y).abs();

                    // TODO: rename
                    let prob = |xi, yi| {
                        let mut count = 0;
                        for xj in 0..=10 {
                            for yj in 0..=10 {
                                let xv = (xi as f32 + 0.1 * xj as f32).powi(2)
                                    / (x_radius as f32).powi(2);
                                let yv = (yi as f32 + 0.1 * yj as f32).powi(2)
                                    / (y_radius as f32).powi(2);
                                if xv + yv <= 1.0 {
                                    count += 1;
                                }
                            }
                        }
                        count as f32 / (11 * 11) as f32
                    };

                    let x0 = center.x;
                    let y0 = center.y;
                    let mut xi = 0;
                    let mut yi = y_radius - 1;
                    while xi < x_radius && yi >= 0 {
                        self.marked.insert(PixelPosition::from_xy(x0 + xi, y0 + yi));
                        self.marked.insert(PixelPosition::from_xy(x0 - xi, y0 - yi));
                        self.marked.insert(PixelPosition::from_xy(x0 + xi, y0 - yi));
                        self.marked.insert(PixelPosition::from_xy(x0 - xi, y0 + yi));

                        if prob(xi + 1, yi) >= 0.5 {
                            xi += 1;
                        } else if prob(xi + 1, yi - 1) >= 0.5 {
                            xi += 1;
                            yi -= 1;
                        } else {
                            yi -= 1;
                        }
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
