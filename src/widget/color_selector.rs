use super::{
    block::BlockWidget, hsv_selector::HsvSelectorWidget, rgb_selector,
    rgb_selector::RgbSelectorWidget, slider::SliderWidget, FixedSizeWidget, VariableSizeWidget,
    Widget,
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
    hsv: BlockWidget<HsvSelectorWidget>,
    rgb: BlockWidget<RgbSelectorWidget>,
    alpha: BlockWidget<SliderWidget>, // toggle
}

impl ColorSelectorWidget {
    pub fn new(app: &App) -> Self {
        let color = app.models().config.color.get();
        Self {
            region: Region::default(),
            hsv: BlockWidget::new(
                "HSV".parse().expect("unreachable"),
                HsvSelectorWidget::new(app),
            ),
            rgb: BlockWidget::new(
                "RGB".parse().expect("unreachable"),
                RgbSelectorWidget::new(app),
            ),
            alpha: BlockWidget::new(
                "ALPHA".parse().expect("unreachable"),
                SliderWidget::new(
                    "A".parse().expect("unreachable"),
                    0,
                    u32::from(color.a),
                    255,
                    |slider, app, canvas| {
                        rgb_selector::render_color_bar(
                            app,
                            canvas,
                            slider.bar_region(),
                            |color, i| color.a = i,
                        );
                    },
                ),
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
        self.hsv.render_if_need(app, canvas);
        self.rgb.render_if_need(app, canvas);
        self.alpha.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let old = app.models().config.color.get();

        self.hsv.handle_event(app, event).or_fail()?;
        self.rgb.handle_event(app, event).or_fail()?;

        let alpha = self.alpha.body().value();
        self.alpha.handle_event(app, event).or_fail()?;
        if alpha != self.alpha.body().value() {
            let mut c = app.models().config.color.get();
            c.a = self.alpha.body().value() as u8;
            app.models_mut().config.color.set(c);
        }

        let new = app.models().config.color.get();
        if old != new {
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.rgb, &mut self.hsv, &mut self.alpha]
    }
}

impl FixedSizeWidget for ColorSelectorWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let preview = Size::from_wh(0, COLOR_PREVIEW_HEIGHT);
        let hsv = self.hsv.requiring_size(app);
        let rgb = self.rgb.requiring_size(app);
        let alpha = self.alpha.requiring_size(app);

        Size::from_wh(
            preview.width.max(rgb.width).max(hsv.width).max(alpha.width),
            preview.height + MARGIN + hsv.height + MARGIN + rgb.height + MARGIN + alpha.height,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut offset = position;
        offset.y += (COLOR_PREVIEW_HEIGHT + MARGIN) as i32;
        self.hsv
            .set_region(app, Region::new(offset, self.hsv.requiring_size(app)));

        offset.y = self.hsv.region().end().y + MARGIN as i32;
        self.rgb
            .set_region(app, Region::new(offset, self.rgb.requiring_size(app)));

        offset.y = self.rgb.region().end().y + MARGIN as i32;
        self.alpha
            .set_region(app, Region::new(offset, self.alpha.requiring_size(app)));
    }
}
