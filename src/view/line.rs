use std::{cmp, ops::Range};

#[derive(Debug)]
pub struct Line {
    string: String,
}

impl Line {
    fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }

    fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp.min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}
