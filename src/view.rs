#![allow(clippy::integer_division)]
use super::terminal::{Operations, Position, Size, Terminal};
mod buffer;
use buffer::Buffer;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.need_redraw {
            return Ok(());
        }
        let Size { width, height } = self.size;
        if width == 0 && height == 0 {
            return Ok(());
        }

        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let trancated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, trancated_line)?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.need_redraw = false;
        Ok(())
    }

    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_cursor_to(Position { x: 0, y: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }
}
