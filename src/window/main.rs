use super::{Window, WindowEvent};
use crate::app::App;
use pagurus::{
    spatial::{Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct MainWindow {
    size: Size,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            size: Size::default(),
        }
    }
}

impl Window for MainWindow {
    fn region(&self) -> Region {
        self.size.to_region()
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        todo!()
    }

    fn handle_event(&mut self, app: &mut App, event: WindowEvent) -> Result<()> {
        todo!()
    }
}
