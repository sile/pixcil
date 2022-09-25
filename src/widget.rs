use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

pub mod block;
pub mod bottom_bar;
pub mod button;
pub mod color_config;
pub mod color_selector;
pub mod config;
pub mod draw_tool;
pub mod erase_tool;
pub mod hsv_selector;
pub mod manipulate;
pub mod manipulate_tool;
pub mod move_camera;
pub mod move_tool;
pub mod number_box;
pub mod pixel_canvas;
pub mod preview;
pub mod rgb_selector;
pub mod save_load;
pub mod select_box;
pub mod select_tool;
pub mod side_bar;
pub mod slider;
pub mod toggle;
pub mod tool_box;
pub mod undo_redo;
pub mod zoom;

pub trait Widget: std::fmt::Debug + 'static {
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
