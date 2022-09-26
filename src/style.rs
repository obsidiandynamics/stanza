use std::collections::HashMap;

pub enum Style {
    Bold(bool),
}

impl Style {
    pub fn key(&self) -> String {
        match self {
            Style::Bold(_) => "bold".into(),
        }
    }
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
    pub fn insert(&mut self, style: Style) -> Option<Style> {
        self.0.insert(style.key(), style)
    }
}

pub trait Styled {
    fn styles(&self) -> &Styles;
}