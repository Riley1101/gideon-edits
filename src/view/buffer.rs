use super::line::Line;
use super::view::Location;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Default, Debug)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            let line = Line::from(value);
            lines.push(line);
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn delete(&mut self, at: &Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            line.delete(at.grapheme_index);
        };
    }

    pub fn insert_char(&mut self, c: char, at: &Location) {
        if at.line_index > self.lines.len() {
            self.lines.push(Line::from(&c.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(c, at.grapheme_index);
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
}
