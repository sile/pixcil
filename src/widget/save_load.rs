use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    io::IoRequest,
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
pub struct SaveLoadWidget {
    region: Region,
    save: ButtonWidget,
    load: ButtonWidget,
    import: ButtonWidget,
}

impl Default for SaveLoadWidget {
    fn default() -> Self {
        Self {
            region: Default::default(),
            save: ButtonWidget::new(ButtonKind::Basic, IconId::Save),
            load: ButtonWidget::new(ButtonKind::Basic, IconId::Load),
            import: ButtonWidget::new(ButtonKind::Basic, IconId::Import),
        }
    }
}

impl Widget for SaveLoadWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.save.render_if_need(app, canvas);
        self.load.render_if_need(app, canvas); // TODO: Disable if there are dirty pixels
        self.import.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.save.handle_event(app, event).or_fail()?;
        if self.save.take_clicked(app) {
            app.enqueue_io_request(IoRequest::SaveWorkspace);
        }

        self.load.handle_event(app, event).or_fail()?;
        if self.load.take_clicked(app) {
            app.enqueue_io_request(IoRequest::LoadWorkspace);
        }

        self.import.handle_event(app, event).or_fail()?;
        if self.import.take_clicked(app) {
            app.enqueue_io_request(IoRequest::ImportImage);
        }

        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.save, &mut self.load, &mut self.import]
    }
}

impl FixedSizeWidget for SaveLoadWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let button_size = self.save.requiring_size(app);
        Size::from_wh(
            button_size.width + MARGIN * 2,
            button_size.height * 3 + MARGIN * 6,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut block = self.region;
        block.size.height /= 3;

        self.save
            .set_position(app, block.without_margin(MARGIN).position);
        self.load
            .set_position(app, block.shift_y(1).without_margin(MARGIN).position);
        self.import
            .set_position(app, block.shift_y(2).without_margin(MARGIN).position);
    }
}
