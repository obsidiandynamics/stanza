use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MinWidth, Styles};
use stanza::table::{Col, Row, Table};

fn main() {
    const NUMS: i8 = 6; // the table will multiply from 1 to NUMS

    // builds just the header row -- there should be two (top and bottom)
    fn build_header_row() -> Row {
        let mut cells = vec!["".into()];
        for col in 1..=NUMS {
            cells.push(col.into());
        }
        cells.push("".into());
        Row::new(Styles::default().with(Header(true)), cells)
    }

    // builds all body rows (each row also contains a pair column header cells)
    fn build_body_rows() -> Vec<Row> {
        (1..=NUMS)
            .map(|row| {
                let mut cells = vec![row.into()];
                for col in 1..=NUMS {
                    cells.push((row * col).into());
                }
                cells.push(row.into());
                Row::new(Styles::default(), cells)
            })
            .collect()
    }

    let mut table = Table::with_styles(
        Styles::default()
            .with(HAlign::Right) // numbers should be right-aligned
            .with(MinWidth(5)), // give each column some space
    )
    .with_cols(
        (0..NUMS + 2)
            .map(|col| {
                Col::new(
                    // only the first and last columns are headers
                    Styles::default().with(Header(col == 0 || col == NUMS + 1)),
                )
            })
            .collect(),
    );

    table.push_row(build_header_row());
    table.push_rows(build_body_rows());
    table.push_row(build_header_row());

    println!("{}", Console::default().render(&table));
}
