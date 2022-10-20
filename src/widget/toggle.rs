use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    event::{Event, MouseAction},
};
use pagurus::{
    spatial::{Contains, Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug, Default)]
pub struct ToggleWidget {
    region: Region,
    state: ToggleState,
}

impl ToggleWidget {
    pub fn new(on: bool) -> Self {
        Self {
            state: if on {
                ToggleState::OnNeutral
            } else {
                ToggleState::OffNeutral
            },
            ..Default::default()
        }
    }

    pub fn default_off() -> Self {
        Self::new(false)
    }

    pub fn is_on(&self) -> bool {
        matches!(self.state, ToggleState::OnNeutral | ToggleState::OnFocused)
    }
}

impl Widget for ToggleWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let sprite = match self.state {
            ToggleState::OnNeutral => &app.assets().toggle.on_neutral,
            ToggleState::OnFocused => &app.assets().toggle.on_focused,
            ToggleState::OffNeutral => &app.assets().toggle.off_neutral,
            ToggleState::OffFocused => &app.assets().toggle.off_focused,
        };
        canvas.offset(self.region.position).draw_sprite(sprite);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let prev_state = self.state;
        match event {
            Event::Mouse {
                action,
                position,
                consumed: false,
            } if self.region.contains(position) => {
                self.state.focus();
                if *action == MouseAction::Up {
                    self.state.toggle();
                }
                event.consume();
            }
            Event::Mouse { .. } => {
                self.state.unfocus();
            }
            _ => {}
        }
        if prev_state != self.state {
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl FixedSizeWidget for ToggleWidget {
    fn requiring_size(&self, app: &App) -> Size {
        app.assets().toggle.on_neutral.size()
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum ToggleState {
    #[default]
    OnNeutral,
    OnFocused,
    OffNeutral,
    OffFocused,
}

impl ToggleState {
    fn toggle(&mut self) {
        match self {
            ToggleState::OnNeutral => *self = Self::OffNeutral,
            ToggleState::OnFocused => *self = Self::OffFocused,
            ToggleState::OffNeutral => *self = Self::OnNeutral,
            ToggleState::OffFocused => *self = Self::OnFocused,
        }
    }

    fn focus(&mut self) {
        match self {
            ToggleState::OnNeutral => {
                *self = ToggleState::OnFocused;
            }
            ToggleState::OffNeutral => {
                *self = ToggleState::OffFocused;
            }
            _ => {}
        }
    }

    fn unfocus(&mut self) {
        match self {
            ToggleState::OnFocused => {
                *self = ToggleState::OnNeutral;
            }
            ToggleState::OffFocused => {
                *self = ToggleState::OffNeutral;
            }
            _ => {}
        }
    }
}
