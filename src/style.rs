use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bold(pub bool);

impl Style for Bold {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("bold")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Bold> {
    fn from(style: &'a StyleKind) -> Self {
        match style {
            StyleKind::Bold(spec) => Some(spec),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum HAlign {
    Left,
    Centred,
    Right,
}

impl Style for HAlign {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("h_align")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a HAlign> {
    fn from(style: &'a StyleKind) -> Self {
        match style {
            StyleKind::HAlign(spec) => Some(spec),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct MinWidth(pub usize);

impl Style for MinWidth {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MinWidth> {
    fn from(style: &'a StyleKind) -> Self {
        match style {
            StyleKind::MinWidth(spec) => Some(spec),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaxWidth(pub usize);

impl Style for MaxWidth {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MaxWidth> {
    fn from(style: &'a StyleKind) -> Self {
        match style {
            StyleKind::MaxWidth(spec) => Some(spec),
            _ => None
        }
    }
}

pub trait Style where Self: Sized, for <'a> Option<&'a Self>: From<&'a StyleKind> {
    fn key() -> Cow<'static, str>;

    fn resolve(styles: &Styles) -> Option<&Self> {
        let style = styles.get(&Self::key());
        match style {
            None => None,
            Some(style) => style.into()
        }
    }
}

#[derive(Debug, Clone)]
pub enum StyleKind {
    Bold(Bold),
    HAlign(HAlign),
    MinWidth(MinWidth),
    MaxWidth(MaxWidth)
}

impl StyleKind {
    pub fn key(&self) -> String {
        match self {
            StyleKind::Bold(_) => Bold::key().into(),
            StyleKind::HAlign(_) => HAlign::key().into(),
            StyleKind::MinWidth(_) => MinWidth::key().into(),
            StyleKind::MaxWidth(_) => MaxWidth::key().into()
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
    pub fn with(mut self, style: StyleKind) -> Self {
        self.insert(style);
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
