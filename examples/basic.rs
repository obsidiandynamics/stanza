use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{HAlignment, Style, Styles};

fn main() {
    let table = Table::default()
        .with_cols(vec![Col::Body(
            Styles::default().with(Style::HAlign(HAlignment::Right)),
        )])
        .with_row(Row::Header(
            Styles::default(),
            vec![Cell::from("Department"), Cell::from("Personnel"), Cell::from("Budget")],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![Cell::from("Sales"), Cell::from(39), Cell::from(300_000.0)],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![Cell::from("Engineering"), Cell::from(117), Cell::from(1_150_000.0)],
        ));
    println!("{}", Markdown::default().render(&table));
}
