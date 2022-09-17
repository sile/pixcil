use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    pixel::{PixelPosition, PixelRegion},
};
use pagurus::{
    spatial::{Contains, Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;
use std::collections::BTreeSet;

const BORDER: u32 = 1;

#[derive(Debug, Default)]
pub struct PreviewWidget {
    region: Region,
}

impl PreviewWidget {
    pub fn handle_dirty_pixels(&self, app: &mut App, dirty_pixels: &BTreeSet<PixelPosition>) {
        let pixel_region = PixelRegion::from_positions(dirty_pixels.iter().copied());
        let pixel_frame_start = app.models().config.frame.get().start;
        let preview_frame_region = self.frame_region();
        let mut drawing_region = preview_frame_region;
        drawing_region.position.x += i32::from(pixel_region.start.x - pixel_frame_start.x);
        drawing_region.position.y += i32::from(pixel_region.start.y - pixel_frame_start.y);
        drawing_region.size.width = u32::from(pixel_region.size().width);
        drawing_region.size.height = u32::from(pixel_region.size().height);
        app.request_redraw(preview_frame_region.intersection(drawing_region));
    }

    fn render_pixels(&self, app: &App, canvas: &mut Canvas) {
        let preview_frame_region = self.frame_region();
        let drawing_region = preview_frame_region.intersection(canvas.drawing_region());
        let pixel_frame_start = app.models().config.frame.get().start;
        let mut offset = preview_frame_region.start();
        offset.x -= i32::from(pixel_frame_start.x);
        offset.y -= i32::from(pixel_frame_start.y);
        let pixel_region = PixelRegion::new(
            PixelPosition::from_xy(
                (drawing_region.start().x - offset.x) as i16,
                (drawing_region.start().y - offset.y) as i16,
            ),
            PixelPosition::from_xy(
                (drawing_region.end().x - offset.x) as i16,
                (drawing_region.end().y - offset.y) as i16,
            ),
        );
        for pixel in app.models().pixel_canvas.get_pixels(pixel_region) {
            canvas.draw_pixel(
                Position::from_xy(
                    i32::from(pixel.position.x) + offset.x,
                    i32::from(pixel.position.y) + offset.y,
                ),
                pixel.color.into(),
            );
        }
    }

    fn frame_region(&self) -> Region {
        let mut region = self.region;
        region.position = region.position + BORDER as i32;
        region.size = region.size - (BORDER * 2);
        region
    }
}

impl Widget for PreviewWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::PREVIEW_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.render_pixels(app, canvas);
    }

    fn handle_event(&mut self, _app: &mut App, event: &mut Event) -> Result<()> {
        if let Some(position) = event.position() {
            if self.region.contains(&position) {
                event.consume();
            }
        }
        Ok(())
    }
}

impl FixedSizeWidget for PreviewWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let frame = app.models().config.frame.get().size();
        Size::from_wh(u32::from(frame.width), u32::from(frame.height)) + (BORDER * 2)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
