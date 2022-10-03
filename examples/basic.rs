use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::Styles;
use stanza::table::{Cell, Col, Content, Row, Table};

fn main() {
    basic_minimalistic();
    basic_expanded();
}

fn basic_minimalistic() {
    // build a table model
    let table = Table::default()
        .with_row(["Department", "Budget"])
        .with_row(["Sales", "90000"])
        .with_row(["Engineering", "270000"]);

    // configure a renderer that will later turn the model into a string
    let renderer = Console::default();

    // render the table, outputting to stdout
    println!("{}", renderer.render(&table));
}

fn basic_expanded() {
    let table = Table::with_styles(Styles::default())
        .with_cols(vec![
            Col::new(Styles::default()),
            Col::new(Styles::default()),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(
                    Styles::default(),
                    Content::Label(String::from("Department")),
                ),
                Cell::new(Styles::default(), Content::Label(String::from("Budget"))),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(Styles::default(), Content::Label(String::from("Sales"))),
                Cell::new(Styles::default(), Content::Label(String::from("90000"))),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(
                    Styles::default(),
                    Content::Label(String::from("Engineering")),
                ),
                Cell::new(Styles::default(), Content::Label(String::from("270000"))),
            ],
        ));

    let renderer = Console::default();
    println!("{}", renderer.render(&table));
}
