use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Strikethrough(pub bool);

impl Style for Strikethrough {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("strikethrough")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Strikethrough> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Strikethrough(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Strikethrough {
    fn into(self) -> StyleKind {
        StyleKind::Strikethrough(self)
    }
}