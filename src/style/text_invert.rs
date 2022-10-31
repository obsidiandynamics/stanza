use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct TextInvert(pub bool);

impl Style for TextInvert {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}