use super::{FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{Alphabet, Text},
    canvas_ext::CanvasExt,
    color,
    event::{Event, InputId, MouseAction},
    pixel::PixelSize,
    region_ext::RegionExt,
};
use orfail::Result;
use pagurus::{
    image::Canvas,
    spatial::{Contains, Position, Region, Size},
};

#[derive(Debug)]
pub struct SizeBoxWidget {
    region: Region,
    value: PixelSize,
    focused: bool,
    input: Option<InputId>,
}

impl SizeBoxWidget {
    pub fn new(value: PixelSize) -> Self {
        Self {
            region: Region::default(),
            value,
            focused: false,
            input: None,
        }
    }

    pub fn value(&self) -> PixelSize {
        self.value
    }

    pub fn set_value(&mut self, app: &mut App, v: PixelSize) {
        if self.value != v {
            self.value = v;
            app.request_redraw(self.region);
        }
    }
}

impl Widget for SizeBoxWidget {
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
            .draw_sprite(&app.assets().size_box);

        let mut number_position = self.region.position;
        number_position.x = self.region.end().x - 20;
        number_position.y += 10;

        if self.value.is_square() {
            canvas.draw_number(
                number_position,
                self.value.width as u32,
                &app.assets().digits_10x14,
            );
        } else {
            let mut offset = canvas.draw_number(
                number_position,
                self.value.height as u32,
                &app.assets().digits_10x14,
            );
            offset.x -= canvas
                .offset(offset)
                .draw_text(&Text::new(vec![Alphabet::X]), &app.assets().alphabet_10x14)
                .x;
            canvas.draw_number(offset, self.value.width as u32, &app.assets().digits_10x14);
        }
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
                    let input_id = app.enqueue_input_size_request();
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
                if let Ok(value) = text.parse::<PixelSize>() {
                    self.value = value;
                    app.request_redraw(self.region);
                } else {
                    log::debug!("not a pixel size: {text:?}");
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

impl FixedSizeWidget for SizeBoxWidget {
    fn requiring_size(&self, app: &App) -> Size {
        app.assets().size_box.size()
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
    }
}
