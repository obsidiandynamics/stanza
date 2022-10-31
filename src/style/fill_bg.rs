use crate::style::{Assignability, Palette16, Style};

#[derive(Debug, Clone)]
pub struct FillBg(pub Palette16);

impl Style for FillBg {
    fn assignability(&self) -> Assignability {
        Assignability::CellRowColTable
    }
}