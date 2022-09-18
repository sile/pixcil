use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    event::{Event, MouseAction},
};
use pagurus::{
    spatial::{Contains, Position, Region, Size},
    Result,
};
use pagurus_game_std::image::{Canvas, Sprite};

#[derive(Debug)]
pub struct ButtonWidget {
    region: Region,
    kind: ButtonKind,
    icon: IconId,
    state: ButtonState,
}

impl ButtonWidget {
    pub fn new(kind: ButtonKind, icon: IconId) -> Self {
        Self {
            region: Region::default(),
            kind,
            icon,
            state: ButtonState::default(),
        }
    }

    pub fn take_clicked(&mut self, app: &mut App) -> bool {
        if self.state == ButtonState::Clicked {
            app.request_redraw(self.region);
            self.state = ButtonState::Focused;
            true
        } else {
            false
        }
    }
}

impl Widget for ButtonWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let mut canvas = canvas.offset(self.region.position);

        let button = self.state.get_sprite(app, self.kind);
        canvas.draw_sprite(button);

        let mut canvas = canvas.offset(self.state.offset(self.kind));
        let icon = app.assets().get_icon(self.icon);
        canvas.draw_sprite(icon);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        match event {
            Event::Mouse {
                consumed: false,
                action,
                position,
            } => {
                let mut state = self.state;
                if self.region.contains(position) {
                    match action {
                        MouseAction::Move if self.state == ButtonState::Neutral => {
                            state = ButtonState::Focused;
                        }
                        MouseAction::Down => {
                            state = ButtonState::Pressed;
                        }
                        MouseAction::Up if self.state == ButtonState::Pressed => {
                            state = ButtonState::Clicked;
                        }
                        _ => {}
                    }
                    event.consume();
                } else {
                    state = ButtonState::Neutral;
                }
                if state != self.state {
                    self.state = state;
                    app.request_redraw(self.region);
                }
            }
            Event::Timeout(_) => {}
            Event::Mouse { .. } => {}
        }
        Ok(())
    }
}

impl FixedSizeWidget for ButtonWidget {
    fn requiring_size(&self, _app: &App) -> Size {
        self.kind.size()
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Neutral,
    Focused,
    Pressed,
    Clicked,
}

impl ButtonState {
    fn get_sprite(self, app: &App, kind: ButtonKind) -> &Sprite {
        let button = app.assets().get_button(kind);
        match self {
            ButtonState::Neutral => &button.neutral,
            ButtonState::Focused => &button.focused,
            ButtonState::Pressed => &button.pressed,
            ButtonState::Clicked => &button.pressed,
        }
    }

    fn offset(self, kind: ButtonKind) -> Position {
        let offset = Position::ORIGIN;
        match kind {
            ButtonKind::Basic => match self {
                ButtonState::Neutral => offset,
                ButtonState::Focused => offset.move_y(4),
                ButtonState::Pressed => offset.move_y(8),
                ButtonState::Clicked => offset.move_y(8),
            },
        }
    }
}
