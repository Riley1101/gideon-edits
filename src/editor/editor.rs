use super::editor_commands;
use super::terminal::{self, Operations};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use editor_commands::EditorCommand;
use std::panic::{set_hook, take_hook};
use std::{env, io::Error};
use terminal::Terminal;

use crate::view::view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
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
        Ok(())
    }

    fn evalutate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(0, 1) => true,
            _ => false,
        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    }else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    panic!("Could not handle command {err}")
                }
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        let _ = Terminal::hide_cursor();
        self.view.render()?;
        Ok(())
    }
}
