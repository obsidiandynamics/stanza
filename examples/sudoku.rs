use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, MinWidth, Styles};
use stanza::table::{Col, Row, Table};

fn main() {
    let table = Table::with_styles(Styles::default().with(MinWidth(3)).with(HAlign::Centred))
        .with_cols(vec![
            Col::default(),
            Col::default(),
            Col::default(),
            Col::separator(),
            Col::default(),
            Col::default(),
            Col::default(),
            Col::separator(),
            Col::default(),
            Col::default(),
            Col::default(),
        ])
        .with_row(["5", "3", " ", "", " ", "7", " ", "", " ", " ", " "])
        .with_row(["6", " ", " ", "", "1", "9", "5", "", " ", " ", " "])
        .with_row([" ", "9", "8", "", " ", " ", " ", "", " ", "6", " "])
        .with_row(Row::separator())
        .with_row(["8", " ", " ", "", " ", "6", " ", "", " ", " ", "3"])
        .with_row(["4", " ", " ", "", "8", " ", "3", "", " ", " ", "1"])
        .with_row(["7", " ", " ", "", " ", "2", " ", "", " ", " ", "6"])
        .with_row(Row::separator())
        .with_row([" ", "6", " ", "", " ", " ", " ", "", " ", "2", "8"])
        .with_row([" ", " ", " ", "", "4", "1", "9", "", " ", " ", "5"])
        .with_row([" ", " ", " ", "", " ", "8", " ", "", " ", "7", "9"]);

    println!("{}", Console::default().render(&table));
}