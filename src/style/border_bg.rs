use crate::style::{Assignability, Palette16, Style};

#[derive(Debug, Clone)]
pub struct BorderBg(pub Palette16);

impl Style for BorderBg {
    fn assignability(&self) -> Assignability {
        Assignability::TableOnly
    }
}