use std::{
    io::Error,
    time::{Duration, Instant},
};

use super::{
    terminal::{Operations, Size, Terminal},
    uicomponent::UIComponent,
};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

#[derive(Debug)]
struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Debug, Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    clear_after_expiry: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.clear_after_expiry = false;
        self.mark_draw(true);
    }
}

impl UIComponent for MessageBar {
    fn mark_draw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn need_redraws(&self) -> bool {
        (!self.clear_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _size: Size) {}

    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.clear_after_expiry = true;
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin, message)
    }
}
