#![warn(
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::integer_division
)]

mod editor;
mod view;
use editor::editor::Editor;

fn main() -> Result<(), std::io::Error> {
    match Editor::new() {
        Ok(mut e) => {
            e.run();
        }
        Err(_) => {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not initialize editor",
            ))?;
        }
    };

    Ok(())
}
