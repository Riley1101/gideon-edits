#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
use editor::Editor;

fn main() {
    Editor::default().run();
    let somesize = (10, 20);
}
