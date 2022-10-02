use stanza::renderer::console::Console;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MinWidth, Styles};
use stanza::table::{Cell, Col, Row, Table};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::new(Styles::default().with(MinWidth(20))),
            Col::new(Styles::default().with(MinWidth(15)).with(HAlign::Right))
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![Cell::from("Department"), Cell::from("Budget")],
        ))
        .with_row(vec!["Sales", "90000"])
        .with_row(vec!["Engineering", "270000"]);

    let renderer = Markdown::default();
    println!("{}", renderer.render(&table, &[]));
}
