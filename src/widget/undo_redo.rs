use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    pixel::PixelRegion,
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

impl UndoRedoWidget {
    fn request_redraw_dirty_canvas_region(&self, app: &mut App) {
        let pixel_region = PixelRegion::from_positions(
            app.models().pixel_canvas.dirty_positions().iter().copied(),
        );
        app.request_redraw(pixel_region.to_screen_region(app));
    }
}

impl Default for UndoRedoWidget {
    fn default() -> Self {
        let mut redo = ButtonWidget::new(ButtonKind::Basic, IconId::Redo);
        redo.set_disabled_callback(|app| {
            let canvas = &app.models().pixel_canvas;
            canvas.command_log().len() == canvas.command_log_tail()
        });
        redo.set_number_callback(0, |app| {
            let canvas = &app.models().pixel_canvas;
            (canvas.command_log().len() - canvas.command_log_tail()) as u32
        });

        let mut undo = ButtonWidget::new(ButtonKind::Basic, IconId::Undo);
        undo.set_disabled_callback(|app| app.models().pixel_canvas.command_log_tail() == 0);
        undo.set_number_callback(0, |app| app.models().pixel_canvas.command_log_tail() as u32);
        Self {
            region: Default::default(),
            undo,
            redo,
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
            app.models_mut().pixel_canvas.redo_command().or_fail()?;
            self.request_redraw_dirty_canvas_region(app);
        }

        self.undo.handle_event(app, event).or_fail()?;
        if self.undo.take_clicked(app) {
            app.models_mut().pixel_canvas.undo_command().or_fail()?;
            self.request_redraw_dirty_canvas_region(app);
        }

        // TODO: long press
        event.consume_if_contained(self.region);
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        let max_undo = app.models().config.max_undo.get() as usize;
        while app.models().pixel_canvas.command_log_tail() > max_undo {
            app.models_mut().pixel_canvas.forget_oldest_command();
        }
        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.redo, &mut self.undo]
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
