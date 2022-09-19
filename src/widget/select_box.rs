use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{app::App, canvas_ext::CanvasExt, color, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct SelectBoxWidget {
    region: Region,
    buttons: Vec<ButtonWidget>,
    selected: usize,
}

impl SelectBoxWidget {
    pub fn new(app: &mut App, mut buttons: Vec<ButtonWidget>, selected: usize) -> Result<Self> {
        (selected < buttons.len()).or_fail()?;
        buttons[selected].set_clicked(app);
        Ok(Self {
            region: Region::default(),
            buttons,
            selected,
        })
    }

    pub fn selected(&self) -> usize {
        self.selected
    }
}

impl Widget for SelectBoxWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        for button in &self.buttons {
            button.render_if_need(app, canvas);
        }
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let mut new_selected = None;
        for (i, button) in self.buttons.iter_mut().enumerate() {
            if i == self.selected {
                continue;
            }

            button.handle_event(app, event).or_fail()?;
            if button.is_clicked() {
                new_selected = Some(i);
            }
        }
        if let Some(i) = new_selected {
            self.buttons[self.selected]
                .handle_event(app, event)
                .or_fail()?;
            self.selected = i;
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        self.buttons
            .iter_mut()
            .map(|x| x as &mut dyn Widget)
            .collect()
    }
}

impl FixedSizeWidget for SelectBoxWidget {
    fn requiring_size(&self, app: &App) -> Size {
        Size::from_wh(
            self.buttons
                .iter()
                .map(|x| x.requiring_size(app).width)
                .sum(),
            self.buttons[0].requiring_size(app).height,
        )
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
