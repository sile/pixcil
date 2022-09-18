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
pub struct UndoRedoWidget {
    region: Region,
    undo: ButtonWidget,
    redo: ButtonWidget,
}

impl Default for UndoRedoWidget {
    fn default() -> Self {
        Self {
            region: Default::default(),
            undo: ButtonWidget::new(ButtonKind::Basic, IconId::Undo),
            redo: ButtonWidget::new(ButtonKind::Basic, IconId::Redo),
        }
    }
}

impl Widget for UndoRedoWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.redo.render_if_need(app, canvas);
        self.undo.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.redo.handle_event(app, event).or_fail()?;
        if self.redo.take_clicked(app) {
            // TODO
        }

        self.undo.handle_event(app, event).or_fail()?;
        if self.undo.take_clicked(app) {
            // TODO
        }

        event.consume_if_contained(self.region);
        Ok(())
    }
}

impl FixedSizeWidget for UndoRedoWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let undo_size = self.undo.requiring_size(app);
        let redo_size = self.redo.requiring_size(app);
        Size::from_wh(
            redo_size.width + MARGIN * 2,
            redo_size.height + undo_size.height + MARGIN * 4,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut block = self.region;
        block.size.height /= 2;

        self.redo
            .set_position(app, block.without_margin(MARGIN).position);
        self.undo
            .set_position(app, block.shift_y(1).without_margin(MARGIN).position);
    }
}
