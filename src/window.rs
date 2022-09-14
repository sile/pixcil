use crate::app::App;
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

pub mod main;

pub trait Window: 'static {
    fn region(&self) -> Region;
    fn render(&self, app: &App, canvas: &mut Canvas);
    fn handle_event(&mut self, app: &mut App, event: WindowEvent) -> Result<()>;
}

#[derive(Debug)]
pub struct WindowEvent {}
