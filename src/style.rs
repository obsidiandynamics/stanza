pub mod bg_16;
pub mod blink;
pub mod bold;
pub mod fg_16;
pub mod halign;
pub mod header;
pub mod italic;
pub mod max_width;
pub mod min_width;
pub mod separator;
pub mod strikethrough;
pub mod underline;

pub use bg_16::Bg16;
pub use bold::Bold;
pub use blink::Blink;
pub use fg_16::Fg16;
pub use halign::HAlign;
pub use header::Header;
pub use italic::Italic;
pub use max_width::MaxWidth;
pub use min_width::MinWidth;
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
    fn key() -> Cow<'static, str>;

    fn resolve(styles: &Styles) -> Option<&Self> {
        let kind = styles.get(&Self::key());
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
    pub fn key(&self) -> String {
        match self {
            StyleKind::Bg16(_) => Bg16::key().into(),
            StyleKind::Blink(_) => Blink::key().into(),
            StyleKind::Bold(_) => Bold::key().into(),
            StyleKind::Fg16(_) => Fg16::key().into(),
            StyleKind::HAlign(_) => HAlign::key().into(),
            StyleKind::Header(_) => Header::key().into(),
            StyleKind::Italic(_) => Italic::key().into(),
            StyleKind::MaxWidth(_) => MaxWidth::key().into(),
            StyleKind::MinWidth(_) => MinWidth::key().into(),
            StyleKind::Separator(_) => Separator::key().into(),
            StyleKind::Strikethrough(_) => Strikethrough::key().into(),
            StyleKind::Underline(_) => Underline::key().into(),
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
        self.0.insert(style.key(), style)
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
}

pub trait Styled {
    fn styles(&self) -> &Styles;
}
