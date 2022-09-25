use super::{
    block::BlockWidget, button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    asset::{ButtonKind, IconId, Text},
    event::Event,
    model::tool::SelectTool,
};
use pagurus::{
    failure::{Failure, OrFail},
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct SelectToolWidget {
    region: Region,
    tools: BlockWidget<SelectBoxWidget>,
    current: SelectTool,
}

impl SelectToolWidget {
    pub fn new(app: &App) -> Result<Self> {
        let current = app.models().tool.select;
        let mut buttons = vec![
            ButtonWidget::new(ButtonKind::Basic, IconId::Select),
            ButtonWidget::new(ButtonKind::Basic, IconId::Lasso),
        ];
        buttons[tool_to_index(current)].set_kind(ButtonKind::BasicPressed);
        Ok(Self {
            region: Region::default(),
            tools: BlockWidget::new(
                "SELECTING TOOL".parse::<Text>().or_fail()?,
                SelectBoxWidget::new(buttons, tool_to_index(current)).or_fail()?,
            ),
            current,
        })
    }
}

impl Widget for SelectToolWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.tools.render(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.tools.handle_event(app, event).or_fail()?;
        self.tools
            .body_mut()
            .on_selected(|state, button| {
                if state.is_selected() {
                    button.set_kind(ButtonKind::BasicPressed);
                    let selected = icon_to_tool(button.icon()).or_fail()?;
                    if self.current != selected {
                        self.current = selected;
                        app.models_mut().tool.select = selected;
                    }
                } else {
                    button.set_kind(ButtonKind::Basic);
                }
                app.request_redraw(button.region());
                Ok(())
            })
            .or_fail()?;
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.tools]
    }
}

impl FixedSizeWidget for SelectToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.tools.requiring_size(app)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.tools.set_region(app, self.region);
    }
}

fn tool_to_index(tool: SelectTool) -> usize {
    match tool {
        SelectTool::Rectangle => 0,
        SelectTool::Lasso => 1,
    }
}

fn icon_to_tool(icon: IconId) -> Result<SelectTool> {
    match icon {
        IconId::Select => Ok(SelectTool::Rectangle),
        IconId::Lasso => Ok(SelectTool::Lasso),
        _ => Err(Failure::unreachable()),
    }
}
