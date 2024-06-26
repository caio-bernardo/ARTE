use std::path::Path;

use crate::{
    buffer::Buffer,
    term::{clear_line, move_carret_to, print, screen_size, Position, Result, ScreenSize},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    pub buf: Buffer,
}

impl View {
    pub fn render(&self) -> Result<()> {
        if self.buf.is_empty() {
            View::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }

    fn render_buffer(&self) -> Result<()> {
        let ScreenSize { height, .. } = screen_size()?;
        for row in 0..height {
            move_carret_to(Position::new(row, 0))?;
            clear_line()?;
            // Welcome msg
            if let Some(string) = self.buf.lines.get(row as usize) {
                View::draw_row(string)?;
            } else {
                View::draw_empty_row()?;
            }
        }

        Ok(())
    }

    fn render_welcome_screen() -> Result<()> {
        let ScreenSize { height, .. } = screen_size()?;
        for row in 0..height {
            move_carret_to(Position::new(row, 0))?;
            clear_line()?;
            // Welcome msg
            if row == height / 3 {
                View::draw_welcome_msg()?;
            } else {
                View::draw_empty_row()?;
            }
        }

        Ok(())
    }

    fn draw_welcome_msg() -> Result<()> {
        let width = screen_size()?.width;
        let mensage = format!("{NAME} editor - version {VERSION}");
        let mut welcome_msg = format!("~{:^1$}", mensage, (width - 1) as usize);
        welcome_msg.truncate(width.into());
        print(&welcome_msg)
    }

    fn draw_empty_row() -> Result<()> {
        print("~")
    }

    fn draw_row(string: &str) -> Result<()> {
        print(string)
    }

    // Loads a file into memory in buf
    pub fn load(&mut self, path: &Path) {
        if let Ok(buffer) = Buffer::load(path) {
            self.buf = buffer;
        }
    }
}
