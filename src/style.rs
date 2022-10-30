pub mod blink;
pub mod bold;
pub mod border_bg;
pub mod border_fg;
pub mod fill_bg;
pub mod halign;
pub mod header;
pub mod italic;
pub mod max_width;
pub mod min_width;
pub mod palette_16;
pub mod separator;
pub mod strikethrough;
pub mod text_bg;
pub mod text_fg;
pub mod underline;

use alloc::borrow::Cow;
use alloc::collections::btree_map::Iter;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
pub use blink::Blink;
pub use bold::Bold;
pub use border_bg::BorderBg;
pub use border_fg::BorderFg;
use core::any;
pub use fill_bg::FillBg;
pub use halign::HAlign;
pub use header::Header;
pub use italic::Italic;
pub use max_width::MaxWidth;
pub use min_width::MinWidth;
pub use palette_16::Palette16;
pub use separator::Separator;
pub use strikethrough::Strikethrough;
pub use text_bg::TextBg;
pub use text_fg::TextFg;
pub use underline::Underline;

pub trait Style: Clone
where
    for<'a> Option<&'a Self>: From<&'a StyleKind>,
    StyleKind: From<Self>,
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
#[allow(clippy::module_name_repetitions)]
pub enum StyleKind {
    __Blink(Blink),
    __Bold(Bold),
    __BorderBg(BorderBg),
    __BorderFg(BorderFg),
    __FillBg(FillBg),
    __HAlign(HAlign),
    __Header(Header),
    __Italic(Italic),
    __MaxWidth(MaxWidth),
    __MinWidth(MinWidth),
    __Separator(Separator),
    __Strikethrough(Strikethrough),
    __TextFg(TextFg),
    __TextBg(TextBg),
    __Underline(Underline),
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
            StyleKind::__Blink(_) => Statics::capture::<Blink>(),
            StyleKind::__Bold(_) => Statics::capture::<Bold>(),
            StyleKind::__BorderBg(_) => Statics::capture::<BorderBg>(),
            StyleKind::__BorderFg(_) => Statics::capture::<BorderFg>(),
            StyleKind::__FillBg(_) => Statics::capture::<FillBg>(),
            StyleKind::__HAlign(_) => Statics::capture::<HAlign>(),
            StyleKind::__Header(_) => Statics::capture::<Header>(),
            StyleKind::__Italic(_) => Statics::capture::<Italic>(),
            StyleKind::__MaxWidth(_) => Statics::capture::<MaxWidth>(),
            StyleKind::__MinWidth(_) => Statics::capture::<MinWidth>(),
            StyleKind::__Separator(_) => Statics::capture::<Separator>(),
            StyleKind::__Strikethrough(_) => Statics::capture::<Strikethrough>(),
            StyleKind::__TextBg(_) => Statics::capture::<TextBg>(),
            StyleKind::__TextFg(_) => Statics::capture::<TextFg>(),
            StyleKind::__Underline(_) => Statics::capture::<Underline>(),
        }
    }
}

struct Statics {
    id: Cow<'static, str>,
    assignability: Assignability,
}

impl Statics {
    fn capture<S: Style>() -> Self
    where
        for<'a> Option<&'a S>: From<&'a StyleKind>,
        StyleKind: From<S>,
    {
        Self {
            id: S::id(),
            assignability: S::assignability(),
        }
    }
}

#[derive(Default)]
pub struct Styles(BTreeMap<String, StyleKind>);

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
    #[must_use]
    pub fn with<S: Into<StyleKind>>(mut self, style: S) -> Self {
        self.insert(style.into());
        self
    }

    pub fn insert(&mut self, style: StyleKind) -> Option<StyleKind> {
        self.0.insert(style.id(), style)
    }

    pub fn insert_all(&mut self, styles: &Styles) {
        for (key, style) in &styles.0 {
            self.0.insert(key.into(), style.clone());
        }
    }

    pub fn get(&self, key: &str) -> Option<&StyleKind> {
        self.0.get(key)
    }

    pub fn take(&mut self, key: &str) -> Option<StyleKind> {
        self.0.remove(key)
    }

    /// Verifies the assignability of all styles by evaluating the given `check` predicate, panicking
    /// if the predicate evaluates to `false` for any style. The type `S` is used purely for generating the
    /// panic message.
    ///
    /// # Panics
    /// If one of the styles is assignment-incompatible according to the predicate.
    pub fn assert_assignability<S>(&self, mut check: impl FnMut(Assignability) -> bool) {
        for entry in self {
            assert!(
                check(entry.1.assignability()),
                "cannot assign style {} to a {}",
                entry.1.id(),
                any::type_name::<S>()
            );
        }
    }
}

impl<'a> IntoIterator for &'a Styles {
    type Item = (&'a String, &'a StyleKind);
    type IntoIter = Iter<'a, String, StyleKind>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub trait Styled {
    fn styles(&self) -> &Styles;
}

/// Indicates the element types to which a particular [`Style`] may be assigned.
pub enum Assignability {
    /// At the table level only.
    TableOnly,

    /// At the column level and at the table level.
    ColTable,

    /// At the row level and at the table level.
    RowTable,

    /// At the row, column and table level.
    RowColTable,

    /// At the cell, row, column and table level.
    CellRowColTable,
}

impl Assignability {
    pub fn at_col(&self) -> bool {
        matches!(
            self,
            Assignability::ColTable | Assignability::RowColTable | Assignability::CellRowColTable
        )
    }

    pub fn at_row(&self) -> bool {
        matches!(
            self,
            Assignability::RowTable | Assignability::RowColTable | Assignability::CellRowColTable
        )
    }

    pub fn at_cell(&self) -> bool {
        matches!(self, Assignability::CellRowColTable)
    }
}

#[cfg(test)]
mod tests;
