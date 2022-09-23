use super::{slider::SliderWidget, FixedSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct RgbSelectorWidget {
    region: Region,
    r: SliderWidget,
    g: SliderWidget,
    b: SliderWidget,
}

impl RgbSelectorWidget {
    pub fn new(app: &App) -> Self {
        let color = app.models().config.color.get();
        Self {
            region: Region::default(),
            r: SliderWidget::new(
                "R".parse().expect("unreachable"),
                0,
                u32::from(color.r),
                255,
                |slider, app, canvas| {
                    canvas.fill_rectangle(slider.bar_region(), color::PREVIEW_BACKGROUND)
                },
            ),
            g: SliderWidget::new(
                "G".parse().expect("unreachable"),
                0,
                u32::from(color.g),
                255,
                |slider, app, canvas| {
                    canvas.fill_rectangle(slider.bar_region(), color::BUTTONS_BACKGROUND)
                },
            ),
            b: SliderWidget::new(
                "B".parse().expect("unreachable"),
                0,
                u32::from(color.b),
                255,
                |slider, app, canvas| {
                    canvas.fill_rectangle(slider.bar_region(), color::BUTTONS_BACKGROUND)
                },
            ),
        }
    }
}

impl Widget for RgbSelectorWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.r.render_if_need(app, canvas);
        self.g.render_if_need(app, canvas);
        self.b.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.r.handle_event(app, event).or_fail()?;
        self.g.handle_event(app, event).or_fail()?;
        self.b.handle_event(app, event).or_fail()?;
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.r, &mut self.g, &mut self.b]
    }
}

impl FixedSizeWidget for RgbSelectorWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.r.requiring_size(app);
        size.height = size.height * 3 + MARGIN * 2;
        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut offset = self.region.position;
        self.r.set_position(app, offset);
        offset.y += (MARGIN + self.r.requiring_size(app).height) as i32;

        self.g.set_position(app, offset);
        offset.y += (MARGIN + self.g.requiring_size(app).height) as i32;

        self.b.set_position(app, offset);
        offset.y += (MARGIN + self.b.requiring_size(app).height) as i32;
    }
}
