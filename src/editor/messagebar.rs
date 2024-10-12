use std::io::Error;

use super::{
    terminal::{Operations, Size, Terminal},
    uicomponent::UIComponent,
};

#[derive(Debug, Default)]
pub struct MessageBar {
    current_message: String,
    needs_redraw: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: String) {
        if new_message != self.current_message {
            self.current_message = new_message;
            self.mark_draw(true);
        }
    }
}

impl UIComponent for MessageBar {
    fn mark_draw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn need_redraws(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, _size: Size) {}

    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        Terminal::print_row(origin, &self.current_message)
    }
}
