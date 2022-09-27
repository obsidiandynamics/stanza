use stanza::model::{Cell, Row, Table};
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::Styles;

fn main() {
    let table = Table::default()
        .with_row(Row::Header(
            Styles::default(),
            vec![Cell::from("Department"), Cell::from("Personnel")],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![Cell::from("Sales"), Cell::from(39)],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![Cell::from("Engineering"), Cell::from(117)],
        ));
    println!("{}", Markdown::default().render(&table));
}
