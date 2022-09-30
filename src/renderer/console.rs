use crate::model::Table;
use crate::renderer::{pad, wrap, Renderer, NEWLINE};
use crate::style::{Blink, Bold, Fg16, HAlign, Header, Italic, Separator, Strikethrough, Style, Styles, Underline};

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

        let is_header_col_pair =
            |col| grid.is_header_col(col) || grid.is_header_col(col + 1);
        let is_header_row_pair =
            |row| grid.is_header_row(row) || grid.is_header_row(row + 1);

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
                let row_separator_below = grid.is_separator_row(0);
                let down = if is_header_col_pair(col) { Line::Bold } else if row_separator_below { Line::None } else { Line::Norm };
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
            let row_separator = grid.is_separator_row(row);
            let grid_row = &grid.cells[row];
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
                    let alignment = HAlign::resolve_or_default(&grid_cell.styles);
                    let line = pad(line, ' ', col_widths[col], &alignment);
                    append(&mut buf, &line, &grid_cell.styles);

                    // vertical cell separator
                    if col < col_widths.len() - 1 {
                        let (up, down) = if is_header_col_pair(col) {
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
                let header_row_pair = is_header_row_pair(row);
                let row_separator_below = grid.is_separator_row(row + 1);

                // vertical line with possible right junction
                let col_separator_right = grid.is_separator_col(0);
                let right = if header_row_pair { Line::Bold } else if col_separator_right { Line::None } else { Line::Norm };
                buf.push(decor.lookup(Line::Bold, right, Line::Bold, Line::None));

                // horizontal line below the cell
                for (col, &width) in col_widths.iter().enumerate() {
                    let col_separator = grid.is_separator_col(col);
                    let (right, left) = if header_row_pair { (Line::Bold, Line::Bold) } else if col_separator { (Line::None, Line::None) } else { (Line::Norm, Line::Norm) };
                    let border = decor.lookup(Line::None, right, Line::None, left);
                    (0..width).for_each(|_| buf.push(border));

                    if col < col_widths.len() - 1 {
                        // junction between cells
                        let header_col_pair = is_header_col_pair(col);
                        let col_separator_right = grid.is_separator_col(col + 1);
                        let up = if header_col_pair { Line::Bold } else if row_separator { Line::None } else { Line::Norm };
                        let down = if header_col_pair { Line::Bold } else if row_separator_below { Line::None } else { Line::Norm };
                        let right = if header_row_pair { Line::Bold } else if col_separator_right { Line::None } else { Line::Norm };
                        let left = if header_row_pair { Line::Bold } else if col_separator { Line::None } else { Line::Norm };
                        buf.push(decor.lookup(up, right, down, left));
                    }
                }

                // vertical line with possible left junction
                let col_separator_left = grid.is_separator_col(col_widths.len() - 1);
                let left = if header_row_pair { Line::Bold } else if col_separator_left { Line::None } else { Line::Norm };
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
                let row_separator_above = grid.is_separator_row(table.num_rows() - 1);
                let up = if is_header_col_pair(col) { Line::Bold } else if row_separator_above { Line::None } else { Line::Norm };
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

impl Fg16 {
    fn escape_code(&self) -> &'static str {
        match self {
            Fg16::Black => "\x1b[30m",
            Fg16::Red => "\x1b[31m",
            Fg16::Green => "\x1b[32m",
            Fg16::Yellow => "\x1b[33m",
            Fg16::Blue => "\x1b[34m",
            Fg16::Magenta => "\x1b[35m",
            Fg16::Cyan => "\x1b[36m",
            Fg16::White => "\x1b[371m",
            Fg16::BrightBlack => "\x1b[30;1m",
            Fg16::BrightRed => "\x1b[31;1m",
            Fg16::BrightGreen => "\x1b[32;1m",
            Fg16::BrightYellow => "\x1b[33;1m",
            Fg16::BrightBlue => "\x1b[34;1m",
            Fg16::BrightMagenta => "\x1b[35;1m",
            Fg16::BrightCyan => "\x1b[36;1m",
            Fg16::BrightWhite => "\x1b[371;1m"
        }
    }
}

fn append(buf: &mut String, s: &str, styles: &Styles) {
    const BOLD: &str = "\x1b[1m";
    const ITALIC: &str = "\x1b[3m";
    const UNDERLINE: &str = "\x1b[4m";
    const BLINK: &str = "\x1b[5m";
    const STRIKETHROUGH: &str = "\x1b[9m";

    const RESET: &str = "\x1b[0m";

    let mut styled = false;
    let mut apply_style = |code| {
        styled = true;
        buf.push_str(code);
    };

    if Blink::resolve_or_default(styles).0 {
        apply_style(BLINK);
    }

    if Bold::resolve_or_default(styles).0 {
        apply_style(BOLD);
    }

    if Italic::resolve_or_default(styles).0 {
        apply_style(ITALIC);
    }

    if Strikethrough::resolve_or_default(styles).0 {
        apply_style(STRIKETHROUGH);
    }

    if Underline::resolve_or_default(styles).0 {
        apply_style(UNDERLINE);
    }

    if let Some(colour) = Fg16::resolve(styles) {
        apply_style(colour.escape_code());
    }

    buf.push_str(s);
    if styled {
        buf.push_str(RESET);
    }
}

// pub(super) mod ansi {
// 
// }

fn pre_render(table: &Table, col_widths: &[usize]) -> Grid {
    let cells = (0..table.num_rows())
        .into_iter()
        .map(|row| {
            (0..table.num_cols())
                .into_iter()
                .map(|col| {
                    let cell = table.cell(col, row);
                    let data = cell.as_ref().map(|cell| cell.data()).unwrap_or("");
                    let lines = wrap(data, col_widths[col]);
                    let styles = cell.map(|cell| cell.combined_styles()).unwrap_or_default();
                    GridCell { lines, styles }
                })
                .collect()
        })
        .collect();

    let col_styles = (0..table.num_cols())
        .into_iter()
        .map(|col| table.col(col).unwrap().combined_styles())
        .collect();

    let row_styles = (0..table.num_rows())
        .into_iter()
        .map(|row| table.row(row).unwrap().combined_styles())
        .collect();

    Grid {
        cells,
        col_styles,
        row_styles
    }
}

struct Grid {
    cells: Vec<Vec<GridCell>>,
    col_styles: Vec<Styles>,
    row_styles: Vec<Styles>,
}

impl Grid {
    fn is_header_col(&self, col: usize) -> bool {
        Header::resolve_or_default(&self.col_styles[col]).0
    }

    fn is_header_row(&self, row: usize) -> bool {
        Header::resolve_or_default(&self.row_styles[row]).0
    }
    fn is_separator_col(&self, col: usize) -> bool {
        Separator::resolve_or_default(&self.col_styles[col]).0
    }

    fn is_separator_row(&self, row: usize) -> bool {
        Separator::resolve_or_default(&self.row_styles[row]).0
    }
}

struct GridCell {
    lines: Vec<String>,
    styles: Styles,
}

