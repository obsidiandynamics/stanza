use crate::style::{Assignability, Style};

#[derive(Debug, Clone)]
pub enum HAlign {
    Left,
    Centred,
    Right,
}

impl Default for HAlign {
    fn default() -> Self {
        Self::Left
    }
}

impl Style for HAlign {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}