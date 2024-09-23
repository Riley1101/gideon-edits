#![allow(clippy::integer_division)]
use super::{buffer, line::Line};
use crate::editor::{
    self,
    editor::DocumentStatus,
    editor_commands::{Direction, EditorCommand},
};
use buffer::Buffer;
use editor::terminal::{Operations, Position, Size, Terminal};
use std::cmp::min;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

#[derive(Debug, Default)]
pub struct View {
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
    margin_bottom: usize,
    text_location: Location,
    scroll_offset: Position,
}

impl View {
    pub fn text_location_to_position(&self) -> Position {
        let y = self.text_location.line_index;
        let x = self.buffer.lines.get(y).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { x, y }
    }

    pub fn cursor_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }
        let Size { width, height } = self.size;

        if width == 0 || height == 0 {
            return;
        }

        let vertical_center = height / 3;
        let top = self.scroll_offset.y;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.need_redraw = false;
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

    pub fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.y {
            self.scroll_offset.y = to;
            true
        } else if to >= self.scroll_offset.x.saturating_add(height) {
            self.scroll_offset.y = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        self.need_redraw = self.need_redraw || offset_changed;
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.x {
            self.scroll_offset.x = to;
            true
        } else if to >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = to.saturating_sub(width).saturating_sub(1);
            true
        } else {
            false
        };
        self.need_redraw = self.need_redraw || offset_changed;
    }

    pub fn scroll_location_into_view(&mut self) {
        let Position { x, y } = self.text_location_to_position();
        self.scroll_vertically(y);
        self.scroll_horizontally(x);
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => {
                self.move_up(1);
            }
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_of_line(),
            Direction::End => self.move_to_end_of_line(),
        }
        self.scroll_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.need_redraw = true;
    }

    pub fn insert_char(&mut self, character: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        self.buffer.insert_char(character, &self.text_location);

        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            self.move_right();
        }
        self.need_redraw = true;
    }

    fn delete_backwards(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.move_text_location(&Direction::Left);
            self.delete();
        }
    }

    fn delete(&mut self) {
        self.buffer.delete(&self.text_location);
        self.need_redraw = true;
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(&self.text_location);
        self.move_text_location(&Direction::Right);
        self.need_redraw = true;
    }

    fn save(&mut self) {
        let _ = self.buffer.save();
    }

    pub fn new(margin_bottom: usize) -> Self {
        let terminal_size = Terminal::size().unwrap_or_default();
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Size {
                width: terminal_size.width,
                height: terminal_size.height.saturating_sub(margin_bottom),
            },
            margin_bottom,
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Save => self.save(),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Insert(char) => self.insert_char(char),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {}
            EditorCommand::Delete => self.delete(),
            EditorCommand::Backspace => self.delete_backwards(),
            EditorCommand::Enter => self.insert_newline(),
        }
    }
}

#[cfg(test)]
mod view_movements_checks {
    use super::Location;
    use super::*;
    #[test]
    fn default_location_check() {
        let view = View::default();
        let default_location = Location::default();
        assert_eq!(&view.text_location, &default_location);
    }

    #[test]
    fn text_location_to_position_check() {
        let view = View::default();
        let position = view.text_location_to_position();
        assert_eq!(position, Position { x: 0, y: 0 });
    }

    #[test]
    fn should_load_buffer_correctly() {
        let mut view = View::default();
        view.load("tests/wisper.txt");
        assert_eq!(view.buffer.height(), 11);
    }

    #[test]
    fn move_up() {
        let mut view = View::default();
        view.load("tests/wisper.txt");
        view.move_up(1);
        assert_eq!(view.text_location.line_index, 0);
    }

    #[test]
    fn move_down() {
        let mut view = View::default();
        view.load("tests/wisper.txt");
        view.move_down(3);
        assert_eq!(view.text_location.line_index, 3);
    }

    #[test]
    fn move_left() {
        let mut view = View::default();
        view.load("tests/wisper.txt");
        view.move_left();
        assert_eq!(view.text_location.grapheme_index, 0);

        view.move_down(1);
        view.move_left();
        assert_eq!(view.text_location.grapheme_index, 25);
    }

    #[test]
    fn move_right() {
        let mut view = View::default();
        view.load("tests/wisper.txt");
        view.move_right();
        view.move_right();
        view.move_right();
        assert_eq!(view.text_location.grapheme_index, 3);

        for _ in 0..24 {
            view.move_right();
        }
        assert_eq!(view.text_location.grapheme_index, 0);
    }

    #[test]
    fn scroll_horizontally() {
        let mut view = View::default();
        view.load("tests/wisper.txt");
        view.resize(Size {
            width: 2,
            height: 2,
        });

        view.scroll_horizontally(5);
        dbg!(view);
    }
}
