use crate::gesture::PointerEvent;
use crate::png::decode_sprite;
use crate::tags::RENDERING_TAG;
use crate::{
    app::App,
    event::Event,
    io::Input,
    model::Models,
    window::{main::MainWindow, Window},
};
use orfail::OrFail;
#[cfg(feature = "auto-scaling")]
use pagurus::fixed_window::FixedWindow;
use pagurus::image::Canvas;
#[cfg(feature = "auto-scaling")]
use pagurus::spatial::Size;
use pagurus::{event::Event as PagurusEvent, video::VideoFrame, Game, Result, System};
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
pagurus::export_wasm_functions!(PixcilGame);

const MAX_FPS: u8 = 120;

#[derive(Default)]
pub struct PixcilGame {
    video_frame: VideoFrame,
    windows: Vec<Box<dyn Window>>,
    app: Option<App>,
    waiting_rendering: bool,
    #[cfg(feature = "auto-scaling")]
    screen: FixedWindow,
}

impl PixcilGame {
    fn handle_pixcil_event<S: System>(
        &mut self,
        system: &mut S,
        event: Option<Event>,
    ) -> Result<()> {
        let app = self.app.as_mut().or_fail()?;
        if let Some(mut event) = event {
            let mut terminated = false;
            for window in self.windows.iter_mut().rev() {
                window.handle_event(app, &mut event).or_fail()?;
                if window.is_terminated() {
                    terminated = true;
                    app.request_redraw(window.region());
                }
            }

            if terminated {
                self.windows = std::mem::take(&mut self.windows)
                    .into_iter()
                    .filter(|w| !w.is_terminated())
                    .collect();
            }
        }
        self.windows.extend(app.take_spawned_windows());
        app.set_pending_timeouts(system);

        if app.is_redraw_needed() && !self.waiting_rendering {
            self.waiting_rendering = true;
            system.clock_set_timeout(
                RENDERING_TAG,
                Duration::from_millis(1000 / u64::from(MAX_FPS)),
            );
        }
        Ok(())
    }

    fn render<S: System>(&mut self, system: &mut S) -> Result<()> {
        let mut canvas = Canvas::new(&mut self.video_frame);
        let app = self.app.as_mut().or_fail()?;

        if let Some(region) = app.take_redraw_requests() {
            let mut canvas = canvas.mask_region(region);
            for window in &mut self.windows {
                window.render(app, &mut canvas);
            }
        }

        system.video_draw(self.video_frame.as_ref());

        Ok(())
    }
}

impl<S: System> Game<S> for PixcilGame {
    fn initialize(&mut self, _system: &mut S) -> Result<()> {
        self.windows.push(Box::new(MainWindow::new()));
        self.app = Some(App::new().or_fail()?);
        Ok(())
    }

    fn handle_event(&mut self, system: &mut S, event: PagurusEvent) -> Result<bool> {
        #[cfg(feature = "auto-scaling")]
        let event = self.screen.handle_event(event);
        match event {
            PagurusEvent::Timeout(RENDERING_TAG) => {
                self.waiting_rendering = false;
                self.render(system).or_fail()?;
            }
            PagurusEvent::WindowResized(size) => {
                #[cfg(feature = "auto-scaling")]
                {
                    if size.width < size.height && (1..800).contains(&size.width) {
                        self.screen =
                            FixedWindow::new(Size::from_wh(800, 800 * size.height / size.width));
                    } else if size.height < size.width && (1..800).contains(&size.height) {
                        self.screen =
                            FixedWindow::new(Size::from_wh(800 * size.width / size.height, 800));
                    } else {
                        self.screen = FixedWindow::new(size);
                    }
                    let event = PagurusEvent::WindowResized(size);
                    self.screen.handle_event(event);
                }

                #[cfg(feature = "auto-scaling")]
                let size = self.screen.size();
                let app = self.app.as_mut().or_fail()?;
                app.request_redraw(size.to_region());
                if size != app.screen_size() {
                    self.video_frame = VideoFrame::new(system.video_init(size));
                    app.set_screen_size(size);
                    for window in &mut self.windows {
                        window.handle_screen_resized(app).or_fail()?;
                    }
                }
            }
            _ => {}
        };

        let event = Event::from_pagurus_event(event);
        self.handle_pixcil_event(system, event).or_fail()?;

        Ok(true)
    }

    fn query(&mut self, system: &mut S, name: &str) -> Result<Vec<u8>> {
        match name {
            "nextIoRequest" => {
                if let Some(req) = self.app.as_mut().or_fail()?.dequeue_io_request() {
                    Ok(serde_json::to_vec(&req).or_fail()?)
                } else {
                    Ok(Vec::new())
                }
            }
            "workspacePng" => {
                self.app
                    .as_mut()
                    .or_fail()?
                    .models_mut()
                    .config
                    .attrs
                    .update_time(system);

                let app = self.app.as_ref().or_fail()?;
                let data = app.models().to_png().or_fail()?;
                Ok(data)
            }
            "stateVersion" => {
                let app = self.app.as_ref().or_fail()?;
                let version = app.models().pixel_canvas.state_version();
                Ok(version.to_be_bytes().to_vec())
            }
            _ => Err(orfail::Failure::new(format!("unknown query: {name:?}"))),
        }
    }

    fn command(&mut self, system: &mut S, name: &str, data: &[u8]) -> Result<()> {
        match name {
            "notifyInputNumber" | "notifyInputSize" => {
                let data: Input = serde_json::from_slice(data).or_fail()?;
                let event = Event::Input {
                    id: data.id,
                    text: data.number,
                };
                self.handle_pixcil_event(system, Some(event)).or_fail()?;
                Ok(())
            }
            "loadWorkspace" => {
                let app = self.app.as_mut().or_fail()?;
                *app.models_mut() = Models::from_png(data).or_fail()?;
                app.request_redraw(app.screen_size().to_region());
                self.handle_pixcil_event(system, Some(Event::Noop))
                    .or_fail()?;

                Ok(())
            }
            "importImage" => {
                let image = decode_sprite(data).or_fail()?;
                let event = Event::Import { image };
                let app = self.app.as_mut().or_fail()?;
                app.request_redraw(app.screen_size().to_region());
                self.handle_pixcil_event(system, Some(event)).or_fail()?;
                Ok(())
            }
            "disableSaveWorkspaceButton" => {
                let app = self.app.as_mut().or_fail()?;
                app.runtime_options.disable_save_workspace_button = true;
                Ok(())
            }
            "handlePointerEvent" => {
                let mut pointer_event: PointerEvent = serde_json::from_slice(data).or_fail()?;
                let pagurus_event = PagurusEvent::Mouse(pointer_event.to_mouse_event());

                #[cfg(feature = "auto-scaling")]
                let pagurus_event = self.screen.handle_event(pagurus_event);

                let mut event = Event::from_pagurus_event(pagurus_event).or_fail()?;
                let Event::Mouse {
                    pointer, position, ..
                } = &mut event
                else {
                    return Err(orfail::Failure::new("unreachable"));
                };
                pointer_event.set_position(*position);
                *pointer = Some(pointer_event);
                self.handle_pixcil_event(system, Some(event)).or_fail()?;

                Ok(())
            }
            _ => Err(orfail::Failure::new(format!("unknown command: {name:?}"))),
        }
    }
}
