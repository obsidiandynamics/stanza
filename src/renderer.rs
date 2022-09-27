use std::borrow::Cow;
use crate::model::Table;
use std::fmt::Display;
use crate::style::HAlignment;

pub mod ansi;
pub mod markdown;

pub const NEWLINE: &'static str = "\n";

pub trait Renderer {
    type Output: Display;

    fn render(&self, table: &Table) -> Self::Output;
}

impl Table {
    pub fn col_widths(&self) -> Vec<usize> {
        // let (num_cols, num_rows) = (self.num_cols(), self.num_rows());
        // let mut widths = Vec::with_capacity(num_cols);
        // for col in 0..num_cols {
        //     let mut max_width = 0;
        //     for row in 0..num_rows {
        //         let cell = self.cell(col, row);
        //         if let Some(cell) = cell {
        //             let data_len = cell.data().len();
        //             if data_len > max_width {
        //                 max_width = data_len;
        //             }
        //         }
        //     }
        //     widths.push(max_width);
        // }
        // widths
        (0..self.num_cols())
            .into_iter()
            .map(|col| self.col_width(col))
            .collect()
    }

    pub fn col_width(&self, col: usize) -> usize {
        (0..self.num_rows())
            .into_iter()
            .map(|row| self.cell(col, row))
            .map(|cell| cell.map_or(0, |cell| cell.data().len()))
            .max()
            .unwrap_or(0)

        // let mut max_width = 0;
        // for row in 0..self.num_rows() {
        //     let cell = self.cell(col, row);
        //     if let Some(cell) = cell {
        //         let data_len = cell.data().len();
        //         if data_len > max_width {
        //             max_width = data_len;
        //         }
        //     }
        // }
        // max_width
    }
}

pub fn pad<'a>(s: &'a str, p: char, width: usize, alignment: &HAlignment) -> Cow<'a, str> {
    let chars = s.chars().collect::<Vec<_>>();
    if chars.len() >= width {
        Cow::Borrowed(s)
    } else {
        let mut buf = String::with_capacity(width);
        let mut consumed = 0;
        for ch in chars {
            buf.push(ch);
            consumed += 1;
        }
        match alignment {
            HAlignment::Left => {
                for _ in consumed..width {
                    buf.push(p);
                }
            }
            HAlignment::Centred => {
                let mut added = 0;
                for _ in consumed..width {
                    if added % 2 == 0 {
                        buf.push(p);
                    } else {
                        buf.insert(0, p);
                    }
                    added += 1;
                }
            }
            HAlignment::Right => {
                for _ in consumed..width {
                    buf.insert(0, p);
                }
            }
        }
        Cow::Owned(buf)
    }
}
