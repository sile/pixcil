use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::{Event, MouseAction, TimeoutId},
    widget::{
        bottom_bar::BottomBarWidget, pixel_canvas::PixelCanvasWidget, preview::PreviewWidget,
        side_bar::SideBarWidget, FixedSizeWidget, VariableSizeWidget, Widget,
    },
};
use pagurus::image::{Canvas, Color};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct MainWindow {
    size: Size,
    pixel_canvas: PixelCanvasWidget,
    preview: PreviewWidget,
    side_bar: SideBarWidget,
    bottom_bar: BottomBarWidget,
    finger: bool,
    timeout: Option<(Position, TimeoutId)>,
    cursor: Option<Position>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Window for MainWindow {
    fn region(&self) -> Region {
        self.size.to_region()
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let preview_mode = app.models().preview_mode;

        if preview_mode {
            canvas.fill_rectangle(self.region(), color::CANVAS_PREVIEW_MODE_BACKGROUND);

            let config = &app.models().config;
            let region = config.frame.get_animation_frames_region(config);
            let mut canvas = canvas.mask_region(region.to_screen_region(app));
            self.pixel_canvas.render(app, &mut canvas);
        } else {
            self.pixel_canvas.render(app, canvas);
        }
        self.preview.render_if_need(app, canvas);
        if !preview_mode {
            self.side_bar.render_if_need(app, canvas);
            self.bottom_bar.render_if_need(app, canvas);
        }
        canvas.draw_rectangle(self.region(), color::WINDOW_BORDER);

        if let Some(p) = self.cursor {
            canvas.fill_rectangle(Region::new(p, Size::square(5)), Color::RED);
        }
    }

    fn is_terminated(&self) -> bool {
        false
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        self.size = app.screen_size();

        self.pixel_canvas.set_region(app, self.region());

        let preview_margin = 16;
        let preview_size = self.preview.requiring_size(app);
        let preview_position = Position::from_xy(
            app.screen_size().width as i32 - preview_size.width as i32 - preview_margin,
            preview_margin,
        );
        self.preview.set_position(app, preview_position);

        self.side_bar.set_region(app, self.region());
        self.bottom_bar.set_region(app, self.region());

        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // if let Some((p, id)) = self.timeout {
        //     if let Event::Timeout(id0) = *event {
        //         log::info!("timeout: {id0:?}");
        //         if id == id0 {
        //             // TODO: vibration
        //             self.finger = true;
        //             self.timeout = None;
        //             *event = Event::Mouse {
        //                 action: MouseAction::Down,
        //                 position: p,
        //                 consumed: false,
        //             };
        //         }
        //     }
        // }

        if app.models().preview_mode {
            self.preview.handle_event_before(app).or_fail()?;
            self.preview.handle_event(app, event).or_fail()?;
            self.preview.handle_event_after(app).or_fail()?;
            return Ok(());
        }

        self.pixel_canvas.handle_event_before(app).or_fail()?;
        self.side_bar.handle_event_before(app).or_fail()?;
        self.bottom_bar.handle_event_before(app).or_fail()?;
        self.preview.handle_event_before(app).or_fail()?;

        if !self.pixel_canvas.marker_handler().is_operating() {
            self.side_bar.handle_event(app, event).or_fail()?;
            self.bottom_bar.handle_event(app, event).or_fail()?;
            self.preview.handle_event(app, event).or_fail()?;
        }

        // if let Event::Mouse {
        //     position, action, ..
        // } = event
        // {
        //     // TODO: convert to CM
        //     let p = *position;
        //     position.y -= 100;

        //     if *action == MouseAction::Down {
        //         if self.finger == false {
        //             *action = MouseAction::Move;
        //         }
        //     } else if *action == MouseAction::Up {
        //         self.finger = false;
        //         self.timeout = None;
        //         self.cursor = None;
        //     }
        //     if *action != MouseAction::Up {
        //         if let Some(old) = self.cursor {
        //             app.request_redraw(Region::new(old, Size::square(5)));
        //         }
        //         self.cursor = Some(*position);
        //         app.request_redraw(Region::new(*position, Size::square(5)));
        //     }

        //     if self.finger == false && *action == MouseAction::Move {
        //         let id = app.set_timeout(Duration::from_millis(500));
        //         log::info!("set timeout: {id:?}");
        //         self.timeout = Some((p, id));
        //     }
        // }
        self.pixel_canvas.handle_event(app, event).or_fail()?;

        self.preview.handle_event_after(app).or_fail()?;
        self.bottom_bar.handle_event_after(app).or_fail()?;
        self.side_bar.handle_event_after(app).or_fail()?;
        self.pixel_canvas.handle_event_after(app).or_fail()?;

        Ok(())
    }
}
