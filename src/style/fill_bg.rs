use crate::style::{Assignability, Palette16, Style, StyleKind};

#[derive(Debug, Clone)]
pub struct FillBg(pub Palette16);

impl Style for FillBg {
    fn assignability() -> Assignability {
        Assignability::CellRowColTable
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a FillBg> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::FillBg(style) => Some(style),
            _ => None,
        }
    }
}

impl From<FillBg> for StyleKind {
    fn from(style: FillBg) -> Self {
        StyleKind::FillBg(style)
    }
}