use super::{VariableSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct BottomBarWidget {
    region: Region,
}

impl Widget for BottomBarWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::GRID_LINE_1);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl VariableSizeWidget for BottomBarWidget {
    fn set_region(&mut self, app: &App, region: Region) {
        self.region = region;
        self.region.position.y = self.region.size.height as i32 - 80; // TODO
        self.region.size.height = 64;
    }
}
