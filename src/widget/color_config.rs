use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    region_ext::RegionExt,
    window::{color_selector::ColorSelectorWindow, config::ConfigWindow},
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::{color::Rgba, image::Canvas};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct ColorConfigWidget {
    region: Region,
    color: ButtonWidget,
    config: ButtonWidget,
}

impl ColorConfigWidget {
    fn render_color_label(&self, app: &App, canvas: &mut Canvas) {
        let color = app.models().config.color.get();
        let offset = self.color.state().offset(self.color.kind()).y;
        let mut label_region = self.color.region();
        label_region.position.x += 4;
        label_region.size.width -= 8;

        label_region.position.y += 4 + offset;
        label_region.size.height -= 4 + 12;

        canvas.fill_rectangle(label_region, Rgba::new(255, 255, 255, 255).into());
        canvas.fill_rectangle(label_region.without_margin(2), color.into());
    }
}

impl Default for ColorConfigWidget {
    fn default() -> Self {
        Self {
            region: Default::default(),
            color: ButtonWidget::new(ButtonKind::Basic, IconId::Null),
            config: ButtonWidget::new(ButtonKind::Basic, IconId::Settings),
        }
    }
}

impl Widget for ColorConfigWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);

        self.color.render_if_need(app, canvas);
        self.render_color_label(app, canvas);

        self.config.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.color.handle_event(app, event).or_fail()?;
        if self.color.take_clicked(app) {
            app.spawn_window(ColorSelectorWindow::default()).or_fail()?;
        }

        self.config.handle_event(app, event).or_fail()?;
        if self.config.take_clicked(app) {
            app.spawn_window(ConfigWindow::default()).or_fail()?;
        }

        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.color, &mut self.config]
    }
}

impl FixedSizeWidget for ColorConfigWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let button_size = self.color.requiring_size(app);
        Size::from_wh(
            button_size.width * 2 + MARGIN * 4,
            button_size.height + MARGIN * 2,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut block = self.region;
        block.size.width /= 2;

        self.color
            .set_position(app, block.without_margin(MARGIN).position);
        self.config
            .set_position(app, block.shift_x(1).without_margin(MARGIN).position);
    }
}
