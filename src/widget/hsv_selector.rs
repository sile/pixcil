use super::{slider::SliderWidget, FixedSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color::Hsv, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct HsvSelectorWidget {
    region: Region,
    h: SliderWidget,
    s: SliderWidget,
    v: SliderWidget,
}

impl HsvSelectorWidget {
    pub fn new(app: &App) -> Self {
        let color = app.models().config.color.get();
        let hsv = Hsv::from_rgb(color.to_rgb());
        Self {
            region: Region::default(),
            h: SliderWidget::new(
                "H".parse().expect("unreachable"),
                0,
                (hsv.h * 360.0).round() as u32,
                360,
                |slider, app, canvas| {
                    render_color_bar(app, canvas, slider.bar_region(), 360, |color, i| {
                        color.h = i
                    });
                },
            ),
            s: SliderWidget::new(
                "S".parse().expect("unreachable"),
                0,
                (hsv.h * 100.0).round() as u32,
                100,
                |slider, app, canvas| {
                    render_color_bar(app, canvas, slider.bar_region(), 100, |color, i| {
                        color.s = i
                    });
                },
            ),
            v: SliderWidget::new(
                "V".parse().expect("unreachable"),
                0,
                (hsv.h * 100.0).round() as u32,
                100,
                |slider, app, canvas| {
                    render_color_bar(app, canvas, slider.bar_region(), 100, |color, i| {
                        color.v = i
                    });
                },
            ),
        }
    }

    fn hsv(&self) -> Hsv {
        Hsv {
            h: self.h.value() as f64 / 360.0,
            s: self.s.value() as f64 / 100.0,
            v: self.v.value() as f64 / 100.0,
        }
    }
}

impl Widget for HsvSelectorWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.h.render_if_need(app, canvas);
        self.s.render_if_need(app, canvas);
        self.v.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let old = self.hsv();

        self.h.handle_event(app, event).or_fail()?;
        self.s.handle_event(app, event).or_fail()?;
        self.v.handle_event(app, event).or_fail()?;

        let new = self.hsv();
        if old != new {
            let mut c = app.models().config.color.get();
            let rgb = new.to_rgb();
            c.r = rgb.r;
            c.g = rgb.g;
            c.b = rgb.b;
            app.models_mut().config.color.set(c);
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        let hsv = Hsv::from_rgb(app.models().config.color.get().to_rgb());
        if self.hsv().to_rgb() != hsv.to_rgb() {
            self.h.set_value(app, (hsv.h * 360.0).round() as u32);
            self.s.set_value(app, (hsv.s * 100.0).round() as u32);
            self.v.set_value(app, (hsv.v * 100.0).round() as u32);
        }
        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.h, &mut self.s, &mut self.v]
    }
}

impl FixedSizeWidget for HsvSelectorWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.h.requiring_size(app);
        size.height = size.height * 3 + MARGIN * 2;
        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut offset = self.region.position;
        self.h.set_position(app, offset);
        offset.y += (MARGIN + self.h.requiring_size(app).height) as i32;

        self.s.set_position(app, offset);
        offset.y += (MARGIN + self.s.requiring_size(app).height) as i32;

        self.v.set_position(app, offset);
        offset.y += (MARGIN + self.v.requiring_size(app).height) as i32;
    }
}

fn render_color_bar<F>(app: &App, canvas: &mut Canvas, mut region: Region, max: usize, f: F)
where
    F: Fn(&mut Hsv, f64),
{
    let mut color = Hsv::from_rgb(app.models().config.color.get().to_rgb());
    let alpha = app.models().config.color.get().a;
    let w = region.size.width as f64 / max as f64;
    let mut start_x = region.position.x as f64;
    let mut end_x = start_x + w;
    for i in 0..=max {
        region.position.x = start_x.round() as i32;
        region.size.width = (end_x.round() - start_x.round()) as u32;
        f(&mut color, i as f64 / max as f64);
        canvas.fill_rectangle(region, color.to_rgb().alpha(alpha).into());
        start_x = end_x;
        end_x += w;
    }
}
