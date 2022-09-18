use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

pub mod bottom_bar;
pub mod button;
pub mod pixel_canvas;
pub mod preview;
pub mod side_bar;
pub mod tool_box;
pub mod undo_redo;
pub mod zoom;

pub trait Widget {
    fn region(&self) -> Region;
    fn render(&self, app: &App, canvas: &mut Canvas);
    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()>;
    fn children(&mut self) -> Vec<&mut dyn Widget>;

    fn handle_event_before(&mut self, app: &mut App) -> Result<()> {
        for child in self.children() {
            child.handle_event_before(app).or_fail()?;
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
        Ok(())
    }

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
