use crate::terminal::{self, Operations, Position, Size};
use crate::view::View;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
use std::{cmp::min, env, io::Error};
use terminal::Terminal;

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalutate_event(&event)?;
        }
        Ok(())
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    pub fn move_points(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

        match key_code {
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_add(1), x.saturating_add(1));
            }
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            _ => (),
        }
        self.location = Location { x, y };

        Ok(())
    }

    fn evalutate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    Terminal::clear_screen()?;
                    Terminal::show_cursor()?;
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_points(*code)?;
                }

                _ => {
                    let p = Position { x: 1, y: 1 };
                    let _ = Terminal::move_cursor_to(p);
                }
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
