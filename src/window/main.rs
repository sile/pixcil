use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    widget::{pixel_canvas::PixelCanvasWidget, VariableSizeWidget, Widget},
};
use pagurus::{
    failure::OrFail,
    spatial::{Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct MainWindow {
    size: Size,
    pixel_canvas: PixelCanvasWidget,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            size: Size::default(),
            pixel_canvas: PixelCanvasWidget::default(),
        }
    }
}

impl Window for MainWindow {
    fn region(&self) -> Region {
        self.size.to_region()
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.pixel_canvas.render(app, canvas);
        canvas.draw_rectangle(self.region(), color::WINDOW_BORDER);
        log::info!("render: {:?}", self.size);
    }

    fn is_terminated(&self) -> bool {
        false
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        self.size = app.screen_size();
        self.pixel_canvas.set_region(app, self.region());
        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.pixel_canvas.handle_event(app, event).or_fail()?;
        Ok(())
    }
}
