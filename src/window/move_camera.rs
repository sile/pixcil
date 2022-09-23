use super::Window;
use crate::{app::App, event::Event};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct MoveCameraWindow {
    region: Region,
}

impl Window for MoveCameraWindow {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        todo!()
    }

    fn is_terminated(&self) -> bool {
        todo!()
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        todo!()
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        todo!()
    }
}
