mod buffer;
mod editor;
mod term;
mod view;

fn main() {
    editor::Editor::new().unwrap().run();
}
