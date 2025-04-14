use super::{FixedSizeWidget, Widget, button::ButtonWidget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    region_ext::RegionExt,
};
use orfail::{OrFail, Result};
use pagurus::{
    event::Key,
    image::Canvas,
    spatial::{Position, Region, Size},
};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct MoveFrameWidget {
    region: Region,
    prev_frame: ButtonWidget,
    next_frame: ButtonWidget,
}

impl MoveFrameWidget {
    fn handle_key_event(&mut self, event: &mut Event) -> Result<(bool, bool)> {
        let Event::Key { event, consumed } = event else {
            return Ok((false, false));
        };

        match event.key {
            Key::Char('<') => {
                *consumed = true;
                return Ok((true, false));
            }
            Key::Char('>') => {
                *consumed = true;
                return Ok((false, true));
            }
            _ => {}
        }

        Ok((false, false))
    }
}

impl Default for MoveFrameWidget {
    fn default() -> Self {
        Self {
            region: Region::default(),
            prev_frame: ButtonWidget::new(ButtonKind::Basic, IconId::GoLeft),
            next_frame: ButtonWidget::new(ButtonKind::Basic, IconId::GoRight),
        }
    }
}

impl Widget for MoveFrameWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);

        self.prev_frame.render_if_need(app, canvas);
        self.next_frame.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let (prev_pressed, next_pressed) = self.handle_key_event(event).or_fail()?;

        let mut delta = Position::ORIGIN;

        self.prev_frame.handle_event(app, event).or_fail()?;
        if self.prev_frame.take_clicked(app) || prev_pressed {
            let frames = app.models().pixel_canvas.get_frames(&app.models().config) as usize;
            let current = app.models().config.camera.current_frame(app);
            let width = app.models().config.frame.get_base_region().size().width as i32;
            if current > 0 {
                delta.x -= width;
            } else {
                delta.x += width * (frames - 1) as i32;
            }
        }

        self.next_frame.handle_event(app, event).or_fail()?;
        if self.next_frame.take_clicked(app) || next_pressed {
            let frames = app.models().pixel_canvas.get_frames(&app.models().config) as usize;
            let current = app.models().config.camera.current_frame(app);
            let width = app.models().config.frame.get_base_region().size().width as i32;
            if current + 1 < frames {
                delta.x += width;
            } else {
                delta.x -= width * (frames - 1) as i32;
            }
        }

        if delta != Position::ORIGIN {
            let delta = delta * app.models().config.zoom.get() as u32;
            app.models_mut().config.camera.r#move(delta);
            app.request_redraw(app.screen_size().to_region());
        }

        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.prev_frame, &mut self.next_frame]
    }
}

impl FixedSizeWidget for MoveFrameWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let button_size = self.prev_frame.requiring_size(app);
        Size::from_wh(
            button_size.width * 2 + MARGIN * 4,
            button_size.height + MARGIN * 2,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut block = self.region;
        block.size.width /= 2;

        self.prev_frame
            .set_position(app, block.without_margin(MARGIN).position);
        self.next_frame
            .set_position(app, block.shift_x(1).without_margin(MARGIN).position);
    }
}
