use super::{
    documentstatus::DocumentStatus,
    terminal::{Operations, Size, Terminal},
    uicomponent::UIComponent,
};
use std::io::Error;

#[derive(Debug, Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,

    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.mark_draw(true);
        }
    }
}

impl UIComponent for StatusBar {
    fn mark_draw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn need_redraws(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size
    }

    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let beginning = format!(
            "{} - {line_count} {modified_indicator}",
            self.current_status.file_name
        );
        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator}:>{remainder_len}$");
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };
        Terminal::print_inverted_row(origin, &to_print)?;
        Ok(())
    }
}
