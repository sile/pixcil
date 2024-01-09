use super::{button::ButtonWidget, number_box::NumberBoxWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId, Text},
    canvas_ext::CanvasExt,
    event::{Event, MouseAction},
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Contains, Position, Region, Size};

const BAR_WIDTH: u32 = 360;
const MARGIN: u32 = 8;

pub struct SliderWidget {
    region: Region,
    label: Text,
    input: NumberBoxWidget,
    left: ButtonWidget,
    right: ButtonWidget,
    render_bar: fn(&Self, &App, &mut Canvas),
}

impl SliderWidget {
    pub fn new(
        label: Text,
        min: u32,
        value: u32,
        max: u32,
        render_bar: fn(&Self, &App, &mut Canvas),
    ) -> Self {
        let mut left = ButtonWidget::new(ButtonKind::SliderLeft, IconId::Null);
        let mut right = ButtonWidget::new(ButtonKind::SliderRight, IconId::Null);
        left.enable_long_press();
        right.enable_long_press();
        Self {
            region: Default::default(),
            label,
            render_bar,
            input: NumberBoxWidget::new(min, value, max),
            left,
            right,
        }
    }

    pub fn bar_region(&self) -> Region {
        let mut region = self.region;

        region.position.x = self.left.region().end().x + MARGIN as i32;
        region.size.width = BAR_WIDTH;

        region.position.y += 8;
        region.size.height -= 10;

        region
    }

    pub fn value(&self) -> u32 {
        self.input.value()
    }

    pub fn set_value(&mut self, app: &mut App, value: u32) {
        self.input.set_value(app, value);
    }

    fn render_cursor(&self, app: &App, canvas: &mut Canvas) {
        let cursor = &app.assets().slider_cursor;
        let mut position = self.bar_region().position;
        position.y -= 8;
        position.x -= 3;
        position.x += (self.bar_region().size.width as f64
            * (self.input.value() - self.input.min()) as f64
            / (self.input.max() - self.input.min()) as f64)
            .round() as i32;
        canvas.offset(position).draw_sprite(cursor);
    }
}

impl std::fmt::Debug for SliderWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SliderWidget {{ .. }}")
    }
}

impl Widget for SliderWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let mut label_position = self.region.position;
        label_position.y += 8;
        canvas
            .offset(label_position)
            .draw_text(&self.label, &app.assets().alphabet_10x14);

        self.input.render_if_need(app, canvas);
        self.left.render_if_need(app, canvas);
        (self.render_bar)(self, app, canvas);
        self.render_cursor(app, canvas);
        self.right.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let value = self.input.value();
        self.input.handle_event(app, event).or_fail()?;

        self.left.handle_event(app, event).or_fail()?;
        if self.left.take_clicked(app) {
            self.input
                .set_value(app, self.input.value().saturating_sub(1));
        }

        self.right.handle_event(app, event).or_fail()?;
        if self.right.take_clicked(app) {
            self.input.set_value(app, self.input.value() + 1);
        }

        if let Event::Mouse {
            consumed: false,
            action: MouseAction::Up,
            position,
        } = event
        {
            let bar_region = self.bar_region();
            if bar_region.contains(position) {
                let value =
                    (position.x - bar_region.position.x) as f64 / bar_region.size.width as f64;
                let value = ((self.input.max() - self.input.min()) as f64 * value).round() as u32
                    + self.input.min();
                self.input.set_value(app, value);
            }
        }
        event.consume_if_contained(self.region);

        if value != self.input.value() {
            app.request_redraw(self.region);
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.input, &mut self.left, &mut self.right]
    }
}

impl FixedSizeWidget for SliderWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = Size::EMPTY;
        size.height = self.left.requiring_size(app).height;

        size.width += self.label.size().width;
        size.width += self.input.requiring_size(app).width;
        size.width += MARGIN + self.left.requiring_size(app).width;
        size.width += MARGIN + BAR_WIDTH;
        size.width += MARGIN + self.right.requiring_size(app).width;

        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut offset = self.region.position;
        offset.x += self.label.size().width as i32;
        self.input.set_position(app, offset);

        offset.x = self.input.region().end().x + MARGIN as i32;
        self.left.set_position(app, offset);

        offset.x = self.bar_region().end().x + MARGIN as i32;
        self.right.set_position(app, offset);
    }
}
