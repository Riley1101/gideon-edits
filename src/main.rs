#![warn(clippy::all)]
mod editor;
mod terminal;
mod view;

use editor::Editor;

fn main() {
    Editor::default().run();
}
