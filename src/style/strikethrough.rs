use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Strikethrough(pub bool);

impl Style for Strikethrough {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}