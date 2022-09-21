use crate::marker::MarkerKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolModel {
    pub current: ToolKind,
    pub draw: DrawToolState,
    pub erase: EraseToolState,
}

impl ToolModel {
    pub fn tool_kind(&self) -> ToolKind {
        self.current
    }

    pub fn marker_kind(&self) -> MarkerKind {
        match self.current {
            ToolKind::Draw => self.draw.marker,
            ToolKind::Erase => self.erase.marker,
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Draw,
    Erase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawToolState {
    pub marker: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EraseToolState {
    pub marker: MarkerKind,
}
