use crate::{
    app::App,
    event::{Event, MouseAction},
    pixel::{PixelPosition, PixelRegion},
};
use pagurus::Result;
use std::collections::HashSet;

use self::{
    fill::FillMarker, lasso::LassoMarker, line::LineMarker, noop::NoopMarker, pick::PickMarker,
    stroke::StrokeMarker,
};

pub mod fill;
pub mod lasso;
pub mod line;
pub mod noop;
pub mod pick;
pub mod stroke;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseState {
    #[default]
    Neutral,
    Pressing,
    Clicked,
}

pub trait Mark: Default {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState);
    fn marked_pixels(&self, app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkerKind {
    #[default]
    Stroke,
    Line,
    Noop,
    Lasso,
    Pick,
    Fill,
}

#[derive(Debug)]
pub enum Marker {
    Stroke(StrokeMarker),
    Line(LineMarker),
    Noop(NoopMarker),
    Lasso(LassoMarker),
    Pick(PickMarker),
    Fill(FillMarker),
}

impl Marker {
    fn from_kind(kind: MarkerKind) -> Self {
        match kind {
            MarkerKind::Stroke => Self::Stroke(Default::default()),
            MarkerKind::Line => Self::Line(Default::default()),
            MarkerKind::Noop => Self::Noop(Default::default()),
            MarkerKind::Lasso => Self::Lasso(Default::default()),
            MarkerKind::Pick => Self::Pick(Default::default()),
            MarkerKind::Fill => Self::Fill(Default::default()),
        }
    }
}

impl Default for Marker {
    fn default() -> Self {
        Self::Stroke(Default::default())
    }
}

impl Mark for Marker {
    fn mark(&mut self, app: &App, position: PixelPosition, mouse: MouseState) {
        match self {
            Marker::Stroke(x) => x.mark(app, position, mouse),
            Marker::Line(x) => x.mark(app, position, mouse),
            Marker::Noop(x) => x.mark(app, position, mouse),
            Marker::Lasso(x) => x.mark(app, position, mouse),
            Marker::Pick(x) => x.mark(app, position, mouse),
            Marker::Fill(x) => x.mark(app, position, mouse),
        }
    }

    fn marked_pixels(&self, app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        match self {
            Marker::Stroke(x) => x.marked_pixels(app),
            Marker::Line(x) => x.marked_pixels(app),
            Marker::Noop(x) => x.marked_pixels(app),
            Marker::Lasso(x) => x.marked_pixels(app),
            Marker::Pick(x) => x.marked_pixels(app),
            Marker::Fill(x) => x.marked_pixels(app),
        }
    }
}

#[derive(Debug, Default)]
pub struct MarkerHandler {
    marker: Marker,
    mouse: MouseState,
    last_event: Option<(PixelPosition, MouseAction)>,
    last_marked: HashSet<PixelPosition>,
    updated: bool,
}

impl MarkerHandler {
    pub fn marker_kind(&self) -> MarkerKind {
        match self.marker {
            Marker::Stroke(_) => MarkerKind::Stroke,
            Marker::Line(_) => MarkerKind::Line,
            Marker::Noop(_) => MarkerKind::Noop,
            Marker::Lasso(_) => MarkerKind::Lasso,
            Marker::Pick(_) => MarkerKind::Pick,
            Marker::Fill(_) => MarkerKind::Fill,
        }
    }

    pub fn set_marker_kind(&mut self, kind: MarkerKind) {
        self.marker = Marker::from_kind(kind);
    }

    pub fn marked_pixels(&self, app: &App) -> Box<dyn '_ + Iterator<Item = PixelPosition>> {
        if self.updated {
            self.marker.marked_pixels(app)
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn is_completed(&self) -> bool {
        self.mouse == MouseState::Clicked
    }

    pub fn is_operating(&self) -> bool {
        self.mouse == MouseState::Pressing
    }

    pub fn is_neutral(&self) -> bool {
        self.mouse == MouseState::Neutral
    }

    pub fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.updated = false;

        let (pixel_position, action) = match event {
            Event::Mouse { consumed: true, .. } => {
                self.request_redraw(app, self.last_marked.iter().copied());

                let kind = self.marker_kind();
                *self = Self {
                    marker: Marker::from_kind(kind),
                    ..Self::default()
                };
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

        let old_mouse = self.mouse;
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
        let marked = self.marker.marked_pixels(app).collect::<HashSet<_>>();
        if old_mouse != self.mouse {
            self.request_redraw(app, marked.union(&self.last_marked).copied());
        } else {
            self.request_redraw(app, marked.symmetric_difference(&self.last_marked).copied());
        }
        self.last_marked = marked;
        self.updated = true;

        Ok(())
    }

    fn request_redraw(&self, app: &mut App, pixels: impl Iterator<Item = PixelPosition>) {
        let region = PixelRegion::from_positions(pixels).to_screen_region(app);
        app.request_redraw(region);
    }
}
