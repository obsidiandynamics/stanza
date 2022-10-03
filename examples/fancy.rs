use stanza::renderer::console::Console;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{
    Blink, Bold, BorderBg, FillBg, HAlign, Header, Italic, MaxWidth, MinWidth, Palette16,
    Separator, Strikethrough, Styles, TextBg, TextFg, Underline,
};
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
            "Department".into(),
            "Personnel".into(),
            "Training budget".into(),
            "".into(),
            "To-do list".into(),
        ],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default().with(TextFg(Palette16::BrightGreen)),
                "Sales".into(),
            ),
            39.into(),
            300_000.0.into(),
            "".into(),
            Cell::new(
                Styles::default()
                    .with(Strikethrough(true))
                    .with(TextFg(Palette16::BrightBlack)),
                "Walk the dog".into(),
            ),
        ],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default().with(TextFg(Palette16::BrightBlue)),
                "Engineering".into(),
            ),
            117.into(),
            1_150_000.0.into(),
            "".into(),
            Cell::new(
                Styles::default()
                    .with(Strikethrough(true))
                    .with(TextFg(Palette16::BrightBlack)),
                "Wash the car".into(),
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
            20.into(),
            250_000.into(),
            "".into(),
            "Buy groceries".into(),
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
                "WARNING".into(),
            ),
            Cell::new(
                Styles::default()
                    .with(Underline(true))
                    .with(TextFg(Palette16::BrightYellow)),
                "Check oil temp".into(),
            ),
            Cell::new(
                Styles::default()
                    .with(FillBg(Palette16::BrightMagenta))
                    .with(Italic(true)),
                Content::Computed(Box::new(|| "fill".into())), // deferred computation
            ),
            "".into(),
            "".into(),
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
                "Self destruct sequence initiated".into(),
            ),
            nested_table().into(),
            "".into(),
            "".into(),
            Cell::new(
                Styles::default().with(Italic(true)),
                "The quick brown fox jumped over the lazy dog.\n\nThat's all folks!".into(),
            ),
        ],
    ));
    println!("{}", Console::default().render(&table, &[]));
    println!("{}", Markdown::default().render(&table, &[]));
}

fn nested_table() -> Table {
    Table::default()
        .with_row(Row::new(
            Styles::default().with(Header(true)).with(Bold(true)),
            vec!["Sensor".into(), "Temperature".into()],
        ))
        .with_row(Row::from(["Oil", "95.4"]))
        .with_row(Row::from(["Water", "57.1"]))
}
