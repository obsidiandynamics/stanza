use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct MinWidth(pub usize);

impl Style for MinWidth {
    fn assignability() -> Assignability {
        Assignability::ColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MinWidth> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::MinWidth(style) => Some(style),
            _ => None,
        }
    }
}

impl From<MinWidth> for StyleKind {
    fn from(style: MinWidth) -> Self {
        StyleKind::MinWidth(style)
    }
}