use super::{widget::WidgetWindow, Window};
use crate::{app::App, event::Event, widget::config::ConfigWidget};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct ConfigWindow(WidgetWindow<ConfigWidget>);

impl ConfigWindow {
    pub fn new(app: &App) -> Self {
        Self(WidgetWindow::new(ConfigWidget::new(app)))
    }
}

impl Window for ConfigWindow {
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
