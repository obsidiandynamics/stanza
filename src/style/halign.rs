use std::borrow::Cow;
use crate::style::{Style, StyleKind};

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
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("h_align")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a HAlign> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::HAlign(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for HAlign {
    fn into(self) -> StyleKind {
        StyleKind::HAlign(self)
    }
}