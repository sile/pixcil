use super::{
    block::BlockWidget, button::ButtonWidget, FixedSizeWidget, VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    event::Event,
    region_ext::RegionExt,
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;
const GROUP_MARGIN: u32 = 16;

#[derive(Debug)]
pub struct MoveToolWidget {
    region: Region,
    go_center: BlockWidget<ButtonWidget>,
    go_top: BlockWidget<ButtonWidget>,
    go_bottom: BlockWidget<ButtonWidget>,
    go_left: BlockWidget<ButtonWidget>,
    go_right: BlockWidget<ButtonWidget>,
}

impl MoveToolWidget {
    pub fn new(_app: &App) -> Self {
        Self {
            region: Region::default(),

            go_center: BlockWidget::new(
                "FRAME CENTER".parse().expect("unreachable"),
                ButtonWidget::new(ButtonKind::Basic, IconId::GoCenter),
            ),
            go_top: BlockWidget::new(
                "PREV LAYER".parse().expect("unreachable"),
                ButtonWidget::new(ButtonKind::Basic, IconId::GoTop).with_disabled_callback(|app| {
                    app.models().config.camera.current_layer(app) == 0
                }),
            ),
            go_bottom: BlockWidget::new(
                "NEXT LAYER".parse().expect("unreachable"),
                ButtonWidget::new(ButtonKind::Basic, IconId::GoBottom).with_disabled_callback(
                    |app| {
                        app.models().config.camera.current_layer(app)
                            >= app.models().config.layer.enabled_count() as usize - 1
                    },
                ),
            ),
            go_left: BlockWidget::new(
                "PREV FRAME".parse().expect("unreachable"),
                ButtonWidget::new(ButtonKind::Basic, IconId::GoLeft).with_disabled_callback(
                    |app| app.models().config.camera.current_frame(app) == 0,
                ),
            ),
            go_right: BlockWidget::new(
                "NEXT FRAME".parse().expect("unreachable"),
                ButtonWidget::new(ButtonKind::Basic, IconId::GoRight).with_disabled_callback(
                    |app| {
                        app.models().config.camera.current_frame(app)
                            >= app.models().config.animation.enabled_frame_count() as usize - 1
                    },
                ),
            ),
        }
    }
}

impl Widget for MoveToolWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.go_center.render_if_need(app, canvas);
        self.go_top.render_if_need(app, canvas);
        self.go_bottom.render_if_need(app, canvas);
        self.go_left.render_if_need(app, canvas);
        self.go_right.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let mut do_move = false;
        let mut layer = app.models().config.camera.current_layer(app);
        let mut frame = app.models().config.camera.current_frame(app);
        self.go_center.handle_event(app, event).or_fail()?;
        if self.go_center.body_mut().take_clicked(app) {
            do_move = true;
        }

        self.go_top.handle_event(app, event).or_fail()?;
        if self.go_top.body_mut().take_clicked(app) {
            do_move = true;
            layer -= 1;
        }

        self.go_bottom.handle_event(app, event).or_fail()?;
        if self.go_bottom.body_mut().take_clicked(app) {
            do_move = true;
            layer += 1;
        }

        self.go_left.handle_event(app, event).or_fail()?;
        if self.go_left.body_mut().take_clicked(app) {
            do_move = true;
            frame -= 1;
        }

        self.go_right.handle_event(app, event).or_fail()?;
        if self.go_right.body_mut().take_clicked(app) {
            do_move = true;
            frame += 1;
        }

        if do_move {
            let screen_center = app.screen_size().to_region().center();
            let frame_center = app.models().config.camera.frame_center(&app, frame, layer);
            app.models_mut()
                .config
                .camera
                .r#move(frame_center - screen_center);
            app.request_redraw(app.screen_size().to_region());
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            &mut self.go_center,
            &mut self.go_top,
            &mut self.go_bottom,
            &mut self.go_left,
            &mut self.go_right,
        ]
    }
}

impl FixedSizeWidget for MoveToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let first_row = self.go_center.requiring_size(app);

        let mut second_row = self.go_top.requiring_size(app);
        second_row.width += MARGIN + self.go_bottom.requiring_size(app).width;

        let mut third_row = self.go_left.requiring_size(app);
        third_row.width += MARGIN + self.go_right.requiring_size(app).width;

        Size::from_wh(
            first_row.width.max(second_row.width).max(third_row.width),
            first_row.height + GROUP_MARGIN + second_row.height + GROUP_MARGIN + third_row.height,
        ) + MARGIN * 2
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut region = self.region.without_margin(MARGIN);

        // First row.
        let mut go_center_region = region;
        go_center_region.size.height = self.go_center.requiring_size(app).height;
        self.go_center.set_region(app, go_center_region);
        region.consume_y(go_center_region.size.height + GROUP_MARGIN);

        // Second row.
        let mut go_top_region = region;
        go_top_region.size = self.go_top.requiring_size(app);
        self.go_top.set_region(app, go_top_region);

        let mut go_bottom_region = region;
        go_bottom_region.position.x = go_top_region.end().x + MARGIN as i32;
        go_bottom_region.size = self.go_bottom.requiring_size(app);
        self.go_bottom.set_region(app, go_bottom_region);
        region.consume_y(go_bottom_region.size.height + GROUP_MARGIN);

        // Third row.
        let mut go_left_region = region;
        go_left_region.size = self.go_left.requiring_size(app);
        self.go_left.set_region(app, go_left_region);

        let mut go_right_region = region;
        go_right_region.position.x = go_left_region.end().x + MARGIN as i32;
        go_right_region.size = self.go_right.requiring_size(app);
        self.go_right.set_region(app, go_right_region);
        region.consume_y(go_right_region.size.height + GROUP_MARGIN);
    }
}
