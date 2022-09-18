use super::{FixedSizeWidget, VariableSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct ToolBoxWidget {
    region: Region,
}

impl Widget for ToolBoxWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl FixedSizeWidget for ToolBoxWidget {
    fn requiring_size(&self, app: &App) -> Size {
        Size::from_wh(100, 100) //TODO
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
