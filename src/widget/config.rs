use super::{
    block::BlockWidget, number_box::NumberBoxWidget, toggle::ToggleWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

// - frame
//   - [o] frame preview on/off (switch)
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
    frame_width: BlockWidget<NumberBoxWidget>,
    frame_preview: BlockWidget<ToggleWidget>,
}

impl Default for ConfigWidget {
    fn default() -> Self {
        Self {
            region: Region::default(),
            frame_width: BlockWidget::new(
                "FRAME WIDTH".parse().expect("unreachable"),
                NumberBoxWidget::new(1, 64, 9999),
            ),
            frame_preview: BlockWidget::new(
                "FRAME PREVIEW".parse().expect("unreachable"),
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
        self.frame_width.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.frame_preview.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame_preview
            .set(self.frame_preview.body().is_on());

        self.frame_width.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame
            .set_width(self.frame_width.body().value() as u16);
        // TODO: update preview region

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.frame_preview, &mut self.frame_width]
    }
}

impl FixedSizeWidget for ConfigWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.frame_preview.requiring_size(app);
        size.width += self.frame_width.requiring_size(app).width + 8;
        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut frame_preview_region = self.region;
        frame_preview_region.size.width = self.frame_preview.requiring_size(app).width;
        self.frame_preview.set_region(app, frame_preview_region);

        let mut frame_size_region = self.region;
        frame_size_region.position.x = frame_preview_region.end().x + 8;
        frame_size_region.size.width = self.frame_width.requiring_size(app).width;
        self.frame_width.set_region(app, frame_size_region);
    }
}
