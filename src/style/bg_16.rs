use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone)]
pub enum Bg16 {
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

impl Style for Bg16 {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("bg_16")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Bg16> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Bg16(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Bg16 {
    fn into(self) -> StyleKind {
        StyleKind::Bg16(self)
    }
}