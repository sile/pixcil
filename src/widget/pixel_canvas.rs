use std::collections::HashSet;

use super::{
    manipulate::ManipulateWidget, move_camera::MoveCameraWidget, VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    marker::MarkerHandler,
    model::tool::{ToolKind, ToolModel},
    pixel::{Pixel, PixelRegion},
};
use pagurus::{failure::OrFail, spatial::Region, Result};
use pagurus_game_std::{
    color::{Color, Rgba},
    image::Canvas,
};

#[derive(Debug, Default)]
pub struct PixelCanvasWidget {
    region: Region,
    marker_handler: MarkerHandler,
    preview_focused: bool,
    tool: ToolModel,
    manipulate: Option<ManipulateWidget>,
    move_camera: Option<MoveCameraWidget>,
}

impl PixelCanvasWidget {
    pub fn set_preview_focused(&mut self, app: &mut App, focused: bool) {
        if self.preview_focused != focused {
            self.preview_focused = focused;

            let mut region = app
                .models()
                .config
                .frame
                .get_preview_region(&app.models().config)
                .to_screen_region(app);
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

    fn render_drawn_pixels(&self, app: &App, canvas: &mut Canvas) {
        let color = app.models().config.color.get();
        if self.marker_handler.is_neutral() {
            let pixel_region = PixelRegion::from_positions(self.marker_handler.marked_pixels(app));
            let region = pixel_region.to_screen_region(app);
            canvas.draw_rectangle(region, color.into());
        } else {
            for pixel_position in self.marker_handler.marked_pixels(app) {
                let region = pixel_position.to_screen_region(app);
                if canvas.drawing_region().intersection(region).is_empty() {
                    continue;
                }
                canvas.fill_rectangle(region, color.into());
            }
        }
    }

    fn render_selected_pixels(&self, app: &App, canvas: &mut Canvas) {
        let color = Rgba::new(200, 200, 200, 200); // TODO
        if self.marker_handler.is_neutral() {
            let pixel_region = PixelRegion::from_positions(self.marker_handler.marked_pixels(app));
            let region = pixel_region.to_screen_region(app);
            canvas.draw_rectangle(region, color.into());
        } else {
            for pixel_position in self.marker_handler.marked_pixels(app) {
                let region = pixel_position.to_screen_region(app);
                if canvas.drawing_region().intersection(region).is_empty() {
                    continue;
                }
                canvas.fill_rectangle(region, color.into());
            }
        }
    }

    fn render_pixels(&self, app: &App, canvas: &mut Canvas) {
        let erasing_pixels = if self.tool.tool_kind() == ToolKind::Erase {
            self.marker_handler
                .marked_pixels(app)
                .filter(|p| app.models().pixel_canvas.get_direct_pixel(*p).is_some())
                .collect()
        } else {
            HashSet::new()
        };

        let config = &app.models().config;
        let pixel_region = PixelRegion::from_screen_region(app, canvas.drawing_region());
        for pixel in app
            .models()
            .pixel_canvas
            .get_pixels(&app.models().config, pixel_region)
        {
            let mut color = pixel.color;
            let mut alpha = 255;
            if let Some(w) = &self.manipulate {
                if w.selected_pixels().contains(&pixel.position) {
                    alpha = 0;
                }
            }

            let region = pixel.position.to_screen_region(app);
            if erasing_pixels.contains(&pixel.position) {
                if self.marker_handler.is_neutral() {
                    alpha = alpha.min(color.a / 5);
                } else {
                    alpha = alpha.min(color.a / 10);
                }
            }

            if alpha != 255 {
                if let Some(c) =
                    app.models()
                        .pixel_canvas
                        .get_pixel_with_alpha(config, pixel.position, alpha)
                {
                    color = c;
                } else {
                    continue;
                }
            }

            canvas.fill_rectangle(region, color.into());
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
        if self.tool.tool_kind() == ToolKind::Draw {
            self.render_drawn_pixels(app, canvas);
        } else if self.tool.tool_kind() == ToolKind::Select {
            self.render_selected_pixels(app, canvas);
        }
        if self.preview_focused {
            let mut region = app
                .models()
                .config
                .frame
                .get_preview_region(&app.models().config)
                .to_screen_region(app);
            region.size = region.size + 1;
            canvas.draw_rectangle(region, color::PREVIEW_FOCUSED_BORDER);
        }
        if let Some(w) = &self.manipulate {
            w.render(app, canvas);
        } else if let Some(w) = &self.move_camera {
            w.render(app, canvas);
        }
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if let Some(w) = &mut self.manipulate {
            w.handle_event(app, event).or_fail()?;
            if w.is_terminated() {
                app.request_redraw(w.region());
                self.manipulate = None;
            }
        } else if let Some(w) = &mut self.move_camera {
            w.handle_event(app, event).or_fail()?;
        }

        self.marker_handler.handle_event(app, event).or_fail()?;
        if self.marker_handler.is_completed() {
            let config = app.models().config.clone();
            match self.tool.tool_kind() {
                ToolKind::Draw => {
                    let color = app.models().config.color.get();
                    let pixels = self
                        .marker_handler
                        .marked_pixels(app)
                        .map(|pos| Pixel::new(pos, color));
                    app.models_mut()
                        .pixel_canvas
                        .draw_pixels(&config, pixels)
                        .or_fail()?;
                }
                ToolKind::Erase => {
                    let pixels = self.marker_handler.marked_pixels(app);
                    app.models_mut()
                        .pixel_canvas
                        .erase_pixels(&config, pixels)
                        .or_fail()?;
                }
                ToolKind::Select => {
                    self.manipulate = Some(ManipulateWidget::new(
                        app,
                        self.marker_handler.marked_pixels(app).collect(),
                    ));
                }
                ToolKind::Move => {}
                ToolKind::Pick => {
                    if let Some(position) = self.marker_handler().marked_pixels(app).next() {
                        if let Some(color) = app.models().pixel_canvas.get_pixel(&config, position)
                        {
                            app.models_mut().config.color.set(color);
                        }
                    }
                    app.models_mut().tool.current = ToolKind::Draw;
                }
            }
        } else if self.tool.tool_kind() == ToolKind::Pick {
            if let Some(position) = self.marker_handler().marked_pixels(app).next() {
                if let Some(color) = app
                    .models()
                    .pixel_canvas
                    .get_pixel(&app.models().config, position)
                {
                    app.models_mut().tool.pick.preview_color = Some(color);
                } else {
                    app.models_mut().tool.pick.preview_color = None;
                }
            }
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        let dirty_pixels = app.models_mut().pixel_canvas.take_dirty_positions();
        if !dirty_pixels.is_empty() {
            let dirty_region =
                PixelRegion::from_positions(dirty_pixels.into_iter()).to_screen_region(app);
            app.request_redraw(dirty_region);
        }

        if self.tool != app.models().tool {
            self.tool = app.models().tool.clone();
            self.marker_handler.set_marker_kind(self.tool.marker_kind());

            if self.tool.tool_kind() == ToolKind::Move {
                self.move_camera = Some(MoveCameraWidget::new(app));
            } else {
                if self.move_camera.take().is_some() {
                    app.request_redraw(self.region);
                }
            }
        }

        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        let mut children = Vec::new();
        if let Some(w) = &mut self.manipulate {
            children.push(w as &mut dyn Widget);
        } else if let Some(w) = &mut self.move_camera {
            children.push(w as &mut dyn Widget);
        }
        children
    }
}

impl VariableSizeWidget for PixelCanvasWidget {
    fn set_region(&mut self, app: &App, region: Region) {
        self.region = region;
        if let Some(w) = &mut self.manipulate {
            w.set_region(app, region);
        } else if let Some(w) = &mut self.move_camera {
            w.set_region(app, region);
        }
    }
}
