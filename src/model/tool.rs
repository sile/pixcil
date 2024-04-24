use crate::{asset::IconId, marker::MarkerKind};
use pagurus::image::Rgba;
use pagurus::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolModel {
    pub current: ToolKind,
    pub draw: DrawTool,
    pub fill: FillToolState,
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
            ToolKind::Fill => self.fill.marker,
        }
    }
}

impl Default for ToolModel {
    fn default() -> Self {
        Self {
            current: ToolKind::Draw,
            draw: DrawTool::default(),
            erase: EraseTool,
            select: SelectTool::default(),
            r#move: MoveToolState {
                marker: MarkerKind::Noop,
            },
            pick: PickToolState {
                marker: MarkerKind::Pick,
                preview_color: None,
            },
            fill: FillToolState {
                marker: MarkerKind::Fill,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    #[default]
    Draw,
    Fill,
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
            IconId::Bucket => Ok(Self::Fill),
            IconId::Erase => Ok(Self::Erase),
            IconId::ScissorRectangle => Ok(Self::Erase),
            IconId::ScissorLasso => Ok(Self::Erase),
            IconId::Select => Ok(Self::Select),
            IconId::Lasso => Ok(Self::Select),
            IconId::SelectBucket => Ok(Self::Select),
            IconId::Pick => Ok(Self::Pick),
            IconId::Move => Ok(Self::Move),
            _ => Err(orfail::Failure::new("unreachable")),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DrawTool;

impl DrawTool {
    fn marker(self) -> MarkerKind {
        MarkerKind::Stroke
    }

    pub fn icon(self) -> IconId {
        IconId::PenStroke
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EraseTool;

impl EraseTool {
    fn marker(self) -> MarkerKind {
        MarkerKind::Stroke
    }

    pub fn icon(self) -> IconId {
        IconId::Erase
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SelectTool;

impl SelectTool {
    fn marker(self) -> MarkerKind {
        MarkerKind::Lasso
    }

    pub fn icon(self) -> IconId {
        IconId::Lasso
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FillToolState {
    pub marker: MarkerKind,
}
