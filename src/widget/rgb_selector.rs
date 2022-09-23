use super::{FixedSizeWidget, Widget};
use crate::{app::App, event::Event};
use pagurus::{
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct RgbSelector {
    region: Region,
}

impl Widget for RgbSelector {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {}

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl FixedSizeWidget for RgbSelector {
    fn requiring_size(&self, app: &App) -> Size {
        Size::square(100)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
