use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, MinWidth, Separator, Styles};
use stanza::table::{Col, Content, Row, Table};

fn main() {
    let table = Table::with_styles(Styles::default().with(HAlign::Centred))
        .with_cols(vec![
            Col::default(),
            Col::new(Styles::default().with(Separator(true)).with(MinWidth(5))),
            Col::default(),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Content::Composite(vec![
                    "Sensor temps\n".into(),
                    Table::default()
                        .with_row(Row::from(["Water", "19.3"]))
                        .with_row(Row::from(["Oil", "65.1"]))
                        .into(),
                ])
                .into(),
                "".into(),
                Content::Composite(vec![
                    "Stock prices\n".into(),
                    Table::default()
                        .with_row(Row::from(["AAPL", "138.20"]))
                        .with_row(Row::from(["AMZN", "113.20"]))
                        .with_row(Row::from(["IBM", "118.81"]))
                        .into(),
                ])
                .into(),
            ],
        ));

    println!("{}", Console::default().render(&table, &[]));
}
