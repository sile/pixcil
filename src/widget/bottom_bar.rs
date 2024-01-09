use super::{
    color_config::ColorConfigWidget, tool_box::ToolBoxWidget, FixedSizeWidget, VariableSizeWidget,
    Widget,
};
use crate::{app::App, event::Event};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::Region;

const MARGIN: u32 = 16;

#[derive(Debug, Default)]
pub struct BottomBarWidget {
    region: Region,
    tool_box: ToolBoxWidget,
    color_config: ColorConfigWidget,
}

impl Widget for BottomBarWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.tool_box.render_if_need(app, canvas);
        self.color_config.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.tool_box.handle_event(app, event).or_fail()?;
        self.color_config.handle_event(app, event).or_fail()?;
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.tool_box, &mut self.color_config]
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

        let mut color_config_position = self.region.position;
        color_config_position.x = region.size.width as i32
            - MARGIN as i32
            - self.color_config.requiring_size(app).width as i32;
        self.color_config.set_position(app, color_config_position);
    }
}
