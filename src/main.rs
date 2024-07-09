mod buffer;
mod editor;
mod term;
mod view;

fn main() {
    editor::Editor::default().run();
}
