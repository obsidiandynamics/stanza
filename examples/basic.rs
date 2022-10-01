use stanza::renderer::console::{Console, Decor};
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{
    Bg16, Blink, Bold, Fg16, HAlign, Header, Italic, MaxWidth, MinWidth, Separator, Strikethrough,
    Styles, Underline,
};
use stanza::table::{Cell, Col, Row, Table};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::new(Styles::default().with(HAlign::Left).with(MaxWidth(40))),
            Col::new(Styles::default().with(MinWidth(20)).with(HAlign::Centred)),
            Col::new(
                Styles::default()
                    .with(HAlign::Right)
                    .with(MinWidth(10))
                    .with(MaxWidth(20)),
            ),
            Col::new(Styles::default().with(Separator(true)).with(MinWidth(5))),
            Col::new(Styles::default().with(MinWidth(10)).with(MaxWidth(20))),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)).with(Bold(true)),
            vec![
                Cell::from("Department"),
                Cell::from("Personnel"),
                Cell::from("Training budget"),
                Cell::from(""),
                Cell::from("To-do list"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(Styles::default().with(Fg16::BrightGreen), "Sales"),
                Cell::from(39),
                Cell::from(300_000.0),
                Cell::from(""),
                Cell::new(
                    Styles::default()
                        .with(Strikethrough(true))
                        .with(Fg16::BrightBlack),
                    "Walk the dog",
                ),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(Styles::default().with(Fg16::BrightBlue), "Engineering"),
                Cell::from(117),
                Cell::from(1_150_000.0),
                Cell::from(""),
                Cell::new(
                    Styles::default()
                        .with(Strikethrough(true))
                        .with(Fg16::BrightBlack),
                    "Wash the car",
                ),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(Styles::default().with(Fg16::BrightCyan), "Manufacturing"),
                Cell::from(20),
                Cell::from(250_000),
                Cell::from(""),
                Cell::from("Buy groceries"),
            ],
        ))
        .with_row(Row::new(Styles::default().with(Separator(true)), vec![]))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(
                    Styles::default()
                        .with(Bold(true))
                        .with(Bg16::BrightRed)
                        .with(Blink(true)),
                    "WARNING",
                ),
                Cell::new(
                    Styles::default()
                        .with(Underline(true))
                        .with(Fg16::BrightYellow),
                    "Check oil temp",
                ),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::new(
                    Styles::default()
                        .with(Bold(true))
                        .with(Fg16::BrightRed)
                        .with(Blink(true)),
                    "Self destruct sequence initiated",
                ),
                Cell::from(Console(Decor::default().suppress_escape_codes()).render(&nested_table())),
                Cell::from(""),
                Cell::from(""),
                Cell::new(
                    Styles::default().with(Italic(true)),
                    "The quick brown fox jumped over the lazy dog.\nThat's all folks!",
                ),
            ],
        ));
    println!("{}", Console::default().render(&table));
    println!("{}", Markdown::default().render(&table));
}

fn nested_table() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(Styles::default()),
            Col::new(Styles::default()),
        ])
        .with_row(Row::new(Styles::default().with(Header(true)).with(Bold(true)), vec![Cell::from("Sensor"), Cell::from("Temperature")]))
        .with_row(Row::new(Styles::default(), vec![Cell::from("Oil"), Cell::from(95.4)]))
        .with_row(Row::new(Styles::default(), vec![Cell::from("Water"), Cell::from(57.1)]))
}
