use crate::{
    app::App,
    event::{Event, MouseAction},
    pixel::{PixelPosition, PixelRegion},
};
use pagurus::Result;
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseState {
    #[default]
    Neutral,
    Pressing,
    Clicked,
}

pub trait Mark: Default {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState);
    fn marked_pixels(&self) -> Box<dyn '_ + Iterator<Item = PixelPosition>>;
}

#[derive(Debug, Default)]
pub struct MarkerHandler<M> {
    marker: M,
    mouse: MouseState,
    last_event: Option<(PixelPosition, MouseAction)>,
    last_marked: HashSet<PixelPosition>,
}

impl<M: Mark> MarkerHandler<M> {
    pub fn marked_pixels(&self) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        self.marker.marked_pixels()
    }

    pub fn is_completed(&self) -> bool {
        self.mouse == MouseState::Clicked
    }

    pub fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let (pixel_position, action) = match event {
            Event::Mouse { consumed: true, .. } => {
                self.request_redraw(app, self.last_marked.iter().copied());
                *self = Self::default();
                return Ok(());
            }
            Event::Mouse {
                action, position, ..
            } => {
                let pixel_position = PixelPosition::from_screen_position(app, *position);
                (pixel_position, *action)
            }
            _ => return Ok(()),
        };
        event.consume();

        if self.last_event == Some((pixel_position, action)) {
            return Ok(());
        }
        self.last_event = Some((pixel_position, action));

        match action {
            MouseAction::Move if self.mouse == MouseState::Pressing => {}
            MouseAction::Down => {
                self.mouse = MouseState::Pressing;
            }
            MouseAction::Up if self.mouse == MouseState::Pressing => {
                self.mouse = MouseState::Clicked;
            }
            _ => {
                self.mouse = MouseState::Neutral;
            }
        }

        self.marker.mark(app, pixel_position, self.mouse);
        let marked = self.marker.marked_pixels().collect::<HashSet<_>>();
        if self.is_completed() {
            self.request_redraw(app, marked.union(&self.last_marked).copied());
        } else {
            self.request_redraw(app, marked.symmetric_difference(&self.last_marked).copied());
        }
        self.last_marked = marked;

        Ok(())
    }

    fn request_redraw(&self, app: &mut App, pixels: impl Iterator<Item = PixelPosition>) {
        app.request_redraw(PixelRegion::from_positions(pixels).to_screen_region(app));
    }
}
