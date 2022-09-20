use crate::{
    asset::Assets,
    event::{InputId, TimeoutId},
    io::IoRequest,
    model::Models,
    window::Window,
};
use pagurus::{
    failure::OrFail,
    spatial::{Region, Size},
    ActionId, Result, System,
};
use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

#[derive(Debug)]
pub struct App {
    screen_size: Size,
    assets: Assets,
    models: Models,
    spawned_windows: Vec<Box<dyn Window>>,
    io_requests: VecDeque<IoRequest>,
    redraw_requests: Vec<Region>,
    next_timeout_id: TimeoutId,
    pending_timeouts: Vec<(TimeoutId, Duration)>,
    timeouts: HashMap<ActionId, TimeoutId>,
    next_input_id: InputId,
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
            next_timeout_id: TimeoutId::default(),
            pending_timeouts: Vec::new(),
            timeouts: HashMap::new(),
            next_input_id: InputId::default(),
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

    pub fn enqueue_input_text_request(&mut self) -> InputId {
        let id = self.next_input_id.next();
        let request = IoRequest::InputText { id };
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

    pub fn set_timeout(&mut self, duration: Duration) -> TimeoutId {
        let id = self.next_timeout_id.next();
        self.pending_timeouts.push((id, duration));
        id
    }

    pub fn take_timeout_id(&mut self, action_id: ActionId) -> Option<TimeoutId> {
        self.timeouts.remove(&action_id)
    }

    pub fn set_pending_timeouts<S: System>(&mut self, system: &mut S) {
        for (id, durtion) in self.pending_timeouts.drain(..) {
            self.timeouts.insert(system.clock_set_timeout(durtion), id);
        }
    }
}
