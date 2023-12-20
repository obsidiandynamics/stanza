use alloc::boxed::Box;

/// A colour palette optimised for use with ANSI in-band terminal signalling. While designed for
/// terminal environments, this palette may be mapped to alternate colour spaces.

/// A 16-colour palette.
#[derive(Debug, Clone)]
pub enum Palette16 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Default,
    Hidden
}