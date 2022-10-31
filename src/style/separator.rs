use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Separator(pub bool);

impl Style for Separator {
    fn assignability(&self) -> Assignability {
        Assignability::RowColTable
    }
}