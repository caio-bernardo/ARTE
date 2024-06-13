#![warn(clippy::all, clippy::pedantic)]
use std::io;

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        Self::init().unwrap();
        let res = self.repl();
        Self::terminate().unwrap();
        res.unwrap();
    }

    fn init() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evalute_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evalute_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
}
