use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Underline(pub bool);

impl Style for Underline {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Underline> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Underline(style) => Some(style),
            _ => None,
        }
    }
}

impl From<Underline> for StyleKind {
    fn from(style: Underline) -> Self {
        StyleKind::Underline(style)
    }
}