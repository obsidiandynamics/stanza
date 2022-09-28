use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, MaxWidth, MinWidth, StyleKind, Styles};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::Header(Styles::default().with(StyleKind::MinWidth(MinWidth(20))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(20))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(20))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(20))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Header(Styles::default().with(StyleKind::MinWidth(MinWidth(20))).with(StyleKind::HAlign(HAlign::Centred))),
        ])
        .with_row(Row::Header(
            Styles::default(),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("X"),
            ],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from("A"),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::Header(
            Styles::default(),
            vec![
                Cell::from("Y"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("Z"),
            ],
        ));
    println!("{}", Console::default().render(&table));
}
