use crate::{app::App, event::Event};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

pub mod color_selector;
pub mod config;
pub mod draw_tool;
pub mod erase_tool;
pub mod move_tool;
pub mod pick_tool;
pub mod select_tool;

pub mod main;

pub trait Window: 'static + std::fmt::Debug {
    fn region(&self) -> Region;
    fn render(&self, app: &App, canvas: &mut Canvas);
    fn is_terminated(&self) -> bool;
    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()>;
    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()>;
}
