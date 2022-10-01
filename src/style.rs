pub mod bg_16;
pub mod blink;
pub mod bold;
pub mod border_bg;
pub mod border_fg;
pub mod fg_16;
pub mod halign;
pub mod header;
pub mod italic;
pub mod max_width;
pub mod min_width;
pub mod palette_16;
pub mod separator;
pub mod strikethrough;
pub mod underline;

use std::any;
pub use bg_16::Bg16;
pub use bold::Bold;
pub use border_bg::BorderBg;
pub use border_fg::BorderFg;
pub use blink::Blink;
pub use fg_16::Fg16;
pub use halign::HAlign;
pub use header::Header;
pub use italic::Italic;
pub use max_width::MaxWidth;
pub use min_width::MinWidth;
pub use palette_16::Palette16;
pub use separator::Separator;
pub use strikethrough::Strikethrough;
pub use underline::Underline;

use std::borrow::Cow;
use std::collections::HashMap;

pub trait Style
where
    Self: Clone,
    for<'a> Option<&'a Self>: From<&'a StyleKind>,
    Self: Into<StyleKind>,
{
    fn id() -> Cow<'static, str> {
        Cow::Borrowed(any::type_name::<Self>())
    }

    fn assignability() -> Assignability;

    fn resolve(styles: &Styles) -> Option<&Self> {
        let kind = styles.get(&Self::id());
        match kind {
            None => None,
            Some(kind) => kind.into(),
        }
    }

    fn resolve_or_default(styles: &Styles) -> Cow<Self>
    where
        Self: Default,
    {
        let style = Self::resolve(styles);
        match style {
            None => Cow::Owned(Self::default()),
            Some(style) => Cow::Borrowed(style),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StyleKind {
    Bg16(Bg16),
    Blink(Blink),
    Bold(Bold),
    BorderBg(BorderBg),
    BorderFg(BorderFg),
    Fg16(Fg16),
    HAlign(HAlign),
    Header(Header),
    Italic(Italic),
    MaxWidth(MaxWidth),
    MinWidth(MinWidth),
    Separator(Separator),
    Strikethrough(Strikethrough),
    Underline(Underline),
}

impl StyleKind {
    pub fn id(&self) -> String {
        self.statics().id.into()
    }
    
    pub fn assignability(&self) -> Assignability {
        self.statics().assignability
    }
    
    fn statics(&self) -> Statics {
        match self {
            StyleKind::Bg16(_) => Statics::capture::<Bg16>(),
            StyleKind::Blink(_) => Statics::capture::<Blink>(),
            StyleKind::Bold(_) => Statics::capture::<Bold>(),
            StyleKind::BorderBg(_) => Statics::capture::<BorderBg>(),
            StyleKind::BorderFg(_) => Statics::capture::<BorderFg>(),
            StyleKind::Fg16(_) => Statics::capture::<Fg16>(),
            StyleKind::HAlign(_) => Statics::capture::<HAlign>(),
            StyleKind::Header(_) => Statics::capture::<Header>(),
            StyleKind::Italic(_) => Statics::capture::<Italic>(),
            StyleKind::MaxWidth(_) => Statics::capture::<MaxWidth>(),
            StyleKind::MinWidth(_) => Statics::capture::<MinWidth>(),
            StyleKind::Separator(_) => Statics::capture::<Separator>(),
            StyleKind::Strikethrough(_) => Statics::capture::<Strikethrough>(),
            StyleKind::Underline(_) => Statics::capture::<Underline>(),
        }  
    }
}

struct Statics {
    id: Cow<'static, str>,
    assignability: Assignability
}

impl Statics {
    fn capture<S: Style>() -> Self where for<'a> Option<&'a S>: From<&'a StyleKind> {
        Self {
            id: S::id(),
            assignability: S::assignability()
        }
    }
}

#[derive(Default)]
pub struct Styles(HashMap<String, StyleKind>);

impl From<Vec<StyleKind>> for Styles {
    fn from(vec: Vec<StyleKind>) -> Self {
        let mut styles = Styles::default();
        for style in vec {
            styles.insert(style);
        }
        styles
    }
}

impl Styles {
    pub fn with<S: Into<StyleKind>>(mut self, style: S) -> Self {
        self.insert(style.into());
        self
    }

    pub fn insert(&mut self, style: StyleKind) -> Option<StyleKind> {
        self.0.insert(style.id(), style)
    }

    pub fn insert_all(&mut self, styles: &Styles) {
        for (key, style) in styles.0.iter() {
            self.0.insert(key.into(), style.clone());
        }
    }

    pub fn get(&self, key: &str) -> Option<&StyleKind> {
        self.0.get(key)
    }

    pub fn take(&mut self, key: &str) -> Option<StyleKind> {
        self.0.remove(key)
    }
    
    pub fn entries(&self) -> &HashMap<String, StyleKind> {
        &self.0
    }

    pub fn assert_assignability<S>(&self, mut check: impl FnMut(Assignability) -> bool) {
        for entry in &self.0 {
            assert!(check(entry.1.assignability()), "cannot assign style {} to a {}", entry.1.id(), any::type_name::<S>())
        }
    }
}

pub trait Styled {
    fn styles(&self) -> &Styles;
}

/// Indicates the element types to which a particular [`Style`] may be assigned.
pub enum Assignability {
    /// At the table level only.
    Table,

    /// At the column level and at the table level.
    ColTable,

    /// At the row level and at the table level.
    RowTable,

    /// At the row, column and table level.
    RowColTable,

    /// At the cell, row, column and table level.
    CellRowColTable
}

impl Assignability {
    pub fn at_col(&self) -> bool {
        matches!(self, Assignability::ColTable | Assignability::RowColTable | Assignability::CellRowColTable)
    }

    pub fn at_row(&self) -> bool {
        matches!(self, Assignability::RowTable | Assignability::RowColTable | Assignability::CellRowColTable)
    }

    pub fn at_cell(&self) -> bool {
        matches!(self, Assignability::CellRowColTable)
    }
}

#[cfg(test)]
mod tests;
