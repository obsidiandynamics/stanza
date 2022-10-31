use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Bold(pub bool);

impl Style for Bold {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}