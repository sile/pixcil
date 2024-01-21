use super::{
    block::BlockWidget, number_box::NumberBoxWidget, pixel_size::PixelSizeWidget,
    size_box::SizeBoxWidget, toggle::ToggleWidget, FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    event::Event,
    model::config::{Animation, Layer},
    region_ext::RegionExt,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Position, Region, Size};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct ConfigWidget {
    region: Region,

    // Size settings
    frame_size: BlockWidget<SizeBoxWidget>,
    pixel_size: BlockWidget<PixelSizeWidget>,

    // Frame settings
    frame_preview: BlockWidget<ToggleWidget>,
    frame_preview_scale: BlockWidget<NumberBoxWidget>,

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
        let frame_size = app.models().config.frame.get_base_region().size();
        let frame_preview = app.models().config.frame_preview.get();
        let frame_preview_scale = app.models().config.frame_preview_scale.get();
        let layer = app.models().config.layer;
        let animation = app.models().config.animation;
        Self {
            region: Region::default(),

            // Size
            frame_size: BlockWidget::new(
                "FRAME SIZE".parse().expect("unreachable"),
                SizeBoxWidget::new(frame_size),
            ),
            pixel_size: BlockWidget::new(
                "PIXEL SIZE".parse().expect("unreachable"),
                PixelSizeWidget::new(app),
            ),

            // Frame
            frame_preview: BlockWidget::new(
                "PREVIEW".parse().expect("unreachable"),
                ToggleWidget::new(frame_preview),
            ),
            frame_preview_scale: BlockWidget::new(
                "PREVIEW SCALE".parse().expect("unreachable"),
                NumberBoxWidget::new(1, frame_preview_scale as u32, 32),
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
        // Size
        self.frame_size.render_if_need(app, canvas);
        self.pixel_size.render_if_need(app, canvas);

        // Frame
        self.frame_preview.render_if_need(app, canvas);
        self.frame_preview_scale.render_if_need(app, canvas);

        // Layer
        self.layer_enable.render_if_need(app, canvas);
        self.layer_count.render_if_need(app, canvas);

        // Animation
        self.animation_enable.render_if_need(app, canvas);
        self.frame_count.render_if_need(app, canvas);
        self.fps.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        // Size
        let frame = app.models().config.frame;
        self.frame_size.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame
            .set_from_pixel_size(self.frame_size.body().value());
        if frame != app.models().config.frame {
            app.request_redraw(app.screen_size().to_region());
        }

        self.pixel_size.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .minimum_pixel_size
            .set(self.pixel_size.body().value());

        // Frame Preview
        self.frame_preview.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame_preview
            .set(self.frame_preview.body().is_on());
        self.frame_preview_scale
            .handle_event(app, event)
            .or_fail()?;
        app.models_mut()
            .config
            .frame_preview_scale
            .set(self.frame_preview_scale.body().value() as u8)
            .or_fail()?;

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
            // Size
            &mut self.frame_size,
            &mut self.pixel_size,
            // Frame
            &mut self.frame_preview,
            &mut self.frame_preview_scale,
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
        // Size
        let mut size_settings_size = self.frame_size.requiring_size(app);
        size_settings_size.width += MARGIN + self.pixel_size.requiring_size(app).width;

        // Frame Preview
        let mut frame_preview_settings_size = self.frame_preview.requiring_size(app);
        frame_preview_settings_size.width +=
            MARGIN + self.frame_preview_scale.requiring_size(app).width;

        // Layer
        let mut layer_settings_size = self.layer_enable.requiring_size(app);
        layer_settings_size.width += MARGIN + self.layer_count.requiring_size(app).width;

        // Animation
        let mut animation_settings_size = self.animation_enable.requiring_size(app);
        animation_settings_size.width += MARGIN + self.frame_count.requiring_size(app).width;
        animation_settings_size.width += MARGIN + self.fps.requiring_size(app).width;

        Size::from_wh(
            size_settings_size
                .width
                .max(frame_preview_settings_size.width)
                .max(layer_settings_size.width)
                .max(animation_settings_size.width),
            size_settings_size.height
                + MARGIN
                + frame_preview_settings_size.height
                + MARGIN
                + layer_settings_size.height
                + MARGIN
                + animation_settings_size.height,
        ) + MARGIN * 2
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut region = self.region.without_margin(MARGIN);

        // Size
        let mut frame_size_region = region;
        frame_size_region.size = self.frame_size.requiring_size(app);
        self.frame_size.set_region(app, frame_size_region);

        let mut pixel_size_region = region;
        pixel_size_region.position.x = frame_size_region.end().x + MARGIN as i32;
        pixel_size_region.size = self.pixel_size.requiring_size(app);
        self.pixel_size.set_region(app, pixel_size_region);
        region.consume_y(pixel_size_region.size.height + MARGIN);

        // Frame Preview
        let mut frame_preview_region = region;
        frame_preview_region.size = self.frame_preview.requiring_size(app);
        self.frame_preview.set_region(app, frame_preview_region);

        let mut preview_scale_region = region;
        preview_scale_region.position.x = frame_preview_region.end().x + MARGIN as i32;
        preview_scale_region.size = self.frame_preview_scale.requiring_size(app);
        self.frame_preview_scale
            .set_region(app, preview_scale_region);
        region.consume_y(frame_preview_region.size.height + MARGIN);

        // Layer
        let mut layer_enable_region = region;
        layer_enable_region.size = self.layer_enable.requiring_size(app);
        self.layer_enable.set_region(app, layer_enable_region);

        let mut layer_count_region = region;
        layer_count_region.position.x = layer_enable_region.end().x + MARGIN as i32;
        layer_count_region.size = self.layer_count.requiring_size(app);
        self.layer_count.set_region(app, layer_count_region);
        region.consume_y(layer_count_region.size.height + MARGIN);

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
