#[derive(Debug)]
pub struct DoumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: String,
}

impl DoumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("{modified}")
        } else {
            String::new()
        }
    }

    pub fn line_count_to_string(&self) {
        format!("{} lines", self.total_lines)
    }

    pub fn position_indicator_to_string(&self) {
        format!(
            "{}/{}",
            self.current_line_index.saturating_add(1),
            self.total_lines
        )
    }
}
