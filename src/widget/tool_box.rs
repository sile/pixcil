use super::{button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    model::tool::ToolKind,
    window::{
        draw_tool::DrawToolWindow, erase_tool::EraseToolWindow, move_tool::MoveToolWindow,
        select_tool::SelectToolWindow,
    },
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
    fn handle_tool_change(&mut self, app: &mut App) -> Result<()> {
        self.tools
            .on_selected(|state, button| {
                if state.is_selected() {
                    let next = Tool::from_icon(button.icon()).or_fail()?;

                    if next == Tool::Pick {
                        button.set_kind(ButtonKind::BasicPressed);
                    } else {
                        button.set_kind(ButtonKind::BasicDeep);
                    }

                    if self.current == next {
                        // Double clicked
                        self.current.spawn_window(app).or_fail()?;
                    } else {
                        self.current = next;
                        match self.current {
                            Tool::Draw => app.models_mut().tool.current = ToolKind::Draw,
                            Tool::Erase => app.models_mut().tool.current = ToolKind::Erase,
                            Tool::Select => app.models_mut().tool.current = ToolKind::Select,
                            Tool::Move => app.models_mut().tool.current = ToolKind::Move,
                            Tool::Pick => app.models_mut().tool.current = ToolKind::Pick,
                        }
                    }
                } else {
                    button.set_kind(ButtonKind::Basic);
                }
                app.request_redraw(button.region());
                Ok(())
            })
            .or_fail()
    }
}

impl Default for ToolBoxWidget {
    fn default() -> Self {
        let mut buttons = vec![
            ButtonWidget::new(ButtonKind::Basic, IconId::Pick),
            ButtonWidget::new(ButtonKind::Basic, IconId::Draw),
            ButtonWidget::new(ButtonKind::Basic, IconId::Erase),
            ButtonWidget::new(ButtonKind::Basic, IconId::Select),
            ButtonWidget::new(ButtonKind::Basic, IconId::Move),
        ];
        buttons[1].set_kind(ButtonKind::BasicDeep);

        Self {
            region: Default::default(),
            tools: SelectBoxWidget::new(buttons, 1).expect("unreachable"),
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
        self.handle_tool_change(app).or_fail()?;
        event.consume_if_contained(self.region);
        Ok(())
    }

    fn handle_event_after(&mut self, app: &mut App) -> Result<()> {
        if self.current.kind() != app.models().tool.current {
            let next = app.models().tool.current;
            let i = self
                .tools
                .buttons()
                .iter()
                .position(|b| Some(next) == Tool::from_icon(b.icon()).ok().map(|x| x.kind()))
                .or_fail()?;
            self.tools.select(app, i).or_fail()?;
            self.handle_tool_change(app).or_fail()?;
        }

        for child in self.children() {
            child.handle_event_after(app).or_fail()?;
        }
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

    fn kind(self) -> ToolKind {
        match self {
            Tool::Draw => ToolKind::Draw,
            Tool::Erase => ToolKind::Erase,
            Tool::Select => ToolKind::Select,
            Tool::Pick => ToolKind::Pick,
            Tool::Move => ToolKind::Move,
        }
    }

    fn spawn_window(self, app: &mut App) -> Result<()> {
        match self {
            Tool::Draw => app
                .spawn_window(DrawToolWindow::new(app).or_fail()?)
                .or_fail(),
            Tool::Erase => app
                .spawn_window(EraseToolWindow::new(app).or_fail()?)
                .or_fail(),
            Tool::Select => app.spawn_window(SelectToolWindow::default()).or_fail(),
            Tool::Move => app.spawn_window(MoveToolWindow::default()).or_fail(),
            Tool::Pick => Ok(()),
        }
    }
}
