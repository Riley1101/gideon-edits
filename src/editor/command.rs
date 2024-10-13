use super::terminal::Size;
use crossterm::event::{
    Event,
    KeyCode::{
        Backspace, Char, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab, Up,
    },
    KeyEvent, KeyModifiers,
};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone, Copy)]
pub enum Edit {
    Insert(char),
    InsertNewLine,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter, KeyModifiers::NONE) => Ok(Self::InsertNewLine),
            (Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err(format!(
                "Unspported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('q') => Ok(Self::Quit),
                Char('s') => Ok(Self::Save),
                _ => Err(format!(
                    "Unspported key code {:?} with modifiers {:?}",
                    event.code, event.modifiers
                )),
            }
        } else {
            Err(format!(
                "Unspported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            ))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event is not supported {key_event:?}")),
            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            }))),
            _ => Err(format!("Event no supported {event:?}")),
        }
    }
}
