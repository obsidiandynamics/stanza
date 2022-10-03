use stanza::renderer::console::Console;
use stanza::renderer::{RenderHint, Renderer};
use stanza::style::{BorderFg, HAlign, Header, MinWidth, Palette16, Styles};
use stanza::table::{Cell, Col, Content, Row, Table};
use std::mem;

fn main() {
    let examples = vec![
        ("cross_headers", cross_headers()),
        ("outer_headers", outer_headers()),
        ("inner_row_separator", inner_row_separator()),
        ("inner_col_separator", inner_col_separator()),
        ("cross_separators", cross_separators()),
        ("window_separators", window_separators()),
        ("grid_separators", grid_separators()),
        ("grid_seps_outer_headers", {
            grid_separators_outer_headers()
        }),
        ("grid_seps_cross_headers", grid_seps_cross_headers()),
        ("outer_row_separators", outer_row_separators()),
        ("outer_col_separators", outer_col_separators()),
        ("dual_row_separators", dual_row_separators()),
        ("dual_col_separators", dual_col_separators()),
        ("single_row", single_row()),
        ("single_col", single_col()),
        ("single_row_separator", single_row_separator()),
        ("single_col_separator", single_col_separator()),
        ("two_by_two_up", two_by_two_up()),
        ("two_by_two_right", two_by_two_right()),
        ("two_by_two_down", two_by_two_down()),
        ("two_by_two_left", two_by_two_left()),
    ];

    let renderer = Console::default();
    let term_width = term_width();
    let col_width = widest_table(&examples, &renderer) + 1;
    let num_cols = (term_width - 1) / col_width;
    let mut outer_table =
        Table::with_styles(Styles::default().with(BorderFg(Palette16::BrightBlack)));
    let cols = (0..num_cols)
        .into_iter()
        .map(|_| Col::new(Styles::default().with(HAlign::Centred)))
        .collect();
    outer_table.set_cols(cols);

    let mut row = Vec::new();
    for (i, example) in examples.into_iter().enumerate() {
        let inner_table = example.1;
        let name = example.0;
        let title = format!("{}) {name}\n", i + 1);
        let cell_data = Cell::from(Content::Composite(vec![
            Content::Label(title),
            Content::Nested(inner_table),
        ]));
        row.push(cell_data);
        if i % num_cols == num_cols - 1 {
            let current_row = mem::replace(&mut row, Vec::new());
            outer_table.push_row(Row::new(Styles::default(), current_row));
        }
    }

    if !row.is_empty() {
        outer_table.push_row(Row::new(Styles::default(), row));
    }

    print!("{}", renderer.render(&outer_table, &[]));
}

fn widest_table(tables: &[(&str, Table)], renderer: &impl Renderer) -> usize {
    tables
        .iter()
        .map(|(_, table)| table_width(table, renderer))
        .max()
        .unwrap_or(0)
}

fn table_width(table: &Table, renderer: &impl Renderer) -> usize {
    let rendered = renderer.render(table, &[RenderHint::Nested]);
    format!("{rendered}")
        .lines()
        .into_iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

fn term_width() -> usize {
    term_size::dimensions().unwrap().0
}

fn table_styles() -> Styles { Styles::default().with(MinWidth(5)).with(HAlign::Centred) }

fn cross_headers() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::default(),
            Col::new(Styles::default().with(Header(true))),
            Col::default(),
            Col::default(),
        ])
        .with_row(Row::from(["NW", "", "N", "", "NE"]))
        .with_row(Row::default())
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::default())
        .with_row(Row::from(["SW", "", "S", "", "SE"]))
}

fn outer_headers() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::new(Styles::default().with(Header(true))),
            Col::default(),
            Col::default(),
            Col::default(),
            Col::new(Styles::default().with(Header(true))),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::default())
        .with_row(Row::from(["W", "", "C", "", "E"]))
        .with_row(Row::default())
        .with_row(Row::new(
            Styles::default().with(Header(true)),
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
    Table::with_styles(table_styles())
        .with_row(Row::from(["NW", "", "N", "", "NE"]))
        .with_row(Row::default())
        .with_row(Row::separator())
        .with_row(Row::default())
        .with_row(Row::from(["SW", "", "S", "", "SE"]))
}

fn inner_col_separator() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::default(),
            Col::separator(),
            Col::default(),
            Col::default(),
        ])
        .with_row(Row::from(["NW", "", "", "", "NE"]))
        .with_row(Row::default())
        .with_row(Row::from(["W", "", "", "", "E"]))
        .with_row(Row::default())
        .with_row(Row::from(["SW", "", "", "", "SE"]))
}

fn cross_separators() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::default(),
            Col::separator(),
            Col::default(),
            Col::default(),
        ])
        .with_row(Row::from(["NW", "", "", "", "NE"]))
        .with_row(Row::default())
        .with_row(Row::separator())
        .with_row(Row::default())
        .with_row(Row::from(["SW", "", "", "", "SE"]))
}

fn window_separators() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::separator(),
            Col::default(),
            Col::separator(),
            Col::default(),
            Col::separator(),
        ])
        .with_row(Row::separator())
        .with_row(Row::from(["", "NW", "", "NE", ""]))
        .with_row(Row::separator())
        .with_row(Row::from(["", "SW", "", "SE", ""]))
        .with_row(Row::separator())
}

fn grid_separators() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::separator(),
            Col::default(),
            Col::separator(),
            Col::default(),
        ])
        .with_row(Row::from(["NW", "", "N", "", "NE"]))
        .with_row(Row::separator())
        .with_row(Row::from(["W", "", "C", "", "E"]))
        .with_row(Row::separator())
        .with_row(Row::from(["SW", "", "S", "", "SE"]))
}

fn grid_separators_outer_headers() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::new(Styles::default().with(Header(true))),
            Col::separator(),
            Col::default(),
            Col::separator(),
            Col::new(Styles::default().with(Header(true))),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![
                Cell::from("NW"),
                Cell::from(""),
                Cell::from("N"),
                Cell::from(""),
                Cell::from("NE"),
            ],
        ))
        .with_row(Row::separator())
        .with_row(Row::from(["W", "", "C", "", "E"]))
        .with_row(Row::separator())
        .with_row(Row::new(
            Styles::default().with(Header(true)),
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
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::separator(),
            Col::new(Styles::default().with(Header(true))),
            Col::separator(),
            Col::default(),
        ])
        .with_row(Row::from(["NW", "", "N", "", "NE"]))
        .with_row(Row::separator())
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![
                Cell::from("W"),
                Cell::from(""),
                Cell::from("C"),
                Cell::from(""),
                Cell::from("E"),
            ],
        ))
        .with_row(Row::separator())
        .with_row(Row::from(["SW", "", "S", "", "SE"]))
}

fn outer_row_separators() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::default(),
            Col::default(),
            Col::default(),
            Col::default(),
        ])
        .with_row(Row::separator())
        .with_row(Row::from(["WNW", "", "", "", "ENE"]))
        .with_row(Row::from(["WSW", "", "", "", "ESE"]))
        .with_row(Row::separator())
}

fn outer_col_separators() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::separator(),
            Col::default(),
            Col::default(),
            Col::separator(),
        ])
        .with_row(Row::from(["", "NNW", "NNE", ""]))
        .with_row(Row::default())
        .with_row(Row::default())
        .with_row(Row::default())
        .with_row(Row::from(["", "SSW", "SSE", ""]))
}

fn dual_row_separators() -> Table {
    Table::with_styles(table_styles())
        .with_row(Row::from(["NW", "", "N", "", "NE"]))
        .with_row(Row::separator())
        .with_row(Row::separator())
        .with_row(Row::from(["SW", "", "S", "", "SE"]))
}

fn dual_col_separators() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::separator(),
            Col::separator(),
            Col::default(),
        ])
        .with_row(Row::from(["NW", "", "", "NE"]))
        .with_row(Row::default())
        .with_row(Row::from(["W", "", "", "E"]))
        .with_row(Row::default())
        .with_row(Row::from(["SW", "", "", "EE"]))
}

fn single_row() -> Table {
    Table::with_styles(table_styles())
        .with_row(Row::new(
            Styles::default(),
            vec![Cell::from("x"), Cell::from("y"), Cell::from("z")],
        ))
}

fn single_col() -> Table {
    Table::with_styles(table_styles())
        .with_row(Row::from(["x"]))
        .with_row(Row::from(["y"]))
        .with_row(Row::from(["z"]))
}

fn single_row_separator() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::default(),
            Col::default(),
        ])
        .with_row(Row::separator())
}

fn single_col_separator() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![Col::separator()])
        .with_row(Row::default())
        .with_row(Row::default())
        .with_row(Row::default())
}

fn two_by_two_up() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![Col::separator(), Col::separator()])
        .with_row(Row::from(["NW", "NE"]))
        .with_row(Row::separator())
}

fn two_by_two_right() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::separator(),
            Col::default(),
        ])
        .with_row(Row::separator())
        .with_row(Row::separator())
}

fn two_by_two_down() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![Col::separator(), Col::separator()])
        .with_row(Row::separator())
        .with_row(Row::from(["SW", "SE"]))
}

fn two_by_two_left() -> Table {
    Table::with_styles(table_styles())
        .with_cols(vec![
            Col::default(),
            Col::separator(),
        ])
        .with_row(Row::separator())
        .with_row(Row::separator())
}
