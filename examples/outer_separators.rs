use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, MinWidth, StyleKind, Styles};

fn main() {
    let table = Table::default()
        .with_cols(vec![
            Col::Body(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(20)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::Body(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(20)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::Body(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(20)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::Body(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(20)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::Body(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(20)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::Separator(
            Styles::default(),
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::Body(
            Styles::default(),
            vec![
                Cell::from("SW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ))
        .with_row(Row::Separator(
            Styles::default(),
        ));
    println!("{}", Console::default().render(&table));
}
