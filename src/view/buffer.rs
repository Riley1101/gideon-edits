use super::line::Line;
use super::view::Location;
use crate::editor::fileinfo::FileInfo;
use std::fs::{read_to_string, File};
use std::io::Error;
use std::io::Write;

#[derive(Default, Debug)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_info: FileInfo,
    pub dirty: bool,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            let line = Line::from(value);
            lines.push(line);
        }
        Ok(Self {
            lines,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.file_info.path {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn delete(&mut self, at: &Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.lines.len() > at.line_index.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {
                self.lines[at.line_index].delete(at.grapheme_index);
            }
            self.dirty = true;
        };
    }

    pub fn insert_char(&mut self, c: char, at: &Location) {
        if at.line_index > self.lines.len() {
            self.lines.push(Line::from(&c.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(c, at.grapheme_index);
            self.dirty = true;
        }
    }

    pub fn insert_newline(&mut self, at: &Location) {
        if at.line_index == self.height() {
            self.lines.push(Line::default());
        }
    }
}

#[cfg(test)]
mod buffer_checks {
    use super::*;
    #[test]
    fn should_load_correct_lines() {
        let world = "tests/world.txt";
        let buffer_one = Buffer::load(world).unwrap();
        assert_eq!(buffer_one.height(), 1);

        let wisper = "tests/wisper.txt";
        let buffer_two = Buffer::load(wisper).unwrap();
        assert_eq!(buffer_two.height(), 11);

        let dawn = "tests/dawn.txt";
        let buffer_two = Buffer::load(dawn).unwrap();
        assert_eq!(buffer_two.height(), 11);
    }

    #[test]
    fn should_insert_another_line() {
        let dawn = "tests/dawn.txt";
        let mut buffer = Buffer::load(dawn).unwrap();
        buffer.insert_newline(&Location {
            grapheme_index: 0,
            line_index: 11,
        });
        assert_eq!(buffer.height(), 12);
    }
}
