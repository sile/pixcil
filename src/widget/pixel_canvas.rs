use super::{VariableSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    marker::MarkerHandler,
    pixel::{Pixel, PixelRegion},
};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus_game_std::{color::Color, image::Canvas};

#[derive(Debug, Default)]
pub struct PixelCanvasWidget {
    region: Region,
    marker_handler: MarkerHandler,
    preview_focused: bool,
}

impl PixelCanvasWidget {
    pub fn set_preview_focused(&mut self, app: &mut App, focused: bool) {
        if self.preview_focused != focused {
            self.preview_focused = focused;

            let mut region = app.models().config.frame.get().to_screen_region(app);
            region.size = region.size + 1;
            app.request_redraw(self.region.intersection(region));
        }
    }

    pub fn marker_handler(&self) -> &MarkerHandler {
        &self.marker_handler
    }

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

    fn render_marked_pixels(&self, app: &App, canvas: &mut Canvas) {
        let color = app.models().config.color.get();
        for pixel_position in self.marker_handler.marked_pixels() {
            let region = pixel_position.to_screen_region(app);
            if canvas.drawing_region().intersection(region).is_empty() {
                continue;
            }
            canvas.fill_rectangle(region, color.into());
        }
    }

    fn render_pixels(&self, app: &App, canvas: &mut Canvas) {
        let pixel_region = PixelRegion::from_screen_region(app, canvas.drawing_region());
        for pixel in app.models().pixel_canvas.get_pixels(pixel_region) {
            let region = pixel.position.to_screen_region(app);
            canvas.fill_rectangle(region, pixel.color.into());
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
        self.render_pixels(app, canvas);
        self.render_marked_pixels(app, canvas);
        if self.preview_focused {
            let mut region = app.models().config.frame.get().to_screen_region(app);
            region.size = region.size + 1;
            canvas.draw_rectangle(region, color::PREVIEW_FOCUSED_BORDER);
        }
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.marker_handler.handle_event(app, event).or_fail()?;
        if self.marker_handler.is_completed() {
            let color = app.models().config.color.get();
            let pixels = self
                .marker_handler
                .marked_pixels()
                .map(|pos| Pixel::new(pos, color));
            app.models_mut()
                .pixel_canvas
                .draw_pixels(pixels)
                .or_fail()?;
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        app.models_mut().pixel_canvas.take_dirty_positions();
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        Vec::new()
    }
}

impl VariableSizeWidget for PixelCanvasWidget {
    fn set_region(&mut self, _app: &App, region: Region) {
        self.region = region;
    }
}
