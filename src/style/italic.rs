use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Italic(pub bool);

impl Style for Italic {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Italic> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__Italic(style) => Some(style),
            _ => None,
        }
    }
}

impl From<Italic> for StyleKind {
    fn from(style: Italic) -> Self {
        StyleKind::__Italic(style)
    }
}