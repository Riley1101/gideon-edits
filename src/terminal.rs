use crossterm::cursor::Hide;
use crossterm::cursor::MoveTo;
use crossterm::cursor::Show;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

pub trait Operations {
    fn terminate() -> Result<(), Error>;

    fn initialize() -> Result<(), Error>;

    fn clear_line() -> Result<(), Error>;

    fn hide_cursor() -> Result<(), Error>;

    fn show_cursor() -> Result<(), Error>;

    fn print(string: &str) -> Result<(), Error>;

    fn clear_screen() -> Result<(), Error>;

    fn move_cursor_to(p: Position) -> Result<(), Error>;

    fn size() -> Result<Size, Error>;

    fn execute() -> Result<(), Error>;

    fn queue_command<T: Command>(command: T) -> Result<(), Error>;
}

impl Operations for Terminal {
    fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    fn move_cursor_to(p: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(p.x, p.y))?;
        Ok(())
    }

    fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
