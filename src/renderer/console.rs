use crate::model::{Table};
use crate::renderer::{pad, wrap, Renderer, NEWLINE};
use crate::style::{HAlign, Style, Styles};

pub struct Decor {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    bold_vertical: char,
    bold_horizontal: char,
    norm_vertical: char,
    norm_horizontal: char,
    norm_horizontal_norm_vertical: char,
    bold_horizontal_norm_vertical: char,
    norm_horizontal_bold_vertical: char,
    bold_horizontal_bold_vertical: char,
    norm_up_bold_horizontal: char,
    norm_down_bold_horizontal: char,
    bold_up_bold_horizontal: char,
    bold_down_bold_horizontal: char,
    norm_left_bold_vertical: char,
    norm_right_bold_vertical: char,
    bold_left_bold_vertical: char,
    bold_right_bold_vertical: char,
}

impl Default for Decor {
    fn default() -> Self {
        Self::double_outline()
    }
}

impl Decor {
    pub fn double_outline() -> Self {
        Self {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            bold_vertical: '║',
            bold_horizontal: '═',
            norm_vertical: '│',
            norm_horizontal: '─',
            norm_horizontal_norm_vertical: '┼',
            bold_horizontal_norm_vertical: '╪',
            norm_horizontal_bold_vertical: '╫',
            bold_horizontal_bold_vertical: '╬',
            norm_up_bold_horizontal: '╧',
            norm_down_bold_horizontal: '╤',
            bold_up_bold_horizontal: '╩',
            bold_down_bold_horizontal: '╦',
            norm_left_bold_vertical: '╢',
            norm_right_bold_vertical: '╟',
            bold_left_bold_vertical: '╣',
            bold_right_bold_vertical: '╠'
        }
    }
}

#[derive(Default)]
pub struct Console(pub Decor);

impl Renderer for Console {
    type Output = String;

    fn render(&self, table: &Table) -> Self::Output {
        assert!(!table.is_empty(), "Table cannot be empty");
        let col_widths = table.col_widths();
        let decor = &self.0;
        let grid = pre_render(table, &col_widths);
        let mut buf = String::new();

        let is_header_col = |col| table.col(col).unwrap().is_header() || table.col(col + 1).unwrap().is_header();
        let is_header_row = |row| table.row(row).unwrap().is_header() || table.row(row + 1).unwrap().is_header();

        // upper outside border
        buf.push(decor.top_left);
        for (col, &width) in col_widths.iter().enumerate() {
            (0..width).for_each(|_| buf.push(decor.bold_horizontal));
            if col < col_widths.len() - 1 {
                let border = if is_header_col(col) { decor.bold_down_bold_horizontal } else { decor.norm_down_bold_horizontal };
                buf.push(border);
            }
        }
        buf.push(decor.top_right);
        buf.push_str(NEWLINE);

        // table (incl. headers and body)
        for row in 0..table.num_rows() {
            let grid_row = &grid[row];
            let max_lines = grid_row.iter().map(|cell| cell.lines.len()).max().unwrap();

            // lines comprising the row
            for line in 0..max_lines {
                buf.push(decor.bold_vertical);
                for col in 0..col_widths.len() {
                    let grid_cell = &grid_row[col];
                    let line = grid_cell
                        .lines
                        .get(line)
                        .map(|line| &line[..])
                        .unwrap_or("");
                    let alignment = HAlign::resolve(&grid_cell.styles).unwrap_or(&DEF_ALIGNMENT);
                    let line = pad(line, ' ', col_widths[col], &alignment);
                    buf.push_str(&line);
                    if col < col_widths.len() - 1 {
                        let border = if is_header_col(col) { decor.bold_vertical } else { decor.norm_vertical };
                        buf.push(border);
                    }
                }
                buf.push(decor.bold_vertical);
                buf.push_str(NEWLINE);
            }

            // border below the row
            if row < table.num_rows() - 1 {
                let header_row = is_header_row(row);
                let border = if header_row { decor.bold_right_bold_vertical } else { decor.norm_right_bold_vertical };
                buf.push(border);
                for (col, &width) in col_widths.iter().enumerate() {
                    let border = if header_row { decor.bold_horizontal } else { decor.norm_horizontal };
                    (0..width).for_each(|_| buf.push(border));
                    if col < col_widths.len() - 1 {
                        let header_col = is_header_col(col);
                        let border = match (header_row, header_col) {
                            (true, true) => decor.bold_horizontal_bold_vertical,
                            (false, true) => decor.norm_horizontal_bold_vertical,
                            (true, false) => decor.bold_horizontal_norm_vertical,
                            (false, false) => decor.norm_horizontal_norm_vertical
                        };
                        buf.push(border);
                    }
                }
                let border = if header_row { decor.bold_left_bold_vertical } else { decor.norm_left_bold_vertical };
                buf.push(border);
                buf.push_str(NEWLINE);
            }
        }

        // lower outside border
        buf.push(decor.bottom_left);
        for (col, &width) in col_widths.iter().enumerate() {
            (0..width).for_each(|_| buf.push(decor.bold_horizontal));
            if col < col_widths.len() - 1 {
                let border = if is_header_col(col) { decor.bold_up_bold_horizontal } else { decor.norm_up_bold_horizontal };
                buf.push(border);
            }
        }
        buf.push(decor.bottom_right);
        buf.push_str(NEWLINE);

        buf
    }
}

fn pre_render(table: &Table, col_widths: &[usize]) -> Vec<Vec<PreCell>> {
    (0..table.num_rows())
        .into_iter()
        .map(|row| {
            (0..table.num_cols())
                .into_iter()
                .map(|col| {
                    let cell = table.cell(col, row);
                    let data = cell.as_ref().map(|cell| cell.data()).unwrap_or("");
                    let lines = wrap(data, col_widths[col]);
                    let styles = cell.map(|cell| cell.combined_styles()).unwrap_or_default();
                    PreCell { lines, styles }
                })
                .collect()
        })
        .collect()
}

struct PreCell {
    lines: Vec<String>,
    styles: Styles,
    // col_type: GroupType,
    // row_type: GroupType,
}

// enum GroupType {
//     Header,
//     Body,
//     Separator
// }
//
// impl Row {
//     fn group_type(row: &Row) -> GroupType {
//         match row {
//             Row::Header(_, _) => GroupType::Header,
//             Row::Body(_, _) => GroupType::Body,
//             Row::Separator(_) => GroupType::Separator
//         }
//     }
// }
//
// impl Col {
//     fn group_type(row: &Col) -> GroupType {
//         match row {
//             Col::Header(_) => GroupType::Header,
//             Col::Body(_) => GroupType::Body,
//             Col::Separator(_) => GroupType::Separator
//         }
//     }
// }

const DEF_ALIGNMENT: HAlign = HAlign::Left;

// fn print_header_format(table: &Table, col_widths: &[usize], buf: &mut String) {
//     buf.push('|');
//     for (col, &width) in col_widths.iter().enumerate() {
//         let col = table.col(col).unwrap();
//         let styles = col.combined_styles();
//         let alignment = HAlign::resolve(&styles).unwrap_or(&DEF_ALIGNMENT);
//         match alignment {
//             HAlign::Left | HAlign::Right => {
//                 // the smallest format is `-:` or `:-`; i.e., no fewer than 2 characters wide
//                 let width = usize::max(2, width);
//                 buf.push_str(&pad(":", '-', width, alignment));
//             }
//             _ => {
//                 // the smallest format is `:-:`; i.e., no fewer than 3 characters wide
//                 let width = usize::max(3, width);
//                 buf.push(':');
//                 (2..width).for_each(|_| buf.push('-'));
//                 buf.push(':');
//             }
//         }
//         buf.push('|');
//     }
//     buf.push_str(NEWLINE);
// }
//
// fn print_row(table: &Table, col_widths: &[usize], row: usize, buf: &mut String) {
//     // first pass: wrap individual cell data over multiple rows
//     let cell_lines = (0..col_widths.len())
//         .into_iter()
//         .map(|col| {
//             let cell = table.cell(col, row);
//             let data = cell.as_ref().map(|cell| cell.data()).unwrap_or("");
//             wrap(data, col_widths[col])
//         })
//         .collect::<Vec<_>>();
//
//     // second pass: obtain the combined styles of each cell
//     let cell_styles = (0..col_widths.len())
//         .into_iter()
//         .map(|col| {
//             let cell = table.cell(col, row);
//             cell.map(|cell| cell.combined_styles()).unwrap_or_default()
//         })
//         .collect::<Vec<_>>();
//
//     // third pass: render each line in the row
//     let max_lines = cell_lines.iter().map(|lines| lines.len()).max().unwrap();
//     for line in 0..max_lines {
//         buf.push('|');
//         for col in 0..col_widths.len() {
//             let line = cell_lines[col]
//                 .get(line)
//                 .map(|line| &line[..])
//                 .unwrap_or("");
//             let styles = &cell_styles[col];
//             let alignment = HAlign::resolve(styles).unwrap_or(&DEF_ALIGNMENT);
//             let line = pad(line, ' ', col_widths[col], &alignment);
//             buf.push_str(&line);
//             buf.push('|');
//         }
//         buf.push_str(NEWLINE);
//     }
// }
