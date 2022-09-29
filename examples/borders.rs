use std::mem;
use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, MinWidth, StyleKind, Styles};

fn main() {
    const NUM_ROWS: usize = 4;
    const EXAMPLES: &[(&str, fn() -> Table); 1] = &[
        ("inner_headers", || inner_headers())
    ];
    const NUM_COLS: usize = div_ceil(EXAMPLES.len(), NUM_ROWS);

    let mut outer_table = Table::default();
    let cols = (0..NUM_COLS).into_iter().map(|_| Col::Body(Styles::default().with(StyleKind::HAlign(HAlign::Centred)))).collect();
    outer_table.set_cols(cols);

    let renderer = Console::default();
    let mut row = Vec::new();
    for (i, example) in EXAMPLES.iter().enumerate() {
        let inner_table = example.1();
        let name = example.0;
        let rendered = renderer.render(&inner_table).to_string();
        let cell_data = Cell::from(format!("{name}\n{rendered}"));
        row.push(cell_data);
        if i == NUM_COLS - 1 {
            let current_row = mem::replace(&mut row, Vec::new());
            outer_table.push_row(Row::Body(Styles::default(), current_row));
        }
    }

    if !row.is_empty() {
        outer_table.push_row(Row::Body(Styles::default(), row));
    }

    println!("{}", Console::default().render(&outer_table));
}

const fn div_ceil(a: usize, b: usize) -> usize {
    let remainder = a % b;
    let quotient = a / b;
    if remainder == 0 { quotient } else { quotient + 1}
}

fn inner_headers() -> Table {
    Table::default()
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
        ))
}