use crate::style::{HAlign, MaxWidth, MinWidth, Style};
use crate::table::{Content, Table};
use alloc::borrow::Cow;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Display;
use core::mem;

pub mod console;
pub mod markdown;

pub const NEWLINE: &str = "\n";

pub trait Renderer {
    type Output: Display;

    #[inline]
    fn render(&self, table: &Table) -> Self::Output {
        self.render_with_hints(table, &[])
    }

    fn render_with_hints(&self, table: &Table, hints: &[RenderHint]) -> Self::Output;
}

#[derive(PartialEq, Eq)]
pub enum RenderHint {
    Nested,
}

impl Content {
    pub fn render<R: Renderer>(&self, renderer: &R) -> Cow<str> {
        match self {
            Content::Label(s) => Cow::Borrowed(s),
            Content::Computed(f) => Cow::Owned(f()),
            Content::Nested(table) => {
                Cow::Owned(format!("{}", renderer.render_with_hints(table, &[RenderHint::Nested])))
            }
            Content::Composite(contents) => {
                let mut buf = String::new();
                for content in contents {
                    buf.push_str(&content.render(renderer));
                }
                Cow::Owned(buf)
            }
        }
    }
}

impl Table {
    pub fn col_widths(&self, renderer: &impl Renderer) -> Vec<usize> {
        (0..self.num_cols())
            .into_iter()
            .map(|col| self.col_width(col, renderer))
            .collect()
    }

    pub fn col_width(&self, col: usize, renderer: &impl Renderer) -> usize {
        (0..self.num_rows())
            .into_iter()
            .map(|row| self.cell(col, row))
            .map(|cell| {
                // if a cell exists at the given col/row coordinate, calculate the width from the combination
                // of its data and the MinWidth/MaxWidth constraints
                let styles = cell.blended_styles();
                let min_width = MinWidth::resolve_or_default(&styles).0;
                let max_width = MaxWidth::resolve_or_default(&styles).0;
                let widest_line = cell
                    .map(|cell| {
                        cell.data()
                            .render(renderer)
                            .lines()
                            .map(|line| line.chars().count())
                            .max()
                            .unwrap_or(0)
                    })
                    .unwrap_or(0);
                usize::min(usize::max(min_width, widest_line), max_width)
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
                for (excess, _) in (consumed..width).enumerate() {
                    if excess % 2 == 0 {
                        buf.push(p);
                    } else {
                        buf.insert(0, p);
                    }
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
        for word in split_preserving_whitespace(input_line) {
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
                        let current_buf = mem::take(&mut buf);
                        wrapped_lines.push(current_buf);
                        buf_chars = 0;
                    }

                    buf.push(ch);
                    buf_chars += 1;
                }
            } else if buf_chars + needed_width > width {
                // can fit on one line but too big to fit on this line
                let current_buf = mem::take(&mut buf);
                wrapped_lines.push(current_buf);
                buf_chars = word_chars;
                buf.push_str(&word);
            } else {
                // can fit on this line
                if buf_chars > 0 {
                    // add a space between words
                    buf_chars += 1;
                    buf.push(' ');
                }
                buf.push_str(&word);
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
        wrapped_lines.push(String::new());
    }

    wrapped_lines
}

/// Splits a string slice by whitespace, while preserving extraneous whitespace that
/// appears after the first encountered separator.
///
/// This splitter has the convenient property that the number of whitespace characters
/// in the input can be deterministically obtained by counting the number of whitespace
/// characters in the output fragments and adding the number of fragments, less one.
///
/// # Examples
/// ```
/// use stanza::renderer::split_preserving_whitespace;
///
/// let input = " what a  wonderful day ";
/// let output = split_preserving_whitespace(input);
/// assert_eq!(vec![" what", "a", " wonderful", "day", ""], output);
///
/// fn count_whitespace(s: &str) -> usize {
///     s.chars().filter(|ch| ch.is_whitespace()).count()
/// }
///
/// let whitespace_in_input = count_whitespace(input);
/// let whitespace_in_output = output.iter().map(|frag| count_whitespace(frag)).sum::<usize>();
/// assert_eq!(whitespace_in_input, whitespace_in_output + output.len() - 1);
/// ```
pub fn split_preserving_whitespace(s: &str) -> Vec<String> {
    let mut frags = Vec::new();
    let mut buf = String::new();

    let mut prev_whitespace = true;
    for ch in s.chars() {
        let whitespace = ch.is_whitespace();
        if prev_whitespace || !whitespace {
            buf.push(ch);
            prev_whitespace = whitespace;
        } else {
            frags.push(mem::take(&mut buf));
            prev_whitespace = true;
        }
    }

    frags.push(buf);
    frags
}

#[cfg(test)]
mod tests;
