use crate::style::{Assignability, Palette16, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct BorderFg(pub Palette16);

impl Style for BorderFg {
    fn assignability() -> Assignability {
        Assignability::Table
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a BorderFg> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__BorderFg(style) => Some(style),
            _ => None,
        }
    }
}

impl From<BorderFg> for StyleKind {
    fn from(style: BorderFg) -> Self {
        StyleKind::__BorderFg(style)
    }
}