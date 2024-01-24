use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    region_ext::RegionExt,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Position, Region, Size};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct ManipulateToolWidget {
    region: Region,
    cut: ButtonWidget,
    copy: ButtonWidget,
    vertical_flip: ButtonWidget,
    horizontal_flip: ButtonWidget,
    clockwise_rotate: ButtonWidget,
    opacity_rotate: ButtonWidget,
}

impl ManipulateToolWidget {
    // TODO: rename
    pub fn is_cut_clicked(&mut self, app: &mut App) -> bool {
        self.cut.take_clicked(app)
    }

    pub fn is_copy_clicked(&mut self, app: &mut App) -> bool {
        self.copy.take_clicked(app)
    }

    pub fn is_vertical_flip_clicked(&mut self, app: &mut App) -> bool {
        self.vertical_flip.take_clicked(app)
    }

    pub fn is_horizontal_flip_clicked(&mut self, app: &mut App) -> bool {
        self.horizontal_flip.take_clicked(app)
    }

    pub fn is_clockwise_rotate_clicked(&mut self, app: &mut App) -> bool {
        self.clockwise_rotate.take_clicked(app)
    }

    pub fn is_opacity_rotate_clicked(&mut self, app: &mut App) -> bool {
        self.opacity_rotate.take_clicked(app)
    }
}

impl Default for ManipulateToolWidget {
    fn default() -> Self {
        Self {
            region: Default::default(),
            cut: ButtonWidget::new(ButtonKind::Basic, IconId::Cut),
            copy: ButtonWidget::new(ButtonKind::Basic, IconId::Copy),
            vertical_flip: ButtonWidget::new(ButtonKind::Basic, IconId::VerticalFlip),
            horizontal_flip: ButtonWidget::new(ButtonKind::Basic, IconId::HorizontalFlip),
            clockwise_rotate: ButtonWidget::new(ButtonKind::Basic, IconId::ClockwiseRotate),
            opacity_rotate: ButtonWidget::new(ButtonKind::Basic, IconId::OpacityRotate),
        }
    }
}

impl Widget for ManipulateToolWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.cut.render_if_need(app, canvas);
        self.copy.render_if_need(app, canvas);
        self.vertical_flip.render_if_need(app, canvas);
        self.horizontal_flip.render_if_need(app, canvas);
        self.clockwise_rotate.render_if_need(app, canvas);
        self.opacity_rotate.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.cut.handle_event(app, event).or_fail()?;
        self.copy.handle_event(app, event).or_fail()?;
        self.vertical_flip.handle_event(app, event).or_fail()?;
        self.horizontal_flip.handle_event(app, event).or_fail()?;
        self.clockwise_rotate.handle_event(app, event).or_fail()?;
        self.opacity_rotate.handle_event(app, event).or_fail()?;
        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            &mut self.cut,
            &mut self.copy,
            &mut self.vertical_flip,
            &mut self.horizontal_flip,
            &mut self.clockwise_rotate,
            &mut self.opacity_rotate,
        ]
    }
}

impl FixedSizeWidget for ManipulateToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let button_size = self.cut.requiring_size(app);
        let buttons = 6;
        Size::from_wh(
            button_size.width + MARGIN * 2,
            (button_size.height + MARGIN) * buttons + MARGIN * 2,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        let buttons = 6;

        let mut block = self.region;
        block.size.height /= buttons;

        self.cut
            .set_position(app, block.without_margin(MARGIN).position);
        self.copy
            .set_position(app, block.shift_y(1).without_margin(MARGIN).position);
        self.vertical_flip
            .set_position(app, block.shift_y(2).without_margin(MARGIN).position);
        self.horizontal_flip
            .set_position(app, block.shift_y(3).without_margin(MARGIN).position);
        self.clockwise_rotate
            .set_position(app, block.shift_y(4).without_margin(MARGIN).position);
        self.opacity_rotate
            .set_position(app, block.shift_y(5).without_margin(MARGIN).position);
    }
}
