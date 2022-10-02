use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use crate::table::Table;
use crate::renderer::{pad, wrap, Renderer, NEWLINE, RenderHint};
use crate::style::{HAlign, Style};

#[derive(Default)]
pub struct Markdown();

impl Renderer for Markdown {
    type Output = String;

    fn render(&self, table: &Table, _: &[RenderHint]) -> Self::Output {
        assert!(!table.is_empty(), "table cannot be empty");
        let col_widths = table.col_widths(self);
        let mut buf = String::new();

        // print the header
        print_row(self, table, &col_widths, 0, &mut buf);

        // print the line between the header and the body
        print_header_format(table, &col_widths, &mut buf);

        // print the body
        for row in 1..table.num_rows() {
            print_row(self, table, &col_widths, row, &mut buf);
        }

        buf
    }
}

fn print_header_format(table: &Table, col_widths: &[usize], buf: &mut String) {
    buf.push('|');
    for (col, &width) in col_widths.iter().enumerate() {
        let col = table.col(col);
        let styles = col.blended_styles();
        let alignment = HAlign::resolve_or_default(&styles);
        let alignment = alignment.borrow();
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

fn print_row(renderer: &Markdown, table: &Table, col_widths: &[usize], row: usize, buf: &mut String) {
    // first pass: wrap individual cell data over multiple rows
    let cell_lines = (0..col_widths.len())
        .into_iter()
        .map(|col| {
            let cell = table.cell(col, row);
            let data = cell.as_ref().map(|cell| cell.data().render(renderer)).unwrap_or(Cow::Borrowed(""));
            wrap(&data, col_widths[col])
        })
        .collect::<Vec<_>>();

    // second pass: obtain the combined styles of each cell
    let cell_styles = (0..col_widths.len())
        .into_iter()
        .map(|col| {
            let cell = table.cell(col, row);
            cell.blended_styles()
        })
        .collect::<Vec<_>>();

    // third pass: render each line in the row
    let max_lines = cell_lines.iter().map(|lines| lines.len()).max().unwrap();
    for line in 0..max_lines {
        buf.push('|');
        for col in 0..col_widths.len() {
            let line = cell_lines[col].get(line).map(|line| &line[..]).unwrap_or("");
            let styles = &cell_styles[col];
            let alignment = HAlign::resolve_or_default(styles);
            let line = pad(line, ' ', col_widths[col], &alignment);
            buf.push_str(&line);
            buf.push('|');
        }
        buf.push_str(NEWLINE);
    }
}