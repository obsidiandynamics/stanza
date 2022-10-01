use alloc::vec;
use crate::style::{Bold, BorderFg, Header, Palette16, Styles};
use crate::table::{Cell, Content, Col, Row};

#[test]
fn cell_style_assignability_allows() {
    Cell::new(Styles::default().with(Bold(true)), Content::from(""));
}

#[test]
#[should_panic(expected="cannot assign style stanza::style::header::Header to a stanza::table::Cell")]
fn cell_style_assignability_panics() {
    Cell::new(Styles::default().with(Header(true)), Content::from(""));
}

#[test]
fn row_style_assignability_allows() {
    Row::new(Styles::default().with(Header(true)), vec![]);
}

#[test]
#[should_panic(expected="cannot assign style stanza::style::border_fg::BorderFg to a stanza::table::Row")]
fn row_style_assignability_panics() {
    Row::new(Styles::default().with(BorderFg(Palette16::Blue)), vec![]);
}

#[test]
fn col_style_assignability_allows() {
    Col::new(Styles::default().with(Header(true)));
}

#[test]
#[should_panic(expected="cannot assign style stanza::style::border_fg::BorderFg to a stanza::table::Col")]
fn col_style_assignability_panics() {
    Col::new(Styles::default().with(BorderFg(Palette16::Blue)));
}