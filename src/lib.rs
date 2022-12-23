#![no_std]

extern crate alloc;

pub mod table;
pub mod renderer;
pub mod style;

#[doc = include_str!("../README.md")]
#[cfg(doc)]
fn readme() {}