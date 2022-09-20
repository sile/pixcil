use super::Window;
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::{Event, MouseAction},
    region_ext::RegionExt,
    widget::{block::BlockWidget, tool_box::ToolBoxWidget, VariableSizeWidget, Widget},
};
use pagurus::{
    failure::OrFail,
    spatial::{Contains, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct ConfigWindow {
    region: Region,
    terminated: bool,
    test: BlockWidget<ToolBoxWidget>,
    // - frame
    //   - frame preview on/off (switch)
    //   - frame size (width / height sliders)
    // - layer count (slider)
    // - animation
    //   - frame count (slider)
    //   - fps (slider)
    // - General
    //   - unit size (slider)
    //   - max undo history (select box)
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            region: Region::default(),
            terminated: false,
            test: BlockWidget::new("TEST".parse().unwrap(), Default::default()),
        }
    }
}

impl Window for ConfigWindow {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::WINDOW_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.test.render_if_need(app, canvas);
    }

    fn is_terminated(&self) -> bool {
        self.terminated
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        let center = app.screen_size().to_region().center();
        self.region = Region::new(center - 300, Size::square(600));
        self.test.set_region(app, self.region.without_margin(8));
        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.test.handle_event(app, event).or_fail()?;

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
