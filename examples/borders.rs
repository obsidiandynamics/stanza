use stanza::model::{Cell, Col, Row, Table};
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MinWidth, Separator, StyleKind, Styles};
use std::mem;

fn main() {
    const NUM_ROWS: usize = 4;
    const EXAMPLES: &[(&str, fn() -> Table); 17] = &[
        ("cross_headers", || cross_headers()),
        ("outer_headers", || outer_headers()),
        ("inner_row_separator", || inner_row_separator()),
        ("inner_col_separator", || inner_col_separator()),
        ("cross_separators", || cross_separators()),
        ("window_separators", || window_separators()),
        ("grid_separators", || grid_separators()),
        ("grid_seps_outer_headers", || {
            grid_separators_outer_headers()
        }),
        ("grid_seps_cross_headers", || grid_seps_cross_headers()),
        ("outer_row_separators", || outer_row_separators()),
        ("outer_col_separators", || outer_col_separators()),
        ("dual_row_separators", || dual_row_separators()),
        ("dual_col_separators", || dual_col_separators()),
        ("single_row", || single_row()),
        ("single_col", || single_col()),
        ("single_row_separator", || single_row_separator()),
        ("single_col_separator", || single_col_separator()),
    ];
    const NUM_COLS: usize = div_ceil(EXAMPLES.len(), NUM_ROWS);

    let mut outer_table = Table::default();
    let cols = (0..NUM_COLS)
        .into_iter()
        .map(|_| Col::new(Styles::default().with(StyleKind::HAlign(HAlign::Centred))))
        .collect();
    outer_table.set_cols(cols);

    let renderer = Console::default();
    let mut row = Vec::new();
    for (i, example) in EXAMPLES.iter().enumerate() {
        let inner_table = example.1();
        let name = example.0;
        let rendered = renderer.render(&inner_table).to_string();
        let cell_data = Cell::from(format!("{}) {name}\n{rendered}", i + 1));
        row.push(cell_data);
        if i % NUM_COLS == NUM_COLS - 1 {
            let current_row = mem::replace(&mut row, Vec::new());
            outer_table.push_row(Row::new(Styles::default(), current_row));
        }
    }

    if !row.is_empty() {
        outer_table.push_row(Row::new(Styles::default(), row));
    }

    print!("{}", Console::default().render(&outer_table));
}

const fn div_ceil(a: usize, b: usize) -> usize {
    let remainder = a % b;
    let quotient = a / b;
    if remainder == 0 {
        quotient
    } else {
        quotient + 1
    }
}

fn cross_headers() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Header(Header(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Header(Header(true))),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
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

fn outer_headers() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::Header(Header(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Header(Header(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default().with(StyleKind::Header(Header(true))),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Header(Header(true))),
            vec![
                Cell::from("SW"),
                Cell::from(""),
                Cell::from("S"),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ))
}

fn inner_row_separator() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
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

fn inner_col_separator() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("SW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ))
}

fn cross_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("SW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ))
}

fn window_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("NE"),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from("SW"),
                Cell::from(""),
                Cell::from("SE"),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
}

fn grid_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
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

fn grid_separators_outer_headers() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::Header(Header(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Header(Header(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default().with(StyleKind::Header(Header(true))),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Header(Header(true))),
            vec![
                Cell::from("SW"),
                Cell::from(""),
                Cell::from("S"),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ))
}

fn grid_seps_cross_headers() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Header(Header(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Header(Header(true))),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
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

fn outer_row_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("WNW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("ENE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("WSW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from("ESE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
}

fn outer_col_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from("NNW"),
                Cell::from("NNE"),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from("SSW"),
                Cell::from("SSE"),
                Cell::from(""),
            ],
        ))
}

fn dual_row_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
        .with_row(Row::new(
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

fn dual_col_separators() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::Separator(Separator(true)))
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                Cell::from("SW"),
                Cell::from(""),
                Cell::from(""),
                Cell::from("SE"),
            ],
        ))
}

fn single_row() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default(),
            vec![Cell::from("x"), Cell::from("y"), Cell::from("z")],
        ))
}

fn single_col() -> Table {
    Table::default()
        .with_cols(vec![Col::new(
            Styles::default()
                .with(StyleKind::MinWidth(MinWidth(5)))
                .with(StyleKind::HAlign(HAlign::Centred)),
        )])
        .with_row(Row::new(Styles::default(), vec![Cell::from("x")]))
        .with_row(Row::new(Styles::default(), vec![Cell::from("y")]))
        .with_row(Row::new(Styles::default(), vec![Cell::from("z")]))
}

fn single_row_separator() -> Table {
    Table::default()
        .with_cols(vec![
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
            Col::new(
                Styles::default()
                    .with(StyleKind::MinWidth(MinWidth(5)))
                    .with(StyleKind::HAlign(HAlign::Centred)),
            ),
        ])
        .with_row(Row::new(
            Styles::default().with(StyleKind::Separator(Separator(true))),
            vec![],
        ))
}

fn single_col_separator() -> Table {
    Table::default()
        .with_cols(vec![Col::new(
            Styles::default()
                .with(StyleKind::Separator(Separator(true)))
                .with(StyleKind::MinWidth(MinWidth(5)))
                .with(StyleKind::HAlign(HAlign::Centred)),
        )])
        .with_row(Row::new(Styles::default(), vec![Cell::from("")]))
        .with_row(Row::new(Styles::default(), vec![Cell::from("")]))
        .with_row(Row::new(Styles::default(), vec![Cell::from("")]))
}
