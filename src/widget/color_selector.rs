use super::{
    block::BlockWidget, hsv_selector::HsvSelectorWidget, rgb_selector,
    rgb_selector::RgbSelectorWidget, slider::SliderWidget, toggle::ToggleWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus::{image::Rgba, image::Canvas};

const COLOR_PREVIEW_HEIGHT: u32 = 64;
const MARGIN: u32 = 16;

#[derive(Debug)]
pub struct ColorSelectorWidget {
    region: Region,
    old_color: Rgba,
    replaced: bool,
    hsv: BlockWidget<HsvSelectorWidget>,
    rgb: BlockWidget<RgbSelectorWidget>,
    alpha: BlockWidget<SliderWidget>,
    replace: BlockWidget<ToggleWidget>,
}

impl ColorSelectorWidget {
    pub fn new(app: &App) -> Self {
        let color = app.models().config.color.get();
        Self {
            region: Region::default(),
            old_color: color,
            replaced: false,
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
            replace: BlockWidget::new(
                "REPLACE OLD COLOR PIXELS".parse().expect("unreachable"),
                ToggleWidget::default_off(),
            ),
        }
    }

    fn render_color_preview(&self, app: &App, canvas: &mut Canvas) {
        let mut region = self.region;
        region.size.height = COLOR_PREVIEW_HEIGHT;

        let color = app.models().config.color.get();
        if !self.replace.body().is_on() {
            canvas.fill_rectangle(region, color.into());
            canvas.draw_rectangle(region, color::WINDOW_BORDER);
        } else {
            let arrow = &app.assets().right_arrow;
            let mut old_region = region;
            old_region.size.width -= arrow.size().width;
            old_region.size.width /= 2;
            canvas.fill_rectangle(old_region, self.old_color.into());
            canvas.draw_rectangle(old_region, color::WINDOW_BORDER);

            let mut arrow_position = old_region.position;
            arrow_position.x = old_region.end().x;
            canvas.offset(arrow_position).draw_sprite(arrow);

            let mut new_region = region;
            new_region.position.x = old_region.end().x + arrow.size().width as i32;
            new_region.size.width = old_region.size.width;
            canvas.fill_rectangle(new_region, color.into());
            canvas.draw_rectangle(new_region, color::WINDOW_BORDER);
        }
    }

    fn cancel_color_replace_if_need(&mut self, app: &mut App) -> Result<()> {
        if self.replaced {
            let config = app.models().config.clone();
            app.models_mut()
                .pixel_canvas
                .undo_command(&config)
                .or_fail()?;
            app.request_redraw(app.screen_size().to_region());
            self.replaced = false;
        }
        Ok(())
    }

    fn replace_color(&mut self, app: &mut App) -> Result<()> {
        self.cancel_color_replace_if_need(app).or_fail()?;

        let new_color = app.models().config.color.get();
        if new_color == self.old_color {
            return Ok(());
        }

        let config = app.models().config.clone();
        let command_log_tail = app.models().pixel_canvas.command_log_tail();
        app.models_mut()
            .pixel_canvas
            .replace_color(&config, self.old_color, new_color)
            .or_fail()?;
        if command_log_tail != app.models().pixel_canvas.command_log_tail() {
            self.replaced = true;
        }
        Ok(())
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
        self.replace.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let old_color = app.models().config.color.get();

        self.hsv.handle_event(app, event).or_fail()?;
        self.rgb.handle_event(app, event).or_fail()?;

        let alpha = self.alpha.body().value();
        self.alpha.handle_event(app, event).or_fail()?;
        if alpha != self.alpha.body().value() {
            let mut c = app.models().config.color.get();
            c.a = self.alpha.body().value() as u8;
            app.models_mut().config.color.set(c);
        }

        let old_replace_mode = self.replace.body().is_on();
        self.replace.handle_event(app, event).or_fail()?;
        let new_replace_mode = self.replace.body().is_on();

        let new_color = app.models().config.color.get();
        if (old_color, old_replace_mode) != (new_color, new_replace_mode) {
            if new_replace_mode {
                self.replace_color(app).or_fail()?;
                app.request_redraw(app.screen_size().to_region());
            } else {
                self.cancel_color_replace_if_need(app).or_fail()?;
                app.request_redraw(self.region);
            }
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            &mut self.rgb,
            &mut self.hsv,
            &mut self.alpha,
            &mut self.replace,
        ]
    }
}

impl FixedSizeWidget for ColorSelectorWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let preview = Size::from_wh(0, COLOR_PREVIEW_HEIGHT);
        let hsv = self.hsv.requiring_size(app);
        let rgb = self.rgb.requiring_size(app);
        let alpha = self.alpha.requiring_size(app);
        let replace = self.alpha.requiring_size(app);

        Size::from_wh(
            preview
                .width
                .max(rgb.width)
                .max(hsv.width)
                .max(alpha.width)
                .max(replace.width),
            preview.height
                + MARGIN
                + hsv.height
                + MARGIN
                + rgb.height
                + MARGIN
                + alpha.height
                + MARGIN
                + replace.height,
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

        offset.y = self.alpha.region().end().y + MARGIN as i32;
        let mut replace_region = Region::new(offset, self.replace.requiring_size(app));
        replace_region.size.width = self.region.size.width;
        self.replace.set_region(app, replace_region);
    }
}
