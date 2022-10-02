use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Blink(pub bool);

impl Style for Blink {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Blink> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Blink(style) => Some(style),
            _ => None,
        }
    }
}

impl From<Blink> for StyleKind {
    fn from(style: Blink) -> Self {
        StyleKind::Blink(style)
    }
}