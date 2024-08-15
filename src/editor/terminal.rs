use crossterm::cursor::Hide;
use crossterm::cursor::MoveTo;
use crossterm::cursor::Show;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

#[derive(Debug, Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the cursor out of these bounds, it will also be truncated.
pub struct Terminal;

pub trait Operations {
    fn print_row(row: usize, text: &str) -> Result<(), Error>;

    fn terminate() -> Result<(), Error>;

    fn initialize() -> Result<(), Error>;

    fn clear_line() -> Result<(), Error>;

    fn hide_cursor() -> Result<(), Error>;

    fn show_cursor() -> Result<(), Error>;

    /// Prints the given string to the terminal.
    /// # Arguments
    /// * `string` - the string to print.
    fn print(string: &str) -> Result<(), Error>;

    fn clear_screen() -> Result<(), Error>;

    /// Moves the cursor to the given Position.
    /// # Arguments
    /// * `Position` - the  `Position`to move the cursor to. Will be truncated to `u16::MAX` if bigger.
    fn move_cursor_to(p: Position) -> Result<(), Error>;

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    fn size() -> Result<Size, Error>;

    fn execute() -> Result<(), Error>;

    /// Queues the given command to be executed.
    /// # Arguments
    /// * `T` - the type of command to queue.
    /// * `command` - the command to queue.
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
        let (x_usize, y_usize) = (p.x, p.y);
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(x_usize as u16, y_usize as u16))?;
        Ok(())
    }

    fn size() -> Result<Size, Error> {
        let (width_16, height_16) = size()?;
        Ok(Size {
            width: width_16 as usize,
            height: height_16 as usize,
        })
    }

    fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    fn print_row(row: usize, text: &str) -> Result<(), Error> {
        Self::move_cursor_to(Position { x: 0, y: row })?;
        Self::clear_line()?;
        Self::print(text)?;
        Self::execute()?;
        Ok(())
    }
}
