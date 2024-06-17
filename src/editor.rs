use crate::terminal;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                self.refresh_screen()?;
                if self.should_quit {
                    break;
                }
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    Char('c') if modifiers == KeyModifiers::CONTROL => {
                        print!("\x1b[2J");
                    }
                    _ => {
                        println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                    }
                }

                if self.should_quit {
                    break;
                }
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Good bye\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0);
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                println!("\r\n");
            }
        }
        Ok(())
    }
}
