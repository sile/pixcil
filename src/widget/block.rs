use super::{FixedSizeWidget, VariableSizeWidget, Widget};
use crate::{
    app::App, asset::Text, canvas_ext::CanvasExt, color, event::Event, region_ext::RegionExt,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Region, Size};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct BlockWidget<W> {
    region: Region,
    label: Text,
    body: W,
}

impl<W: FixedSizeWidget> BlockWidget<W> {
    pub fn new(label: Text, body: W) -> Self {
        Self {
            region: Region::default(),
            label,
            body,
        }
    }

    pub fn body(&self) -> &W {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut W {
        &mut self.body
    }

    pub fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.body.requiring_size(app);
        size.width += MARGIN * 2;
        size.width = std::cmp::max(size.width, self.label.size().width + 8);
        size.height += self.label.size().height + MARGIN * 2;
        size
    }
}

impl<W: FixedSizeWidget> Widget for BlockWidget<W> {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.draw_rectangle(self.region.without_margin(2), color::WINDOW_BORDER);

        let mut label_region = self.region.without_margin(2);
        label_region.size = self.label.size();
        canvas.draw_rectangle(label_region, color::WINDOW_BACKGROUND);

        canvas
            .offset(self.region.position)
            .draw_text(&self.label, &app.assets().alphabet_10x14);

        self.body.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.body.handle_event(app, event).or_fail()?;
        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.body]
    }
}

impl<W: FixedSizeWidget> VariableSizeWidget for BlockWidget<W> {
    fn set_region(&mut self, app: &App, region: Region) {
        self.region = region;

        let mut body_position = self.region.position;
        body_position.x += MARGIN as i32;
        body_position.y += (self.label.size().height + MARGIN) as i32;
        self.body.set_position(app, body_position);
    }
}
