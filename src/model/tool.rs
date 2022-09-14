#[derive(Debug)]
pub enum Tool {
    Draw(DrawKind),
    Erase(EraseKind),
    Select(SelectKind),
    Move,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Draw(Default::default())
    }
}

#[derive(Debug, Default)]
pub enum DrawKind {
    #[default]
    Stroke,
    Line,
    RectangleOutline,
    Fill,
}

#[derive(Debug, Default)]
pub enum EraseKind {
    #[default]
    Stroke,
    Line,
    Rectangle,
}

#[derive(Debug, Default)]
pub enum SelectKind {
    #[default]
    Lasso,
    Rectangle,
}
