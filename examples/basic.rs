use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::console::Console;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{Bold, HAlign, Header, MaxWidth, MinWidth, Separator, Styles};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::new(Styles::default().with(HAlign::Right)),
            Col::new(Styles::default().with(MinWidth(20)).with(HAlign::Centred)),
            Col::new(Styles::default().with(MinWidth(10)).with(MaxWidth(20))),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)).with(Bold(true)),
            vec![
                Cell::from("Department"),
                Cell::from("Personnel"),
                Cell::from("Budget"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![Cell::from("Sales"), Cell::from(39), Cell::from(300_000.0)],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("Engineering"),
                Cell::from(117),
                Cell::from(1_150_000.0),
            ],
        ))
        .with_row(Row::new(Styles::default().with(Separator(true)), vec![]))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from("The quick brown fox jumped over the lazy dog.\nThe end!"),
            ],
        ));
    println!("{}", Console::default().render(&table));
    println!("{}", Markdown::default().render(&table));
}
