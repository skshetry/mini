#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod term;

fn main() -> std::io::Result<()> {
    let editor = editor::Editor {};
    editor.run()
}
