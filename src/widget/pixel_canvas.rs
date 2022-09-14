use super::{VariableSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct PixelCanvasWidget {
    region: Region,
}

impl PixelCanvasWidget {
    fn render_grid(&self, app: &App, canvas: &mut Canvas) {
        // TODO
    }
}

impl Widget for PixelCanvasWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::CANVAS_BACKGROUND);
        self.render_grid(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // TODO
        Ok(())
    }
}

impl VariableSizeWidget for PixelCanvasWidget {
    fn set_region(&mut self, _app: &App, region: Region) {
        self.region = region;
    }
}
