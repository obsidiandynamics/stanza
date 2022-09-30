use std::borrow::Cow;
use crate::style::{Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Header(pub bool);

impl Style for Header {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("header")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Header> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Header(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Header {
    fn into(self) -> StyleKind {
        StyleKind::Header(self)
    }
}