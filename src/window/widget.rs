use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::{Event, MouseAction},
    widget::FixedSizeWidget,
};
use pagurus::{
    failure::OrFail,
    spatial::{Contains, Region},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct WidgetWindow<W> {
    region: Region,
    terminated: bool,
    widget: W,
}

impl<W: FixedSizeWidget> WidgetWindow<W> {
    pub fn new(widget: W) -> Self {
        Self {
            region: Region::default(),
            terminated: false,
            widget,
        }
    }
}

impl<W: FixedSizeWidget> Window for WidgetWindow<W> {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::WINDOW_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.widget.render_if_need(app, canvas);
    }

    fn is_terminated(&self) -> bool {
        self.terminated
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        let size = self.widget.requiring_size(app);
        let mut position = app.screen_size().to_region().center();
        position.x -= size.width as i32 / 2;
        position.y -= size.height as i32 / 2;

        self.region = Region::new(position, size);
        self.widget.set_position(app, self.region.position);
        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.widget.handle_event(app, event).or_fail()?;

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
