use stanza::renderer::console::Console;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{Blink, Bold, BorderBg, FillBg, HAlign, Header, Italic, MaxWidth, MinWidth, Palette16, Separator, Strikethrough, Styles, TextBg, TextFg, Underline};
use stanza::table::{Cell, Col, Content, Row, Table};

fn main() {
    let table = Table::with_styles(
        Styles::default()
            .with(BorderBg(Palette16::Black))
            .with(FillBg(Palette16::Black)),
    )
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
            Cell::new(
                Styles::default().with(TextFg(Palette16::BrightGreen)),
                Content::from("Sales"),
            ),
            Cell::from(39),
            Cell::from(300_000.0),
            Cell::from(""),
            Cell::new(
                Styles::default()
                    .with(Strikethrough(true))
                    .with(TextFg(Palette16::BrightBlack)),
                Content::from("Walk the dog"),
            ),
        ],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default().with(TextFg(Palette16::BrightBlue)),
                Content::from("Engineering"),
            ),
            Cell::from(117),
            Cell::from(1_150_000.0),
            Cell::from(""),
            Cell::new(
                Styles::default()
                    .with(Strikethrough(true))
                    .with(TextFg(Palette16::BrightBlack)),
                Content::from("Wash the car"),
            ),
        ],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default().with(TextFg(Palette16::BrightCyan)),
                Content::from("Manufacturing"),
            ),
            Cell::from(20),
            Cell::from(250_000),
            Cell::from(""),
            Cell::from("Buy groceries"),
        ],
    ))
    .with_row(Row::separator())
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default()
                    .with(Bold(true))
                    .with(TextBg(Palette16::BrightRed))
                    .with(Blink(true)),
                Content::from("WARNING"),
            ),
            Cell::new(
                Styles::default()
                    .with(Underline(true))
                    .with(TextFg(Palette16::BrightYellow)),
                Content::from("Check oil temp"),
            ),
            Cell::new(
                Styles::default()
                    .with(FillBg(Palette16::BrightMagenta))
                    .with(Italic(true)),
                Content::Computed(Box::new(|| "fill".into())), // deferred computation
            ),
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
                    .with(TextFg(Palette16::BrightRed))
                    .with(Blink(true)),
                Content::from("Self destruct sequence initiated"),
            ),
            Cell::from(nested_table()),
            Cell::from(""),
            Cell::from(""),
            Cell::new(
                Styles::default().with(Italic(true)),
                Content::from("The quick brown fox jumped over the lazy dog.\n\nThat's all folks!"),
            ),
        ],
    ));
    println!("{}", Console::default().render(&table, &[]));
    println!("{}", Markdown::default().render(&table, &[]));
}

fn nested_table() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(Styles::default()),
            Col::new(Styles::default()),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)).with(Bold(true)),
            vec![Cell::from("Sensor"), Cell::from("Temperature")],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![Cell::from("Oil"), Cell::from(95.4)],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![Cell::from("Water"), Cell::from(57.1)],
        ))
}
