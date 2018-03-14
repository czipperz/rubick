use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
    White,
    Green,
    Yellow,
    Orange,
}

impl fmt::Debug for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}",
               match self {
                   &Color::Red => "R",
                   &Color::Blue => "B",
                   &Color::White => "W",
                   &Color::Green => "G",
                   &Color::Orange => "O",
                   &Color::Yellow => "Y",
               })
    }
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
