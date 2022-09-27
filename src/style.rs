use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MinWidthSpec(pub usize);

impl StyleSpec for MinWidthSpec {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a Style> for Option<&'a MinWidthSpec> {
    fn from(style: &'a Style) -> Self {
        match style {
            Style::MinWidth(spec) => Some(spec),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaxWidthSpec(pub usize);

impl StyleSpec for MaxWidthSpec {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a Style> for Option<&'a MaxWidthSpec> {
    fn from(style: &'a Style) -> Self {
        match style {
            Style::MaxWidth(spec) => Some(spec),
            _ => None
        }
    }
}

pub trait StyleSpec where Self: Sized, for <'a> Option<&'a Self>: From<&'a Style> {
    fn key() -> Cow<'static, str>;

    fn resolve(styles: &Styles) -> Option<&Self> {
        let style = styles.get(&Self::key());
        match style {
            None => None,
            Some(style) => style.into()
        }
    }
}

pub const KEY_BOLD: &str = "bold";
pub const KEY_H_ALIGN: &str = "h_align";

#[derive(Debug, Clone)]
pub enum Style {
    Bold(bool),
    HAlign(HAlignment),
    MinWidth(MinWidthSpec),
    MaxWidth(MaxWidthSpec)
}

impl Style {
    pub fn key(&self) -> String {
        match self {
            Style::Bold(_) => KEY_BOLD.into(),
            Style::HAlign(_) => KEY_H_ALIGN.into(),
            Style::MinWidth(_) => MinWidthSpec::key().into_owned(),
            Style::MaxWidth(_) => MaxWidthSpec::key().into_owned()
        }
    }

    pub fn as_bold(&self) -> Option<bool> {
        match self {
            Style::Bold(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_h_align(&self) -> Option<&HAlignment> {
        match self {
            Style::HAlign(align) => Some(align),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum HAlignment {
    Left,
    Centred,
    Right,
}

#[derive(Default)]
pub struct Styles(HashMap<String, Style>);

impl From<Vec<Style>> for Styles {
    fn from(vec: Vec<Style>) -> Self {
        let mut styles = Styles::default();
        for style in vec {
            styles.insert(style);
        }
        styles
    }
}

impl Styles {
    pub fn with(mut self, style: Style) -> Self {
        self.insert(style);
        self
    }

    pub fn insert(&mut self, style: Style) -> Option<Style> {
        self.0.insert(style.key(), style)
    }

    pub fn insert_all(&mut self, styles: &Styles) {
        for (key, style) in styles.0.iter() {
            self.0.insert(key.into(), style.clone());
        }
    }

    pub fn get(&self, key: &str) -> Option<&Style> {
        self.0.get(key)
    }

    pub fn take(&mut self, key: &str) -> Option<Style> {
        self.0.remove(key)
    }
}

pub trait Styled {
    fn styles(&self) -> &Styles;
}
