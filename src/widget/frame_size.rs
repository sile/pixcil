use super::{FixedSizeWidget, Widget, button::ButtonWidget, size_box::SizeBoxWidget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    event::Event,
    pixel::PixelSize,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Position, Region, Size};

const MARGIN: u32 = 8;
const HALF_MARGIN: u32 = MARGIN / 2;

#[derive(Debug)]
pub struct FrameSizeWidget {
    region: Region,
    frame_size: SizeBoxWidget,
    halve: ButtonWidget,
    double: ButtonWidget,
}

impl FrameSizeWidget {
    pub fn new(app: &App) -> Self {
        let frame_size = app.models().config.frame.get_base_region().size();
        Self {
            region: Region::default(),
            frame_size: SizeBoxWidget::with_min_max(
                frame_size,
                PixelSize::square(2),
                PixelSize::square(1024),
            ),
            halve: ButtonWidget::new(ButtonKind::Middle, IconId::Halve),
            double: ButtonWidget::new(ButtonKind::Middle, IconId::Double),
        }
    }

    pub fn value(&self) -> PixelSize {
        self.frame_size.value()
    }
}

impl Widget for FrameSizeWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.frame_size.render_if_need(app, canvas);
        self.halve.render_if_need(app, canvas);
        self.double.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.frame_size.handle_event(app, event).or_fail()?;

        self.halve.handle_event(app, event).or_fail()?;
        if self.halve.take_clicked(app) {
            self.frame_size.set_value(app, self.frame_size.value() / 2);
        }

        self.double.handle_event(app, event).or_fail()?;
        if self.double.take_clicked(app) {
            self.frame_size.set_value(app, self.frame_size.value() * 2);
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.frame_size, &mut self.halve, &mut self.double]
    }
}

impl FixedSizeWidget for FrameSizeWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.frame_size.requiring_size(app);
        size.width += MARGIN + self.halve.requiring_size(app).width;
        size.width += HALF_MARGIN + self.double.requiring_size(app).width;
        size
    }

    fn set_position(&mut self, app: &App, mut position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        self.frame_size.set_position(app, position);
        position.x = self.frame_size.region().end().x + MARGIN as i32;
        position.y += 4;

        self.halve.set_position(app, position);
        position.x = self.halve.region().end().x + HALF_MARGIN as i32;

        self.double.set_position(app, position);
    }
}
