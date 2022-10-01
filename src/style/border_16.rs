use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone)]
pub enum Border16 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}

impl Style for Border16 {
    fn assignability() -> Assignability {
        Assignability::Table
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Border16> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Border16(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Border16 {
    fn into(self) -> StyleKind {
        StyleKind::Border16(self)
    }
}