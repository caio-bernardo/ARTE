#![warn(clippy::all, clippy::pedantic)]

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};

use crate::term;

#[derive(Default)]
pub struct Editor {
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
        if self.should_quit {
            term::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            term::draw_rows()?;
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event);
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
