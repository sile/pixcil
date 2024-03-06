use super::{FixedSizeWidget, Widget};
use crate::{app::App, event::Event};
use orfail::Result;
use pagurus::{
    image::Canvas,
    spatial::{Position, Region, Size},
};

#[derive(Debug, Default)]
pub struct ColorPaletteWidget {
    region: Region,
}

impl ColorPaletteWidget {
    pub fn new(_app: &App, width: u32) -> Self {
        Self {
            region: Region::new(Position::default(), Size::from_wh(width, 0)),
        }
    }
}

impl Widget for ColorPaletteWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        // self.render_color_preview(app, canvas);
        // self.hsv.render_if_need(app, canvas);
        // self.rgb.render_if_need(app, canvas);
        // self.alpha.render_if_need(app, canvas);
        // self.replace.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // let old_color = app.models().config.color.get();

        // self.hsv.handle_event(app, event).or_fail()?;
        // self.rgb.handle_event(app, event).or_fail()?;

        // let alpha = self.alpha.body().value();
        // self.alpha.handle_event(app, event).or_fail()?;
        // if alpha != self.alpha.body().value() {
        //     let mut c = app.models().config.color.get();
        //     c.a = self.alpha.body().value() as u8;
        //     app.models_mut().config.color.set(c);
        // }

        // let old_replace_mode = self.replace.body().is_on();
        // self.replace.handle_event(app, event).or_fail()?;
        // let new_replace_mode = self.replace.body().is_on();

        // let new_color = app.models().config.color.get();
        // if (old_color, old_replace_mode) != (new_color, new_replace_mode) {
        //     if new_replace_mode {
        //         self.replace_color(app).or_fail()?;
        //         app.request_redraw(app.screen_size().to_region());
        //     } else {
        //         self.cancel_color_replace_if_need(app).or_fail()?;
        //         app.request_redraw(self.region);
        //     }
        // }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            // &mut self.rgb,
            // &mut self.hsv,
            // &mut self.alpha,
            // &mut self.replace,
        ]
    }
}

impl FixedSizeWidget for ColorPaletteWidget {
    fn requiring_size(&self, app: &App) -> Size {
        // let preview = Size::from_wh(0, COLOR_PREVIEW_HEIGHT);
        // let hsv = self.hsv.requiring_size(app);
        // let rgb = self.rgb.requiring_size(app);
        // let alpha = self.alpha.requiring_size(app);
        // let replace = self.alpha.requiring_size(app);

        // Size::from_wh(
        //     preview
        //         .width
        //         .max(rgb.width)
        //         .max(hsv.width)
        //         .max(alpha.width)
        //         .max(replace.width),
        //     preview.height
        //         + MARGIN
        //         + hsv.height
        //         + MARGIN
        //         + rgb.height
        //         + MARGIN
        //         + alpha.height
        //         + MARGIN
        //         + replace.height,
        // )
        self.region.size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        // let mut offset = position;
        // offset.y += (COLOR_PREVIEW_HEIGHT + MARGIN) as i32;
        // self.hsv
        //     .set_region(app, Region::new(offset, self.hsv.requiring_size(app)));

        // offset.y = self.hsv.region().end().y + MARGIN as i32;
        // self.rgb
        //     .set_region(app, Region::new(offset, self.rgb.requiring_size(app)));

        // offset.y = self.rgb.region().end().y + MARGIN as i32;
        // self.alpha
        //     .set_region(app, Region::new(offset, self.alpha.requiring_size(app)));

        // offset.y = self.alpha.region().end().y + MARGIN as i32;
        // let mut replace_region = Region::new(offset, self.replace.requiring_size(app));
        // replace_region.size.width = self.region.size.width;
        // self.replace.set_region(app, replace_region);
    }
}
