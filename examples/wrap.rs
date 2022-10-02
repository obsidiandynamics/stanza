use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{Header, MaxWidth, MinWidth, Styles};
use stanza::table::{Cell, Col, Row, Table};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::new(Styles::default().with(MinWidth(15))),
            Col::new(Styles::default().with(MinWidth(30)).with(MaxWidth(40))),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![Cell::from("Poem"), Cell::from("Extract")],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("Antigonish"),
                Cell::from("Yesterday, upon the stair,\nI met a man who wasn't there!\nHe wasn't there again today,\nOh how I wish he'd go away!")
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("The Raven"),
                Cell::from("Ah, distinctly I remember it was in the bleak December;\nAnd each separate dying ember wrought its ghost upon the floor.")
            ]
        ));

    println!("{}", Console::default().render(&table, &[]));
}
