use crate::style::{Assignability, Style};

#[derive(Debug, Clone, Default)]
pub struct MinWidth(pub usize);

impl Style for MinWidth {
    fn assignability(&self) -> Assignability {
        Assignability::ColTable
    }
}