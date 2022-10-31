use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct Header(pub bool);

impl Style for Header {
    fn assignability(&self) -> Assignability {
        Assignability::RowColTable
    }
}