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
        let frame_region = self.frame_region();
        let mut region = frame_region;
        region.position.x += i32::from(pixel_region.start.x);
        region.position.y += i32::from(pixel_region.start.y);
        region.size.width = u32::from(pixel_region.size().width);
        region.size.height = u32::from(pixel_region.size().height);
        app.request_redraw(frame_region.intersection(region));
    }

    fn render_pixels(&self, app: &App, canvas: &mut Canvas) {
        log::info!("--------------------------------------");
        let frame_region = self.frame_region();
        let pixel_region = frame_region.intersection(canvas.drawing_region());
        log::info!("{frame_region:?}");
        log::info!("{pixel_region:?}");
        log::info!("{:?}", canvas.drawing_region());
        let offset = frame_region.start();
        let pixel_region = PixelRegion::new(
            PixelPosition::from_xy(
                (pixel_region.start().x - offset.x) as i16,
                (pixel_region.start().y - offset.y) as i16,
            ),
            PixelPosition::from_xy(
                (pixel_region.end().x - offset.x) as i16,
                (pixel_region.end().y - offset.y) as i16,
            ),
        );
        log::info!("{pixel_region:?}");
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
        let frame = app.models().config.frame.get();
        Size::from_wh(u32::from(frame.width), u32::from(frame.height)) + (BORDER * 2)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
