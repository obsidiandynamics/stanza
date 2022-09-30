use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Separator(pub bool);

impl Style for Separator {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("separator")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Separator> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Separator(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Separator {
    fn into(self) -> StyleKind {
        StyleKind::Separator(self)
    }
}