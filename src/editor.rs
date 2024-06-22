#![warn(clippy::all)]

use crossterm::cursor::{
    position, MoveDown, MoveLeft, MoveRight, MoveUp, RestorePosition, SavePosition,
};
use crossterm::event::{read, Event::Key};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::term::{
    self, clear_line, move_carret_begin_of_line, move_carret_end_of_line, move_carret_page_down,
    move_carret_page_up, move_carret_to, queue_command, screen_size, Position, Result, ScreenSize,
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    cursor_position: Position,
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        term::init().unwrap();
        let res = self.repl();
        term::terminate().unwrap();
        res.unwrap();
    }

    fn repl(&mut self) -> Result<()> {
        term::move_carret_to(term::Position::new(0, 0))?;
        loop {
            self.update_carret_position()?;
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event)?;
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        queue_command(SavePosition)?;
        term::hide_carret()?;
        if self.should_quit {
            term::clear_screen()?;
            term::move_carret_to(term::Position::new(0, 0))?;
            print!("Goodbye.\r\n");
        } else {
            term::draw_rows()?;
            self.draw_welcome_msg()?;
            queue_command(RestorePosition)?;
        }
        term::show_carret()?;
        Ok(())
    }

    fn draw_welcome_msg(&self) -> Result<()> {
        let ScreenSize { height, width } = screen_size()?;
        move_carret_to(Position::new(height / 3, 0))?;
        clear_line()?;
        let mensage = format!("{NAME} editor - version {VERSION}");
        let mut welcome_msg = format!("~{:^1$}", mensage, (width - 1) as usize);
        welcome_msg.truncate(width.into());
        term::print(welcome_msg)?;
        Ok(())
    }

    fn evalute_event(&mut self, event: &Event) -> Result<()> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up => queue_command(MoveUp(1))?,
                KeyCode::Down => queue_command(MoveDown(1))?,
                KeyCode::Right => queue_command(MoveRight(1))?,
                KeyCode::Left => queue_command(MoveLeft(1))?,
                KeyCode::Home => move_carret_begin_of_line(self.cursor_position.row)?,
                KeyCode::End => move_carret_end_of_line(self.cursor_position.row)?,
                KeyCode::PageUp => move_carret_page_up(self.cursor_position.column)?,
                KeyCode::PageDown => move_carret_page_down(self.cursor_position.column)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn update_carret_position(&mut self) -> Result<()> {
        let (col, row) = position()?;
        self.cursor_position = Position::new(row, col);
        Ok(())
    }
}
