use std::time::Duration;

use super::{VariableSizeWidget, Widget};
use crate::{
    app::App,
    event::{Event, MouseAction, TimeoutId},
};
use pagurus::{
    spatial::{Position, Region},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct MoveCameraWidget {
    region: Region,
    start: Option<Position>,
    in_redraw_interval: Option<TimeoutId>,
}

impl MoveCameraWidget {
    pub fn new(app: &App) -> Self {
        Self {
            region: app.screen_size().to_region(),
            start: None,
            in_redraw_interval: None,
        }
    }
}

impl Widget for MoveCameraWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, _app: &App, _canvas: &mut Canvas) {
        // TODO: render drag cursor icon
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // NOTE: This widget never consume events.
        match event {
            Event::Mouse {
                consumed: false,
                action: MouseAction::Down,
                position,
            } => {
                self.start = Some(*position);
            }
            Event::Mouse {
                consumed: false,
                action: MouseAction::Move,
                position,
            } if self.start.is_some() => {
                let start = self.start.expect("unreachable");
                let end = *position;
                app.models_mut().config.camera.r#move(start - end);
                if self.in_redraw_interval.is_none() {
                    let fps = 60;
                    self.in_redraw_interval = Some(app.set_timeout(Duration::from_secs(1) / fps));
                }
                self.start = Some(end);
            }
            Event::Mouse { .. } => {
                self.start = None;
            }
            Event::Timeout(id) if self.in_redraw_interval == Some(*id) => {
                app.request_redraw(self.region);
                self.in_redraw_interval = None;
            }
            _ => {}
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl VariableSizeWidget for MoveCameraWidget {
    fn set_region(&mut self, _app: &App, region: Region) {
        self.region = region;
    }
}
