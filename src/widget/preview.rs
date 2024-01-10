use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    pixel::{PixelPosition, PixelRegion, PixelSize},
    region_ext::RegionExt,
    tags::PLAYING_TAG,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Contains, Position, Region, Size};

const MARGIN: u32 = 4;

#[derive(Debug, Default)]
pub struct PreviewWidget {
    region: Region,
    frame_size: Size,
    frame: PreviewFrameWidget,
    preview_off: bool,
}

impl Widget for PreviewWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        if self.preview_off {
            return;
        }

        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);

        self.frame.render(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if self.preview_off {
            return Ok(());
        }

        self.frame.handle_event(app, event).or_fail()?;
        event.consume_if_contained(self.region);
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        if self.preview_off != !app.models().config.frame_preview.get() {
            self.preview_off = !app.models().config.frame_preview.get();
            app.request_redraw(self.region);
            return Ok(());
        }

        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
        if self.frame_size != self.frame.region.size {
            let old_region = self.region;
            self.frame_size = self.frame.region.size;

            let size = self.requiring_size(app);
            self.region.position.x = self.region.end().x - size.width as i32;
            self.set_position(app, self.region.position);
            app.request_redraw(self.region.union(old_region));
        }
        if self.requiring_size(app) != self.region.size {
            let old_region = self.region;
            let mut position = self.region.position;
            position.x = self.region.end().x - self.requiring_size(app).width as i32;
            self.set_position(app, position);
            app.request_redraw(self.region.union(old_region));
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.frame]
    }
}

impl FixedSizeWidget for PreviewWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.frame.requiring_size(app) + MARGIN * 2
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.frame
            .set_position(app, self.region.without_margin(MARGIN).position);
        self.frame_size = self.frame.region.size;
    }
}

#[derive(Debug, Default)]
struct PreviewFrameWidget {
    region: Region,
    frame_size: Option<PixelSize>,
    playing: Option<Playing>,
}

impl PreviewFrameWidget {
    fn render_pixels(&self, app: &App, canvas: &mut Canvas) {
        let scale = app.models().config.frame_preview_scale.get() as i32;
        let current_frame = if let Some(playing) = &self.playing {
            playing.current_frame
        } else {
            app.models().config.camera.current_frame(app)
        };

        let preview_frame_region = self.frame_region();
        let drawing_region = (Region::new(
            preview_frame_region.position,
            preview_frame_region.size / scale as u32,
        ))
        .intersection(canvas.drawing_region());
        let pixel_frame_start = app
            .models()
            .config
            .frame
            .get_preview_region(&app.models().config, current_frame)
            .start;
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
        offset.y -= i32::from(pixel_frame_start.y) * (scale - 1);
        offset.x -= i32::from(pixel_frame_start.x) * (scale - 1);

        let size = Size::square(scale as u32);
        for pixel in app
            .models()
            .pixel_canvas
            .get_pixels(&app.models().config, pixel_region)
        {
            let position = Position::from_xy(
                i32::from(pixel.position.x) * scale + offset.x,
                i32::from(pixel.position.y) * scale + offset.y,
            );
            canvas.fill_rectangle(Region::new(position, size), pixel.color.into());
        }
    }

    fn frame_region(&self) -> Region {
        self.region
    }
}

impl Widget for PreviewFrameWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::PREVIEW_BACKGROUND);
        self.render_pixels(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if let Some(position) = event.position() {
            let mut focused = false;
            if self.region.contains(&position) && !event.is_consumed() {
                focused = true;
                event.consume();
            }
            if focused {
                if !app.models().preview_mode {
                    app.models_mut().preview_mode = true;
                    app.request_redraw(app.screen_size().to_region());
                }
            } else if app.models().preview_mode {
                app.models_mut().preview_mode = false;
                app.request_redraw(app.screen_size().to_region());
            }
            if focused && app.models().config.animation.is_enabled() && self.playing.is_none() {
                self.playing = Some(Playing::start(app));
            }
        }
        if let Some(playing) = &mut self.playing {
            playing.handle_event(app, event, self.region).or_fail()?;
        }
        if !app.models().preview_mode {
            self.playing = None;
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        let current_frame = app.models().config.camera.current_frame(app);
        let preview_pixel_region = app
            .models()
            .config
            .frame
            .get_preview_region(&app.models().config, current_frame);
        if let Some(frame_size) = self.frame_size {
            if frame_size != preview_pixel_region.size() {
                let old = frame_size;
                let new = preview_pixel_region.size();
                self.frame_size = Some(new);

                let old_region = self.region;
                self.region.size.width = u32::from(new.width);
                self.region.size.height = u32::from(new.height);
                self.region.position.x -= new.width as i32 - old.width as i32;
                app.request_redraw(self.region.union(old_region));
            }
        } else {
            let size = preview_pixel_region.size();
            self.frame_size = Some(size);
            self.region.size = Size::from_wh(u32::from(size.width), u32::from(size.height));
        }

        let dirty_pixels = app.models().pixel_canvas.dirty_positions();
        if dirty_pixels.is_empty() {
            return Ok(());
        }

        let scale = app.models().config.frame_preview_scale.get() as i16;
        let pixel_region =
            PixelRegion::from_positions(dirty_pixels.iter().copied()).scale(scale as u16);
        let pixel_frame_start = preview_pixel_region.start;
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

impl FixedSizeWidget for PreviewFrameWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let frame = app.models().config.frame.get_base_region().size();
        Size::from_wh(u32::from(frame.width), u32::from(frame.height))
            * app.models().config.frame_preview_scale.get() as u32
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}

#[derive(Debug)]
struct Playing {
    current_frame: usize,
}

impl Playing {
    fn start(app: &mut App) -> Self {
        let current_frame = app.models().config.camera.current_frame(app);
        let frame_interval = app.models().config.animation.frame_interval();
        app.set_timeout(PLAYING_TAG, frame_interval);
        Self { current_frame }
    }

    fn handle_event(&mut self, app: &mut App, event: &Event, preview_region: Region) -> Result<()> {
        let Event::Timeout(PLAYING_TAG) = event else {
            return Ok(());
        };
        let frame_interval = app.models().config.animation.frame_interval();
        app.set_timeout(PLAYING_TAG, frame_interval);
        self.current_frame += 1;
        if self.current_frame >= app.models().config.animation.enabled_frame_count() as usize {
            self.current_frame = 0;
        }
        app.request_redraw(preview_region);
        Ok(())
    }
}
