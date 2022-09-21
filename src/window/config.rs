use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::{Event, MouseAction},
    widget::{config::ConfigWidget, FixedSizeWidget, Widget},
};
use pagurus::{
    failure::OrFail,
    spatial::{Contains, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct ConfigWindow {
    region: Region,
    terminated: bool,
    config: ConfigWidget,
}

impl ConfigWindow {
    pub fn new(app: &App) -> Self {
        Self {
            region: Region::default(),
            terminated: false,
            config: ConfigWidget::new(app),
        }
    }
}

impl Window for ConfigWindow {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::WINDOW_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.config.render_if_need(app, canvas);
    }

    fn is_terminated(&self) -> bool {
        self.terminated
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        let size = self.config.requiring_size(app);
        let mut position = app.screen_size().to_region().center();
        position.x -= size.width as i32 / 2;
        position.y -= size.height as i32 / 2;

        self.region = Region::new(position, size);
        self.config.set_position(app, self.region.position);
        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.config.handle_event(app, event).or_fail()?;

        if let Event::Mouse {
            action, position, ..
        } = event
        {
            if *action == MouseAction::Up && !self.region.contains(position) {
                self.terminated = true;
            }
            event.consume();
        }
        Ok(())
    }
}
