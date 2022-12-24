use super::{
    block::BlockWidget, button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    asset::{ButtonKind, IconId, Text},
    event::Event,
    model::tool::DrawTool,
};
use pagurus::image::Canvas;
use pagurus::{
    failure::{Failure, OrFail},
    spatial::{Position, Region, Size},
    Result,
};

#[derive(Debug)]
pub struct DrawToolWidget {
    region: Region,
    tools: BlockWidget<SelectBoxWidget>,
    current: DrawTool,
}

impl DrawToolWidget {
    pub fn new(app: &App) -> Result<Self> {
        let current = app.models().tool.draw;
        let mut buttons = vec![
            ButtonWidget::new(ButtonKind::Basic, IconId::PenStroke),
            ButtonWidget::new(ButtonKind::Basic, IconId::PenLine),
            ButtonWidget::new(ButtonKind::Basic, IconId::PenRectangle),
            // TODO
            // ButtonWidget::new(ButtonKind::Basic, IconId::PenCircle),
            ButtonWidget::new(ButtonKind::Basic, IconId::Bucket),
        ];
        buttons[tool_to_index(current)].set_kind(ButtonKind::BasicPressed);
        Ok(Self {
            region: Region::default(),
            tools: BlockWidget::new(
                "DRAWING TOOL".parse::<Text>().or_fail()?,
                SelectBoxWidget::new(buttons, tool_to_index(current)).or_fail()?,
            ),
            current,
        })
    }
}

impl Widget for DrawToolWidget {
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
                        app.models_mut().tool.draw = selected;
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

impl FixedSizeWidget for DrawToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.tools.requiring_size(app)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.tools.set_region(app, self.region);
    }
}

fn tool_to_index(tool: DrawTool) -> usize {
    match tool {
        DrawTool::PenStroke => 0,
        DrawTool::PenLine => 1,
        DrawTool::PenRectangle => 2,
        DrawTool::PenCircle => 3, // TODO
        DrawTool::Bucket => 3,
    }
}

fn icon_to_tool(icon: IconId) -> Result<DrawTool> {
    match icon {
        IconId::PenStroke => Ok(DrawTool::PenStroke),
        IconId::PenLine => Ok(DrawTool::PenLine),
        IconId::PenRectangle => Ok(DrawTool::PenRectangle),
        IconId::PenCircle => Ok(DrawTool::PenCircle),
        IconId::Bucket => Ok(DrawTool::Bucket),
        _ => pagurus::unreachable!(),
    }
}
