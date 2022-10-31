use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Blink(pub bool);

impl Style for Blink {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}