use crate::style::{Assignability, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct MaxWidth(pub usize);

impl Default for MaxWidth {
    fn default() -> Self {
        Self(usize::MAX)
    }
}

impl Style for MaxWidth {
    fn assignability() -> Assignability {
        Assignability::ColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MaxWidth> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::__MaxWidth(style) => Some(style),
            _ => None,
        }
    }
}

impl From<MaxWidth> for StyleKind {
    fn from(style: MaxWidth) -> Self {
        StyleKind::__MaxWidth(style)
    }
}