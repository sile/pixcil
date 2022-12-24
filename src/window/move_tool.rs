use super::{widget::WidgetWindow, Window};
use crate::{app::App, event::Event, widget::move_tool::MoveToolWidget};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus::image::Canvas;

#[derive(Debug)]
pub struct MoveToolWindow(WidgetWindow<MoveToolWidget>);

impl MoveToolWindow {
    pub fn new(app: &App) -> Self {
        Self(WidgetWindow::with_margin(MoveToolWidget::new(app), 8))
    }
}

impl Window for MoveToolWindow {
    fn region(&self) -> Region {
        self.0.region()
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.0.render(app, canvas);
    }

    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        self.0.handle_screen_resized(app).or_fail()
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.0.handle_event(app, event).or_fail()
    }
}
