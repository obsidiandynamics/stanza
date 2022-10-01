use crate::style::{Bold, Border16, Header, Styles};
use crate::table::{Cell, Col, Row};

#[test]
fn cell_style_assignability_allows() {
    Cell::new(Styles::default().with(Bold(true)), "");
}

#[test]
#[should_panic(expected="cannot assign style stanza::style::header::Header to a stanza::table::Cell")]
fn cell_style_assignability_panics() {
    Cell::new(Styles::default().with(Header(true)), "");
}

#[test]
fn row_style_assignability_allows() {
    Row::new(Styles::default().with(Header(true)), vec![]);
}

#[test]
#[should_panic(expected="cannot assign style stanza::style::border_16::Border16 to a stanza::table::Row")]
fn row_style_assignability_panics() {
    Row::new(Styles::default().with(Border16::BrightMagenta), vec![]);
}

#[test]
fn col_style_assignability_allows() {
    Col::new(Styles::default().with(Header(true)));
}

#[test]
#[should_panic(expected="cannot assign style stanza::style::border_16::Border16 to a stanza::table::Col")]
fn col_style_assignability_panics() {
    Col::new(Styles::default().with(Border16::BrightMagenta));
}