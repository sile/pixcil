use super::{
    block::BlockWidget, toggle::ToggleWidget, FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

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

#[derive(Debug)]
pub struct ConfigWidget {
    region: Region,
    frame_preview: BlockWidget<ToggleWidget>,
}

impl Default for ConfigWidget {
    fn default() -> Self {
        Self {
            region: Region::default(),
            frame_preview: BlockWidget::new(
                "PREVIEW".parse().expect("unreachable"),
                ToggleWidget::default(),
            ),
        }
    }
}

impl Widget for ConfigWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.frame_preview.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.frame_preview.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame_preview
            .set(self.frame_preview.body().is_on());

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.frame_preview]
    }
}

impl FixedSizeWidget for ConfigWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.frame_preview.requiring_size(app)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.frame_preview.set_region(app, self.region);
    }
}
