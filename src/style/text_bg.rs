use crate::style::{Assignability, Palette16, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct TextBg(pub Palette16);

impl Style for TextBg {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a TextBg> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__TextBg(style) => Some(style),
            _ => None,
        }
    }
}

impl From<TextBg> for StyleKind {
    fn from(style: TextBg) -> Self {
        StyleKind::__TextBg(style)
    }
}