use crate::{asset::Assets, io::IoRequest, model::Models, window::Window};
use pagurus::{
    failure::OrFail,
    spatial::{Region, Size},
    Result,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct App {
    screen_size: Size,
    assets: Assets,
    models: Models,
    spawned_windows: Vec<Box<dyn Window>>,
    io_requests: VecDeque<IoRequest>,
    redraw_requests: Vec<Region>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            screen_size: Size::default(),
            assets: Assets::load().or_fail()?,
            models: Models::default(),
            spawned_windows: Vec::new(),
            io_requests: VecDeque::new(),
            redraw_requests: Vec::new(),
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
        if region.is_empty() {
            return;
        }
        self.redraw_requests.push(region);
    }

    pub fn take_redraw_requests(&mut self) -> Vec<Region> {
        std::mem::take(&mut self.redraw_requests)
    }

    pub fn enqueue_io_request(&mut self, request: IoRequest) {
        self.io_requests.push_back(request);
    }

    pub fn dequeue_io_request(&mut self) -> Option<IoRequest> {
        self.io_requests.pop_front()
    }

    pub fn spawn_window(&mut self, window: impl Window) {
        self.request_redraw(window.region());
        self.spawned_windows.push(Box::new(window));
    }

    pub fn take_spawned_windows(&mut self) -> Vec<Box<dyn Window>> {
        std::mem::take(&mut self.spawned_windows)
    }
}
