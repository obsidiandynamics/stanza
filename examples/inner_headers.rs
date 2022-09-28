use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, MinWidth, StyleKind, Styles};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(5))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(5))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Header(Styles::default().with(StyleKind::MinWidth(MinWidth(5))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(5))).with(StyleKind::HAlign(HAlign::Centred))),
            Col::Body(Styles::default().with(StyleKind::MinWidth(MinWidth(5))).with(StyleKind::HAlign(HAlign::Centred))),
        ])
        .with_row(Row::Body(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
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
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
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
                Cell::from("SW"),
                Cell::from(""),
                Cell::from("S"),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ));
    println!("{}", Console::default().render(&table));
}
