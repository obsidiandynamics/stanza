use crate::style::{Assignability, Palette16, Style};

#[derive(Debug, Clone)]
pub struct BorderFg(pub Palette16);

impl Style for BorderFg {
    fn assignability(&self) -> Assignability {
        Assignability::TableOnly
    }
}