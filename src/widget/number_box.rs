use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    canvas_ext::CanvasExt,
    color,
    event::{Event, InputId, MouseAction},
    region_ext::RegionExt,
};
use pagurus::{
    spatial::{Contains, Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct NumberBoxWidget {
    region: Region,
    value: u32,
    min: u32,
    max: u32,
    focused: bool,
    input: Option<InputId>,
}

impl NumberBoxWidget {
    pub fn new(min: u32, value: u32, max: u32) -> Self {
        Self {
            region: Region::default(),
            value,
            min,
            max,
            focused: false,
            input: None,
        }
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, app: &mut App, v: u32) {
        let v = self.min.clamp(v, self.max);
        if self.value != v {
            self.value = v;
            app.request_redraw(self.region);
        }
    }
}

impl Widget for NumberBoxWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        if self.focused {
            canvas.fill_rectangle(self.region.without_margin(4), color::TEXT_BOX_FOCUSED);
        } else {
            canvas.fill_rectangle(self.region.without_margin(4), color::TEXT_BOX_UNFOCUSED);
        }

        canvas
            .offset(self.region.position)
            .draw_sprite(&app.assets().number_box);

        let mut number_position = self.region.position;
        number_position.x = self.region.end().x - 20;
        number_position.y += 10;
        canvas.draw_number(number_position, self.value, &app.assets().digits_10x14);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        let prev_focused = self.focused;
        match event {
            Event::Mouse {
                action,
                position,
                consumed: false,
            } if self.region.contains(position) => match action {
                MouseAction::Down => {}
                MouseAction::Up => {
                    let input_id = app.enqueue_input_number_request();
                    self.input = Some(input_id);
                }
                MouseAction::Move => {
                    self.focused = true;
                }
            },
            Event::Mouse { .. } => {
                self.focused = false;
            }
            Event::Input { id, text } if self.input == Some(*id) => {
                if let Ok(value) = text.parse::<u32>() {
                    self.value = std::cmp::min(std::cmp::max(self.min, value), self.max);
                    app.request_redraw(self.region);
                } else {
                    log::debug!("not a number: {text:?}");
                }
            }
            _ => {}
        }

        event.consume_if_contained(self.region);
        if prev_focused != self.focused {
            app.request_redraw(self.region);
        }
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }
}

impl FixedSizeWidget for NumberBoxWidget {
    fn requiring_size(&self, app: &App) -> Size {
        app.assets().number_box.size()
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
