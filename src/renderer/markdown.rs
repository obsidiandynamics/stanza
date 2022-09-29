use crate::model::Table;
use crate::renderer::{pad, wrap, Renderer, NEWLINE};
use crate::style::{HAlign, Style};

#[derive(Default)]
pub struct Markdown();

impl Renderer for Markdown {
    type Output = String;

    fn render(&self, table: &Table) -> Self::Output {
        assert!(!table.is_empty(), "Table cannot be empty");
        let col_widths = table.col_widths();
        let mut buf = String::new();

        // print the header
        print_row(table, &col_widths, 0, &mut buf);

        // print the line between the header and the body
        print_header_format(table, &col_widths, &mut buf);

        // print the body
        for row in 1..table.num_rows() {
            print_row(table, &col_widths, row, &mut buf);
        }

        buf
    }
}

const DEF_ALIGNMENT: HAlign = HAlign::default();

fn print_header_format(table: &Table, col_widths: &[usize], buf: &mut String) {
    buf.push('|');
    for (col, &width) in col_widths.iter().enumerate() {
        let col = table.col(col).unwrap();
        let styles = col.combined_styles();
        let alignment = HAlign::resolve(&styles).unwrap_or(&DEF_ALIGNMENT);
        match alignment {
            HAlign::Left | HAlign::Right => {
                // the smallest format is `-:` or `:-`; i.e., no fewer than 2 characters wide
                let width = usize::max(2, width);
                buf.push_str(&pad(":", '-', width, alignment));
            }
            _ => {
                // the smallest format is `:-:`; i.e., no fewer than 3 characters wide
                let width = usize::max(3, width);
                buf.push(':');
                (2..width).for_each(|_| buf.push('-'));
                buf.push(':');
            }
        }
        buf.push('|');
    }
    buf.push_str(NEWLINE);
}

fn print_row(table: &Table, col_widths: &[usize], row: usize, buf: &mut String) {
    // first pass: wrap individual cell data over multiple rows
    let cell_lines = (0..col_widths.len())
        .into_iter()
        .map(|col| {
            let cell = table.cell(col, row);
            let data = cell.as_ref().map(|cell| cell.data()).unwrap_or("");
            wrap(data, col_widths[col])
        })
        .collect::<Vec<_>>();

    // second pass: obtain the combined styles of each cell
    let cell_styles = (0..col_widths.len())
        .into_iter()
        .map(|col| {
            let cell = table.cell(col, row);
            cell
                .map(|cell| cell.combined_styles())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    // third pass: render each line in the row
    let max_lines = cell_lines.iter().map(|lines| lines.len()).max().unwrap();
    for line in 0..max_lines {
        buf.push('|');
        for col in 0..col_widths.len() {
            let line = cell_lines[col].get(line).map(|line| &line[..]).unwrap_or("");
            let styles = &cell_styles[col];
            let alignment = HAlign::resolve(styles).unwrap_or(&DEF_ALIGNMENT);
            let line = pad(line, ' ', col_widths[col], &alignment);
            buf.push_str(&line);
            buf.push('|');
        }
        buf.push_str(NEWLINE);
    }
}