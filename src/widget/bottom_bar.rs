use super::{tool_box::ToolBoxWidget, FixedSizeWidget, VariableSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 16;

#[derive(Debug, Default)]
pub struct BottomBarWidget {
    region: Region,
    tool_box: ToolBoxWidget,
}

impl Widget for BottomBarWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.tool_box.render(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.tool_box.handle_event(app, event).or_fail()?;
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.tool_box]
    }
}

impl VariableSizeWidget for BottomBarWidget {
    fn set_region(&mut self, app: &App, region: Region) {
        let tool_box_size = self.tool_box.requiring_size(app);

        self.region = region;
        self.region.position.y =
            self.region.size.height as i32 - tool_box_size.height as i32 - MARGIN as i32;
        self.region.size.height = tool_box_size.height;

        let mut tool_box_position = self.region.position;
        tool_box_position.x = region.size.width as i32 / 2 - tool_box_size.width as i32 / 2;
        self.tool_box.set_position(app, tool_box_position);
    }
}
