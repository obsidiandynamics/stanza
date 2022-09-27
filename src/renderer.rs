use crate::model::Table;
use crate::style::{HAlign, MaxWidth, MinWidth, Style};
use std::borrow::Cow;
use std::fmt::Display;
use std::mem;

pub mod ansi;
pub mod markdown;

pub const NEWLINE: &'static str = "\n";

pub trait Renderer {
    type Output: Display;

    fn render(&self, table: &Table) -> Self::Output;
}

impl Table {
    pub fn col_widths(&self) -> Vec<usize> {
        (0..self.num_cols())
            .into_iter()
            .map(|col| self.col_width(col))
            .collect()
    }

    pub fn col_width(&self, col: usize) -> usize {
        (0..self.num_rows())
            .into_iter()
            .map(|row| self.cell(col, row))
            .map(|cell| {
                cell.map_or(0, |cell| {
                    let min_width =
                        MinWidth::resolve(&cell.combined_styles()).map_or(0, |s| s.0);
                    let max_width =
                        MaxWidth::resolve(&cell.combined_styles()).map_or(usize::MAX, |s| s.0);
                    let data_len = cell.data().len();
                    usize::min(usize::max(min_width,data_len), max_width)
                })
            })
            .max()
            .unwrap_or(0)
    }
}

pub fn pad<'a>(s: &'a str, p: char, width: usize, alignment: &HAlign) -> Cow<'a, str> {
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
            HAlign::Left => {
                for _ in consumed..width {
                    buf.push(p);
                }
            }
            HAlign::Centred => {
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
            HAlign::Right => {
                for _ in consumed..width {
                    buf.insert(0, p);
                }
            }
        }
        Cow::Owned(buf)
    }
}

pub fn wrap(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut line_chars = 0;
    for word in s.split_whitespace() {
        let chars = word.chars();
        let word_chars = chars.count();
        let needed_width = if line_chars > 0 { word_chars + 1} else { word_chars };
        if word_chars > width {
            // too big to fit on one line
            if line_chars > 0 && line_chars < width {
                // add a space between words
                line_chars += 1;
                if line_chars < width {
                    // don't add a space if it'll end up being the last character on the line
                    line.push(' ');
                }
            }

            let chars = word.chars();
            for ch in chars {
                if line_chars == width {
                    let current_line = mem::replace(&mut line, String::new());
                    lines.push(current_line);
                    line_chars = 0;
                }

                line.push(ch);
                line_chars += 1;
            }
        } else if line_chars + needed_width > width {
            // can fit on one line but too big to fit on this line
            let current_line = mem::replace(&mut line, String::new());
            lines.push(current_line);
            line_chars = word_chars;
            line.push_str(word)
        } else {
            // can fit on this line
            if line_chars > 0 {
                // add a space between words
                line_chars += 1;
                line.push(' ');
            }
            line.push_str(word);
            line_chars += word_chars;
        }
    }

    if lines.is_empty() || line_chars > 0 {
        lines.push(line);
    }

    lines
}

#[cfg(test)]
mod tests;