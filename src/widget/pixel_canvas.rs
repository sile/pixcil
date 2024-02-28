use super::{
    manipulate::ManipulateWidget, move_camera::MoveCameraWidget, VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::Event,
    gesture::{GestureEvent, GestureRecognizer},
    marker::{MarkerHandler, MarkerKind},
    model::tool::{DrawTool, ToolKind, ToolModel},
    pixel::{Pixel, PixelPosition, PixelRegion},
};
use orfail::{OrFail, Result};
use pagurus::image::{Canvas, Color, Rgba};
use pagurus::spatial::Region;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct PixelCanvasWidget {
    region: Region,
    marker_handler: MarkerHandler,
    tool: ToolModel,
    manipulate: Option<ManipulateWidget>,
    move_camera: Option<MoveCameraWidget>,
    gesture_recognizer: GestureRecognizer,
}

impl PixelCanvasWidget {
    pub fn is_operating(&self) -> bool {
        self.marker_handler.is_operating()
            || self.manipulate.as_ref().map_or(false, |x| x.is_dragging())
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
            (zoom == 1 && i % 32 != 0) || (zoom == 2 && i % 8 != 0)
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

    fn render_frame_edges(&self, app: &App, canvas: &mut Canvas) {
        let config = &app.models().config;
        let pixel_drawing_region = PixelRegion::from_screen_region(app, canvas.drawing_region());
        let pixel_frames_region = config.frame.get_full_region(config);
        let region = pixel_drawing_region.intersection(pixel_frames_region);
        if region.is_empty() {
            return;
        }

        let current_frame = config.camera.current_frame(app) as i16;
        let current_layer = config.camera.current_layer(app) as i16;

        let frame_region = config.frame.get_base_region();
        let frame_size = frame_region.size();
        let first_frame = (region.start.x - frame_region.start.x) / frame_size.width as i16;
        let first_layer = (region.start.y - frame_region.start.y) / frame_size.height as i16;
        let last_frame = (region.end.x - 1 - frame_region.start.x) / frame_size.width as i16;
        let last_layer = (region.end.y - 1 - frame_region.start.y) / frame_size.height as i16;
        for frame in first_frame..=last_frame {
            for layer in first_layer..=last_layer {
                let current = frame_region.shift_x(frame).shift_y(layer);
                let color = if current_frame == frame && current_layer == layer {
                    color::CURRENT_FRAME_EDGE
                } else {
                    color::FRAME_EDGE
                };

                let top_left = current.start;
                let bottom_right = current.end - PixelPosition::from_xy(1, 1);
                let top_right = PixelPosition::from_xy(bottom_right.x, top_left.y);
                let bottom_left = PixelPosition::from_xy(top_left.x, bottom_right.y);

                // TODO: Consider 1-px width / height frames
                for pos in [
                    top_left,
                    top_left.move_x(1),
                    top_left.move_y(1),
                    top_right,
                    top_right.move_x(-1),
                    top_right.move_y(1),
                    bottom_left,
                    bottom_left.move_x(1),
                    bottom_left.move_y(-1),
                    bottom_right,
                    bottom_right.move_x(-1),
                    bottom_right.move_y(-1),
                ] {
                    canvas.fill_rectangle(pos.to_screen_region(app), color);
                }
            }
        }
    }

    fn render_drawn_pixels(&self, app: &App, canvas: &mut Canvas) {
        let color = app.models().config.color.get();
        if self.marker_handler.is_neutral() && app.models().tool.draw != DrawTool::Bucket {
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

    fn handle_gesture(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let Some(gesture) = self.gesture_recognizer.handle_event(app, event).or_fail()? else {
            return Ok(());
        };

        match gesture {
            GestureEvent::Tap => {
                app.models_mut().tool.current = ToolKind::Pick;
            }
            GestureEvent::TwoFingerTap => {
                app.models_mut().tool.current = ToolKind::Select;
            }
            GestureEvent::Swipe { mut delta } => {
                delta.x = -delta.x;
                delta.y = -delta.y;
                app.models_mut().config.camera.r#move(delta);
                app.request_redraw(app.screen_size().to_region());
            }
            GestureEvent::TwoFingerSwipe { undo } => {
                let config = app.models().config.clone();
                if undo {
                    app.models_mut()
                        .pixel_canvas
                        .undo_command(&config)
                        .or_fail()?;
                } else {
                    app.models_mut()
                        .pixel_canvas
                        .redo_command(&config)
                        .or_fail()?;
                }
            }
            GestureEvent::Pinch { zoom_in } => {
                app.zoom(zoom_in);
            }
        }

        Ok(())
    }
}

impl Widget for PixelCanvasWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let preview_mode = app.models().preview_mode;

        canvas.fill_rectangle(self.region, color::CANVAS_BACKGROUND);
        if preview_mode {
            if app.models().config.animation.enabled_frame_count() > 1 {
                self.render_frame_edges(app, canvas);
            }
        } else {
            self.render_grid(app, canvas);
            self.render_frame_edges(app, canvas);
        }

        self.render_pixels(app, canvas);
        if self.tool.tool_kind() == ToolKind::Draw {
            self.render_drawn_pixels(app, canvas);
        } else if self.tool.tool_kind() == ToolKind::Select
            || (self.tool.tool_kind() == ToolKind::Erase
                && self.marker_handler.marker_kind() != MarkerKind::Stroke
                && !self.marker_handler.is_completed())
        {
            self.render_selected_pixels(app, canvas);
        }
        if let Some(w) = &self.manipulate {
            w.render(app, canvas);
        } else if let Some(w) = &self.move_camera {
            w.render(app, canvas);
        }
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if let Event::Import { image } = event {
            self.manipulate = Some(ManipulateWidget::with_imported_image(app, image));
            return Ok(());
        }

        self.handle_gesture(app, event).or_fail()?;

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
        if self.marker_handler.take_completed() {
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
                    let target_pixels: HashSet<_> =
                        self.marker_handler.marked_pixels(app).collect();
                    if target_pixels
                        .iter()
                        .any(|p| app.models().pixel_canvas.get_direct_pixel(*p).is_some())
                    {
                        self.manipulate = Some(ManipulateWidget::new(app, target_pixels));
                    }
                }
                ToolKind::Move => {}
                ToolKind::Pick => {
                    if let Some(position) = self.marker_handler.marked_pixels(app).next() {
                        if let Some(color) = app.models().pixel_canvas.get_pixel(&config, position)
                        {
                            app.models_mut().config.color.set(color);
                        } else {
                            app.models_mut().tool.current = ToolKind::Erase;
                        }
                    }
                    if app.models_mut().tool.current == ToolKind::Pick {
                        app.models_mut().tool.current = ToolKind::Draw;
                    }
                }
            }
        } else if self.tool.tool_kind() == ToolKind::Pick {
            if let Some(position) = self.marker_handler.marked_pixels(app).next() {
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

            let layers = app.models().pixel_canvas.get_layers(&app.models().config);
            if layers != app.models().config.layer.count() {
                app.models_mut().config.layer.set_count(layers);
                app.request_redraw(app.screen_size().to_region());
            }

            let frames = app.models().pixel_canvas.get_frames(&app.models().config);
            if frames != app.models().config.animation.frame_count() {
                app.models_mut().config.animation.set_frame_count(frames);
                app.request_redraw(app.screen_size().to_region());
            }
        }

        if self.tool != app.models().tool {
            self.tool = app.models().tool.clone();
            self.marker_handler.set_marker_kind(self.tool.marker_kind());

            if self.tool.tool_kind() == ToolKind::Move {
                self.move_camera = Some(MoveCameraWidget::new(app));
            } else if self.move_camera.take().is_some() {
                app.request_redraw(self.region);
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
