use crate::{asset::Assets, io::IoRequest, model::Models};
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
    io_requests: VecDeque<IoRequest>,
    redraw_requests: Vec<Region>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            screen_size: Size::default(),
            assets: Assets::load().or_fail()?,
            models: Models::default(),
            io_requests: VecDeque::new(),
            redraw_requests: Vec::new(),
        })
    }

    pub fn screen_size(&self) -> Size {
        self.screen_size
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
}
