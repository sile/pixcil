use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    region_ext::RegionExt,
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct ZoomWidget {
    region: Region,
    zoom_in: ButtonWidget,
    zoom_out: ButtonWidget,
}

impl Default for ZoomWidget {
    fn default() -> Self {
        let mut zoom_in = ButtonWidget::new(ButtonKind::Basic, IconId::ZoomIn);
        zoom_in.set_disabled_callback(|app| app.models().config.zoom.is_max());
        zoom_in.set_number_callback(6, |app| app.models().config.zoom.get() as u32);
        zoom_in.enable_long_press();

        let mut zoom_out = ButtonWidget::new(ButtonKind::Basic, IconId::ZoomOut);
        zoom_out.set_disabled_callback(|app| app.models().config.zoom.is_min());
        zoom_out.enable_long_press();
        Self {
            region: Default::default(),
            zoom_in,
            zoom_out,
        }
    }
}

impl Widget for ZoomWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.zoom_in.render_if_need(app, canvas);
        self.zoom_out.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.zoom_in.handle_event(app, event).or_fail()?;
        if self.zoom_in.take_clicked(app) {
            app.models_mut().config.zoom.increment();
            app.request_redraw(app.screen_size().to_region());
        }

        self.zoom_out.handle_event(app, event).or_fail()?;
        if self.zoom_out.take_clicked(app) {
            app.models_mut().config.zoom.decrement();
            app.request_redraw(app.screen_size().to_region());
        }

        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.zoom_in, &mut self.zoom_out]
    }
}

impl FixedSizeWidget for ZoomWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let zoom_in_size = self.zoom_in.requiring_size(app);
        let zoom_out_size = self.zoom_out.requiring_size(app);
        Size::from_wh(
            zoom_out_size.width + MARGIN * 2,
            zoom_out_size.height + zoom_in_size.height + MARGIN * 4,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut block = self.region;
        block.size.height /= 2;

        self.zoom_in
            .set_position(app, block.without_margin(MARGIN).position);
        self.zoom_out
            .set_position(app, block.shift_y(1).without_margin(MARGIN).position);
    }
}
