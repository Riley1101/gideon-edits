use super::editor_commands;
use super::statusbar::StatusBar;
use super::terminal::{self, Operations};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use editor_commands::EditorCommand;
use std::panic::{set_hook, take_hook};
use std::{env, io::Error};
use terminal::Terminal;

use crate::view::view::View;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: String,
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("{modified}")
        } else {
            String::new()
        }
    }

    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line_index.saturating_add(1),
            self.total_lines
        )
    }
}

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::new(2);
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
            status_bar: StatusBar::new(1),
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evalutate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Cound not read event {err:?}");
                    }
                }
            }
        }
    }

    fn evalutate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                    if let EditorCommand::Resize(size) = command {
                        self.status_bar.resize(size);
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_cursor_to(self.view.cursor_position());
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye!\r\n");
        }
    }
}
