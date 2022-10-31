use crate::style::{Assignability, Style};

#[derive(Debug, Clone)]
pub struct MaxWidth(pub usize);

impl Default for MaxWidth {
    fn default() -> Self {
        Self(usize::MAX)
    }
}

impl Style for MaxWidth {
    fn assignability(&self) -> Assignability {
        Assignability::ColTable
    }
}