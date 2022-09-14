use super::{VariableSizeWidget, Widget};
use crate::{
    app::App, canvas_ext::CanvasExt, color, event::Event, model::pixel_canvas::PixelRegion,
};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::{color::Color, image::Canvas};

#[derive(Debug, Default)]
pub struct PixelCanvasWidget {
    region: Region,
}

impl PixelCanvasWidget {
    fn render_grid(&self, app: &App, canvas: &mut Canvas) {
        let zoom = app.models().config.zoom.get();
        let pixel_region = PixelRegion::from_screen_region(app, canvas.drawing_region());
        let screen_region = pixel_region.to_screen_region(app);

        fn line_color(i: i16) -> Color {
            if i % 32 == 0 {
                color::GRID_LINE_32
            } else if i % 8 == 0 {
                color::GRID_LINE_8
            } else {
                color::GRID_LINE_1
            }
        }

        fn skip(i: i16, zoom: u8) -> bool {
            if zoom == 1 && i % 32 != 0 {
                true
            } else if zoom == 2 && i % 8 != 0 {
                true
            } else {
                false
            }
        }

        let mut current = screen_region.start();
        for y in pixel_region.start.y..=pixel_region.end.y {
            if !skip(y, zoom) {
                canvas.draw_horizontal_line(current, screen_region.size.width, line_color(y));
            }
            current.y += i32::from(zoom);
        }

        let mut current = screen_region.start();
        for x in pixel_region.start.x..=pixel_region.end.x {
            if !skip(x, zoom) {
                canvas.draw_vertical_line(current, screen_region.size.height, line_color(x));
            }
            current.x += i32::from(zoom);
        }
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
