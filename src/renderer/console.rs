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
    right_norm_down_norm: char,
    down_norm_left_norm: char,
    up_norm_right_norm: char,
    up_norm_left_norm: char,
    up_norm_right_norm_down_norm: char,
    up_norm_down_norm_left_norm: char,
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
            right_norm_down_norm: '┌',
            down_norm_left_norm: '┐',
            up_norm_right_norm: '└',
            up_norm_left_norm: '┘',
            up_norm_right_norm_down_norm: '├',
            up_norm_down_norm_left_norm: '┤',
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
            (Line::None, Line::Norm, Line::Norm, Line::None) => self.right_norm_down_norm,
            (Line::None, Line::None, Line::Norm, Line::Norm) => self.down_norm_left_norm,
            (Line::Norm, Line::Norm, Line::None, Line::None) => self.up_norm_right_norm,
            (Line::Norm, Line::None, Line::None, Line::Norm) => self.up_norm_left_norm,
            (Line::Norm, Line::Norm, Line::Norm, Line::None) => self.up_norm_right_norm_down_norm,
            (Line::Norm, Line::None, Line::Norm, Line::Norm) => self.up_norm_down_norm_left_norm,
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

        // upper outside border...
        // top-left corner
        let top_left = decor.lookup(Line::None, Line::Bold, Line::Bold, Line::None);
        buf.push(top_left);
        let horizontal_line = decor.lookup(Line::None, Line::Bold, Line::None, Line::Bold);
        for (col, &width) in col_widths.iter().enumerate() {
            // horizontal line
            (0..width).for_each(|_| buf.push(horizontal_line));

            if col < col_widths.len() - 1 {
                // junction between cells
                let row_separator_below = table.row(0).unwrap().is_separator();
                let down = if is_header_col(col) { Line::Bold } else if row_separator_below { Line::None } else { Line::Norm };
                buf.push(decor.lookup(Line::None, Line::Bold, down, Line::Bold));
            }
        }
        // bottom-right corner
        let top_right = decor.lookup(Line::None, Line::None, Line::Bold, Line::Bold);
        buf.push(top_right);
        buf.push_str(NEWLINE);

        // table (incl. headers and body)...
        let vertical_line = decor.lookup(Line::Bold, Line::None, Line::Bold, Line::None);
        for row in 0..table.num_rows() {
            let row_separator = table.row(row).unwrap().is_separator();
            let grid_row = &grid[row];
            let max_lines = grid_row.iter().map(|cell| cell.lines.len()).max().unwrap();

            // lines comprising the row
            for line in 0..max_lines {
                // right outer vertical separator
                buf.push(vertical_line);

                for col in 0..col_widths.len() {
                    let grid_cell = &grid_row[col];

                    // cell data
                    let line = grid_cell
                        .lines
                        .get(line)
                        .map(|line| &line[..])
                        .unwrap_or("");
                    let alignment = HAlign::resolve(&grid_cell.styles).unwrap_or(&DEF_ALIGNMENT);
                    let line = pad(line, ' ', col_widths[col], &alignment);
                    buf.push_str(&line);

                    // vertical cell separator
                    if col < col_widths.len() - 1 {
                        let (up, down) = if is_header_col(col) {
                            (Line::Bold, Line::Bold)
                        } else if row_separator {
                            (Line::None, Line::None)
                        } else {
                            (Line::Norm, Line::Norm)
                        };
                        buf.push(decor.lookup(up, Line::None, down, Line::None));
                    }
                }

                // right outer vertical separator
                buf.push(vertical_line);
                buf.push_str(NEWLINE);
            }

            // border below the row
            if row < table.num_rows() - 1 {
                let header_row = is_header_row(row);
                let row_separator_below = table.row(row + 1).unwrap().is_separator();

                // vertical line with possible right junction
                let col_separator_right = table.col(0).unwrap().is_separator();
                let right = if header_row { Line::Bold } else if col_separator_right { Line::None } else { Line::Norm };
                buf.push(decor.lookup(Line::Bold, right, Line::Bold, Line::None));

                // horizontal line below the cell
                for (col, &width) in col_widths.iter().enumerate() {
                    let col_separator = table.col(col).unwrap().is_separator();
                    let (right, left) = if header_row { (Line::Bold, Line::Bold) } else if col_separator { (Line::None, Line::None) } else { (Line::Norm, Line::Norm) };
                    let border = decor.lookup(Line::None, right, Line::None, left);
                    (0..width).for_each(|_| buf.push(border));

                    if col < col_widths.len() - 1 {
                        // junction between cells
                        let header_col = is_header_col(col);
                        let col_separator_right = table.col(col + 1).unwrap().is_separator();
                        let up = if header_col { Line::Bold } else if row_separator { Line::None } else { Line::Norm };
                        let down = if header_col { Line::Bold } else if row_separator_below { Line::None } else { Line::Norm };
                        let right = if header_row { Line::Bold } else if col_separator_right { Line::None } else { Line::Norm };
                        let left = if header_row { Line::Bold } else if col_separator { Line::None } else { Line::Norm };
                        buf.push(decor.lookup(up, right, down, left));
                    }
                }

                // vertical line with possible left junction
                let col_separator_left = table.col(col_widths.len() - 1).unwrap().is_separator();
                let left = if header_row { Line::Bold } else if col_separator_left { Line::None } else { Line::Norm };
                buf.push(decor.lookup(Line::Bold, Line::None, Line::Bold, left));
                buf.push_str(NEWLINE);
            }
        }

        // lower outside border...
        // bottom-left corner
        let bottom_left = decor.lookup(Line::Bold, Line::Bold, Line::None, Line::None);
        buf.push(bottom_left);
        for (col, &width) in col_widths.iter().enumerate() {
            // horizontal line
            (0..width).for_each(|_| buf.push(horizontal_line));

            if col < col_widths.len() - 1 {
                // junction between cells
                let row_separator_above = table.row(table.num_rows() - 1).unwrap().is_separator();
                let up = if is_header_col(col) { Line::Bold } else if row_separator_above { Line::None } else { Line::Norm };
                buf.push(decor.lookup(up, Line::Bold, Line::None, Line::Bold));
            }
        }
        // bottom-right corner
        let bottom_right = decor.lookup(Line::Bold, Line::None, Line::None, Line::Bold);
        buf.push(bottom_right);
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
