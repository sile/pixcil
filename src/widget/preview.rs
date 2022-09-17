use super::{FixedSizeWidget, Widget};
use crate::{app::App, event::Event, pixel::PixelPosition};
use pagurus::{
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct PreviewWidget {
    region: Region,
}

impl PreviewWidget {
    pub fn handle_dirty_pixels(&mut self, app: &mut App, dirty_pixels: &BTreeSet<PixelPosition>) {}

    pub fn size(&self, app: &App) -> Size {
        todo!()
    }
}

impl Widget for PreviewWidget {
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

impl FixedSizeWidget for PreviewWidget {
    fn set_position(&mut self, app: &App, position: Position) {
        todo!()
    }
}
