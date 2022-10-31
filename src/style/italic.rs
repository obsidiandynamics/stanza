use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Italic(pub bool);

impl Style for Italic {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}