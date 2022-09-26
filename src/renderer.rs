use std::fmt::Display;

pub mod ansi;
pub mod markdown;

pub trait Renderer {
    type Output: Display;

    fn render(&self) -> Self::Output;
}