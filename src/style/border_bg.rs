use crate::style::{Assignability, Palette16, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct BorderBg(pub Palette16);

impl Style for BorderBg {
    fn assignability() -> Assignability {
        Assignability::Table
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a BorderBg> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::BorderBg(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for BorderBg {
    fn into(self) -> StyleKind {
        StyleKind::BorderBg(self)
    }
}