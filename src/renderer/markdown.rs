use crate::model::Table;
use crate::renderer::{pad, Renderer, NEWLINE};
use crate::style::{HAlignment, KEY_H_ALIGN, Style};

#[derive(Default)]
pub struct Markdown();

impl Renderer for Markdown {
    type Output = String;

    fn render(&self, table: &Table) -> Self::Output {
        const DEF_ALIGNMENT: Style = Style::HAlign(HAlignment::Left);
        assert!(!table.is_empty(), "Table cannot be empty");
        let col_widths = table.col_widths();

        let mut buf = String::new();

        // print the header
        buf.push('|');
        for col in 0..col_widths.len() {
            let cell = table.cell(col, 0);
            let data = cell.as_ref().map(|cell| cell.data()).unwrap_or("");
            let data = pad(data, ' ', col_widths[col], &HAlignment::Centred);
            buf.push_str(&data);
            buf.push('|');
        }
        buf.push_str(NEWLINE);

        // print the line between the header and the body
        buf.push('|');
        for (col, width) in col_widths.iter().enumerate() {
            let col = table.col(col).unwrap();
            let styles = col.combined_styles();
            let alignment = styles.get(KEY_H_ALIGN).unwrap_or(&DEF_ALIGNMENT);
            buf.push_str(&pad(":", '-', *width, alignment.as_h_align().unwrap()));
            buf.push('|');
        }
        buf.push_str(NEWLINE);

        // print the body
        for row in 1..table.num_rows() {
            buf.push('|');
            for col in 0..col_widths.len() {
                let cell = table.cell(col, row);
                let data = cell.as_ref().map(|cell| cell.data()).unwrap_or("");
                let alignment = cell.as_ref().map(|cell| {
                    cell.styles()
                        .get(KEY_H_ALIGN)
                        .unwrap_or(&DEF_ALIGNMENT)
                }).unwrap_or(&DEF_ALIGNMENT);
                let data = pad(data, ' ', col_widths[col], alignment.as_h_align().unwrap());
                buf.push_str(&data);
                buf.push('|');
            }
            buf.push_str(NEWLINE);
        }

        buf
    }
}
