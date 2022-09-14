use super::{VariableSizeWidget, Widget};
use crate::{
    app::App, canvas_ext::CanvasExt, color, event::Event, model::pixel_canvas::PixelRegion,
};
use pagurus::{spatial::Region, Result};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct PixelCanvasWidget {
    region: Region,
}

impl PixelCanvasWidget {
    fn render_grid(&self, app: &App, canvas: &mut Canvas) {
        let zoom = app.models().config.zoom.get();
        let pixel_region = PixelRegion::from_screen_region(app, canvas.drawing_region());

        // let mut current = model.align_screen_position(app.window_size, start);
        // let mut pixel_canvas_pos = model.to_pixel_canvas_position(app.window_size, current);

        // while current.y < end.y {
        //     let mut color = Color::BLACK.alpha(20).to_rgba();
        //     // TODO: adjust by scale and resolution
        //     for v in [8, 32] {
        //         if pixel_canvas_pos.y.abs() % v == 0 {
        //             color.a += 50;
        //         } else {
        //             break;
        //         }
        //     }
        //     if !((scale < 4 && color.a == 20) || (scale == 1 && color.a == 70)) {
        //         canvas.draw_horizontal_line(current, drawing_region.size.width, color.into());
        //     }
        //     current.y += scale;
        //     pixel_canvas_pos.y += 1;
        // }

        // let mut current = model.align_screen_position(app.window_size, start);
        // let mut pixel_canvas_pos = model.to_pixel_canvas_position(app.window_size, current);
        // while current.x < end.x {
        //     let mut color = Color::BLACK.alpha(20).to_rgba();
        //     // TODO: adjust by scale and resolution
        //     for v in [8, 32] {
        //         if pixel_canvas_pos.x.abs() % v == 0 {
        //             color.a += 50;
        //         } else {
        //             break;
        //         }
        //     }
        //     if !((scale < 4 && color.a == 20) || (scale == 1 && color.a == 70)) {
        //         canvas.draw_vertical_line(current, drawing_region.size.height, color.into());
        //     }
        //     current.x += scale;
        //     pixel_canvas_pos.x += 1;
        // }
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
