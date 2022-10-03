use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone, Default)]
pub struct Header(pub bool);

impl Style for Header {
    fn assignability() -> Assignability {
        Assignability::RowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Header> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__Header(style) => Some(style),
            _ => None,
        }
    }
}

impl From<Header> for StyleKind {
    fn from(style: Header) -> Self {
        StyleKind::__Header(style)
    }
}