use crate::style::{Assignability, Palette16, Style};

#[derive(Debug, Clone)]
pub struct TextFg(pub Palette16);

impl Style for TextFg {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}