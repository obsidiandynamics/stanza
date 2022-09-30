use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone)]
pub enum Fg16 {
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

impl Style for Fg16 {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("fg_16")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Fg16> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Fg16(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Fg16 {
    fn into(self) -> StyleKind {
        StyleKind::Fg16(self)
    }
}