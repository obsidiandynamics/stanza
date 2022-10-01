use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Strikethrough(pub bool);

impl Style for Strikethrough {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Strikethrough> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Strikethrough(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Strikethrough {
    fn into(self) -> StyleKind {
        StyleKind::Strikethrough(self)
    }
}