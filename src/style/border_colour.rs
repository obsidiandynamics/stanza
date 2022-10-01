use crate::style::{Assignability, Palette16, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct BorderColour(pub Palette16);

impl Style for BorderColour {
    fn assignability() -> Assignability {
        Assignability::Table
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a BorderColour> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::BorderColour(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for BorderColour {
    fn into(self) -> StyleKind {
        StyleKind::BorderColour(self)
    }
}