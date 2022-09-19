use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::{Event, MouseAction},
};
use pagurus::{
    spatial::{Contains, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct ConfigWindow {
    region: Region,
    terminated: bool,
    // - frame
    //   - frame preview on/off (switch)
    //   - frame size (width / height sliders)
    // - unit size (slider)
    // - max undo history (select box)
    // - layer count (slider)
    // - animation
    //   - frame count (slider)
    //   - fps (slider)
}

impl Window for ConfigWindow {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::WINDOW_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);

        canvas.offset(self.region.position + 10).draw_text(
            &"HELLO".parse().expect("unreachable"),
            2,
            &app.assets().alphabet_10x14,
        );
    }

    fn is_terminated(&self) -> bool {
        self.terminated
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        let center = app.screen_size().to_region().center();
        self.region = Region::new(center - 100, Size::square(200));
        Ok(())
    }

    fn handle_event(&mut self, _app: &mut App, event: &mut Event) -> Result<()> {
        if let Event::Mouse {
            action, position, ..
        } = event
        {
            if *action == MouseAction::Up && !self.region.contains(position) {
                self.terminated = true;
            }
            event.consume();
        }
        Ok(())
    }
}
