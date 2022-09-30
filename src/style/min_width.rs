use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct MinWidth(pub usize);

impl Style for MinWidth {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MinWidth> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::MinWidth(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for MinWidth {
    fn into(self) -> StyleKind {
        StyleKind::MinWidth(self)
    }
}