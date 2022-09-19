use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

use crate::{app::App, asset::Text, event::Event};

use super::{FixedSizeWidget, VariableSizeWidget, Widget};

#[derive(Debug)]
pub struct BlockWidget<W> {
    region: Region,
    label: Text,
    body: W,
}

impl<W: FixedSizeWidget> BlockWidget<W> {
    pub fn new(label: Text, body: W) -> Self {
        Self {
            region: Region::default(),
            label,
            body,
        }
    }

    pub fn body(&self) -> &W {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut W {
        &mut self.body
    }
}

impl<W: FixedSizeWidget> Widget for BlockWidget<W> {
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
        vec![&mut self.body]
    }
}

impl<W: FixedSizeWidget> VariableSizeWidget for BlockWidget<W> {
    fn set_region(&mut self, app: &App, region: Region) {
        self.region = region;
        todo!()
    }
}
