use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Command,
};

pub type Result<T> = core::result::Result<T, std::io::Error>;

#[derive(Default)]
pub struct Position {
    pub row: u16,
    pub column: u16,
}

pub struct ScreenSize {
    pub height: u16,
    pub width: u16,
}

impl Position {
    pub const fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }
}
pub fn init() -> Result<()> {
    enable_raw_mode()?;
    clear_screen()
}

pub fn terminate() -> Result<()> {
    disable_raw_mode()
}

pub fn clear_screen() -> Result<()> {
    queue_command(Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line() -> Result<()> {
    queue_command(Clear(ClearType::CurrentLine))?;
    Ok(())
}

pub fn screen_size() -> Result<ScreenSize> {
    let size = crossterm::terminal::size()?;
    Ok(ScreenSize {
        height: size.1,
        width: size.0,
    })
}

pub fn move_carret_to(pos: Position) -> Result<()> {
    queue_command(MoveTo(pos.column, pos.row))?;
    Ok(())
}

pub fn move_carret_begin_of_line(row_number: u16) -> Result<()> {
    move_carret_to(Position::new(row_number, 0))
}

pub fn move_carret_end_of_line(row_number: u16) -> Result<()> {
    let ScreenSize { width, .. } = screen_size()?;
    move_carret_to(Position::new(row_number, width))
}

pub fn move_carret_page_up(column_number: u16) -> Result<()> {
    move_carret_to(Position::new(0, column_number))
}

pub fn move_carret_page_down(column_number: u16) -> Result<()> {
    let ScreenSize { height, .. } = screen_size()?;
    move_carret_to(Position::new(height, column_number))
}

pub fn hide_carret() -> Result<()> {
    queue_command(Hide)?;
    Ok(())
}

pub fn show_carret() -> Result<()> {
    queue_command(Show)?;
    io::stdout().flush()?;
    Ok(())
}

pub fn print(string: &str) -> Result<()> {
    queue_command(Print(string))?;
    Ok(())
}

pub fn queue_command(command: impl Command) -> Result<()> {
    queue!(io::stdout(), command)?;
    Ok(())
}
