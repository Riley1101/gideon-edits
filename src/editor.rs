use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{:#?}", err);
        }
        println!("{:#?}", err);
    }

    pub fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                print!(" event :?\r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
