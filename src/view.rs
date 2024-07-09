use std::path::Path;

use crate::{
    buffer::Buffer,
    term::{self, clear_line, move_carret_to, print, screen_size, Position, Result, ScreenSize},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: ScreenSize,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buf: Buffer::default(),
            need_redraw: true,
            size: term::screen_size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn render(&mut self) -> Result<()> {
        if !self.need_redraw {
            return Ok(());
        }

        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }

        if self.buf.is_empty() {
            View::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }

        self.need_redraw = false;
        Ok(())
    }

    fn render_buffer(&self) -> Result<()> {
        let ScreenSize { height, width } = screen_size()?;
        for row in 0..height {
            // Welcome msg
            if let Some(string) = self.buf.lines.get(row as usize) {
                let mut string = String::from(string);
                string.truncate(width.into());
                View::draw_row(row, &string)?;
            } else {
                View::draw_empty_row(row)?;
            }
        }

        Ok(())
    }

    fn render_welcome_screen() -> Result<()> {
        let ScreenSize { height, .. } = screen_size()?;
        for row in 0..height {
            // Welcome msg
            if row == height / 3 {
                View::draw_welcome_msg()?;
            } else {
                View::draw_empty_row(row)?;
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

    fn draw_empty_row(at: u16) -> Result<()> {
        move_carret_to(Position::new(at, 0))?;
        clear_line()?;
        print("~")
    }

    fn draw_row(at: u16, text: &str) -> Result<()> {
        move_carret_to(Position::new(at, 0))?;
        clear_line()?;
        print(text)
    }

    // Loads a file into memory in buf
    pub fn load(&mut self, path: &Path) {
        if let Ok(buffer) = Buffer::load(path) {
            self.buf = buffer;
        }
        self.need_redraw = true;
    }

    pub fn resize(&mut self, to: ScreenSize) {
        self.size = to;
        self.need_redraw = true;
    }
}
