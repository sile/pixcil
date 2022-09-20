use super::Window;
use crate::{app::App, asset::Text, event::Event, widget::FixedSizeWidget};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct Tab<W> {
    label: Text,
    widget: W,
}

impl<W> Tab<W> {
    pub const fn new(label: Text, widget: W) -> Self {
        Self { label, widget }
    }
}

#[derive(Debug)]
pub struct TabBarWindow<W, const N: usize> {
    region: Region,
    tabs: [Tab<W>; N],
}

impl<W: FixedSizeWidget, const N: usize> Window for TabBarWindow<W, N> {
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
