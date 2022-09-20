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

const BORDER: u32 = 1;

#[derive(Debug, Default)]
pub struct PreviewWidget {
    region: Region,
    focused: bool,
    preview_off: bool,
}

impl PreviewWidget {
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

    fn set_focused(&mut self, app: &mut App, focused: bool) {
        if self.focused != focused {
            self.focused = focused;
            app.request_redraw(self.region);
        }
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}

impl Widget for PreviewWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        if self.preview_off {
            return;
        }

        canvas.fill_rectangle(self.region, color::PREVIEW_BACKGROUND);
        if self.focused {
            canvas.draw_rectangle(self.region, color::PREVIEW_FOCUSED_BORDER);
        } else {
            canvas.draw_rectangle(self.region, color::PREVIEW_BORDER);
        }
        self.render_pixels(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if self.preview_off {
            return Ok(());
        }

        if let Some(position) = event.position() {
            let mut focused = false;
            if self.region.contains(&position) {
                if !event.is_consumed() {
                    focused = true;
                    event.consume();
                }
            }
            self.set_focused(app, focused);
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        if self.preview_off != !app.models().config.frame_preview.get() {
            self.preview_off = !app.models().config.frame_preview.get();
            app.request_redraw(self.region);
            return Ok(());
        }

        let dirty_pixels = app.models().pixel_canvas.dirty_positions();
        if dirty_pixels.is_empty() {
            return Ok(());
        }

        let pixel_region = PixelRegion::from_positions(dirty_pixels.iter().copied());
        let pixel_frame_start = app.models().config.frame.get().start;
        let preview_frame_region = self.frame_region();
        let mut drawing_region = preview_frame_region;
        drawing_region.position.x += i32::from(pixel_region.start.x - pixel_frame_start.x);
        drawing_region.position.y += i32::from(pixel_region.start.y - pixel_frame_start.y);
        drawing_region.size.width = u32::from(pixel_region.size().width);
        drawing_region.size.height = u32::from(pixel_region.size().height);
        app.request_redraw(preview_frame_region.intersection(drawing_region));
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        Vec::new()
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
