use std::{
    fmt::Display,
    io::{self, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Command,
};

#[derive(Default)]
pub struct Coord {
    y: u16,
    x: u16,
}

pub struct ScreenSize {
    pub height: u16,
    pub width: u16,
}

impl Coord {
    pub const fn new(y: u16, x: u16) -> Self {
        Self { y, x }
    }
}

pub fn clear_screen() -> Result<(), std::io::Error> {
    queue_command(Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line() -> Result<(), std::io::Error> {
    queue_command(Clear(ClearType::CurrentLine))?;
    Ok(())
}

pub fn draw_rows() -> Result<(), std::io::Error> {
    let ScreenSize { height, .. } = screen_size()?;
    for row in 0..height {
        move_cursor_to(Coord::new(row, 0))?;
        clear_line()?;
        print("~\r")?;
    }

    Ok(())
}

pub fn screen_size() -> Result<ScreenSize, std::io::Error> {
    let size = crossterm::terminal::size()?;
    Ok(ScreenSize {
        height: size.1,
        width: size.0,
    })
}

pub fn move_cursor_to(pos: Coord) -> Result<(), std::io::Error> {
    queue_command(MoveTo(pos.x, pos.y))?;
    Ok(())
}

pub fn init() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    clear_screen()?;
    draw_rows()
}

pub fn terminate() -> Result<(), std::io::Error> {
    disable_raw_mode()
}

pub fn hide_cursor() -> Result<(), std::io::Error> {
    queue_command(Hide)?;
    Ok(())
}

pub fn show_cursor() -> Result<(), std::io::Error> {
    queue_command(Show)?;
    io::stdout().flush()?;
    Ok(())
}

pub fn queue_command(command: impl Command) -> Result<(), std::io::Error> {
    queue!(io::stdout(), command)?;
    Ok(())
}

pub fn print(string: impl Display) -> Result<(), std::io::Error> {
    queue_command(Print(string))?;
    Ok(())
}
