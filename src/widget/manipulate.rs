use super::{VariableSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    event::{Event, MouseAction},
    model::tool::ToolKind,
    pixel::PixelPosition,
};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus_game_std::{color::Rgba, image::Canvas};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ManipulateWidget {
    region: Region,
    terminated: bool,
    selected_pixels: HashSet<PixelPosition>,
    manipulating_pixels: HashMap<PixelPosition, Rgba>,
    delta: PixelPosition,
    state: State,
}

impl ManipulateWidget {
    pub fn new(app: &mut App, selected_pixels: HashSet<PixelPosition>) -> Self {
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
            let config = app.models().config.clone();
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

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        if app.models().tool.current != ToolKind::Select {
            self.terminated = true;
            app.request_redraw(self.region);
            let config = app.models().config.clone();
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
