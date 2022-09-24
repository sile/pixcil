use crate::marker::MarkerKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolModel {
    pub current: ToolKind,
    pub draw: DrawToolState,
    pub erase: EraseToolState,
    pub select: SelectToolState,
    pub r#move: MoveToolState,
    pub pick: PickToolState,
}

impl ToolModel {
    pub fn tool_kind(&self) -> ToolKind {
        self.current
    }

    pub fn marker_kind(&self) -> MarkerKind {
        match self.current {
            ToolKind::Draw => self.draw.marker,
            ToolKind::Erase => self.erase.marker,
            ToolKind::Select => self.select.marker,
            ToolKind::Move => self.r#move.marker,
            ToolKind::Pick => self.pick.marker,
        }
    }
}

impl Default for ToolModel {
    fn default() -> Self {
        Self {
            current: ToolKind::Draw,
            draw: DrawToolState {
                marker: MarkerKind::Stroke,
            },
            erase: EraseToolState {
                marker: MarkerKind::Stroke,
            },
            select: SelectToolState {
                marker: MarkerKind::Lasso,
            },
            r#move: MoveToolState {
                marker: MarkerKind::Noop,
            },
            pick: PickToolState {
                marker: MarkerKind::Pick,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Draw,
    Erase,
    Select,
    Move,
    Pick,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawToolState {
    pub marker: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EraseToolState {
    pub marker: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectToolState {
    pub marker: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveToolState {
    pub marker: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickToolState {
    pub marker: MarkerKind,
}
