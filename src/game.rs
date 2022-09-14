use crate::{
    app::App,
    window::{main::MainWindow, Window},
};
use pagurus::{event::Event, failure::OrFail, video::VideoFrame, Game, Result, System};
use pagurus_game_std::logger::Logger;

pagurus_game_std::export_wasm_functions!(PixcilGame);

#[derive(Default)]
pub struct PixcilGame {
    logger: Logger,
    video_frame: VideoFrame,
    windows: Vec<Box<dyn Window>>,
    app: Option<App>,
}

impl PixcilGame {
    fn handle_event_without_log_flush<S: System>(
        &mut self,
        system: &mut S,
        event: Event,
    ) -> Result<bool> {
        todo!()
    }
}

impl<S: System> Game<S> for PixcilGame {
    fn initialize(&mut self, system: &mut S) -> Result<()> {
        self.logger = Logger::init(log::Level::Debug).or_fail()?;
        self.windows.push(Box::new(MainWindow::new()));
        self.app = Some(App::new().or_fail()?);
        self.logger.flush(system);
        Ok(())
    }

    fn handle_event(&mut self, system: &mut S, event: Event) -> Result<bool> {
        let result = self.handle_event_without_log_flush(system, event).or_fail();
        self.logger.flush(system);
        result
    }
}
