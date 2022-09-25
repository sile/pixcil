use super::{
    block::BlockWidget, button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    asset::{ButtonKind, IconId, Text},
    event::Event,
    model::tool::EraseTool,
};
use pagurus::{
    failure::{Failure, OrFail},
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct EraseToolWidget {
    region: Region,
    tools: BlockWidget<SelectBoxWidget>,
    current: EraseTool,
}

impl EraseToolWidget {
    pub fn new(app: &App) -> Result<Self> {
        let current = app.models().tool.erase;
        let mut buttons = vec![
            ButtonWidget::new(ButtonKind::Basic, IconId::Erase),
            ButtonWidget::new(ButtonKind::Basic, IconId::ScissorLasso),
            ButtonWidget::new(ButtonKind::Basic, IconId::ScissorRectangle),
        ];
        buttons[tool_to_index(current)].set_kind(ButtonKind::BasicPressed);
        Ok(Self {
            region: Region::default(),
            tools: BlockWidget::new(
                "ERASING TOOL".parse::<Text>().or_fail()?,
                SelectBoxWidget::new(buttons, 0).or_fail()?,
            ),
            current,
        })
    }
}

impl Widget for EraseToolWidget {
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
                        app.models_mut().tool.erase = selected;
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

impl FixedSizeWidget for EraseToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.tools.requiring_size(app)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.tools.set_region(app, self.region);
    }
}

fn tool_to_index(tool: EraseTool) -> usize {
    match tool {
        EraseTool::Eraser => 0,
        EraseTool::ScissorLasso => 1,
        EraseTool::ScissorRectangle => 2,
    }
}

fn icon_to_tool(icon: IconId) -> Result<EraseTool> {
    match icon {
        IconId::Erase => Ok(EraseTool::Eraser),
        IconId::ScissorLasso => Ok(EraseTool::ScissorLasso),
        IconId::ScissorRectangle => Ok(EraseTool::ScissorRectangle),
        _ => Err(Failure::unreachable()),
    }
}
