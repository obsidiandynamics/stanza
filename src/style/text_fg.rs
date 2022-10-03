use crate::style::{Assignability, Palette16, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct TextFg(pub Palette16);

impl Style for TextFg {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a TextFg> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__TextFg(style) => Some(style),
            _ => None,
        }
    }
}

impl From<TextFg> for StyleKind {
    fn from(style: TextFg) -> Self {
        StyleKind::__TextFg(style)
    }
}