use super::{
    block::BlockWidget, button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    asset::{ButtonKind, IconId, Text},
    event::Event,
    io::IoRequest,
    model::tool::SelectTool,
};
use pagurus::image::Canvas;
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};

const MARGIN: u32 = 8;

#[derive(Debug)]
pub struct SelectToolWidget {
    region: Region,
    current: SelectTool,
    tools: BlockWidget<SelectBoxWidget>,
    import: BlockWidget<ButtonWidget>,
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
            current,
            tools: BlockWidget::new(
                "SELECTING TOOL".parse::<Text>().or_fail()?,
                SelectBoxWidget::new(buttons, tool_to_index(current)).or_fail()?,
            ),
            import: BlockWidget::new(
                "IMPORT IMAGE".parse::<Text>().or_fail()?,
                ButtonWidget::new(ButtonKind::Basic, IconId::Import),
            ),
        })
    }
}

impl Widget for SelectToolWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.tools.render_if_need(app, canvas);
        self.import.render_if_need(app, canvas);
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

        self.import.handle_event(app, event).or_fail()?;
        if self.import.body_mut().take_clicked(app) {
            app.enqueue_io_request(IoRequest::ImportImage);
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.tools, &mut self.import]
    }
}

impl FixedSizeWidget for SelectToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.tools.requiring_size(app);
        size.width += MARGIN + self.import.requiring_size(app).width;
        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut region = self.region;
        region.size = self.tools.requiring_size(app);
        self.tools.set_region(app, region);

        region.position.x = region.end().x + MARGIN as i32;
        region.size = self.import.requiring_size(app);
        self.import.set_region(app, region);
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
        _ => pagurus::unreachable!(),
    }
}
