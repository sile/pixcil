use crate::{
    asset::Assets, event::InputId, io::IoRequest, model::Models, pixel::PixelPosition,
    window::Window,
};
use orfail::OrFail;
use pagurus::{
    event::TimeoutTag,
    spatial::{Region, Size},
    Result, System,
};
use std::{collections::VecDeque, time::Duration};

#[derive(Debug, Default)]
pub struct RuntimeOptions {
    pub disable_save_workspace_button: bool,
}

#[derive(Debug)]
pub struct App {
    screen_size: Size,
    assets: Assets,
    models: Models,
    spawned_windows: Vec<Box<dyn Window>>,
    io_requests: VecDeque<IoRequest>,
    redraw_region: Region,
    pending_timeouts: Vec<(TimeoutTag, Duration)>,
    next_input_id: InputId,
    pub runtime_options: RuntimeOptions,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            screen_size: Size::default(),
            assets: Assets::load().or_fail()?,
            models: Models::default(),
            spawned_windows: Vec::new(),
            io_requests: VecDeque::new(),
            redraw_region: Region::default(),
            pending_timeouts: Vec::new(),
            next_input_id: InputId::default(),
            runtime_options: RuntimeOptions::default(),
        })
    }

    pub fn screen_size(&self) -> Size {
        self.screen_size
    }

    pub fn set_screen_size(&mut self, size: Size) {
        self.screen_size = size;
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }

    pub fn models(&self) -> &Models {
        &self.models
    }

    pub fn models_mut(&mut self) -> &mut Models {
        &mut self.models
    }

    pub fn request_redraw(&mut self, region: Region) {
        self.redraw_region = self.redraw_region.union(region);
    }

    pub fn take_redraw_requests(&mut self) -> Option<Region> {
        if self.redraw_region.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.redraw_region))
        }
    }

    pub fn is_redraw_needed(&self) -> bool {
        !self.redraw_region.is_empty()
    }

    pub fn enqueue_input_number_request(&mut self) -> InputId {
        let id = self.next_input_id.get_and_increment();
        let request = IoRequest::InputNumber { id };
        self.io_requests.push_back(request);
        id
    }

    pub fn enqueue_input_size_request(&mut self) -> InputId {
        let id = self.next_input_id.get_and_increment();
        let request = IoRequest::InputSize { id };
        self.io_requests.push_back(request);
        id
    }

    pub fn enqueue_io_request(&mut self, request: IoRequest) {
        self.io_requests.push_back(request);
    }

    pub fn dequeue_io_request(&mut self) -> Option<IoRequest> {
        self.io_requests.pop_front()
    }

    pub fn spawn_window(&mut self, mut window: impl Window) -> Result<()> {
        window.handle_screen_resized(self).or_fail()?;
        self.request_redraw(window.region());
        self.spawned_windows.push(Box::new(window));
        Ok(())
    }

    pub fn take_spawned_windows(&mut self) -> Vec<Box<dyn Window>> {
        std::mem::take(&mut self.spawned_windows)
    }

    pub fn set_timeout(&mut self, tag: TimeoutTag, duration: Duration) {
        self.pending_timeouts.push((tag, duration));
    }

    pub fn set_pending_timeouts<S: System>(&mut self, system: &mut S) {
        for (tag, durtion) in self.pending_timeouts.drain(..) {
            system.clock_set_timeout(tag, durtion);
        }
    }

    pub fn zoom(&mut self, zoom_in: bool) {
        let screen_center = self.screen_size().to_region().center();
        let center_pixel = PixelPosition::from_screen_position(self, screen_center);
        if zoom_in {
            self.models_mut().config.zoom.increment();
        } else {
            self.models_mut().config.zoom.decrement();
        }
        let delta = center_pixel.to_screen_position(self) - screen_center;
        self.models_mut().config.camera.r#move(delta);
        self.request_redraw(self.screen_size().to_region());
    }
}
