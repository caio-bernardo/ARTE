use std::io;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub fn clear_screen() -> Result<(), std::io::Error> {
    execute!(io::stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn draw_rows() -> Result<(), std::io::Error> {
    let screen_size = crossterm::terminal::size()?;

    for row in 0..screen_size.0 {
        execute!(io::stdout(), MoveTo(0, row))?;
        print!("~\r");
    }

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
