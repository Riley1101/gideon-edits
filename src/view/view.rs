#![allow(clippy::integer_division)]
use super::{buffer, line::Line};
use crate::editor::documentstatus::DocumentStatus;
use crate::editor::uicomponent::UIComponent;
use crate::editor::{
    self,
    command::{Edit, Move},
    editor::{NAME, VERSION},
};
use buffer::Buffer;
use editor::terminal::{Operations, Position, Size, Terminal};
use std::{cmp::min, io::Error};

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

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let buffer = Buffer::load(file_name)?;
        self.buffer = buffer;
        self.mark_draw(true);
        Ok(())
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
        self.mark_draw(self.need_redraw || offset_changed);
    }

    pub fn scroll_location_into_view(&mut self) {
        let Position { x, y } = self.text_location_to_position();
        self.scroll_vertically(y);
        self.scroll_horizontally(x);
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
        self.mark_draw(true);
    }

    fn delete_backwards(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.handle_move_command(Move::Left);
            self.delete();
        }
    }

    fn delete(&mut self) {
        self.buffer.delete(&self.text_location);
        self.mark_draw(true);
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(&self.text_location);
        self.handle_move_command(Move::Right);
        self.mark_draw(true);
    }

    fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()
    }

    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(char) => self.insert_char(char),
            Edit::InsertNewLine => self.insert_newline(),
            Edit::Delete => self.delete(),
            Edit::DeleteBackward => self.delete_backwards(),
        }
    }

    pub fn handle_move_command(&mut self, command: Move) {
        match command {
            Move::PageUp => self.move_up(1),
            Move::PageDown => self.move_down(1),
            Move::StartOfLine => self.move_to_start_of_line(),
            Move::EndOfLine => self.move_to_end_of_line(),
            Move::Up => self.move_up(1),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::Down => self.move_down(1),
        }
        self.scroll_location_into_view();
    }

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_index: self.text_location.line_index,
            is_modified: self.buffer.dirty,
            file_name: format!("{}", self.buffer.file_info),
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

impl UIComponent for View {
    fn mark_draw(&mut self, value: bool) {
        self.need_redraw = value;
    }

    fn need_redraws(&self) -> bool {
        self.need_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_location_into_view();
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        let Size { height, width } = self.size;
        let ends_y = origin_y.saturating_add(height);

        let top_third = height / 3;
        let scroll_top = self.scroll_offset.y;

        for current_row in origin_y..ends_y {
            let line_idx = current_row
                .saturating_sub(origin_y)
                .saturating_add(scroll_top);

            if let Some(line) = self.buffer.lines.get(line_idx) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right));
            } else if current_row == top_third && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~")
            }
        }
        Ok(())
    }
}
