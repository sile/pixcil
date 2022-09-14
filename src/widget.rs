use crate::app::App;
use pagurus::{
    spatial::{Position, Region},
    Result,
};
use pagurus_game_std::image::Canvas;

pub trait Widget {
    fn region(&self) -> Region;
    fn relocate(&mut self, app: &App, position: Position);
    fn render(&self, app: &App, canvas: &mut Canvas);
    fn handle_event(&mut self, app: &mut App, event: &mut WidgetEvent) -> Result<()>;
}

#[derive(Debug)]
pub struct WidgetEvent {}
