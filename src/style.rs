pub mod blink;
pub mod bold;
pub mod border_bg;
pub mod border_fg;
pub mod fill_bg;
pub mod fill_invert;
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
pub mod text_invert;
pub mod underline;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::btree_map::Iter;
use alloc::collections::BTreeMap;
use alloc::string::String;
pub use blink::Blink;
pub use bold::Bold;
pub use border_bg::BorderBg;
pub use border_fg::BorderFg;
use core::any;
use core::any::Any;
use maybe_owned::MaybeOwned;
pub use fill_bg::FillBg;
pub use fill_invert::FillInvert;
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
pub use text_invert::TextInvert;
pub use underline::Underline;

pub trait Replica {
    fn replicate(&self) -> Box<dyn Style>;
}

impl<C: Clone + Style> Replica for C {
    fn replicate(&self) -> Box<dyn Style> {
        Box::new(self.clone())
    }
}

mod private {
    use core::any::Any;
    use super::Style;

    /// Conversion from an arbitrary trait to a [`&dyn Any`] for subsequent downcasting
    /// (or other uses of the [`Any`] type).
    pub trait Upcast {
        /// Casts the current object to a [`&dyn Any`].
        fn as_any_ref(&self) -> &dyn Any;
    }

    /// Upcasts any [`Style`] implementation to a [`&dyn Any`].
    impl<U: Style> Upcast for U {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
    }
}

pub trait Style: Any + Replica + private::Upcast {
    fn id() -> Cow<'static, str> where Self: Sized {
        Cow::Borrowed(any::type_name::<Self>())
    }

    fn resolve(styles: &Styles) -> Option<&Self> where Self: Sized {
        let style_for_id = styles.get(&Self::id());
        match style_for_id {
            None => None,
            Some(style) => {
                style.as_any_ref().downcast_ref::<Self>()
            }
        }
    }

    fn resolve_or_default(styles: &Styles) -> MaybeOwned<Self>
    where
        Self: Default + Sized,
    {
        let style = Self::resolve(styles);
        match style {
            None => MaybeOwned::Owned(Self::default()),
            Some(style) => MaybeOwned::Borrowed(style),
        }
    }

    fn assignability(&self) -> Assignability;
}

#[derive(Default)]
pub struct Styles(BTreeMap<String, Box<dyn Style>>);

impl Styles {
    #[must_use]
    pub fn with(mut self, style: impl Style) -> Self {
        self.insert(style);
        self
    }

    pub fn insert<S: Style>(&mut self, style: S) -> Option<Box<dyn Style>> {
        self.0.insert(S::id().into(), Box::new(style))
    }

    #[must_use]
    pub fn with_all(mut self, styles: &Styles) -> Self {
        self.insert_all(styles);
        self
    }

    pub fn insert_all(&mut self, styles: &Styles) {
        for (key, style) in &styles.0 {
            self.0.insert(key.into(), style.replicate());
        }
    }

    pub fn get(&self, key: &str) -> Option<&dyn Style> {
        self.0.get(key).map(|style| &**style)
    }

    pub fn take(&mut self, key: &str) -> Option<Box<dyn Style>> {
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
                entry.0,
                any::type_name::<S>()
            );
        }
    }
}

impl<'a> IntoIterator for &'a Styles {
    type Item = (&'a String, &'a Box<dyn Style>);
    type IntoIter = Iter<'a, String, Box<dyn Style>>;

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
