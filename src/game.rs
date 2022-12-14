use crate::png::decode_sprite;
use crate::{
    app::App,
    event::Event,
    io::InputNumber,
    model::Models,
    window::{main::MainWindow, Window},
};
use pagurus::image::Canvas;
use pagurus::timeout::TimeoutTag;
use pagurus::{
    event::WindowEvent as PagurusWindowEvent,
    event::{Event as PagurusEvent, TimeoutEvent},
    failure::OrFail,
    video::VideoFrame,
    Game, Result, System,
};
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
pagurus::export_wasm_functions!(PixcilGame);

const MAX_FPS: u8 = 120;

#[derive(Default)]
pub struct PixcilGame {
    video_frame: VideoFrame,
    windows: Vec<Box<dyn Window>>,
    app: Option<App>,
    render_timeout: Option<pagurus::timeout::TimeoutId>,
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

        if app.is_redraw_needed() && self.render_timeout.is_none() {
            self.render_timeout = Some(system.clock_set_timeout(
                TimeoutTag::new(0),
                Duration::from_millis(1000 / u64::from(MAX_FPS)),
            ));
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
        match event {
            PagurusEvent::Terminating => return Ok(false),
            PagurusEvent::Timeout(TimeoutEvent { id, .. }) if Some(id) == self.render_timeout => {
                self.render_timeout = None;
                self.render(system).or_fail()?;
            }
            PagurusEvent::Window(PagurusWindowEvent::RedrawNeeded { size }) => {
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

        let app = self.app.as_mut().or_fail()?;
        let event = Event::from_pagurus_event(app, event);
        self.handle_pixcil_event(system, event).or_fail()?;

        Ok(true)
    }

    fn query(&mut self, _system: &mut S, name: &str) -> Result<Vec<u8>> {
        match name {
            "nextIoRequest" => {
                if let Some(req) = self.app.as_mut().or_fail()?.dequeue_io_request() {
                    Ok(serde_json::to_vec(&req).or_fail()?)
                } else {
                    Ok(Vec::new())
                }
            }
            "workspacePng" => {
                let app = self.app.as_ref().or_fail()?;
                let data = app.models().to_png(app).or_fail()?;
                Ok(data)
            }
            "stateVersion" => {
                let app = self.app.as_ref().or_fail()?;
                let version = app.models().pixel_canvas.state_version();
                Ok(version.to_be_bytes().to_vec())
            }
            _ => Err(pagurus::failure::Failure::new().message(format!("unknown query: {name:?}"))),
        }
    }

    fn command(&mut self, system: &mut S, name: &str, data: &[u8]) -> Result<()> {
        match name {
            "notifyInputNumber" => {
                let data: InputNumber = serde_json::from_slice(data).or_fail()?;
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
                self.handle_pixcil_event(system, Some(event)).or_fail()?;

                let app = self.app.as_mut().or_fail()?;
                app.request_redraw(app.screen_size().to_region());
                Ok(())
            }
            "disableSaveWorkspaceButton" => {
                let mut app = self.app.as_mut().or_fail()?;
                app.runtime_options.disable_save_workspace_button = true;
                Ok(())
            }
            _ => {
                Err(pagurus::failure::Failure::new().message(format!("unknown command: {name:?}")))
            }
        }
    }
}
