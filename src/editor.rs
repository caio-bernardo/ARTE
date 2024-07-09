use std::env;
use std::panic::{set_hook, take_hook};
use std::path::Path;

use crossterm::cursor::{position, MoveDown, MoveLeft, MoveRight, MoveUp};
use crossterm::event::read;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::term::{
    self, move_carret_begin_of_line, move_carret_end_of_line, move_carret_page_down,
    move_carret_page_up, queue_command, Position, Result,
};
use crate::view::{self, View};

pub struct Editor {
    cursor_position: Position,
    view: view::View,
    should_quit: bool,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = term::terminate();
        if self.should_quit {
            let _ = term::print("Goodbye.");
        }
    }
}

impl Editor {
    pub fn new() -> Result<Self> {
        let curr_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = term::terminate();
            curr_hook(panic_info);
        }));
        term::init()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(Path::new(file_name));
        }
        Ok(Self {
            cursor_position: Position::default(),
            view,
            should_quit: false,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evalute_event(&event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = term::hide_carret();
        self.view.render();
        let _ = term::move_carret_to(self.cursor_position);
        let _ = term::show_carret();
        let _ = term::execute();
    }

    fn evalute_event(&mut self, event: &Event) {
        match *event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Right
                    | KeyCode::Left
                    | KeyCode::Home
                    | KeyCode::End
                    | KeyCode::PageUp
                    | KeyCode::PageDown,
                    _,
                ) => {
                    let _ = self.move_point(code);
                }
                _ => (),
            },
            Event::Resize(width, height) => {
                self.view.resize(term::ScreenSize { height, width });
            }
            _ => (),
        }
    }

    // TODO: remove this, update the carret when changing it
    fn update_carret_position(&mut self) {
        let (col, row) = position().unwrap_or_default();
        self.cursor_position = Position::new(row, col);
    }

    fn move_point(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Up => queue_command(MoveUp(1))?, // TODO: remove this, editor should not
            // access queue_command directy
            KeyCode::Down => queue_command(MoveDown(1))?,
            KeyCode::Right => queue_command(MoveRight(1))?,
            KeyCode::Left => queue_command(MoveLeft(1))?,
            KeyCode::Home => move_carret_begin_of_line(self.cursor_position.row)?,
            KeyCode::End => move_carret_end_of_line(self.cursor_position.row)?,
            KeyCode::PageUp => move_carret_page_up(self.cursor_position.column)?,
            KeyCode::PageDown => move_carret_page_down(self.cursor_position.column)?,
            _ => (),
        }
        self.update_carret_position();
        Ok(())
    }
}
