use std::convert::TryFrom;
use super::terminal::Size;

#[derive(Debug)]
pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
pub enum Name {
    Move(Direction),
    Resize(Size),
    Quit,
}
