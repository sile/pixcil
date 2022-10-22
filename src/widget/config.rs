use super::{
    block::BlockWidget, number_box::NumberBoxWidget, pixel_size::PixelSizeWidget,
    toggle::ToggleWidget, FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    event::Event,
    model::config::{Animation, Layer},
    pixel::PixelSize,
    region_ext::RegionExt,
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;
const GROUP_MARGIN: u32 = 24;

#[derive(Debug)]
pub struct ConfigWidget {
    region: Region,

    // General settings
    minimum_pixel_size: BlockWidget<PixelSizeWidget>,
    max_undos: BlockWidget<NumberBoxWidget>,

    // Frame settings
    frame_width: BlockWidget<NumberBoxWidget>,
    frame_height: BlockWidget<NumberBoxWidget>,
    frame_preview: BlockWidget<ToggleWidget>,

    // Layer settings
    layer_enable: BlockWidget<ToggleWidget>,
    layer_count: BlockWidget<NumberBoxWidget>,

    // Animation
    animation_enable: BlockWidget<ToggleWidget>,
    frame_count: BlockWidget<NumberBoxWidget>,
    fps: BlockWidget<NumberBoxWidget>,
}

impl ConfigWidget {
    pub fn new(app: &App) -> Self {
        let max_undos = app.models().config.max_undos.get();
        let frame_size = app.models().config.frame.get_base_region().size();
        let layer = app.models().config.layer;
        let animation = app.models().config.animation;
        Self {
            region: Region::default(),

            // General
            minimum_pixel_size: BlockWidget::new(
                "PIXEL SIZE".parse().expect("unreachable"),
                PixelSizeWidget::new(app),
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
                ToggleWidget::default(), // TOOD: use saved value
            ),

            // Layer
            layer_enable: BlockWidget::new(
                "ENABLE LAYER".parse().expect("unreachable"),
                ToggleWidget::new(layer.is_enabled()),
            ),
            layer_count: BlockWidget::new(
                "LAYER COUNT".parse().expect("unreachable"),
                NumberBoxWidget::new(Layer::MIN as u32, layer.count() as u32, Layer::MAX as u32),
            ),

            // Animation
            animation_enable: BlockWidget::new(
                "ENABLE ANIMATION".parse().expect("unreachable"),
                ToggleWidget::new(animation.is_enabled()),
            ),
            frame_count: BlockWidget::new(
                "FRAME COUNT".parse().expect("unreachable"),
                NumberBoxWidget::new(
                    Animation::MIN_FRAME_COUNT as u32,
                    animation.frame_count() as u32,
                    Animation::MAX_FRAME_COUNT as u32,
                ),
            ),
            fps: BlockWidget::new(
                "FPS".parse().expect("unreachable"),
                NumberBoxWidget::new(
                    Animation::MIN_FPS as u32,
                    animation.fps() as u32,
                    Animation::MAX_FPS as u32,
                ),
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

        // Layer
        self.layer_enable.render_if_need(app, canvas);
        self.layer_count.render_if_need(app, canvas);

        // Animation
        self.animation_enable.render_if_need(app, canvas);
        self.frame_count.render_if_need(app, canvas);
        self.fps.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // General
        self.minimum_pixel_size.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .minimum_pixel_size
            .set(PixelSize::square(self.minimum_pixel_size.body().value()));

        self.max_undos.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .max_undos
            .set(self.max_undos.body().value());

        // Frame
        let frame = app.models().config.frame;
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
        if frame != app.models().config.frame {
            app.request_redraw(app.screen_size().to_region());
        }

        // Layer
        let layer = app.models_mut().config.layer;
        self.layer_enable.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .layer
            .set_enabled(self.layer_enable.body().is_on());

        self.layer_count.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .layer
            .set_count(self.layer_count.body().value() as u16);
        if layer != app.models_mut().config.layer {
            app.request_redraw(app.screen_size().to_region());
        }

        // Animation
        let animation = app.models_mut().config.animation;
        self.animation_enable.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .animation
            .set_enabled(self.animation_enable.body().is_on());

        self.frame_count.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .animation
            .set_frame_count(self.frame_count.body().value() as u16);

        self.fps.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .animation
            .set_fps(self.fps.body().value() as u8);
        if animation != app.models_mut().config.animation {
            app.request_redraw(app.screen_size().to_region());
        }

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
            // Layer
            &mut self.layer_enable,
            &mut self.layer_count,
            // Animation
            &mut self.animation_enable,
            &mut self.frame_count,
            &mut self.fps,
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

        // Layer
        let mut layer_settings_size = self.layer_enable.requiring_size(app);
        layer_settings_size.width += MARGIN + self.layer_count.requiring_size(app).width;

        // Animation
        let mut animation_settings_size = self.animation_enable.requiring_size(app);
        animation_settings_size.width += MARGIN + self.frame_count.requiring_size(app).width;
        animation_settings_size.width += MARGIN + self.fps.requiring_size(app).width;

        Size::from_wh(
            general_settings_size
                .width
                .max(frame_settings_size.width)
                .max(layer_settings_size.width)
                .max(animation_settings_size.width),
            general_settings_size.height
                + GROUP_MARGIN
                + frame_settings_size.height
                + GROUP_MARGIN
                + layer_settings_size.height
                + GROUP_MARGIN
                + animation_settings_size.height,
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
        frame_width_region.size = self.frame_width.requiring_size(app);
        self.frame_width.set_region(app, frame_width_region);

        let mut frame_height_region = region;
        frame_height_region.position.x = frame_width_region.end().x + MARGIN as i32;
        frame_height_region.size = self.frame_height.requiring_size(app);
        self.frame_height.set_region(app, frame_height_region);

        let mut frame_preview_region = region;
        frame_preview_region.position.x = frame_height_region.end().x + MARGIN as i32;
        frame_preview_region.size = self.frame_preview.requiring_size(app);
        self.frame_preview.set_region(app, frame_preview_region);
        region.consume_y(frame_preview_region.size.height + GROUP_MARGIN);

        // Layer
        let mut layer_enable_region = region;
        layer_enable_region.size = self.layer_enable.requiring_size(app);
        self.layer_enable.set_region(app, layer_enable_region);

        let mut layer_count_region = region;
        layer_count_region.position.x = layer_enable_region.end().x + MARGIN as i32;
        layer_count_region.size = self.layer_count.requiring_size(app);
        self.layer_count.set_region(app, layer_count_region);
        region.consume_y(layer_count_region.size.height + GROUP_MARGIN);

        // Animation
        let mut animation_enable_region = region;
        animation_enable_region.size = self.animation_enable.requiring_size(app);
        self.animation_enable
            .set_region(app, animation_enable_region);

        let mut frame_count_region = region;
        frame_count_region.position.x = animation_enable_region.end().x + MARGIN as i32;
        frame_count_region.size = self.frame_count.requiring_size(app);
        self.frame_count.set_region(app, frame_count_region);

        let mut fps_region = region;
        fps_region.position.x = frame_count_region.end().x + MARGIN as i32;
        fps_region.size = self.fps.requiring_size(app);
        self.fps.set_region(app, fps_region);
    }
}
