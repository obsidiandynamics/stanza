use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Underline(pub bool);

impl Style for Underline {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}