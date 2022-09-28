use crate::model::Table;
use crate::style::{HAlign, MaxWidth, MinWidth, Style};
use std::borrow::Cow;
use std::fmt::Display;
use std::mem;

pub mod console;
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
                    let min_width = MinWidth::resolve(&cell.combined_styles()).map_or(0, |s| s.0);
                    let max_width =
                        MaxWidth::resolve(&cell.combined_styles()).map_or(usize::MAX, |s| s.0);
                    let data_len = cell.data().len();
                    usize::min(usize::max(min_width, data_len), max_width)
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
    let mut wrapped_lines = Vec::new();
    let mut wrapped_lines_before;
    for input_line in s.lines() {
        wrapped_lines_before = wrapped_lines.len();
        let mut buf_chars = 0;
        let mut buf = String::new();
        for word in input_line.split_whitespace() {
            let chars = word.chars();
            let word_chars = chars.count();
            let needed_width = if buf_chars > 0 {
                word_chars + 1
            } else {
                word_chars
            };
            if word_chars > width {
                // too big to fit on one line
                if buf_chars > 0 && buf_chars < width {
                    // add a space between words
                    buf_chars += 1;
                    if buf_chars < width {
                        // don't add a space if it'll end up being the last character on the line
                        buf.push(' ');
                    }
                }

                let chars = word.chars();
                for ch in chars {
                    if buf_chars == width {
                        let current_buf = mem::replace(&mut buf, String::new());
                        wrapped_lines.push(current_buf);
                        buf_chars = 0;
                    }

                    buf.push(ch);
                    buf_chars += 1;
                }
            } else if buf_chars + needed_width > width {
                // can fit on one line but too big to fit on this line
                let current_buf = mem::replace(&mut buf, String::new());
                wrapped_lines.push(current_buf);
                buf_chars = word_chars;
                buf.push_str(word)
            } else {
                // can fit on this line
                if buf_chars > 0 {
                    // add a space between words
                    buf_chars += 1;
                    buf.push(' ');
                }
                buf.push_str(word);
                buf_chars += word_chars;
            }
        }

        if wrapped_lines.len() == wrapped_lines_before || buf_chars > 0 {
            // always add a wrapped line if it is either nonempty (i.e., we've accumulated
            // characters in the buffer but haven't flushed it yet)  or we haven't wrapped at least
            // one line as part of this input line
            wrapped_lines.push(buf);
        }
    }

    if wrapped_lines.is_empty() {
        // the output should contain at least one, albeit empty, line
        wrapped_lines.push(String::new())
    }

    wrapped_lines
}

#[cfg(test)]
mod tests;
