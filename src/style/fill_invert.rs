use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct FillInvert(pub bool);

impl Style for FillInvert {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}