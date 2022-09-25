use super::{VariableSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    event::{Event, MouseAction},
    model::tool::ToolKind,
    pixel::{Pixel, PixelPosition},
};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus_game_std::{
    color::Rgba,
    image::{Canvas, Sprite},
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ManipulateWidget {
    region: Region,
    terminated: bool,
    selected_pixels: HashSet<PixelPosition>,
    manipulating_pixels: HashMap<PixelPosition, Rgba>,
    delta: PixelPosition,
    state: State,
    imported: bool,
}

impl ManipulateWidget {
    pub fn new(app: &App, selected_pixels: HashSet<PixelPosition>) -> Self {
        let manipulating_pixels = selected_pixels
            .iter()
            .copied()
            .filter_map(|pos| {
                app.models()
                    .pixel_canvas
                    .get_direct_pixel(pos)
                    .map(|color| (pos, color))
            })
            .collect();
        Self {
            region: app.screen_size().to_region(),
            selected_pixels,
            manipulating_pixels,
            delta: PixelPosition::from_xy(0, 0),
            terminated: false,
            state: State::Neutral,
            imported: false,
        }
    }

    pub fn with_imported_image(app: &App, image: &Sprite) -> Self {
        let screen_region = app.screen_size().to_region();
        let mut base = PixelPosition::from_screen_position(app, screen_region.center());

        let image_center = image.size().to_region().center();
        base.x -= image_center.x as i16;
        base.y -= image_center.y as i16;

        let manipulating_pixels = image
            .pixels()
            .filter_map(|(position, color)| {
                if color.a == 0 {
                    return None;
                }

                let mut pixel_position = base;
                pixel_position.x += position.x as i16;
                pixel_position.y += position.y as i16;
                Some((pixel_position, color))
            })
            .collect();
        Self {
            region: screen_region,
            selected_pixels: HashSet::default(),
            manipulating_pixels,
            delta: PixelPosition::from_xy(0, 0),
            terminated: false,
            state: State::Neutral,
            imported: true,
        }
    }

    pub fn selected_pixels(&self) -> &HashSet<PixelPosition> {
        &self.selected_pixels
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }

    fn render_manipulating_pixels(&self, app: &App, canvas: &mut Canvas) {
        for (position, color) in &self.manipulating_pixels {
            let region = (*position + self.delta).to_screen_region(app);
            let mut color = *color;
            color.a /= match self.state {
                State::Neutral => 2,
                State::Focused => 3,
                State::Dragging { .. } => 4,
            }; // TODO
            canvas.fill_rectangle(region, color.into());
        }
    }

    fn handle_terminate(&mut self, app: &mut App) -> Result<()> {
        let config = app.models().config.clone();
        if self.imported {
            app.models_mut()
                .pixel_canvas
                .draw_pixels(
                    &config,
                    self.manipulating_pixels
                        .iter()
                        .map(|(position, color)| Pixel::new(*position + self.delta, *color)),
                )
                .or_fail()?;
        } else {
            app.models_mut()
                .pixel_canvas
                .move_pixels(
                    &config,
                    self.manipulating_pixels.keys().copied(),
                    self.delta,
                )
                .or_fail()?;
        }

        Ok(())
    }
}

impl Widget for ManipulateWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.render_manipulating_pixels(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let prev = (self.state, self.delta);
        match (self.state, &event) {
            (
                State::Neutral | State::Focused,
                Event::Mouse {
                    consumed: false,
                    action: MouseAction::Move,
                    position,
                },
            ) => {
                let unit = app.models().config.minimum_pixel_size;
                let abs_pixel_position =
                    unit.align(PixelPosition::from_screen_position(app, *position));
                let pixel_position = abs_pixel_position - self.delta;
                if self.manipulating_pixels.contains_key(&pixel_position) {
                    self.state = State::Focused;
                } else {
                    self.state = State::Neutral;
                }
            }
            (
                State::Neutral | State::Focused,
                Event::Mouse {
                    consumed: false,
                    action: MouseAction::Down,
                    position,
                },
            ) => {
                let unit = app.models().config.minimum_pixel_size;
                let abs_pixel_position =
                    unit.align(PixelPosition::from_screen_position(app, *position));
                let pixel_position =
                    PixelPosition::from_screen_position(app, *position) - self.delta;
                // TODO: Consider non drawn pixels in the target region
                if self.manipulating_pixels.contains_key(&pixel_position) {
                    self.state = State::Dragging {
                        start: abs_pixel_position,
                    };
                } else {
                    self.terminated = true;
                }
            }
            (
                State::Dragging { start },
                Event::Mouse {
                    consumed: false,
                    action: MouseAction::Move,
                    position,
                },
            ) => {
                let unit = app.models().config.minimum_pixel_size;
                let abs_pixel_position =
                    unit.align(PixelPosition::from_screen_position(app, *position));
                let moved = abs_pixel_position - start;
                self.delta.x += moved.x;
                self.delta.y += moved.y;
                self.state = State::Dragging {
                    start: abs_pixel_position,
                };
            }
            (
                State::Dragging { start },
                Event::Mouse {
                    consumed: false,
                    action: MouseAction::Up,
                    position,
                },
            ) => {
                let unit = app.models().config.minimum_pixel_size;
                let abs_pixel_position =
                    unit.align(PixelPosition::from_screen_position(app, *position));
                let moved = abs_pixel_position - start;
                self.delta.x += moved.x;
                self.delta.y += moved.y;
                self.state = State::Focused;
            }
            (_, Event::Mouse { .. }) => {
                self.state = State::Neutral;
            }
            _ => {}
        }
        event.consume_if_contained(self.region);

        if prev != (self.state, self.delta) {
            // TODO: optimize
            app.request_redraw(self.region);
        }

        if self.terminated {
            self.handle_terminate(app).or_fail()?;
        }

        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        if app.models().tool.current != ToolKind::Select {
            self.terminated = true;
            app.request_redraw(self.region);
            self.handle_terminate(app).or_fail()?;
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl VariableSizeWidget for ManipulateWidget {
    fn set_region(&mut self, _app: &App, region: Region) {
        self.region = region;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Neutral,
    Focused,
    Dragging {
        start: PixelPosition,
    },
}
