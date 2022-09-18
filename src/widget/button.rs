use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    event::{Event, MouseAction},
};
use pagurus::{
    spatial::{Contains, Position, Region, Size},
    Result,
};
use pagurus_game_std::image::{Canvas, Sprite};

const DISABLED_ALPHA: u8 = 100;

pub struct ButtonWidget {
    region: Region,
    kind: ButtonKind,
    icon: IconId,
    state: ButtonState,
    disabled: Option<fn(&App) -> bool>,
    number: Option<fn(&App) -> u32>,
    number_margin: u32,
    prev_state: ButtonState,
    prev_disabled: bool,
    prev_number: u32,
}

impl ButtonWidget {
    pub fn new(kind: ButtonKind, icon: IconId) -> Self {
        Self {
            region: Region::default(),
            kind,
            icon,
            state: ButtonState::default(),
            disabled: None,
            number: None,
            number_margin: 0,
            prev_state: ButtonState::default(),
            prev_disabled: false,
            prev_number: 0,
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

    pub fn set_disabled_callback(&mut self, f: fn(&App) -> bool) {
        self.disabled = Some(f);
    }

    pub fn set_number_callback(&mut self, margin: u32, f: fn(&App) -> u32) {
        self.number = Some(f);
        self.number_margin = margin;
    }

    pub fn is_disabled(&self, app: &App) -> bool {
        self.disabled.map_or(false, |f| f(app))
    }

    pub fn number(&self, app: &App) -> u32 {
        self.number.map_or(0, |f| f(app))
    }

    fn render_number(&self, app: &App, canvas: &mut Canvas) {
        let disabled = self.is_disabled(app);
        let mut number = self.number(app);
        let mut offset = Position::from_xy(
            self.region.size.width as i32 - 18 - self.number_margin as i32,
            self.region.size.height as i32 - 28,
        );
        let margin = 2;
        loop {
            let digit = (number % 10) as usize;
            let sprite = &app.assets().digits_10x14[digit];
            if disabled {
                canvas
                    .offset(offset)
                    .draw_sprite_with_alpha(sprite, DISABLED_ALPHA);
            } else {
                canvas.offset(offset).draw_sprite(sprite);
            }
            offset.x -= sprite.size().width as i32 + margin;
            number /= 10;
            if number == 0 {
                break;
            }
        }
    }
}

impl std::fmt::Debug for ButtonWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ButtonWidget {{ .. }}")
    }
}

impl Widget for ButtonWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let mut canvas = canvas.offset(self.region.position);
        let disabled = self.is_disabled(app);

        let button = self.state.get_sprite(app, self.kind);
        if disabled {
            canvas.draw_sprite_with_alpha(button, DISABLED_ALPHA);
        } else {
            canvas.draw_sprite(button);
        }

        let mut canvas = canvas.offset(self.state.offset(self.kind));
        let icon = app.assets().get_icon(self.icon);
        if disabled {
            canvas.draw_sprite_with_alpha(icon, DISABLED_ALPHA);
        } else {
            canvas.draw_sprite(icon);
        }

        if self.number.is_some() {
            self.render_number(app, &mut canvas);
        }
    }

    fn handle_event_before(&mut self, app: &mut App) -> Result<()> {
        self.prev_disabled = self.is_disabled(app);
        self.prev_state = self.state;
        self.prev_number = self.number(app);
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        let disabled = self.is_disabled(app);
        if disabled {
            self.state = ButtonState::Neutral;
        }
        let number = self.number(app);
        if self.prev_disabled != disabled
            || self.prev_state != self.state
            || self.prev_number != number
        {
            self.prev_disabled = disabled;
            self.prev_state = self.state;
            self.prev_number = number;
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        match event {
            Event::Mouse {
                consumed: false,
                action,
                position,
            } => {
                let disabled = self.is_disabled(app);
                if !disabled && self.region.contains(position) {
                    match action {
                        MouseAction::Move if self.state == ButtonState::Neutral => {
                            self.state = ButtonState::Focused;
                        }
                        MouseAction::Down => {
                            self.state = ButtonState::Pressed;
                        }
                        MouseAction::Up if self.state == ButtonState::Pressed => {
                            self.state = ButtonState::Clicked;
                        }
                        _ => {}
                    }
                    event.consume();
                } else {
                    self.state = ButtonState::Neutral;
                }
            }
            Event::Timeout(_) => {}
            Event::Mouse { .. } => {}
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        Vec::new()
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
