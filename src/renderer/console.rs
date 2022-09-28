use crate::model::Table;
use crate::renderer::{pad, wrap, Renderer, NEWLINE};
use crate::style::{HAlign, Style, Styles};

pub struct Decor {
    blank: char,
    up_bold_down_bold: char,
    right_bold_left_bold: char,
    right_bold_down_bold: char,
    down_bold_left_bold: char,
    up_bold_right_bold: char,
    up_bold_left_bold: char,
    up_bold_right_norm_down_bold: char,
    up_bold_right_bold_down_bold: char,
    up_bold_down_bold_left_norm: char,
    up_bold_down_bold_left_bold: char,
    right_bold_down_norm_left_bold: char,
    right_bold_down_bold_left_bold: char,
    up_norm_right_bold_left_bold: char,
    up_bold_right_bold_left_bold: char,
    up_norm_right_bold_down_norm_left_bold: char,
    up_bold_right_norm_down_bold_left_norm: char,
    up_bold_right_bold_down_bold_left_bold: char,
    right_norm_left_norm: char,
    up_norm_down_norm: char,
    right_norm_down_norm_left_norm: char,
    up_norm_right_norm_left_norm: char,
    up_norm_right_norm_down_norm_left_norm: char,
}

impl Default for Decor {
    fn default() -> Self {
        Self::double_outline()
    }
}

impl Decor {
    pub fn double_outline() -> Self {
        Self {
            blank: ' ',
            right_bold_left_bold: '═',
            up_bold_down_bold: '║',
            right_bold_down_bold: '╔',
            down_bold_left_bold: '╗',
            up_bold_right_bold: '╚',
            up_bold_left_bold: '╝',
            up_bold_right_norm_down_bold: '╟',
            up_bold_right_bold_down_bold: '╠',
            up_bold_down_bold_left_norm: '╢',
            up_bold_down_bold_left_bold: '╣',
            right_bold_down_norm_left_bold: '╤',
            right_bold_down_bold_left_bold: '╦',
            up_norm_right_bold_left_bold: '╧',
            up_bold_right_bold_left_bold: '╩',
            up_norm_right_bold_down_norm_left_bold: '╪',
            up_bold_right_norm_down_bold_left_norm: '╫',
            up_bold_right_bold_down_bold_left_bold: '╬',
            right_norm_left_norm: '─',
            up_norm_down_norm: '│',
            right_norm_down_norm_left_norm: '┬',
            up_norm_right_norm_left_norm: '┴',
            up_norm_right_norm_down_norm_left_norm: '┼',
        }
    }

    fn lookup(&self, up: Line, right: Line, down: Line, left: Line) -> char {
        // dbg!(&up, &right, &down, &left);
        match (up, right, down, left) {
            (Line::None, Line::None, Line::None, Line::None) => self.blank,
            (Line::None, Line::Bold, Line::None, Line::Bold) => self.right_bold_left_bold,
            (Line::Bold, Line::None, Line::Bold, Line::None) => self.up_bold_down_bold,
            (Line::None, Line::Bold, Line::Bold, Line::None) => self.right_bold_down_bold,
            (Line::None, Line::None, Line::Bold, Line::Bold) => self.down_bold_left_bold,
            (Line::Bold, Line::Bold, Line::None, Line::None) => self.up_bold_right_bold,
            (Line::Bold, Line::None, Line::None, Line::Bold) => self.up_bold_left_bold,
            (Line::Bold, Line::Norm, Line::Bold, Line::None) => self.up_bold_right_norm_down_bold,
            (Line::Bold, Line::Bold, Line::Bold, Line::None) => self.up_bold_right_bold_down_bold,
            (Line::Bold, Line::None, Line::Bold, Line::Norm) => self.up_bold_down_bold_left_norm,
            (Line::Bold, Line::None, Line::Bold, Line::Bold) => self.up_bold_down_bold_left_bold,
            (Line::None, Line::Bold, Line::Norm, Line::Bold) => self.right_bold_down_norm_left_bold,
            (Line::None, Line::Bold, Line::Bold, Line::Bold) => self.right_bold_down_bold_left_bold,
            (Line::Norm, Line::Bold, Line::None, Line::Bold) => self.up_norm_right_bold_left_bold,
            (Line::Bold, Line::Bold, Line::None, Line::Bold) => self.up_bold_right_bold_left_bold,
            (Line::Norm, Line::Bold, Line::Norm, Line::Bold) => self.up_norm_right_bold_down_norm_left_bold,
            (Line::Bold, Line::Norm, Line::Bold, Line::Norm) => self.up_bold_right_norm_down_bold_left_norm,
            (Line::Bold, Line::Bold, Line::Bold, Line::Bold) => self.up_bold_right_bold_down_bold_left_bold,
            (Line::None, Line::Norm, Line::None, Line::Norm) => self.right_norm_left_norm,
            (Line::Norm, Line::None, Line::Norm, Line::None) => self.up_norm_down_norm,
            (Line::None, Line::Norm, Line::Norm, Line::Norm) => self.right_norm_down_norm_left_norm,
            (Line::Norm, Line::Norm, Line::None, Line::Norm) => self.up_norm_right_norm_left_norm,
            (Line::Norm, Line::Norm, Line::Norm, Line::Norm) => self.up_norm_right_norm_down_norm_left_norm,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Line {
    None,
    Norm,
    Bold
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

        let is_header_col =
            |col| table.col(col).unwrap().is_header() || table.col(col + 1).unwrap().is_header();
        let is_header_row =
            |row| table.row(row).unwrap().is_header() || table.row(row + 1).unwrap().is_header();

        // upper outside border
        buf.push(decor.right_bold_down_bold);
        for (col, &width) in col_widths.iter().enumerate() {
            (0..width).for_each(|_| buf.push(decor.right_bold_left_bold));
            if col < col_widths.len() - 1 {
                let row_separator_below = table.row(0).unwrap().is_separator();
                let down = if is_header_col(col) { Line::Bold } else if row_separator_below { Line::None } else { Line::Norm };
                buf.push(decor.lookup(Line::None, Line::Bold, down, Line::Bold));
            }
        }
        buf.push(decor.down_bold_left_bold);
        buf.push_str(NEWLINE);

        // table (incl. headers and body)
        for row in 0..table.num_rows() {
            let row_separator = table.row(row).unwrap().is_separator();
            let grid_row = &grid[row];
            let max_lines = grid_row.iter().map(|cell| cell.lines.len()).max().unwrap();

            // lines comprising the row
            for line in 0..max_lines {
                buf.push(decor.up_bold_down_bold);
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
                        let (up, down) = if row_separator {
                            (Line::None, Line::None)
                        } else if is_header_col(col) {
                            (Line::Bold, Line::Bold)
                        } else {
                            (Line:: Norm, Line::Norm)
                        };
                        buf.push(decor.lookup(up, Line::None, down, Line::None));
                    }
                }
                buf.push(decor.up_bold_down_bold);
                buf.push_str(NEWLINE);
            }

            // border below the row
            if row < table.num_rows() - 1 {
                let header_row = is_header_row(row);
                let row_separator_below = table.row(row + 1).unwrap().is_separator();
                let right = if header_row { Line::Bold } else { Line::Norm };
                buf.push(decor.lookup(Line::Bold, right, Line::Bold, Line::None));
                for (col, &width) in col_widths.iter().enumerate() {
                    let (right, left) = if header_row { (Line::Bold, Line::Bold) } else { (Line::Norm, Line::Norm) };
                    let border = decor.lookup(Line::None, right, Line::None, left);
                    (0..width).for_each(|_| buf.push(border));
                    if col < col_widths.len() - 1 {
                        let header_col = is_header_col(col);
                        let up = if header_col { Line::Bold } else if row_separator { Line::None } else { Line::Norm };
                        let down = if header_col { Line::Bold } else if row_separator_below { Line::None } else { Line::Norm };
                        let (right, left) = if header_row { (Line::Bold, Line::Bold) } else { (Line::Norm, Line::Norm ) };
                        buf.push(decor.lookup(up, right, down, left));
                    }
                }
                let left = if header_row { Line::Bold } else { Line::Norm };
                buf.push(decor.lookup(Line::Bold, Line::None, Line::Bold, left));
                buf.push_str(NEWLINE);
            }
        }

        // lower outside border
        buf.push(decor.up_bold_right_bold);
        for (col, &width) in col_widths.iter().enumerate() {
            (0..width).for_each(|_| buf.push(decor.right_bold_left_bold));
            if col < col_widths.len() - 1 {
                let row_separator_above = table.row(table.num_rows() - 1).unwrap().is_separator();
                let up = if is_header_col(col) { Line::Bold } else if row_separator_above { Line::None } else { Line::Norm };
                buf.push(decor.lookup(up, Line::Bold, Line::None, Line::Bold));
            }
        }
        buf.push(decor.up_bold_left_bold);
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
}

const DEF_ALIGNMENT: HAlign = HAlign::Left;
