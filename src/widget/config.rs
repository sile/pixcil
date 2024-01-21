use super::{
    block::BlockWidget, number_box::NumberBoxWidget, pixel_size::PixelSizeWidget,
    size_box::SizeBoxWidget, toggle::ToggleWidget, FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{app::App, event::Event, model::config::Animation, region_ext::RegionExt};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Position, Region, Size};

const MARGIN_X: u32 = 8;
const MARGIN_Y: u32 = 12;

#[derive(Debug)]
pub struct ConfigWidget {
    region: Region,

    // Size settings
    frame_size: BlockWidget<SizeBoxWidget>,
    pixel_size: BlockWidget<PixelSizeWidget>,

    // Preview settings
    frame_preview: BlockWidget<ToggleWidget>,
    frame_preview_scale: BlockWidget<NumberBoxWidget>,
    silhouette: BlockWidget<ToggleWidget>,

    // Layer settings
    layer_enable: BlockWidget<ToggleWidget>,

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
        let silhouette_preview = app.models().config.silhouette_preview;
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

            // Preview
            frame_preview: BlockWidget::new(
                "PREVIEW".parse().expect("unreachable"),
                ToggleWidget::new(frame_preview),
            ),
            frame_preview_scale: BlockWidget::new(
                "PREVIEW SCALE".parse().expect("unreachable"),
                NumberBoxWidget::new(1, frame_preview_scale as u32, 32),
            ),
            silhouette: BlockWidget::new(
                "SILHOUETTE".parse().expect("unreachable"),
                ToggleWidget::new(silhouette_preview),
            ),

            // Layer
            layer_enable: BlockWidget::new(
                "LAYER".parse().expect("unreachable"),
                ToggleWidget::new(layer.is_enabled()),
            ),

            // Animation
            animation_enable: BlockWidget::new(
                "ANIMATION".parse().expect("unreachable"),
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

        // Preview
        self.frame_preview.render_if_need(app, canvas);
        self.frame_preview_scale.render_if_need(app, canvas);
        self.silhouette.render_if_need(app, canvas);

        // Layer
        self.layer_enable.render_if_need(app, canvas);

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

        // Preview
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
        self.silhouette.handle_event(app, event).or_fail()?;
        app.models_mut().config.silhouette_preview = self.silhouette.body().is_on();

        // Layer
        let layer = app.models().config.layer;
        self.layer_enable.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .layer
            .set_enabled(self.layer_enable.body().is_on());
        if layer != app.models().config.layer {
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
            // Preview
            &mut self.frame_preview,
            &mut self.frame_preview_scale,
            &mut self.silhouette,
            // Layer
            &mut self.layer_enable,
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
        size_settings_size.width += MARGIN_X + self.pixel_size.requiring_size(app).width;

        // Preview
        let mut preview_settings_size = self.frame_preview.requiring_size(app);
        preview_settings_size.width +=
            MARGIN_X + self.frame_preview_scale.requiring_size(app).width;
        preview_settings_size.width += MARGIN_X + self.silhouette.requiring_size(app).width;

        // Layer
        let layer_settings_size = self.layer_enable.requiring_size(app);

        // Animation
        let mut animation_settings_size = self.animation_enable.requiring_size(app);
        animation_settings_size.width += MARGIN_X + self.frame_count.requiring_size(app).width;
        animation_settings_size.width += MARGIN_X + self.fps.requiring_size(app).width;

        Size::from_wh(
            size_settings_size
                .width
                .max(preview_settings_size.width)
                .max(layer_settings_size.width)
                .max(animation_settings_size.width),
            size_settings_size.height
                + MARGIN_Y
                + preview_settings_size.height
                + MARGIN_Y
                + layer_settings_size.height
                + MARGIN_Y
                + animation_settings_size.height,
        ) + MARGIN_X * 2
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut region = self.region.without_margin(MARGIN_X);

        // Size
        let mut frame_size_region = region;
        frame_size_region.size = self.frame_size.requiring_size(app);
        self.frame_size.set_region(app, frame_size_region);

        let mut pixel_size_region = region;
        pixel_size_region.position.x = frame_size_region.end().x + MARGIN_X as i32;
        pixel_size_region.size = self.pixel_size.requiring_size(app);
        self.pixel_size.set_region(app, pixel_size_region);
        region.consume_y(pixel_size_region.size.height + MARGIN_Y);

        // Preview
        let mut frame_preview_region = region;
        frame_preview_region.size = self.frame_preview.requiring_size(app);
        self.frame_preview.set_region(app, frame_preview_region);

        let mut preview_scale_region = region;
        preview_scale_region.position.x = frame_preview_region.end().x + MARGIN_X as i32;
        preview_scale_region.size = self.frame_preview_scale.requiring_size(app);
        self.frame_preview_scale
            .set_region(app, preview_scale_region);

        let mut silhouette_region = region;
        silhouette_region.position.x = preview_scale_region.end().x + MARGIN_X as i32;
        silhouette_region.size = self.silhouette.requiring_size(app);
        self.silhouette.set_region(app, silhouette_region);

        region.consume_y(frame_preview_region.size.height + MARGIN_Y);

        // Layer
        let mut layer_enable_region = region;
        layer_enable_region.size = self.layer_enable.requiring_size(app);
        self.layer_enable.set_region(app, layer_enable_region);
        region.consume_y(layer_enable_region.size.height + MARGIN_Y);

        // Animation
        let mut animation_enable_region = region;
        animation_enable_region.size = self.animation_enable.requiring_size(app);
        self.animation_enable
            .set_region(app, animation_enable_region);

        let mut frame_count_region = region;
        frame_count_region.position.x = animation_enable_region.end().x + MARGIN_X as i32;
        frame_count_region.size = self.frame_count.requiring_size(app);
        self.frame_count.set_region(app, frame_count_region);

        let mut fps_region = region;
        fps_region.position.x = frame_count_region.end().x + MARGIN_X as i32;
        fps_region.size = self.fps.requiring_size(app);
        self.fps.set_region(app, fps_region);
    }
}
