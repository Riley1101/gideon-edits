use super::editor_commands;
use super::messagebar::MessageBar;
use super::plugins::Plugin;
use super::statusbar::StatusBar;
use super::terminal::{self, Operations, Size};
use super::uicomponent::UIComponent;
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use editor_commands::EditorCommand;
use std::panic::{set_hook, take_hook};
use std::{env, io::Error};
use terminal::Terminal;

use crate::view::view::View;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(dead_code)]
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    terminal_size: Size,
    title: String,
    plugins: Plugin,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let args: Vec<String> = env::args().collect();
        let statusbar_margin = 2;

        let mut editor = Editor::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);

        editor
            .message_bar
            .update_message("HELP: Ctrl-S = save | Ctrl-Q = quit".to_string());

        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }
        editor.refresh_status();
        Ok(editor)
    }

    pub fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
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
            let status = self.view.get_status();
            self.status_bar.update_status(status);
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
                } else if let EditorCommand::Resize(size) = command {
                    self.resize(size);
                } else {
                    self.view.handle_command(command);
                }
            }
        }
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_cursor();
        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        };

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

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
