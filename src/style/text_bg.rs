use crate::style::{Assignability, Palette16, Style};

#[derive(Debug, Clone)]
pub struct TextBg(pub Palette16);

impl Style for TextBg {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}