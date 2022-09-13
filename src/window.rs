use crate::app::App;
use pagurus::spatial::Region;
use pagurus_game_std::image::Canvas;

pub trait Window {
    fn region(&self) -> Region;
    fn render(&self, app: &App, canvas: &mut Canvas);
}
