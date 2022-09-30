use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Blink(pub bool);

impl Style for Blink {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("blink")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Blink> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Blink(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Blink {
    fn into(self) -> StyleKind {
        StyleKind::Blink(self)
    }
}