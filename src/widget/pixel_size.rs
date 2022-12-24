use super::{button::ButtonWidget, number_box::NumberBoxWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    event::Event,
    model::config::MinimumPixelSize,
};
use pagurus::image::Canvas;
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct PixelSizeWidget {
    region: Region,
    pixel_size: NumberBoxWidget,
    halve: ButtonWidget,
    double: ButtonWidget,
}

impl PixelSizeWidget {
    pub fn new(app: &App) -> Self {
        let pixel_size = app.models().config.minimum_pixel_size.get();
        Self {
            region: Region::default(),
            pixel_size: NumberBoxWidget::new(
                MinimumPixelSize::MIN.get().width as u32,
                pixel_size.width as u32,
                MinimumPixelSize::MAX.get().width as u32,
            ),
            halve: ButtonWidget::new(ButtonKind::Middle, IconId::Halve),
            double: ButtonWidget::new(ButtonKind::Middle, IconId::Double),
        }
    }

    pub fn value(&self) -> u16 {
        self.pixel_size.value() as u16
    }
}

impl Widget for PixelSizeWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.pixel_size.render_if_need(app, canvas);
        self.halve.render_if_need(app, canvas);
        self.double.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.pixel_size.handle_event(app, event).or_fail()?;

        self.halve.handle_event(app, event).or_fail()?;
        if self.halve.take_clicked(app) {
            self.pixel_size.set_value(app, self.pixel_size.value() / 2);
        }

        self.double.handle_event(app, event).or_fail()?;
        if self.double.take_clicked(app) {
            self.pixel_size.set_value(app, self.pixel_size.value() * 2);
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.pixel_size, &mut self.halve, &mut self.double]
    }
}

impl FixedSizeWidget for PixelSizeWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.pixel_size.requiring_size(app);
        size.width += (MARGIN * 3) + self.halve.requiring_size(app).width;
        size.width += MARGIN + self.double.requiring_size(app).width;
        size
    }

    fn set_position(&mut self, app: &App, mut position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        self.pixel_size.set_position(app, position);
        position.x = self.pixel_size.region().end().x + (MARGIN * 3) as i32;
        position.y += 4;

        self.halve.set_position(app, position);
        position.x = self.halve.region().end().x + MARGIN as i32;

        self.double.set_position(app, position);
    }
}
