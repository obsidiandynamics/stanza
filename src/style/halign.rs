use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone)]
pub enum HAlign {
    Left,
    Centred,
    Right,
}

impl Default for HAlign {
    fn default() -> Self {
        Self::Left
    }
}

impl Style for HAlign {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a HAlign> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__HAlign(style) => Some(style),
            _ => None,
        }
    }
}

impl From<HAlign> for StyleKind {
    fn from(style: HAlign) -> Self {
        StyleKind::__HAlign(style)
    }
}