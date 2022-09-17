use crate::{app::App, event::Event};
use pagurus::{
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

pub mod pixel_canvas;
pub mod preview;

pub trait Widget {
    fn region(&self) -> Region;
    fn render(&self, app: &App, canvas: &mut Canvas);
    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()>;

    fn render_if_need(&self, app: &App, canvas: &mut Canvas) {
        if !self
            .region()
            .intersection(canvas.drawing_region())
            .is_empty()
        {
            self.render(app, canvas);
        }
    }
}

pub trait FixedSizeWidget: Widget {
    fn requiring_size(&self, app: &App) -> Size;
    fn set_position(&mut self, app: &App, position: Position);
}

pub trait VariableSizeWidget: Widget {
    fn set_region(&mut self, app: &App, region: Region);
}
