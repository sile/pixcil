use super::{manipulate_tool::ManipulateToolWidget, FixedSizeWidget, VariableSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    event::{Event, MouseAction},
    model::tool::ToolKind,
    pixel::{Pixel, PixelPosition, PixelRegion},
};
use pagurus::image::{Canvas, Rgba, Sprite};
use pagurus::{
    failure::OrFail,
    spatial::{Contains, Position, Region},
    Result,
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
    tool: ManipulateToolWidget,
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
        let mut this = Self {
            region: app.screen_size().to_region(),
            selected_pixels,
            manipulating_pixels,
            delta: PixelPosition::from_xy(0, 0),
            terminated: false,
            state: State::Neutral,
            tool: ManipulateToolWidget::default(),
        };
        this.set_region(app, app.screen_size().to_region());
        app.request_redraw(this.tool.region());
        this
    }

    pub fn is_dragging(&self) -> bool {
        matches!(self.state, State::Dragging { .. })
    }

    pub fn with_imported_image(app: &App, image: &Sprite) -> Self {
        let screen_region = app.screen_size().to_region();
        let mut base = PixelPosition::from_screen_position(app, screen_region.center());

        let image_center = image.size().to_region().center();
        base.x -= image_center.x as i16;
        base.y -= image_center.y as i16;

        base = app.models().config.minimum_pixel_size.align(base);

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
        let mut this = Self {
            region: screen_region,
            selected_pixels: HashSet::default(),
            manipulating_pixels,
            delta: PixelPosition::from_xy(0, 0),
            terminated: false,
            state: State::Neutral,
            tool: ManipulateToolWidget::default(),
        };
        this.set_region(app, app.screen_size().to_region());
        this
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
        app.models_mut()
            .pixel_canvas
            .erase_and_draw_pixels(
                &config,
                self.selected_pixels.iter().copied(),
                self.manipulating_pixels
                    .iter()
                    .map(|(position, color)| Pixel::new(*position + self.delta, *color)),
            )
            .or_fail()?;

        Ok(())
    }

    fn vertical_flip(&mut self, app: &mut App) {
        let region = PixelRegion::from_positions(self.manipulating_pixels.keys().copied());
        let center = region.center();
        let is_even = region.size().height % 2 == 0;
        self.manipulating_pixels = self
            .manipulating_pixels
            .drain()
            .map(|(mut position, color)| {
                position.y = center.y - (position.y - center.y);
                if is_even {
                    position.y -= 1;
                }
                (position, color)
            })
            .collect();
        app.request_redraw(region.to_screen_region(app));
    }

    fn horizontal_flip(&mut self, app: &mut App) {
        let region = PixelRegion::from_positions(self.manipulating_pixels.keys().copied());
        let center = region.center();
        let is_even = region.size().width % 2 == 0;
        self.manipulating_pixels = self
            .manipulating_pixels
            .drain()
            .map(|(mut position, color)| {
                position.x = center.x - (position.x - center.x);
                if is_even {
                    position.x -= 1;
                }
                (position, color)
            })
            .collect();
        app.request_redraw(region.to_screen_region(app));
    }

    fn clockwise_rotate(&mut self, app: &mut App) {
        let region = PixelRegion::from_positions(self.manipulating_pixels.keys().copied());
        app.request_redraw(region.to_screen_region(app));

        let start = region.start;
        let center = region.center();

        let temp = self
            .manipulating_pixels
            .drain()
            .map(|(position, color)| {
                let delta = position - center;
                let position = PixelPosition::from_xy(center.x - delta.y, center.y + delta.x);
                (position, color)
            })
            .collect::<Vec<_>>();

        let delta = PixelRegion::from_positions(temp.iter().map(|x| x.0)).start - start;
        self.manipulating_pixels = temp
            .into_iter()
            .map(|(mut position, color)| {
                position.x -= delta.x;
                position.y -= delta.y;
                (position, color)
            })
            .collect();

        let region = PixelRegion::from_positions(self.manipulating_pixels.keys().copied());
        app.request_redraw(region.to_screen_region(app));
    }

    pub fn is_consumed_by_tool(&self, event: &Event) -> bool {
        if let Some(p) = event.position() {
            !matches!(self.state, State::Dragging { .. }) && self.tool.region().contains(&p)
        } else {
            false
        }
    }
}

impl Widget for ManipulateWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.render_manipulating_pixels(app, canvas);
        self.tool.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if !matches!(self.state, State::Dragging { .. }) {
            self.tool.handle_event(app, event).or_fail()?;
            if self.tool.is_cut_clicked(app) {
                self.terminated = true;
                let config = app.models().config.clone();
                app.models_mut()
                    .pixel_canvas
                    .erase_pixels(&config, self.selected_pixels.iter().copied())
                    .or_fail()?;
                app.request_redraw(app.screen_size().to_region());
                return Ok(());
            }
            if self.tool.is_copy_clicked(app) {
                self.handle_terminate(app).or_fail()?; // TODO: rename
                self.selected_pixels.clear();

                let unit = app.models().config.minimum_pixel_size.get();
                self.delta.x += unit.width as i16;
                self.delta.y += unit.height as i16;
                app.request_redraw(app.screen_size().to_region());
            }
            if self.tool.is_vertical_flip_clicked(app) {
                self.vertical_flip(app);
            }
            if self.tool.is_horizontal_flip_clicked(app) {
                self.horizontal_flip(app);
            }
            if self.tool.is_clockwise_rotate_clicked(app) {
                self.clockwise_rotate(app);
            }
        }

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
        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.tool]
    }
}

impl VariableSizeWidget for ManipulateWidget {
    fn set_region(&mut self, app: &App, region: Region) {
        self.region = region;

        let margin = 8;
        let tool_size = self.tool.requiring_size(app);
        let tool_position = Position::from_xy(
            region.size.width as i32 - margin - tool_size.width as i32,
            region.size.height as i32 / 2 - tool_size.height as i32 / 2,
        );
        self.tool.set_position(app, tool_position);
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
