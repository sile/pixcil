use super::{button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    model::tool::ToolKind,
    window::{draw_tool::DrawToolWindow, move_tool::MoveToolWindow, select_tool::SelectToolWindow},
};
use pagurus::image::Canvas;
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct ToolBoxWidget {
    region: Region,
    tools: SelectBoxWidget,
    current: ToolKind,
}

impl ToolBoxWidget {
    fn handle_tool_change(&mut self, app: &mut App) -> Result<()> {
        self.tools
            .on_selected(|state, button| {
                if state.is_selected() {
                    let next = ToolKind::from_icon(button.icon()).or_fail()?;

                    if matches!(next, ToolKind::Pick | ToolKind::Erase) {
                        button.set_kind(ButtonKind::BasicPressed);
                    } else {
                        button.set_kind(ButtonKind::BasicDeep);
                    }

                    if self.current == next {
                        // Double clicked
                        spawn_window(next, app).or_fail()?;
                    } else {
                        self.current = next;
                        app.models_mut().tool.current = next;
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
            ButtonWidget::new(ButtonKind::Basic, IconId::PenStroke),
            ButtonWidget::new(ButtonKind::Basic, IconId::Erase),
            ButtonWidget::new(ButtonKind::Basic, IconId::Select),
            ButtonWidget::new(ButtonKind::Basic, IconId::Move),
        ];
        buttons[1].set_kind(ButtonKind::BasicDeep);

        buttons[0].set_disabled_callback(|app| app.models().tool.current == ToolKind::Pick);
        buttons[2].set_disabled_callback(|app| app.models().tool.current == ToolKind::Erase);

        Self {
            region: Default::default(),
            tools: SelectBoxWidget::new(buttons, 1).expect("unreachable"),
            current: ToolKind::default(),
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
        if self.current != app.models().tool.current {
            let next = app.models().tool.current;
            let i = self
                .tools
                .buttons()
                .iter()
                .position(|b| Some(next) == ToolKind::from_icon(b.icon()).ok())
                .or_fail()?;
            self.tools.select(app, i).or_fail()?;
            self.handle_tool_change(app).or_fail()?;
        }

        const DRAW_INDEX: usize = 1;
        let draw_icon = app.models().tool.draw.icon();
        if self.tools.buttons()[DRAW_INDEX].icon() != draw_icon {
            self.tools.buttons_mut()[DRAW_INDEX].set_icon(app, draw_icon);
        }

        const ERASE_INDEX: usize = 2;
        let erase_icon = app.models().tool.erase.icon();
        if self.tools.buttons()[ERASE_INDEX].icon() != erase_icon {
            self.tools.buttons_mut()[ERASE_INDEX].set_icon(app, erase_icon);
        }

        const SELECT_INDEX: usize = 3;
        let select_icon = app.models().tool.select.icon();
        if self.tools.buttons()[SELECT_INDEX].icon() != select_icon {
            self.tools.buttons_mut()[SELECT_INDEX].set_icon(app, select_icon);
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

fn spawn_window(tool: ToolKind, app: &mut App) -> Result<()> {
    match tool {
        ToolKind::Draw => app
            .spawn_window(DrawToolWindow::new(app).or_fail()?)
            .or_fail(),
        ToolKind::Erase => Ok(()),
        ToolKind::Select => app
            .spawn_window(SelectToolWindow::new(app).or_fail()?)
            .or_fail(),
        ToolKind::Move => app.spawn_window(MoveToolWindow::new(app)).or_fail(),
        ToolKind::Pick => Ok(()),
    }
}
