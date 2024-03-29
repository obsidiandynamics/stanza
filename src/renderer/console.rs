use crate::renderer::{pad, wrap, RenderHint, Renderer, NEWLINE};
use crate::style::{Blink, Bold, BorderBg, BorderFg, FillBg, FillInvert, HAlign, Header, Italic, Palette16, Separator, Strikethrough, Style, Styled, Styles, TextBg, TextFg, TextInvert, Underline};
use crate::table::Table;
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decor {
    pub blank: char,
    pub up_bold_down_bold: char,
    pub right_bold_left_bold: char,
    pub right_bold_down_bold: char,
    pub down_bold_left_bold: char,
    pub up_bold_right_bold: char,
    pub up_bold_left_bold: char,
    pub up_bold_right_thin_down_bold: char,
    pub up_bold_right_bold_down_bold: char,
    pub up_bold_down_bold_left_thin: char,
    pub up_bold_down_bold_left_bold: char,
    pub right_bold_down_thin_left_bold: char,
    pub right_bold_down_bold_left_bold: char,
    pub up_thin_right_bold_left_bold: char,
    pub up_bold_right_bold_left_bold: char,
    pub up_thin_right_bold_down_thin_left_bold: char,
    pub up_bold_right_thin_down_bold_left_thin: char,
    pub up_bold_right_bold_down_bold_left_bold: char,
    pub right_thin_left_thin: char,
    pub up_thin_down_thin: char,
    pub right_thin_down_thin: char,
    pub down_thin_left_thin: char,
    pub up_thin_right_thin: char,
    pub up_thin_left_thin: char,
    pub up_thin_right_thin_down_thin: char,
    pub up_thin_down_thin_left_thin: char,
    pub right_thin_down_thin_left_thin: char,
    pub up_thin_right_thin_left_thin: char,
    pub up_thin_right_thin_down_thin_left_thin: char,
    pub up_thin: char,
    pub right_thin: char,
    pub down_thin: char,
    pub left_thin: char,
    pub print_escape_codes: bool,
    pub draw_outer_border: bool,
    pub draw_inner_horizontal_border: bool,
    pub remap_thin_to: Line,
    pub remap_bold_to: Line,
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
            up_bold_right_thin_down_bold: '╟',
            up_bold_right_bold_down_bold: '╠',
            up_bold_down_bold_left_thin: '╢',
            up_bold_down_bold_left_bold: '╣',
            right_bold_down_thin_left_bold: '╤',
            right_bold_down_bold_left_bold: '╦',
            up_thin_right_bold_left_bold: '╧',
            up_bold_right_bold_left_bold: '╩',
            up_thin_right_bold_down_thin_left_bold: '╪',
            up_bold_right_thin_down_bold_left_thin: '╫',
            up_bold_right_bold_down_bold_left_bold: '╬',
            right_thin_left_thin: '─',
            up_thin_down_thin: '│',
            right_thin_down_thin: '┌',
            down_thin_left_thin: '┐',
            up_thin_right_thin: '└',
            up_thin_left_thin: '┘',
            up_thin_right_thin_down_thin: '├',
            up_thin_down_thin_left_thin: '┤',
            right_thin_down_thin_left_thin: '┬',
            up_thin_right_thin_left_thin: '┴',
            up_thin_right_thin_down_thin_left_thin: '┼',
            up_thin: '╵',
            right_thin: '╶',
            down_thin: '╷',
            left_thin: '╴',
            print_escape_codes: true,
            draw_outer_border: true,
            draw_inner_horizontal_border: true,
            remap_thin_to: Line::Thin,
            remap_bold_to: Line::Bold
        }
    }

    #[must_use]
    pub fn suppress_escape_codes(mut self) -> Self {
        self.print_escape_codes = false;
        self
    }

    #[must_use]
    pub fn suppress_outer_border(mut self) -> Self {
        self.draw_outer_border = false;
        self
    }

    #[must_use]
    pub fn suppress_inner_horizontal_border(mut self) -> Self {
        self.draw_inner_horizontal_border = false;
        self
    }

    #[must_use]
    pub fn suppress_all_lines(mut self) -> Self {
        self.remap_thin_to = Line::None;
        self.remap_bold_to = Line::None;
        self
    }

    fn remap_line(&self, line: Line) -> Line {
        match line {
            Line::None => Line::None,
            Line::Thin => self.remap_thin_to,
            Line::Bold => self.remap_bold_to
        }
    }

    fn lookup(&self, up: Line, right: Line, down: Line, left: Line) -> char {
        let up = self.remap_line(up);
        let right = self.remap_line(right);
        let down = self.remap_line(down);
        let left = self.remap_line(left);

        // dbg!(&up, &right, &down, &left);
        match (up, right, down, left) {
            (Line::None, Line::None, Line::None, Line::None) => self.blank,
            (Line::None, Line::Bold, Line::None, Line::Bold) => self.right_bold_left_bold,
            (Line::Bold, Line::None, Line::Bold, Line::None) => self.up_bold_down_bold,
            (Line::None, Line::Bold, Line::Bold, Line::None) => self.right_bold_down_bold,
            (Line::None, Line::None, Line::Bold, Line::Bold) => self.down_bold_left_bold,
            (Line::Bold, Line::Bold, Line::None, Line::None) => self.up_bold_right_bold,
            (Line::Bold, Line::None, Line::None, Line::Bold) => self.up_bold_left_bold,
            (Line::Bold, Line::Thin, Line::Bold, Line::None) => self.up_bold_right_thin_down_bold,
            (Line::Bold, Line::Bold, Line::Bold, Line::None) => self.up_bold_right_bold_down_bold,
            (Line::Bold, Line::None, Line::Bold, Line::Thin) => self.up_bold_down_bold_left_thin,
            (Line::Bold, Line::None, Line::Bold, Line::Bold) => self.up_bold_down_bold_left_bold,
            (Line::None, Line::Bold, Line::Thin, Line::Bold) => self.right_bold_down_thin_left_bold,
            (Line::None, Line::Bold, Line::Bold, Line::Bold) => self.right_bold_down_bold_left_bold,
            (Line::Thin, Line::Bold, Line::None, Line::Bold) => self.up_thin_right_bold_left_bold,
            (Line::Bold, Line::Bold, Line::None, Line::Bold) => self.up_bold_right_bold_left_bold,
            (Line::Thin, Line::Bold, Line::Thin, Line::Bold) => {
                self.up_thin_right_bold_down_thin_left_bold
            }
            (Line::Bold, Line::Thin, Line::Bold, Line::Thin) => {
                self.up_bold_right_thin_down_bold_left_thin
            }
            (Line::Bold, Line::Bold, Line::Bold, Line::Bold) => {
                self.up_bold_right_bold_down_bold_left_bold
            }
            (Line::None, Line::Thin, Line::None, Line::Thin) => self.right_thin_left_thin,
            (Line::Thin, Line::None, Line::Thin, Line::None) => self.up_thin_down_thin,
            (Line::None, Line::Thin, Line::Thin, Line::None) => self.right_thin_down_thin,
            (Line::None, Line::None, Line::Thin, Line::Thin) => self.down_thin_left_thin,
            (Line::Thin, Line::Thin, Line::None, Line::None) => self.up_thin_right_thin,
            (Line::Thin, Line::None, Line::None, Line::Thin) => self.up_thin_left_thin,
            (Line::Thin, Line::Thin, Line::Thin, Line::None) => self.up_thin_right_thin_down_thin,
            (Line::Thin, Line::None, Line::Thin, Line::Thin) => self.up_thin_down_thin_left_thin,
            (Line::None, Line::Thin, Line::Thin, Line::Thin) => self.right_thin_down_thin_left_thin,
            (Line::Thin, Line::Thin, Line::None, Line::Thin) => self.up_thin_right_thin_left_thin,
            (Line::Thin, Line::Thin, Line::Thin, Line::Thin) => {
                self.up_thin_right_thin_down_thin_left_thin
            }
            (Line::Thin, Line::None, Line::None, Line::None) => self.up_thin,
            (Line::None, Line::Thin, Line::None, Line::None) => self.right_thin,
            (Line::None, Line::None, Line::Thin, Line::None) => self.down_thin,
            (Line::None, Line::None, Line::None, Line::Thin) => self.left_thin,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Line {
    None,
    Thin,
    Bold,
}

#[derive(Default)]
pub struct Console(pub Decor);

impl Renderer for Console {
    type Output = String;

    #[allow(clippy::too_many_lines, clippy::similar_names)]
    fn render_with_hints(&self, table: &Table, hints: &[RenderHint]) -> Self::Output {
        assert!(!table.is_empty(), "table cannot be empty");
        let col_widths = table.col_widths(self);
        let decor = &self.0;
        let grid = pre_render(self, table, &col_widths);
        let print_escape_codes = self.0.print_escape_codes && !hints.contains(&RenderHint::Nested);
        let border_fg = BorderFg::resolve(table.styles());
        let border_bg = BorderBg::resolve(table.styles());
        let mut buf = String::new();

        let is_header_col_pair = |col| grid.is_header_col(col) || grid.is_header_col(col + 1);
        let is_header_row_pair = |row| grid.is_header_row(row) || grid.is_header_row(row + 1);

        let horizontal_line = decor.lookup(Line::None, Line::Bold, Line::None, Line::Bold);
        if decor.draw_outer_border {
            // upper outside border...
            // top-left corner
            let top_left = decor.lookup(Line::None, Line::Bold, Line::Bold, Line::None);
            append_border(&mut buf, top_left, border_fg, border_bg, print_escape_codes);
            for (col, &width) in col_widths.iter().enumerate() {
                // horizontal line
                for _ in 0..width {
                    append_border(
                        &mut buf,
                        horizontal_line,
                        border_fg,
                        border_bg,
                        print_escape_codes,
                    );
                }

                if col < col_widths.len() - 1 {
                    // junction between cells
                    let row_separator_below = grid.is_separator_row(0);
                    let down = if is_header_col_pair(col) {
                        Line::Bold
                    } else if row_separator_below {
                        Line::None
                    } else {
                        Line::Thin
                    };
                    append_border(
                        &mut buf,
                        decor.lookup(Line::None, Line::Bold, down, Line::Bold),
                        border_fg,
                        border_bg,
                        print_escape_codes,
                    );
                }
            }
            // bottom-right corner
            let top_right = decor.lookup(Line::None, Line::None, Line::Bold, Line::Bold);
            append_border(
                &mut buf,
                top_right,
                border_fg,
                border_bg,
                print_escape_codes,
            );
            buf.push_str(NEWLINE);
        }

        // table (incl. headers and body)...
        let vertical_line = decor.lookup(Line::Bold, Line::None, Line::Bold, Line::None);
        for row in 0..table.num_rows() {
            let row_separator = grid.is_separator_row(row);
            let grid_row = &grid.cells[row];
            let max_lines = grid_row.iter().map(|cell| cell.lines.len()).max().unwrap();

            // lines comprising the row
            for line in 0..max_lines {
                if decor.draw_outer_border {
                    // left outer vertical separator
                    append_border(
                        &mut buf,
                        vertical_line,
                        border_fg,
                        border_bg,
                        print_escape_codes,
                    );
                }

                for col in 0..col_widths.len() {
                    let grid_cell = &grid_row[col];

                    // cell data
                    let line = grid_cell
                        .lines
                        .get(line)
                        .map_or("", |line| &line[..]);
                    let alignment = HAlign::resolve_or_default(&grid_cell.styles);
                    let line = pad(line, ' ', col_widths[col], &alignment);
                    append_content(&mut buf, &line, &grid_cell.styles, print_escape_codes);

                    // vertical cell separator
                    if col < col_widths.len() - 1 {
                        let (up, down) = if is_header_col_pair(col) {
                            (Line::Bold, Line::Bold)
                        } else if row_separator {
                            (Line::None, Line::None)
                        } else {
                            (Line::Thin, Line::Thin)
                        };
                        append_border(
                            &mut buf,
                            decor.lookup(up, Line::None, down, Line::None),
                            border_fg,
                            border_bg,
                            print_escape_codes,
                        );
                    }
                }

                if decor.draw_outer_border {
                    // right outer vertical separator
                    append_border(
                        &mut buf,
                        vertical_line,
                        border_fg,
                        border_bg,
                        print_escape_codes,
                    );
                }
                buf.push_str(NEWLINE);
            }

            if decor.draw_inner_horizontal_border {
                // border below the row
                if row < table.num_rows() - 1 {
                    let header_row_pair = is_header_row_pair(row);
                    let row_separator_below = grid.is_separator_row(row + 1);

                    if decor.draw_outer_border {
                        // vertical line with possible right junction
                        let col_separator_right = grid.is_separator_col(0);
                        let right = if header_row_pair {
                            Line::Bold
                        } else if col_separator_right {
                            Line::None
                        } else {
                            Line::Thin
                        };
                        append_border(
                            &mut buf,
                            decor.lookup(Line::Bold, right, Line::Bold, Line::None),
                            border_fg,
                            border_bg,
                            print_escape_codes,
                        );
                    }

                    // horizontal line below the cell
                    for (col, &width) in col_widths.iter().enumerate() {
                        let col_separator = grid.is_separator_col(col);
                        let (right, left) = if header_row_pair {
                            (Line::Bold, Line::Bold)
                        } else if col_separator {
                            (Line::None, Line::None)
                        } else {
                            (Line::Thin, Line::Thin)
                        };
                        let border = decor.lookup(Line::None, right, Line::None, left);
                        for _ in 0..width {
                            append_border(&mut buf, border, border_fg, border_bg, print_escape_codes);
                        }

                        if col < col_widths.len() - 1 {
                            // junction between cells
                            let header_col_pair = is_header_col_pair(col);
                            let col_separator_right = grid.is_separator_col(col + 1);
                            let up = if header_col_pair {
                                Line::Bold
                            } else if row_separator {
                                Line::None
                            } else {
                                Line::Thin
                            };
                            let down = if header_col_pair {
                                Line::Bold
                            } else if row_separator_below {
                                Line::None
                            } else {
                                Line::Thin
                            };
                            let right = if header_row_pair {
                                Line::Bold
                            } else if col_separator_right {
                                Line::None
                            } else {
                                Line::Thin
                            };
                            let left = if header_row_pair {
                                Line::Bold
                            } else if col_separator {
                                Line::None
                            } else {
                                Line::Thin
                            };
                            append_border(
                                &mut buf,
                                decor.lookup(up, right, down, left),
                                border_fg,
                                border_bg,
                                print_escape_codes,
                            );
                        }
                    }

                    if decor.draw_outer_border {
                        // vertical line with possible left junction
                        let col_separator_left = grid.is_separator_col(col_widths.len() - 1);
                        let left = if header_row_pair {
                            Line::Bold
                        } else if col_separator_left {
                            Line::None
                        } else {
                            Line::Thin
                        };
                        append_border(
                            &mut buf,
                            decor.lookup(Line::Bold, Line::None, Line::Bold, left),
                            border_fg,
                            border_bg,
                            print_escape_codes,
                        );
                    }
                    buf.push_str(NEWLINE);
                }
            }
        }

        if decor.draw_outer_border {
            // lower outside border...
            // bottom-left corner
            let bottom_left = decor.lookup(Line::Bold, Line::Bold, Line::None, Line::None);
            append_border(
                &mut buf,
                bottom_left,
                border_fg,
                border_bg,
                print_escape_codes,
            );
            for (col, &width) in col_widths.iter().enumerate() {
                // horizontal line
                for _ in 0..width {
                    append_border(
                        &mut buf,
                        horizontal_line,
                        border_fg,
                        border_bg,
                        print_escape_codes,
                    );
                }

                if col < col_widths.len() - 1 {
                    // junction between cells
                    let row_separator_above = grid.is_separator_row(table.num_rows() - 1);
                    let up = if is_header_col_pair(col) {
                        Line::Bold
                    } else if row_separator_above {
                        Line::None
                    } else {
                        Line::Thin
                    };
                    append_border(
                        &mut buf,
                        decor.lookup(up, Line::Bold, Line::None, Line::Bold),
                        border_fg,
                        border_bg,
                        print_escape_codes,
                    );
                }
            }
            // bottom-right corner
            let bottom_right = decor.lookup(Line::Bold, Line::None, Line::None, Line::Bold);
            append_border(
                &mut buf,
                bottom_right,
                border_fg,
                border_bg,
                print_escape_codes,
            );
        }

        buf
    }
}

impl Palette16 {
    /// Obtains a pair of ANSI escape codes in the form `(foreground, background)`.
    fn escape_codes(&self) -> (&'static str, &'static str) {
        match self {
            Palette16::Black => ("\x1b[30m", "\x1b[40m"),
            Palette16::Red => ("\x1b[31m", "\x1b[41m"),
            Palette16::Green => ("\x1b[32m", "\x1b[42m"),
            Palette16::Yellow => ("\x1b[33m", "\x1b[43m"),
            Palette16::Blue => ("\x1b[34m", "\x1b[44m"),
            Palette16::Magenta => ("\x1b[35m", "\x1b[45m"),
            Palette16::Cyan => ("\x1b[36m", "\x1b[46m"),
            Palette16::White => ("\x1b[37m", "\x1b[47m"),
            Palette16::BrightBlack => ("\x1b[30;1m", "\x1b[40;1m"),
            Palette16::BrightRed => ("\x1b[31;1m", "\x1b[41;1m"),
            Palette16::BrightGreen => ("\x1b[32;1m", "\x1b[42;1m"),
            Palette16::BrightYellow => ("\x1b[33;1m", "\x1b[43;1m"),
            Palette16::BrightBlue => ("\x1b[34;1m", "\x1b[44;1m"),
            Palette16::BrightMagenta => ("\x1b[35;1m", "\x1b[45;1m"),
            Palette16::BrightCyan => ("\x1b[36;1m", "\x1b[46;1m"),
            Palette16::BrightWhite => ("\x1b[37;1m", "\x1b[47;1m"),
            Palette16::Default => ("\x1b[0m", ""),
            Palette16::Hidden => ("\x1b[8m", ""),
        }
    }
}

mod ansi {
    pub const BOLD: &str = "\x1b[1m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";
    pub const BLINK: &str = "\x1b[5m";
    pub const REVERSE: &str = "\x1b[7m";
    pub const STRIKETHROUGH: &str = "\x1b[9m";

    pub const RESET: &str = "\x1b[0m";
}

fn append_border(
    buf: &mut String,
    b: char,
    fg: Option<&BorderFg>,
    bg: Option<&BorderBg>,
    print_escape_codes: bool,
) {
    if print_escape_codes {
        match (fg, bg) {
            (None, None) => buf.push(b),
            _ => {
                if let Some(fg) = fg {
                    buf.push_str(fg.0.escape_codes().0);
                }
                if let Some(bg) = bg {
                    buf.push_str(bg.0.escape_codes().1);
                }
                buf.push(b);
                buf.push_str(ansi::RESET);
            }
        }
    } else {
        buf.push(b);
    }
}

fn find_first_printable(chars: impl Iterator<Item = char>) -> Option<usize> {
    chars
        .enumerate()
        .find(|(_, ch)| !ch.is_whitespace())
        .map(|(i, _)| i)
}

fn append_content(buf: &mut String, s: &str, styles: &Styles, print_escape_codes: bool) {
    if print_escape_codes {
        // formatting that applies to the entire line (both whitespace and printable characters)
        let mut line_format = String::new();
        if let Some(bg) = FillBg::resolve(styles) {
            line_format.push_str(bg.0.escape_codes().1);
        }
        if FillInvert::resolve_or_default(styles).0 {
            line_format.push_str(ansi::REVERSE);
        }

        // formatting only of the printable characters
        let mut char_format = line_format.clone();
        if Blink::resolve_or_default(styles).0 {
            char_format.push_str(ansi::BLINK);
        }
        if Bold::resolve_or_default(styles).0 {
            char_format.push_str(ansi::BOLD);
        }
        if Italic::resolve_or_default(styles).0 {
            char_format.push_str(ansi::ITALIC);
        }
        if Strikethrough::resolve_or_default(styles).0 {
            char_format.push_str(ansi::STRIKETHROUGH);
        }
        if let Some(bg) = TextBg::resolve(styles) {
            char_format.push_str(bg.0.escape_codes().1);
        }
        if let Some(fg) = TextFg::resolve(styles) {
            char_format.push_str(fg.0.escape_codes().0);
        }
        if TextInvert::resolve_or_default(styles).0 {
            char_format.push_str(ansi::REVERSE);
        }
        if Underline::resolve_or_default(styles).0 {
            char_format.push_str(ansi::UNDERLINE);
        }

        buf.push_str(&line_format);
        let first_char = find_first_printable(s.chars());
        if let Some(first_char) = first_char {
            let total_chars = s.chars().count();
            let last_char = total_chars - find_first_printable(s.chars().rev()).unwrap() - 1;
            for (i, ch) in s.chars().enumerate() {
                if i == first_char {
                    switch_format(buf, &char_format, &line_format);
                }
                buf.push(ch);
                if i == last_char {
                    switch_format(buf, &line_format, &char_format);
                }
            }
        } else {
            buf.push_str(s);
        }
        switch_format(buf, "", &line_format);
    } else {
        buf.push_str(s);
    }
}

fn switch_format(buf: &mut String, new_format: &str, old_format: &str) {
    if new_format != old_format {
        buf.push_str(ansi::RESET);
        buf.push_str(new_format);
    }
}

fn pre_render(renderer: &Console, table: &Table, col_widths: &[usize]) -> Grid {
    let col_styles = (0..table.num_cols())
        .into_iter()
        .map(|col| table.col(col).blended_styles())
        .collect::<Vec<_>>();

    let row_styles = (0..table.num_rows())
        .into_iter()
        .map(|row| table.row(row).blended_styles())
        .collect::<Vec<_>>();

    let cells = (0..table.num_rows())
        .into_iter()
        .map(|row| {
            (0..table.num_cols())
                .into_iter()
                .map(|col| {
                    let cell = table.cell(col, row);
                    let data = cell
                        .as_ref()
                        .map_or(Cow::Borrowed(""), |cell| cell.data().render(renderer));
                    let lines = wrap(&data, col_widths[col]);
                    let styles = cell.blended_styles();
                    GridCell { lines, styles }
                })
                .collect()
        })
        .collect();

    Grid {
        cells,
        col_styles,
        row_styles,
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
