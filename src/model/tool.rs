use crate::{asset::IconId, marker::MarkerKind};
use pagurus::image::Rgba;
use pagurus::{failure::Failure, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolModel {
    pub current: ToolKind,
    pub draw: DrawTool,
    pub erase: EraseTool,
    pub select: SelectTool,
    pub r#move: MoveToolState,
    pub pick: PickToolState,
}

impl ToolModel {
    pub fn tool_kind(&self) -> ToolKind {
        self.current
    }

    pub fn marker_kind(&self) -> MarkerKind {
        match self.current {
            ToolKind::Draw => self.draw.marker(),
            ToolKind::Erase => self.erase.marker(),
            ToolKind::Select => self.select.marker(),
            ToolKind::Move => self.r#move.marker,
            ToolKind::Pick => self.pick.marker,
        }
    }
}

impl Default for ToolModel {
    fn default() -> Self {
        Self {
            current: ToolKind::Draw,
            draw: DrawTool::default(),
            erase: EraseTool::default(),
            select: SelectTool::default(),
            r#move: MoveToolState {
                marker: MarkerKind::Noop,
            },
            pick: PickToolState {
                marker: MarkerKind::Pick,
                preview_color: None,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    #[default]
    Draw,
    Erase,
    Select,
    Move,
    Pick,
}

impl ToolKind {
    pub fn from_icon(icon: IconId) -> Result<Self> {
        match icon {
            IconId::Draw => Ok(Self::Draw),
            IconId::PenStroke => Ok(Self::Draw),
            IconId::PenLine => Ok(Self::Draw),
            IconId::PenRectangle => Ok(Self::Draw),
            IconId::PenCircle => Ok(Self::Draw),
            IconId::Bucket => Ok(Self::Draw),
            IconId::Erase => Ok(Self::Erase),
            IconId::ScissorRectangle => Ok(Self::Erase),
            IconId::ScissorLasso => Ok(Self::Erase),
            IconId::Select => Ok(Self::Select),
            IconId::Lasso => Ok(Self::Select),
            IconId::Pick => Ok(Self::Pick),
            IconId::Move => Ok(Self::Move),
            _ => pagurus::unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DrawTool {
    #[default]
    PenStroke,
    PenLine,
    PenRectangle,
    PenCircle,
    Bucket,
}

impl DrawTool {
    fn marker(self) -> MarkerKind {
        match self {
            DrawTool::PenStroke => MarkerKind::Stroke,
            DrawTool::PenLine => MarkerKind::Line,
            DrawTool::PenRectangle => MarkerKind::Rectangle,
            DrawTool::PenCircle => MarkerKind::Ellipse,
            DrawTool::Bucket => MarkerKind::Fill,
        }
    }

    pub fn icon(self) -> IconId {
        match self {
            DrawTool::PenStroke => IconId::PenStroke,
            DrawTool::PenLine => IconId::PenLine,
            DrawTool::PenRectangle => IconId::PenRectangle,
            DrawTool::PenCircle => IconId::PenCircle,
            DrawTool::Bucket => IconId::Bucket,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EraseTool {
    #[default]
    Eraser,
    ScissorLasso,
    ScissorRectangle,
}

impl EraseTool {
    fn marker(self) -> MarkerKind {
        match self {
            EraseTool::Eraser => MarkerKind::Stroke,
            EraseTool::ScissorLasso => MarkerKind::Lasso,
            EraseTool::ScissorRectangle => MarkerKind::FillRectangle,
        }
    }

    pub fn icon(self) -> IconId {
        match self {
            EraseTool::Eraser => IconId::Erase,
            EraseTool::ScissorLasso => IconId::ScissorLasso,
            EraseTool::ScissorRectangle => IconId::ScissorRectangle,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectTool {
    #[default]
    Rectangle,
    Lasso,
}

impl SelectTool {
    fn marker(self) -> MarkerKind {
        match self {
            SelectTool::Rectangle => MarkerKind::FillRectangle,
            SelectTool::Lasso => MarkerKind::Lasso,
        }
    }

    pub fn icon(self) -> IconId {
        match self {
            SelectTool::Rectangle => IconId::Select,
            SelectTool::Lasso => IconId::Lasso,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveToolState {
    pub marker: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickToolState {
    pub marker: MarkerKind,
    pub preview_color: Option<Rgba>,
}
