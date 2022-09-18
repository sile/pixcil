use super::{
    undo_redo::UndoRedoWidget, zoom::ZoomWidget, FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 16;

#[derive(Debug, Default)]
pub struct SideBarWidget {
    region: Region,
    undo_redo: UndoRedoWidget,
    zoom: ZoomWidget,
}

impl Widget for SideBarWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.undo_redo.render_if_need(app, canvas);
        self.zoom.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.undo_redo.handle_event(app, event).or_fail()?;
        self.zoom.handle_event(app, event).or_fail()?;
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.undo_redo, &mut self.zoom]
    }
}

impl VariableSizeWidget for SideBarWidget {
    fn set_region(&mut self, app: &App, region: Region) {
        self.region.position.x = MARGIN as i32;

        let undo_redo_size = self.undo_redo.requiring_size(app);
        let undo_redo_position = Position::from_xy(
            MARGIN as i32,
            region.size.height as i32 / 2 - undo_redo_size.height as i32 / 2,
        );
        self.undo_redo.set_position(app, undo_redo_position);

        let zoom_size = self.zoom.requiring_size(app);
        let zoom_position = Position::from_xy(
            MARGIN as i32,
            region.size.height as i32 * 3 / 4 - zoom_size.height as i32 / 2,
        );
        self.zoom.set_position(app, zoom_position);

        self.region.size = Size::from_wh(undo_redo_size.width, region.size.height);
    }
}
