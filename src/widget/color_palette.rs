use std::collections::HashSet;

use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    color::Hsv,
    event::Event,
};
use orfail::{OrFail, Result};
use pagurus::{
    image::{Canvas, Rgba},
    spatial::{Contains, Position, Region, Size},
};

#[derive(Debug, Default)]
pub struct ColorPaletteWidget {
    region: Region,
    colors: Vec<Rgba>,
    buttons: Vec<ButtonWidget>,
}

impl ColorPaletteWidget {
    pub fn new(app: &App, width: u32) -> Self {
        let colors = Self::get_colors(app);
        let buttons = colors
            .iter()
            .map(|_| ButtonWidget::new(ButtonKind::Middle, IconId::Null))
            .collect();
        Self {
            region: Region::new(Position::default(), Size::from_wh(width, 0)),
            colors,
            buttons,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    fn get_colors(app: &App) -> Vec<Rgba> {
        let models = app.models();
        let frame_count = models.config.animation.enabled_frame_count();
        let mut colors = (0..frame_count)
            .flat_map(|frame| {
                models
                    .config
                    .frame
                    .get_preview_region(&models.config, frame as usize)
                    .pixels()
                    .map(|position| {
                        models
                            .pixel_canvas
                            .get_pixel(&models.config, position)
                            .unwrap_or(Rgba::new(0, 0, 0, 0))
                    })
            })
            .filter(|c| c.a > 0)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        colors.sort_by_key(|rgba| {
            let hsv = Hsv::from_rgb(rgba.to_rgb());
            let mut k = 0;
            if hsv.s > 0.1 {
                k += ((hsv.h * 10.0).round() as u32 + 1) * 0xFF_FF;
                k += (hsv.s * 3.0).round() as u32 * 0xFF;
            }
            k += (hsv.v * 100.0).round() as u32;
            k
        });

        colors
    }
}

impl Widget for ColorPaletteWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let mut canvas = canvas.mask_region(self.region);
        for color in &self.buttons {
            color.render(app, &mut canvas);
        }
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if let Some(position) = event.position() {
            if !self.region.contains(&position) {
                return Ok(());
            }
        }
        for color in &mut self.buttons {
            color.handle_event(app, event).or_fail()?;
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        let mut children = Vec::new();
        children.extend(self.buttons.iter_mut().map(|c| c as &mut dyn Widget));
        children
    }
}

impl FixedSizeWidget for ColorPaletteWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.region.size;
        size.height = self
            .buttons
            .get(0)
            .map(|c| c.requiring_size(app).height)
            .unwrap_or_default();
        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut position = position;
        for color in &mut self.buttons {
            color.set_position(app, position);
            position.x += color.requiring_size(app).width as i32;
        }
    }
}
