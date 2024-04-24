use super::{button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget, Widget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    color,
    event::Event,
    model::tool::ToolKind,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Position, Region, Size};

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
                    button.set_kind(ButtonKind::BasicPressed);

                    self.current = next;
                    app.models_mut().tool.current = next;
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
            ButtonWidget::new(ButtonKind::Basic, IconId::Bucket),
            ButtonWidget::new(ButtonKind::Basic, IconId::Erase),
            ButtonWidget::new(ButtonKind::Basic, IconId::Lasso),
            ButtonWidget::new(ButtonKind::Basic, IconId::Move),
        ];

        buttons[0].set_disabled_callback(|app| app.models().tool.current == ToolKind::Pick);
        buttons[1].set_disabled_callback(|app| app.models().tool.current == ToolKind::Draw);
        buttons[2].set_disabled_callback(|app| app.models().tool.current == ToolKind::Fill);
        buttons[3].set_disabled_callback(|app| app.models().tool.current == ToolKind::Erase);
        buttons[4].set_disabled_callback(|app| app.models().tool.current == ToolKind::Select);
        buttons[5].set_disabled_callback(|app| app.models().tool.current == ToolKind::Move);

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
