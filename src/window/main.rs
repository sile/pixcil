use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    widget::{
        bottom_bar::BottomBarWidget, pixel_canvas::PixelCanvasWidget, preview::PreviewWidget,
        side_bar::SideBarWidget, FixedSizeWidget, VariableSizeWidget, Widget,
    },
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct MainWindow {
    size: Size,
    pixel_canvas: PixelCanvasWidget,
    preview: PreviewWidget,
    side_bar: SideBarWidget,
    bottom_bar: BottomBarWidget,
}

impl MainWindow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Window for MainWindow {
    fn region(&self) -> Region {
        self.size.to_region()
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.pixel_canvas.render(app, canvas);
        self.preview.render_if_need(app, canvas);
        self.side_bar.render_if_need(app, canvas);
        self.bottom_bar.render_if_need(app, canvas);
        canvas.draw_rectangle(self.region(), color::WINDOW_BORDER);
    }

    fn is_terminated(&self) -> bool {
        false
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        self.size = app.screen_size();

        self.pixel_canvas.set_region(app, self.region());

        let preview_margin = 16;
        let preview_size = self.preview.requiring_size(app);
        let preview_position = Position::from_xy(
            app.screen_size().width as i32 - preview_size.width as i32 - preview_margin,
            preview_margin,
        );
        self.preview.set_position(app, preview_position);

        self.side_bar.set_region(app, self.region());
        self.bottom_bar.set_region(app, self.region());

        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.pixel_canvas.handle_event_before(app).or_fail()?;
        self.preview.handle_event_before(app).or_fail()?;
        self.side_bar.handle_event_before(app).or_fail()?;
        self.bottom_bar.handle_event_before(app).or_fail()?;

        if !self.pixel_canvas.marker_handler().is_operating() {
            self.preview.handle_event(app, event).or_fail()?;
            self.side_bar.handle_event(app, event).or_fail()?;
            self.bottom_bar.handle_event(app, event).or_fail()?;
        }
        self.pixel_canvas.handle_event(app, event).or_fail()?;
        self.pixel_canvas
            .set_preview_focused(app, self.preview.is_focused());

        self.bottom_bar.handle_event_after(app).or_fail()?;
        self.side_bar.handle_event_after(app).or_fail()?;
        self.preview.handle_event_after(app).or_fail()?;
        self.pixel_canvas.handle_event_after(app).or_fail()?;

        Ok(())
    }
}
