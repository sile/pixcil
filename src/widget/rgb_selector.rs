use super::{FixedSizeWidget, Widget, slider::SliderWidget};
use crate::{app::App, canvas_ext::CanvasExt, event::Event};
use orfail::{OrFail, Result};
use pagurus::spatial::{Position, Region, Size};
use pagurus::{image::Canvas, image::Rgb, image::Rgba};

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
                    render_color_bar(app, canvas, slider.bar_region(), |color, i| color.r = i);
                },
            ),
            g: SliderWidget::new(
                "G".parse().expect("unreachable"),
                0,
                u32::from(color.g),
                255,
                |slider, app, canvas| {
                    render_color_bar(app, canvas, slider.bar_region(), |color, i| color.g = i);
                },
            ),
            b: SliderWidget::new(
                "B".parse().expect("unreachable"),
                0,
                u32::from(color.b),
                255,
                |slider, app, canvas| {
                    render_color_bar(app, canvas, slider.bar_region(), |color, i| color.b = i);
                },
            ),
        }
    }

    fn rgb(&self) -> Rgb {
        Rgb::new(
            self.r.value() as u8,
            self.g.value() as u8,
            self.b.value() as u8,
        )
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
        let old = self.rgb();

        self.r.handle_event(app, event).or_fail()?;
        self.g.handle_event(app, event).or_fail()?;
        self.b.handle_event(app, event).or_fail()?;

        let new = self.rgb();
        if old != new {
            let mut c = app.models().config.color.get();
            c.r = new.r;
            c.g = new.g;
            c.b = new.b;
            app.models_mut().config.color.set(c);
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        let rgb = app.models().config.color.get().to_rgb();
        if self.rgb() != rgb {
            self.r.set_value(app, u32::from(rgb.r));
            self.g.set_value(app, u32::from(rgb.g));
            self.b.set_value(app, u32::from(rgb.b));
        }
        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
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

pub fn render_color_bar<F>(app: &App, canvas: &mut Canvas, mut region: Region, f: F)
where
    F: Fn(&mut Rgba, u8),
{
    let mut color = app.models().config.color.get();
    let w = region.size.width as f64 / 255.0;
    let mut start_x = region.position.x as f64;
    let mut end_x = start_x + w;
    for i in 0..=255 {
        region.position.x = start_x.round() as i32;
        region.size.width = (end_x.round() - start_x.round()) as u32;
        f(&mut color, i);
        canvas.fill_rectangle(region, color.into());
        start_x = end_x;
        end_x += w;
    }
}
