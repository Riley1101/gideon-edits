#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::integer_division
)]
mod editor;
mod terminal;
mod view;

use editor::Editor;

fn main() {
    Editor::default().run();
}

