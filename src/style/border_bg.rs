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
            StyleKind::__BorderBg(style) => Some(style),
            _ => None,
        }
    }
}

impl From<BorderBg> for StyleKind {
    fn from(style: BorderBg) -> Self {
        StyleKind::__BorderBg(style)
    }
}