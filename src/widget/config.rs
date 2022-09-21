use super::{
    block::BlockWidget, number_box::NumberBoxWidget, toggle::ToggleWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{app::App, event::Event, pixel::PixelSize, region_ext::RegionExt};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;
const GROUP_MARGIN: u32 = 24;

// TODO
// - layer count (slider)
// - animation
//   - frame count (slider)
//   - fps (slider)

#[derive(Debug)]
pub struct ConfigWidget {
    region: Region,

    // General settings
    minimum_pixel_size: BlockWidget<NumberBoxWidget>,
    max_undos: BlockWidget<NumberBoxWidget>,

    // Frame settings
    frame_width: BlockWidget<NumberBoxWidget>,
    frame_height: BlockWidget<NumberBoxWidget>,
    frame_preview: BlockWidget<ToggleWidget>,
}

impl ConfigWidget {
    pub fn new(app: &App) -> Self {
        let minimum_pixel_size = app.models().config.minimum_pixel_size.get();
        let max_undos = app.models().config.max_undos.get();
        let frame_size = app.models().config.frame.get().size();
        Self {
            region: Region::default(),

            // General
            minimum_pixel_size: BlockWidget::new(
                "MINIMUM PIXEL SIZE".parse().expect("unreachable"),
                NumberBoxWidget::new(1, minimum_pixel_size.width as u32, 9999),
            ),
            max_undos: BlockWidget::new(
                "MAX UNDOS".parse().expect("unreachable"),
                NumberBoxWidget::new(0, max_undos, u32::MAX),
            ),

            // Frame
            frame_width: BlockWidget::new(
                "FRAME WIDTH".parse().expect("unreachable"),
                NumberBoxWidget::new(1, frame_size.width as u32, 9999),
            ),
            frame_height: BlockWidget::new(
                "FRAME HEIGHT".parse().expect("unreachable"),
                NumberBoxWidget::new(1, frame_size.height as u32, 9999),
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
        // General
        self.minimum_pixel_size.render_if_need(app, canvas);
        self.max_undos.render_if_need(app, canvas);

        // Frame
        self.frame_width.render_if_need(app, canvas);
        self.frame_height.render_if_need(app, canvas);
        self.frame_preview.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // General
        self.minimum_pixel_size.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .minimum_pixel_size
            .set(PixelSize::square(
                self.minimum_pixel_size.body().value() as u16
            ));

        self.max_undos.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .max_undos
            .set(self.max_undos.body().value());

        // Frame
        self.frame_width.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame
            .set_width(self.frame_width.body().value() as u16);

        self.frame_height.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame
            .set_height(self.frame_height.body().value() as u16);

        self.frame_preview.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame_preview
            .set(self.frame_preview.body().is_on());

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            // General
            &mut self.minimum_pixel_size,
            &mut self.max_undos,
            // Frame
            &mut self.frame_width,
            &mut self.frame_height,
            &mut self.frame_preview,
        ]
    }
}

impl FixedSizeWidget for ConfigWidget {
    fn requiring_size(&self, app: &App) -> Size {
        // General
        let mut general_settings_size = self.minimum_pixel_size.requiring_size(app);
        general_settings_size.height += MARGIN + self.max_undos.requiring_size(app).height;

        // Frame
        let mut frame_settings_size = self.frame_preview.requiring_size(app);
        frame_settings_size.width += MARGIN + self.frame_width.requiring_size(app).width;
        frame_settings_size.width += MARGIN + self.frame_height.requiring_size(app).width;

        Size::from_wh(
            general_settings_size.width.max(frame_settings_size.width),
            general_settings_size.height + GROUP_MARGIN + frame_settings_size.height,
        ) + MARGIN * 2
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut region = self.region.without_margin(MARGIN);

        // General
        let mut minimum_pixel_size_region = region;
        minimum_pixel_size_region.size.height = self.minimum_pixel_size.requiring_size(app).height;
        self.minimum_pixel_size
            .set_region(app, minimum_pixel_size_region);
        region.consume_y(minimum_pixel_size_region.size.height + MARGIN);

        let mut max_undos_region = region;
        max_undos_region.size.height = self.max_undos.requiring_size(app).height;
        self.max_undos.set_region(app, max_undos_region);
        region.consume_y(max_undos_region.size.height + GROUP_MARGIN);

        // Frame
        let mut frame_width_region = region;
        frame_width_region.size.width = self.frame_width.requiring_size(app).width;
        self.frame_width.set_region(app, frame_width_region);

        let mut frame_height_region = region;
        frame_height_region.position.x = frame_width_region.end().x + MARGIN as i32;
        frame_height_region.size.width = self.frame_height.requiring_size(app).width;
        self.frame_height.set_region(app, frame_height_region);

        let mut frame_preview_region = region;
        frame_preview_region.position.x = frame_height_region.end().x + MARGIN as i32;
        frame_preview_region.size.width = self.frame_preview.requiring_size(app).width;
        self.frame_preview.set_region(app, frame_preview_region);
    }
}
