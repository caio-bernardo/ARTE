use std::path::Path;

use crate::{
    buffer::Buffer,
    term::{self, ScreenSize},
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
    fn draw_row(at: u16, text: &str) {
        let res = term::print_line(at, text);
        debug_assert!(res.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }

        let ScreenSize { height, width } = self.size;

        if width == 0 || height == 0 {
            return;
        }

        let center = height / 3;
        for row in 0..height {
            // Welcome msg
            if let Some(string) = self.buf.lines.get(row as usize) {
                let mut string = String::from(string);
                string.truncate(width.into());
                View::draw_row(row, &string);
            } else if self.buf.is_empty() && row == center {
                View::draw_row(row, &Self::welcome_msg(width));
            } else {
                View::draw_row(row, "~");
            }
        }

        self.need_redraw = false;
    }

    fn welcome_msg(width: u16) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let mensage = format!("{NAME} editor - version {VERSION}");
        let mut welcome_msg = format!("~{:^1$}", mensage, (width - 1) as usize);
        welcome_msg.truncate(width.into());
        welcome_msg
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
