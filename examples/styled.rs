use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MinWidth, Styles};
use stanza::table::{Col, Row, Table};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::new(Styles::default().with(MinWidth(20))),
            Col::new(Styles::default().with(MinWidth(15)).with(HAlign::Right))
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec!["Department".into(), "Budget".into()],
        ))
        .with_row(["Sales", "90000"])
        .with_row(["Engineering", "270000"]);

    let renderer = Console::default();
    println!("{}", renderer.render(&table, &[]));
}
