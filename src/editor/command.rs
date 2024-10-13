use super::terminal::Size;
use crossterm::event::{
    Event,
    KeyCode::{
        Backspace, Char, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab, Up,
    },
    KeyEvent, KeyModifiers,
};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Move {
    PageUp,
    PageDown,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),

                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                PageUp => Ok(Self::PageUp),
                PageDown => Ok(Self::PageDown),
                _ => Err(format!("Unspported code {code:?}")),
            }
        } else {
            Err(format!(
                "Unspported key code {code:?} or modifiers {modifiers:?}"
            ))
        }
    }
    // add code here
}
