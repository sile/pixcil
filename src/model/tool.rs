use crate::marker::MarkerKind;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    #[default]
    DrawStroke,
    EraseStroke,
}

impl Tool {
    pub fn tool_kind(self) -> ToolKind {
        match self {
            Tool::DrawStroke => ToolKind::Draw,
            Tool::EraseStroke => ToolKind::Erase,
        }
    }

    pub fn marker_kind(self) -> MarkerKind {
        match self {
            Tool::DrawStroke | Tool::EraseStroke => MarkerKind::Stroke,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Draw,
    Erase,
}
