use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone)]
pub struct MaxWidth(pub usize);

impl Default for MaxWidth {
    fn default() -> Self {
        Self(usize::MAX)
    }
}

impl Style for MaxWidth {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MaxWidth> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::MaxWidth(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for MaxWidth {
    fn into(self) -> StyleKind {
        StyleKind::MaxWidth(self)
    }
}