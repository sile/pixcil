use super::{FixedSizeWidget, Widget};
use crate::{app::App, asset::Text, event::Event};
use pagurus::{
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct Tab<W> {
    label: Text,
    widget: W,
}

#[derive(Debug)]
pub struct TabBarWidget<W, const N: usize> {
    region: Region,
    tabs: [Tab<W>; N],
}

impl<W: FixedSizeWidget, const N: usize> Widget for TabBarWidget<W, N> {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        todo!()
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        todo!()
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        self.tabs
            .iter_mut()
            .map(|tab| &mut tab.widget as &mut dyn Widget)
            .collect()
    }
}

impl<W: FixedSizeWidget, const N: usize> FixedSizeWidget for TabBarWidget<W, N> {
    fn requiring_size(&self, app: &App) -> Size {
        todo!()
    }

    fn set_position(&mut self, app: &App, position: Position) {
        todo!()
    }
}
