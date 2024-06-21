#![warn(clippy::all)]

use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveUp, RestorePosition, SavePosition};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};

use crate::term::{self, move_cursor_to, queue_command, screen_size, Coord, ScreenSize};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    // cursor_position: Coord, // TODO: store cursor position
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        term::init().unwrap();
        let res = self.repl();
        term::terminate().unwrap();
        res.unwrap();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        term::hide_cursor()?;
        if self.should_quit {
            term::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            term::draw_rows()?;
            self.draw_welcome_msg()?;
            term::move_cursor_to(term::Coord::new(0, 0))?;
        }
        term::show_cursor()?;
        Ok(())
    }

    fn draw_welcome_msg(&self) -> Result<(), std::io::Error> {
        let ScreenSize { height, width } = screen_size()?;
        move_cursor_to(Coord::new(height / 3, 0))?;
        queue_command(Clear(ClearType::CurrentLine))?;
        let mensage = format!("{NAME} editor - version {VERSION}");
        let mut welcome_msg = format!("~{:^1$}", mensage, (width - 1) as usize);
        welcome_msg.truncate(width.into());
        term::print(welcome_msg)?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event)?;
        }
        Ok(())
    }

    fn evalute_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up => queue_command(MoveUp(1))?,
                KeyCode::Down => queue_command(MoveDown(1))?,
                KeyCode::Right => queue_command(MoveRight(1))?,
                KeyCode::Left => queue_command(MoveLeft(1))?,
                // TODO: Enable more keys
                _ => (),
            }
        }
        Ok(())
    }
}
