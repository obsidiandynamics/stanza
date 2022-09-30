use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Bold(pub bool);

impl Style for Bold {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("bold")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Bold> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Bold(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Bold {
    fn into(self) -> StyleKind {
        StyleKind::Bold(self)
    }
}