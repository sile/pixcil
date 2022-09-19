use super::{button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
};
use pagurus::{
    failure::{Failure, OrFail},
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct ToolBoxWidget {
    region: Region,
    tools: SelectBoxWidget,
    current: Tool,
}

impl ToolBoxWidget {
    pub fn current_tool(&self) -> Tool {
        self.current
    }
}

impl Default for ToolBoxWidget {
    fn default() -> Self {
        let mut buttons = vec![
            ButtonWidget::new(ButtonKind::Basic, IconId::Draw),
            ButtonWidget::new(ButtonKind::Basic, IconId::Erase),
            ButtonWidget::new(ButtonKind::Basic, IconId::Select),
            ButtonWidget::new(ButtonKind::Basic, IconId::Pick),
            ButtonWidget::new(ButtonKind::Basic, IconId::Move),
        ];
        buttons[0].set_kind(ButtonKind::BasicDeep);

        Self {
            region: Default::default(),
            tools: SelectBoxWidget::new(buttons, 0).expect("unreachable"),
            current: Tool::default(),
        }
    }
}

impl Widget for ToolBoxWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        canvas.fill_rectangle(self.region, color::BUTTONS_BACKGROUND);
        canvas.draw_rectangle(self.region, color::WINDOW_BORDER);
        self.tools.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.tools.handle_event(app, event).or_fail()?;

        self.tools
            .on_selected(|state, button| {
                if state.is_selected() {
                    self.current = Tool::from_icon(button.icon()).or_fail()?;
                    button.set_kind(ButtonKind::BasicDeep);
                } else {
                    button.set_kind(ButtonKind::Basic);
                }
                app.request_redraw(button.region());
                Ok(())
            })
            .or_fail()?;

        event.consume_if_contained(self.region);
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        self.tools.children()
    }
}

impl FixedSizeWidget for ToolBoxWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.tools.requiring_size(app) + (MARGIN * 2)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.tools.set_position(app, position + MARGIN as i32);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tool {
    #[default]
    Draw,
    Erase,
    Select,
    Pick,
    Move,
}

impl Tool {
    fn from_icon(icon: IconId) -> Result<Self> {
        Ok(match icon {
            IconId::Draw => Self::Draw,
            IconId::Erase => Self::Erase,
            IconId::Select => Self::Select,
            IconId::Pick => Self::Pick,
            IconId::Move => Self::Move,
            _ => return Err(Failure::new(format!("unexpected icon: {icon:?}"))),
        })
    }
}
