use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Bold(pub bool);

impl Style for Bold {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Bold> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__Bold(style) => Some(style),
            _ => None,
        }
    }
}

impl From<Bold> for StyleKind {
    fn from(style: Bold) -> Self {
        StyleKind::__Bold(style)
    }
}