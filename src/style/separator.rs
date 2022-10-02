use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Separator(pub bool);

impl Style for Separator {
    fn assignability() -> Assignability {
        Assignability::RowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Separator> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Separator(style) => Some(style),
            _ => None,
        }
    }
}

impl From<Separator> for StyleKind {
    fn from(style: Separator) -> Self {
        StyleKind::Separator(style)
    }
}