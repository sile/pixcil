use super::{button::ButtonWidget, FixedSizeWidget, Widget};
use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemState {
    Selected,
    Deselected,
}

impl ItemState {
    pub fn is_selected(self) -> bool {
        self == Self::Selected
    }
}

#[derive(Debug)]
pub struct SelectBoxWidget {
    region: Region,
    buttons: Vec<ButtonWidget>,
    selected: usize,
    prev_selected: Option<usize>,
}

impl SelectBoxWidget {
    pub fn new(buttons: Vec<ButtonWidget>, selected: usize) -> Result<Self> {
        (selected < buttons.len()).or_fail()?;
        Ok(Self {
            region: Region::default(),
            buttons,
            selected,
            prev_selected: None,
        })
    }

    pub fn select(&mut self, app: &mut App, i: usize) -> Result<()> {
        if self.selected != i {
            self.prev_selected = Some(self.selected);
            self.selected = i;
            app.request_redraw(self.region);
        }
        Ok(())
    }

    pub fn buttons(&self) -> &[ButtonWidget] {
        &self.buttons
    }

    pub fn buttons_mut(&mut self) -> &mut [ButtonWidget] {
        &mut self.buttons
    }

    pub fn on_selected<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(ItemState, &mut ButtonWidget) -> Result<()>,
    {
        if let Some(prev) = self.prev_selected {
            f(ItemState::Deselected, &mut self.buttons[prev]).or_fail()?;
            f(ItemState::Selected, &mut self.buttons[self.selected]).or_fail()?;
        }
        Ok(())
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
        self.prev_selected = None;

        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.handle_event(app, event).or_fail()?;
            if button.take_clicked(app) {
                self.prev_selected = Some(self.selected);
                self.selected = i;
            }
        }

        event.consume_if_contained(self.region);
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
                .map(|x| x.requiring_size(app).width + MARGIN)
                .sum::<u32>()
                - MARGIN,
            self.buttons[0].requiring_size(app).height,
        )
    }

    fn set_position(&mut self, app: &App, mut position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        for button in &mut self.buttons {
            button.set_position(app, position);
            position.x += (button.region().size.width + MARGIN) as i32;
        }
    }
}
