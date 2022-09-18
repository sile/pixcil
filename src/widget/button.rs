use super::{FixedSizeWidget, Widget};
use crate::{app::App, event::Event};
use pagurus::{
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct ButtonWidget {
    region: Region,
}

impl Widget for ButtonWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        todo!()
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        todo!()
    }
}

impl FixedSizeWidget for ButtonWidget {
    fn requiring_size(&self, app: &App) -> Size {
        todo!()
    }

    fn set_position(&mut self, app: &App, position: Position) {
        todo!()
    }
}

// #[derive(Debug, Default)]
// pub enum ButtonState {}
