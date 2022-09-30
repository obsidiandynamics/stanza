use std::borrow::Cow;
use std::collections::HashMap;

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

#[derive(Debug, Clone, Default)]
pub struct Separator(pub bool);

impl Style for Separator {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("separator")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Separator> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Separator(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Separator {
    fn into(self) -> StyleKind {
        StyleKind::Separator(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Bold(pub bool);

impl Style for Bold {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("bold")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a Bold> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::Bold(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for Bold {
    fn into(self) -> StyleKind {
        StyleKind::Bold(self)
    }
}

#[derive(Debug, Clone)]
pub enum HAlign {
    Left,
    Centred,
    Right,
}

// impl HAlign {
//     /// The `const` default value of [`HAlign`].
//     ///
//     /// Note, since `const fn` cannot be used in traits, an explicit `const` default value
//     /// is provided here.
//     pub const fn default() -> Self {
//         HAlign::Left
//     }
// }

impl Default for HAlign {
    fn default() -> Self {
        Self::Left
    }
}

impl Style for HAlign {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("h_align")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a HAlign> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::HAlign(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for HAlign {
    fn into(self) -> StyleKind {
        StyleKind::HAlign(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MinWidth(pub usize);

impl Style for MinWidth {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MinWidth> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::MinWidth(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for MinWidth {
    fn into(self) -> StyleKind {
        StyleKind::MinWidth(self)
    }
}

#[derive(Debug, Clone)]
pub struct MaxWidth(pub usize);

impl Default for MaxWidth {
    fn default() -> Self {
        Self(usize::MAX)
    }
}

impl Style for MaxWidth {
    fn key() -> Cow<'static, str> {
        Cow::Borrowed("min_width")
    }
}

impl<'a> From<&'a StyleKind> for Option<&'a MaxWidth> {
    fn from(kind: &'a StyleKind) -> Self {
        match kind {
            StyleKind::MaxWidth(style) => Some(style),
            _ => None,
        }
    }
}

impl Into<StyleKind> for MaxWidth {
    fn into(self) -> StyleKind {
        StyleKind::MaxWidth(self)
    }
}

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
    Header(Header),
    Separator(Separator),
    Bold(Bold),
    HAlign(HAlign),
    MinWidth(MinWidth),
    MaxWidth(MaxWidth),
}

impl StyleKind {
    pub fn key(&self) -> String {
        match self {
            StyleKind::Header(_) => Header::key().into(),
            StyleKind::Separator(_) => Separator::key().into(),
            StyleKind::Bold(_) => Bold::key().into(),
            StyleKind::HAlign(_) => HAlign::key().into(),
            StyleKind::MinWidth(_) => MinWidth::key().into(),
            StyleKind::MaxWidth(_) => MaxWidth::key().into(),
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
