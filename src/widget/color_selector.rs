use super::{
    block::BlockWidget, hsv_selector::HsvSelectorWidget, rgb_selector::RgbSelectorWidget,
    FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const COLOR_PREVIEW_HEIGHT: u32 = 64;
const MARGIN: u32 = 16;

#[derive(Debug)]
pub struct ColorSelectorWidget {
    region: Region,
    rgb: BlockWidget<RgbSelectorWidget>,
    hsv: BlockWidget<HsvSelectorWidget>,
}

impl ColorSelectorWidget {
    pub fn new(app: &App) -> Self {
        Self {
            region: Region::default(),
            rgb: BlockWidget::new(
                "RGB".parse().expect("unreachable"),
                RgbSelectorWidget::new(app),
            ),
            hsv: BlockWidget::new(
                "HSV".parse().expect("unreachable"),
                HsvSelectorWidget::new(app),
            ),
        }
    }

    fn render_color_preview(&self, app: &App, canvas: &mut Canvas) {
        let mut region = self.region;
        region.size.height = COLOR_PREVIEW_HEIGHT;
        canvas.fill_rectangle(region, app.models().config.color.get().into());
        canvas.draw_rectangle(region, color::WINDOW_BORDER);
    }
}

impl Widget for ColorSelectorWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.render_color_preview(app, canvas);
        self.rgb.render_if_need(app, canvas);
        self.hsv.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let old = app.models().config.color.get();

        self.rgb.handle_event(app, event).or_fail()?;
        self.hsv.handle_event(app, event).or_fail()?;

        let new = app.models().config.color.get();
        if old != new {
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.rgb, &mut self.hsv]
    }
}

impl FixedSizeWidget for ColorSelectorWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let preview_size = Size::from_wh(0, COLOR_PREVIEW_HEIGHT);
        let rgb_size = self.rgb.requiring_size(app);
        let hsv_size = self.hsv.requiring_size(app);

        Size::from_wh(
            preview_size.width.max(rgb_size.width).max(hsv_size.width),
            preview_size.height + MARGIN + rgb_size.height + MARGIN + hsv_size.height,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut offset = position;
        offset.y += (COLOR_PREVIEW_HEIGHT + MARGIN) as i32;
        self.rgb
            .set_region(app, Region::new(offset, self.rgb.requiring_size(app)));

        offset.y = self.rgb.region().end().y + MARGIN as i32;
        self.hsv
            .set_region(app, Region::new(offset, self.hsv.requiring_size(app)));
    }
}
