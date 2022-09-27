use crate::model::Table;
use crate::renderer::{NEWLINE, pad, Renderer};

#[derive(Default)]
pub struct Markdown();

impl Renderer for Markdown {
    type Output = String;

    fn render(&self, table: &Table) -> Self::Output {
        assert!(!table.is_empty(), "Table cannot be empty");
        let col_widths = table.col_widths();

        let mut buf = String::new();

        // print the header
        buf.push('|');
        for col in 0..col_widths.len() {
            let cell = table.cell(col, 0);
            let data = cell.map(|cell| cell.data()).unwrap_or("");
            let data = pad(data, ' ', col_widths[col]);
            buf.push_str(&data);
            buf.push('|');
        }
        buf.push_str(NEWLINE);

        // print the line between the header and the body
        buf.push('|');
        for &width in col_widths.iter() {
            buf.push_str(&pad(":", '-', width));
            buf.push('|');
        }
        buf.push_str(NEWLINE);

        // print the body
        for row in 1..table.num_rows() {
            buf.push('|');
            for col in 0..col_widths.len() {
                let cell = table.cell(col, row);
                let data = cell.map(|cell| cell.data()).unwrap_or("");
                let data = pad(data, ' ', col_widths[col]);
                buf.push_str(&data);
                buf.push('|');
            }
            buf.push_str(NEWLINE);
        }

        buf
    }
}

// pub struct MarkdownOutput<'a>(&'a Table);